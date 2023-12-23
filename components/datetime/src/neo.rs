// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::format::neo::*;
use crate::input::ExtractedDateTimeInput;
use crate::input::{DateInput, DateTimeInput, IsoTimeInput};
use crate::neo_pattern::DateTimePattern;
use crate::options::length;
use crate::provider::neo::*;
use crate::raw::neo::*;
use crate::CldrCalendar;
use crate::DateTimeFormatterOptions;
use crate::Error;
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::week::WeekCalculator;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::prelude::*;
use writeable::Writeable;

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoDateFormatter<C: CldrCalendar> {
    selection: DatePatternSelectionData,
    names: RawDateTimeNames,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar> TypedNeoDateFormatter<C> {
    /// Creates a [`TypedNeoDateFormatter`] for a date length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = TypedNeoDateFormatter::<Gregorian>::try_new_with_length(
    ///     &locale!("es-MX").into(),
    ///     length::Date::Full,
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Date::try_new_gregorian_date(2023, 12, 20).unwrap()),
    ///     "miércoles, 20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(locale: &DataLocale, length: length::Date) -> Result<Self, Error>
    where
        crate::provider::Baked: DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>,
    {
        let selection = DatePatternSelectionData::try_new_with_length::<C::DatePatternV1Marker, _>(
            &crate::provider::Baked,
            locale,
            length,
        )?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern::<C::YearNamesV1Marker, C::MonthNamesV1Marker>(
            Some(&crate::provider::Baked), // year
            Some(&crate::provider::Baked), // month
            Some(&crate::provider::Baked), // weekday
            None::<&PhantomProvider>,      // day period
            locale,
            selection.pattern_items_for_data_loading(),
            |options| FixedDecimalFormatter::try_new(locale, options),
            || WeekCalculator::try_new(locale),
        )?;
        Ok(Self {
            selection,
            names,
            _calendar: PhantomData,
        })
    }

    /// Formats a date.
    ///
    /// For an example, see [`TypedNeoDateFormatter`].
    pub fn format<T>(&self, date: &T) -> FormattedNeoDate
    where
        T: DateInput<Calendar = C>,
    {
        let datetime = ExtractedDateTimeInput::extract_from_date(date);
        FormattedNeoDate {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoDate<'a> {
    pattern: DatePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a> Writeable for FormattedNeoDate<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        DateTimeWriter {
            datetime: &self.datetime,
            names: self.names,
            pattern_items: self.pattern.iter_items(),
            pattern_metadata: self.pattern.metadata(),
        }
        .write_to(sink)
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedNeoDate<'_>);

impl<'a> FormattedNeoDate<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct NeoTimeFormatter {
    selection: TimePatternSelectionData,
    names: RawDateTimeNames,
}

impl NeoTimeFormatter {
    /// Creates a [`NeoTimeFormatter`] for a time length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::types::Time;
    /// use icu::datetime::neo::NeoTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = NeoTimeFormatter::try_new_with_length(
    ///     &locale!("es-MX").into(),
    ///     length::Time::Medium,
    /// )
    /// .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(&Time::try_new(14, 48, 58, 0).unwrap()),
    ///     "2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(locale: &DataLocale, length: length::Time) -> Result<Self, Error>
    where
        crate::provider::Baked:
            DataProvider<TimePatternV1Marker> + DataProvider<DayPeriodNamesV1Marker>,
    {
        let selection =
            TimePatternSelectionData::try_new_with_length(&crate::provider::Baked, locale, length)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        // NOTE: The Gregorian types below are placeholders only. They are not actually linked.
        names.load_for_pattern::<GregorianYearNamesV1Marker, GregorianMonthNamesV1Marker>(
            None::<&PhantomProvider>,      // year
            None::<&PhantomProvider>,      // month
            None::<&PhantomProvider>,      // weekday
            Some(&crate::provider::Baked), // day period
            locale,
            selection.pattern_items_for_data_loading(),
            |options| FixedDecimalFormatter::try_new(locale, options),
            || WeekCalculator::try_new(locale),
        )?;
        Ok(Self { selection, names })
    }

    /// Formats a time of day.
    ///
    /// For an example, see [`TypedNeoTimeFormatter`].
    pub fn format<T>(&self, time: &T) -> FormattedNeoTime
    where
        T: IsoTimeInput,
    {
        let datetime = ExtractedDateTimeInput::extract_from_time(time);
        FormattedNeoTime {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoTime<'a> {
    pattern: TimePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a> Writeable for FormattedNeoTime<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        DateTimeWriter {
            datetime: &self.datetime,
            names: self.names,
            pattern_items: self.pattern.iter_items(),
            pattern_metadata: self.pattern.metadata(),
        }
        .write_to(sink)
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedNeoTime<'_>);

impl<'a> FormattedNeoTime<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoDateTimeFormatter<C: CldrCalendar> {
    selection: DateTimePatternSelectionData,
    names: RawDateTimeNames,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar> TypedNeoDateTimeFormatter<C> {
    /// Creates a [`TypedNeoDateTimeFormatter`] for a date length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_date_length(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "miércoles, 20 de diciembre de 2023"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_date_length(
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, Error>
    where
        crate::provider::Baked: DataProvider<C::DatePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>,
    {
        let date_formatter = TypedNeoDateFormatter::<C>::try_new_with_length(locale, length)?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Date(date_formatter.selection),
            names: date_formatter.names,
            _calendar: PhantomData,
        })
    }

    /// Creates a [`TypedNeoDateTimeFormatter`] for a time length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_time_length(
    ///         &locale!("es-MX").into(),
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_time_length(
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, Error>
    where
        crate::provider::Baked:
            DataProvider<TimePatternV1Marker> + DataProvider<DayPeriodNamesV1Marker>,
    {
        let time_formatter = NeoTimeFormatter::try_new_with_length(locale, length)?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Time(time_formatter.selection),
            names: time_formatter.names,
            _calendar: PhantomData,
        })
    }

    /// Creates a [`TypedNeoDateTimeFormatter`] for date and time lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo::TypedNeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter =
    ///     TypedNeoDateTimeFormatter::<Gregorian>::try_new_with_lengths(
    ///         &locale!("es-MX").into(),
    ///         length::Date::Full,
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "miércoles, 20 de diciembre de 2023, 2:48:58 p.m."
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_lengths(
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, Error>
    where
        crate::provider::Baked: DataProvider<C::DatePatternV1Marker>
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DateTimePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
    {
        let selection = DateTimeGluePatternSelectionData::try_new_with_lengths::<
            C::DatePatternV1Marker,
            _,
        >(&crate::provider::Baked, locale, date_length, time_length)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern::<C::YearNamesV1Marker, C::MonthNamesV1Marker>(
            Some(&crate::provider::Baked), // year
            Some(&crate::provider::Baked), // month
            Some(&crate::provider::Baked), // weekday
            Some(&crate::provider::Baked), // day period
            locale,
            selection.pattern_items_for_data_loading(),
            |options| FixedDecimalFormatter::try_new(locale, options),
            || WeekCalculator::try_new(locale),
        )?;
        Ok(Self {
            selection: DateTimePatternSelectionData::DateTimeGlue(selection),
            names,
            _calendar: PhantomData,
        })
    }

    /// Creates a [`TypedNeoDateTimeFormatter`] from [`DateTimeFormatterOptions`].
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale, options: DateTimeFormatterOptions) -> Result<Self, Error>
    where
        crate::provider::Baked: DataProvider<C::DatePatternV1Marker>
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DateTimePatternV1Marker>
            + DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
    {
        match options {
            DateTimeFormatterOptions::Length(length::Bag {
                date: Some(date),
                time: Some(time),
            }) => Self::try_new_with_lengths(locale, date, time),
            DateTimeFormatterOptions::Length(length::Bag {
                date: Some(date),
                time: None,
            }) => Self::try_new_with_date_length(locale, date),
            DateTimeFormatterOptions::Length(length::Bag {
                date: None,
                time: Some(time),
            }) => Self::try_new_with_time_length(locale, time),
            _ => Err(Error::UnsupportedOptions),
        }
    }

    /// Formats a date and time of day.
    ///
    /// For an example, see [`TypedNeoDateTimeFormatter`].
    pub fn format<T>(&self, datetime: &T) -> FormattedNeoDateTime
    where
        T: DateTimeInput<Calendar = C>,
    {
        let datetime = ExtractedDateTimeInput::extract_from(datetime);
        FormattedNeoDateTime {
            pattern: self.selection.select(&datetime),
            datetime,
            names: self.names.as_borrowed(),
        }
    }
}

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct FormattedNeoDateTime<'a> {
    pattern: DateTimePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a> Writeable for FormattedNeoDateTime<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        DateTimeWriter {
            datetime: &self.datetime,
            names: self.names,
            pattern_items: self.pattern.iter_items(),
            pattern_metadata: self.pattern.metadata(),
        }
        .write_to(sink)
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedNeoDateTime<'_>);

impl<'a> FormattedNeoDateTime<'a> {
    /// Gets the pattern used in this formatted value.
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}
