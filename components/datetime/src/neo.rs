// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::format::datetime::write_pattern;
use crate::format::neo::*;
use crate::input::DateTimeInput;
use crate::input::DateTimeInputWithWeekConfig;
use crate::input::ExtractedDateTimeInput;
use crate::neo_pattern::DateTimePattern;
use crate::options::length;
use crate::provider::neo::*;
use crate::raw::neo::*;
use crate::CldrCalendar;
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
    inner: RawNeoDateFormatter,
    _calendar: PhantomData<C>,
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

/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3347">#3347</a>
/// </div>
#[derive(Debug)]
pub struct TypedNeoTimeFormatter<C: CldrCalendar> {
    inner: RawNeoTimeFormatter,
    _calendar: PhantomData<C>,
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
    ///         &locale!("fr").into(),
    ///         length::Date::Full,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "mercredi 20 décembre 2023"
    /// );
    /// ```
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
            selection.pattern_for_data_loading(),
            |options| FixedDecimalFormatter::try_new(locale, options),
            || WeekCalculator::try_new(locale),
        )?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Date(selection),
            names,
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
    ///         &locale!("fr").into(),
    ///         length::Time::Medium,
    ///     )
    ///     .unwrap();
    ///
    /// assert_writeable_eq!(
    ///     formatter.format(
    ///         &DateTime::try_new_gregorian_datetime(2023, 12, 20, 14, 48, 58)
    ///             .unwrap()
    ///     ),
    ///     "14:48:58"
    /// );
    /// ```
    pub fn try_new_with_time_length(
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, Error>
    where
        crate::provider::Baked:
            DataProvider<TimePatternV1Marker> + DataProvider<DayPeriodNamesV1Marker>,
    {
        let selection =
            TimePatternSelectionData::try_new_with_length(&crate::provider::Baked, locale, length)?;
        let mut names = RawDateTimeNames::new_without_fixed_decimal_formatter();
        names.load_for_pattern::<C::YearNamesV1Marker, C::MonthNamesV1Marker>(
            None::<&PhantomProvider>,      // year
            None::<&PhantomProvider>,      // month
            None::<&PhantomProvider>,      // weekday
            Some(&crate::provider::Baked), // day period
            locale,
            selection.pattern_for_data_loading(),
            |options| FixedDecimalFormatter::try_new(locale, options),
            || WeekCalculator::try_new(locale),
        )?;
        Ok(Self {
            selection: DateTimePatternSelectionData::Time(selection),
            names,
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
    ///         &locale!("fr").into(),
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
    ///     "14:48:58"
    /// );
    /// ```
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
        let selection = DateTimePatternSelectionData::try_new_with_lengths::<
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
            selection.pattern_for_data_loading(),
            |options| FixedDecimalFormatter::try_new(locale, options),
            || WeekCalculator::try_new(locale),
        )?;
        Ok(Self {
            selection,
            names,
            _calendar: PhantomData,
        })
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
        let loc_datetime =
            DateTimeInputWithWeekConfig::new(&self.datetime, self.names.week_calculator);
        let Some(fixed_decimal_formatter) = self.names.fixed_decimal_formatter else {
            // TODO(#4340): Make the FixedDecimalFormatter optional
            icu_provider::_internal::log::warn!("FixedDecimalFormatter not loaded");
            return Err(core::fmt::Error);
        };
        let pattern = match self.pattern {
            DateTimePatternDataBorrowed::Date(DatePatternDataBorrowed::Resolved(data)) => {
                &data.pattern
            }
            DateTimePatternDataBorrowed::Time(TimePatternDataBorrowed::Resolved(data)) => {
                &data.pattern
            }
            DateTimePatternDataBorrowed::DateTime { .. } => todo!(),
        };
        write_pattern(
            pattern,
            Some(&self.names),
            Some(&self.names),
            &loc_datetime,
            fixed_decimal_formatter,
            sink,
        )
        .map_err(|_e| {
            icu_provider::_internal::log::warn!("{_e:?}");
            core::fmt::Error
        })
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedNeoDateTime<'_>);

impl<'a> FormattedNeoDateTime<'a> {
    /// Gets the pattern used in this [`FormattedNeoDateTime`].
    pub fn pattern(&self) -> DateTimePattern {
        let pattern = match self.pattern {
            DateTimePatternDataBorrowed::Date(DatePatternDataBorrowed::Resolved(data)) => {
                &data.pattern
            }
            DateTimePatternDataBorrowed::Time(TimePatternDataBorrowed::Resolved(data)) => {
                &data.pattern
            }
            DateTimePatternDataBorrowed::DateTime { .. } => todo!(),
        };
        DateTimePattern::from_runtime_pattern(pattern.clone().into_owned())
    }
}
