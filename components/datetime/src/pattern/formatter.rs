// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::names::RawDateTimeNamesBorrowed;
use super::pattern::{DateTimePattern, DateTimePatternBorrowed};
use crate::external_loaders::*;
use crate::fields::{self, Field, FieldLength, FieldSymbol};
use crate::fieldset::dynamic::CompositeDateTimeFieldSet;
use crate::format::datetime::try_write_pattern_items;
use crate::input;
use crate::input::ExtractedInput;
use crate::provider::neo::*;
use crate::provider::pattern::PatternItem;
use crate::provider::time_zones::tz;
use crate::scaffold::*;
use crate::scaffold::{
    AllInputMarkers, DateInputMarkers, DateTimeMarkers, GetField, IsInCalendar, NeoNeverMarker,
    TimeMarkers, TypedDateDataMarkers, ZoneMarkers,
};
use crate::size_test_macro::size_test;
use crate::DateTimeWriteError;
use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU8;
use icu_calendar::types::FormattingEra;
use icu_calendar::types::MonthCode;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::options::GroupingStrategy;
use icu_decimal::provider::{DecimalDigitsV1Marker, DecimalSymbolsV2Marker};
use icu_decimal::FixedDecimalFormatter;
use icu_provider::marker::NeverMarker;
use icu_provider::prelude::*;
use writeable::TryWriteable;
use yoke::Yokeable;

#[derive(Debug, Copy, Clone)]
pub struct DateTimePatternFormatter<'a, C: CldrCalendar, R> {
    inner: RawDateTimePatternFormatter<'a>,
    _calendar: PhantomData<C>,
    _marker: PhantomData<R>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawDateTimePatternFormatter<'a> {
    pattern: DateTimePatternBorrowed<'a>,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a, C: CldrCalendar, R> DateTimePatternFormatter<'a, C, R> {
    pub(crate) fn new(
        pattern: DateTimePatternBorrowed<'a>,
        names: RawDateTimeNamesBorrowed<'a>,
    ) -> Self {
        Self {
            inner: RawDateTimePatternFormatter { pattern, names },
            _calendar: PhantomData,
            _marker: PhantomData,
        }
    }
}

