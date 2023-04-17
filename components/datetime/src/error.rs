// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::FieldSymbol;
use crate::input::CalendarError;
use crate::pattern::PatternError;
#[cfg(feature = "experimental")]
use crate::skeleton::SkeletonError;
use displaydoc::Display;
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::types::MonthCode;
use icu_decimal::DecimalError;
use icu_plurals::PluralsError;
use icu_provider::DataError;

#[cfg(feature = "std")]
impl std::error::Error for DateTimeError {}

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum DateTimeError {
    /// An error originating from parsing a pattern.
    #[displaydoc("{0}")]
    Pattern(PatternError),
    /// An error originating from the [`Write`](std::fmt::Write) trait.
    #[displaydoc("{0}")]
    Format(core::fmt::Error),
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("{0}")]
    Data(DataError),
    /// An error originating from a missing field in datetime input.
    /// TODO: How can we return which field was missing?
    #[displaydoc("Missing input field")]
    MissingInputField(Option<&'static str>),
    /// An error originating from skeleton matching.
    #[displaydoc("{0}")]
    #[cfg(feature = "experimental")]
    Skeleton(SkeletonError),
    /// An error originating from an unsupported field in a datetime format.
    #[displaydoc("Unsupported field: {0:?}")]
    UnsupportedField(FieldSymbol),
    /// An error due to there being no patterns for the given options.
    #[displaydoc("Unsupported options")]
    UnsupportedOptions,
    /// An error originating from [`PluralRules`][icu_plurals::PluralRules].
    #[displaydoc("{0}")]
    PluralRules(PluralsError),
    /// An error originating from [`DateTimeInput`][crate::input::DateTimeInput].
    #[displaydoc("{0}")]
    DateTimeInput(CalendarError),
    /// An error originating from a missing weekday symbol in the data.
    #[displaydoc("Data file missing weekday symbol for weekday {0}")]
    MissingWeekdaySymbol(usize),
    /// An error originating from a missing month symbol in the data.
    #[displaydoc("Data file missing month symbol for month code {0}")]
    MissingMonthSymbol(MonthCode),
    /// An error while attempting to format the input as a FixedDecimal
    #[displaydoc("FixedDecimal")]
    FixedDecimal,
    /// An error originating from FixedDecimalFormatter
    #[displaydoc("{0}")]
    FixedDecimalFormatter(DecimalError),
    /// An error from mixing calendar types in [`DateTimeFormatter`](crate::DateTimeFormatter)
    #[displaydoc("DateTimeFormatter for {0} calendar was given a {1:?} calendar")]
    MismatchedAnyCalendar(AnyCalendarKind, Option<AnyCalendarKind>),
    /// Missing date symbols
    #[displaydoc("Missing date symbols")]
    MissingDateSymbols,
    /// Missing time symbols
    #[displaydoc("Missing time symbols")]
    MissingTimeSymbols,
    /// ordinal_rules must be set for PatternPlurals::MultipleVariants
    #[displaydoc("ordinal_rules must be set for PatternPlurals::MultipleVariants")]
    MissingOrdinalRules,
}

impl From<PatternError> for DateTimeError {
    fn from(e: PatternError) -> Self {
        DateTimeError::Pattern(e)
    }
}

impl From<DataError> for DateTimeError {
    fn from(e: DataError) -> Self {
        DateTimeError::Data(e)
    }
}

impl From<core::fmt::Error> for DateTimeError {
    fn from(e: core::fmt::Error) -> Self {
        DateTimeError::Format(e)
    }
}

#[cfg(feature = "experimental")]
impl From<SkeletonError> for DateTimeError {
    fn from(e: SkeletonError) -> Self {
        DateTimeError::Skeleton(e)
    }
}

impl From<PluralsError> for DateTimeError {
    fn from(e: PluralsError) -> Self {
        DateTimeError::PluralRules(e)
    }
}

impl From<CalendarError> for DateTimeError {
    fn from(e: CalendarError) -> Self {
        DateTimeError::DateTimeInput(e)
    }
}
