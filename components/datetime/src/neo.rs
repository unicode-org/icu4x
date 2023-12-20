// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! High-level entrypoints for Neo DateTime Formatter

use crate::format::neo::*;
use crate::input::ExtractedDateTimeInput;
use crate::options::length;
use crate::provider::neo::*;
use crate::raw::neo::*;
use crate::CldrCalendar;
use crate::Error;
use core::marker::PhantomData;
use icu_calendar::week::WeekCalculator;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::prelude::*;

// Re-exports
pub use crate::pattern::neo::CustomDateTimePattern;

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
    names: RawDateTimeNames,
    selection: DateTimePatternSelectionData,
    _calendar: PhantomData<C>,
}

impl<C: CldrCalendar> TypedNeoDateTimeFormatter<C> {
    pub(crate) fn try_new_date_with_length(
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
        let names = RawDateTimeNames::try_new_for_date_selection::<
            C::YearNamesV1Marker,
            C::MonthNamesV1Marker,
        >(
            Some(&crate::provider::Baked),
            Some(&crate::provider::Baked),
            Some(&crate::provider::Baked),
            locale,
            &selection,
            |options| FixedDecimalFormatter::try_new(locale, options),
            || WeekCalculator::try_new(locale),
        )?;
        Ok(Self {
            names,
            selection: DateTimePatternSelectionData::Date(selection),
            _calendar: PhantomData,
        })
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

impl<'a> FormattedNeoDateTime<'a> {
    /// Loads a [`TypedDateTimePattern`] for a date length.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::calendar::DateTime;
    /// use icu::datetime::neo::CustomDateTimePattern;
    /// use icu::datetime::neo::TypedNeoDateTimeFormatter;
    /// use icu::datetime::options::length;
    /// use icu::locid::locale;
    ///
    /// let expected_pattern = TypedDateTimePattern::<Gregorian>::try_from_pattern_str("d MMM y").unwrap();
    /// let actual_pattern = TypedNeoDateTimeFormatter::<Gregorian>::try_new_date_with_length_unstable(
    ///     &icu::datetime::provider::Baked,
    ///     &locale!("fr").into(),
    ///     length::Date::Medium,
    /// ).unwrap().format(DateTime::default()).pattern();
    ///
    /// assert_eq!(expected_pattern, actual_pattern);
    /// ```
    pub fn pattern(&self) -> CustomDateTimePattern {
        todo!()
    }
}