impl<'a, C: CldrCalendar, R: DateTimeMarkers> DateTimePatternFormatter<'a, C, R>
where
    R::D: TypedDateDataMarkers<C> + DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
{
    /// Formats a date and time of day.
    ///
    /// For an example, see [`TypedDateTimeNames`].
    pub fn format<I>(&self, datetime: &I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized + IsInCalendar<C> + AllInputMarkers<R>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<R::D, R::T, R::Z, I>(datetime),
            names: self.inner.names,
        }
    }

    /// Formats a date without a time of day.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// // Create an instance that can format wide month and era names:
    /// let mut names: TypedDateTimeNames<Gregorian> =
    ///     TypedDateTimeNames::try_new(&locale!("en-GB").into()).unwrap();
    /// names
    ///     .include_month_names(fields::Month::Format, FieldLength::Four)
    ///     .unwrap()
    ///     .include_year_names(FieldLength::Four)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The date is:' MMMM d, y GGGG";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Test it with some different dates:
    /// // Note: extended year -50 is year 51 BCE
    /// let date_bce = Date::try_new_gregorian(-50, 3, 15).unwrap();
    /// let date_ce = Date::try_new_gregorian(1700, 11, 20).unwrap();
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_date(&date_bce),
    ///     "The date is: March 15, 51 Before Christ"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_date(&date_ce),
    ///     "The date is: November 20, 1700 Anno Domini"
    /// );
    /// ```
    pub fn format_date<I>(&self, datetime: &'a I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized
            + IsInCalendar<C>
            + GetField<<R::D as DateInputMarkers>::YearInput>
            + GetField<<R::D as DateInputMarkers>::MonthInput>
            + GetField<<R::D as DateInputMarkers>::DayOfMonthInput>
            + GetField<<R::D as DateInputMarkers>::DayOfWeekInput>
            + GetField<<R::D as DateInputMarkers>::DayOfYearInput>
            + GetField<()>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<R::D, NeoNeverMarker, NeoNeverMarker, I>(
                datetime,
            ),
            names: self.inner.names,
        }
    }

    /// Formats a time of day without a date.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::calendar::Time;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// // Create an instance that can format abbreviated day periods:
    /// let mut names: TypedDateTimeNames<Gregorian> =
    ///     TypedDateTimeNames::try_new(&locale!("en-US").into()).unwrap();
    /// names
    ///     .include_day_period_names(FieldLength::Three)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The time is:' h:mm b";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Test it with different times of day:
    /// let time_am = Time::try_new(11, 4, 14, 0).unwrap();
    /// let time_pm = Time::try_new(13, 41, 28, 0).unwrap();
    /// let time_noon = Time::try_new(12, 0, 0, 0).unwrap();
    /// let time_midnight = Time::try_new(0, 0, 0, 0).unwrap();
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_am),
    ///     "The time is: 11:04 AM"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_pm),
    ///     "The time is: 1:41 PM"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_noon),
    ///     "The time is: 12:00 noon"
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_midnight),
    ///     "The time is: 12:00 midnight"
    /// );
    /// ```
    pub fn format_time<I>(&self, datetime: &'a I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized
            + IsInCalendar<C>
            + GetField<<R::T as TimeMarkers>::HourInput>
            + GetField<<R::T as TimeMarkers>::MinuteInput>
            + GetField<<R::T as TimeMarkers>::SecondInput>
            + GetField<<R::T as TimeMarkers>::NanoSecondInput>
            + GetField<()>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<NeoNeverMarker, R::T, NeoNeverMarker, I>(
                datetime,
            ),
            names: self.inner.names,
        }
    }

    /// Formats a timezone without a date or time.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::fieldset::dynamic::ZoneFieldSet;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::IxdtfParser;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut london_winter = IxdtfParser::new().try_from_str(
    ///     "2024-01-01T00:00:00+00:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .to_calendar(Gregorian);
    /// let mut london_summer = IxdtfParser::new().try_from_str(
    ///     "2024-07-01T00:00:00+01:00[Europe/London]",
    /// )
    /// .unwrap()
    /// .to_calendar(Gregorian);
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(
    ///         &locale!("en-GB").into(),
    ///     )
    ///     .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_specific_short_names().unwrap();
    ///
    /// // Create a pattern with symbol `z`:
    /// let pattern_str = "'Your time zone is:' z";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_timezone(&london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names.with_pattern(&pattern).format_timezone(&london_summer),
    ///     "Your time zone is: BST",
    /// );
    /// ```
    pub fn format_timezone<I>(&self, datetime: &'a I) -> FormattedDateTimePattern<'a>
    where
        I: ?Sized
            + IsInCalendar<C>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneIdInput>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneOffsetInput>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneVariantInput>
            + GetField<<R::Z as ZoneMarkers>::TimeZoneLocalTimeInput>
            + GetField<()>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            input: ExtractedInput::extract_from_neo_input::<NeoNeverMarker, NeoNeverMarker, R::Z, I>(
                datetime,
            ),
            names: self.inner.names,
        }
    }
}

/// A pattern that has been interpolated and implements [`TryWriteable`].
#[derive(Debug)]
pub struct FormattedDateTimePattern<'a> {
    pattern: DateTimePatternBorrowed<'a>,
    input: ExtractedInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl TryWriteable for FormattedDateTimePattern<'_> {
    type Error = DateTimeWriteError;
    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        try_write_pattern_items(
            self.pattern.0.as_borrowed().metadata,
            self.pattern.0.as_borrowed().items.iter(),
            &self.input,
            &self.names,
            self.names.fixed_decimal_formatter,
            sink,
        )
    }

    // TODO(#489): Implement writeable_length_hint
}

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::super::*;
    use super::*;
    use icu_calendar::{DateTime, Gregorian};
    use icu_locale_core::locale;
    use writeable::assert_try_writeable_eq;

    #[test]
    fn test_basic_pattern_formatting() {
        let locale = locale!("en").into();
        let mut names: TypedDateTimeNames<Gregorian> =
            TypedDateTimeNames::try_new(&locale).unwrap();
        names
            .load_month_names(
                &crate::provider::Baked,
                fields::Month::Format,
                fields::FieldLength::Four,
            )
            .unwrap()
            .load_weekday_names(
                &crate::provider::Baked,
                fields::Weekday::Format,
                fields::FieldLength::Three,
            )
            .unwrap()
            .load_year_names(&crate::provider::Baked, FieldLength::Five)
            .unwrap()
            .load_day_period_names(&crate::provider::Baked, FieldLength::Three)
            .unwrap();
        let pattern: DateTimePattern = "'It is' E, MMMM d, y GGGGG 'at' hh:mm a'!'"
            .parse()
            .unwrap();
        let datetime = DateTime::try_new_gregorian(2023, 10, 25, 15, 0, 55).unwrap();
        let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

        assert_try_writeable_eq!(
            formatted_pattern,
            "It is Wed, October 25, 2023 A at 03:00 PM!",
            Ok(()),
        );
    }

    #[test]
    fn test_era_coverage() {
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<G>",
                field_length: FieldLength::Three,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GG>",
                field_length: FieldLength::Three,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGG>",
                field_length: FieldLength::Three,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGGG>",
                field_length: FieldLength::Four,
                expected: "<нашої ери>",
            },
            TestCase {
                pattern: "<GGGGG>",
                field_length: FieldLength::Five,
                expected: "<н.е.>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_year_names(&crate::provider::Baked, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }

    #[test]
    fn test_month_coverage() {
        // Ukrainian has different values for format and standalone
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_symbol: fields::Month,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            // 'M' and 'MM' are numeric
            TestCase {
                pattern: "<MMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Three,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<MMMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Four,
                expected: "<листопада>",
            },
            TestCase {
                pattern: "<MMMMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Five,
                expected: "<л>",
            },
            // 'L' and 'LL' are numeric
            TestCase {
                pattern: "<LLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Three,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<LLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Four,
                expected: "<листопад>",
            },
            TestCase {
                pattern: "<LLLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Five,
                expected: "<Л>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_symbol,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_month_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }

    #[test]
    fn test_weekday_coverage() {
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_symbol: fields::Weekday,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<E>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Three,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Three,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Three,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Four,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<EEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Five,
                expected: "<П>",
            },
            TestCase {
                pattern: "<EEEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
            // 'e' and 'ee' are numeric
            TestCase {
                pattern: "<eee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Three,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<eeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Four,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<eeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Five,
                expected: "<П>",
            },
            TestCase {
                pattern: "<eeeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
            // 'c' and 'cc' are numeric
            TestCase {
                pattern: "<ccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Three,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<cccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Four,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<ccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Five,
                expected: "<П>",
            },
            TestCase {
                pattern: "<cccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_symbol,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_weekday_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }

    #[test]
    fn test_dayperiod_coverage() {
        // Thai has different values for different lengths of day periods
        // TODO(#487): Support flexible day periods, too
        let locale = locale!("th").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<a>",
                field_length: FieldLength::Three,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aa>",
                field_length: FieldLength::Three,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaa>",
                field_length: FieldLength::Three,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaaa>",
                field_length: FieldLength::Four,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<aaaaa>",
                field_length: FieldLength::Five,
                expected: "<p>",
            },
            TestCase {
                pattern: "<b>",
                field_length: FieldLength::Three,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bb>",
                field_length: FieldLength::Three,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbb>",
                field_length: FieldLength::Three,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbbb>",
                field_length: FieldLength::Four,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<bbbbb>",
                field_length: FieldLength::Five,
                expected: "<p>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_day_period_names(&crate::provider::Baked, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_try_writeable_eq!(formatted_pattern, expected, Ok(()), "{cas:?}");
        }
    }
}
