// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::DateTimeError;
use crate::fields::FieldSymbol;
use crate::pattern::PatternError;
use crate::skeleton::SkeletonError;
use displaydoc::Display;
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::types::MonthCode;
use icu_decimal::FixedDecimalFormatterError;
use icu_plurals::PluralRulesError;
use icu_provider::prelude::DataError;
use tinystr::TinyStr16;

/// A list of possible error outcomes for the [`DateTimeFormatter`](crate::DateTimeFormatter) struct.
#[derive(Display, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum DateTimeFormatterError {
    /// An error originating from parsing a pattern.
    #[displaydoc("{0}")]
    Pattern(PatternError),
    /// An error originating from the [`Write`](std::fmt::Write) trait.
    #[displaydoc("{0}")]
    Format(core::fmt::Error),
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("{0}")]
    DataProvider(DataError),
    /// An error originating from a missing field in datetime input.
    /// TODO: How can we return which field was missing?
    #[displaydoc("Missing input field")]
    MissingInputField,
    /// An error originating from skeleton matching.
    #[displaydoc("{0}")]
    Skeleton(SkeletonError),
    /// An error originating from an unsupported field in a datetime format.
    #[displaydoc("Unsupported field: {0:?}")]
    UnsupportedField(FieldSymbol),
    /// An error due to there being no patterns for the given options.
    #[displaydoc("Unsupported options")]
    UnsupportedOptions,
    /// An error originating from [`PluralRules`][icu_plurals::PluralRules].
    #[displaydoc("{0}")]
    PluralRules(PluralRulesError),
    /// An error originating from [`DateTimeInput`][crate::date::DateTimeInput].
    #[displaydoc("{0}")]
    DateTimeInput(DateTimeError),
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
    FixedDecimalFormatter(FixedDecimalFormatterError),
    /// An error from mixing calendar types in [`AnyDateTimeFormatter`]
    #[displaydoc("AnyDateTimeFormatter for {0} calendar was given a {1:?} calendar")]
    MismatchedAnyCalendar(AnyCalendarKind, Option<AnyCalendarKind>),
    /// An error from mixing calendar types in DateTimeFormat
    #[displaydoc(
        "DateTimeFormatter<{0}> was given a locale asking for incompatible calendar u-ca-{1}"
    )]
    MismatchedCalendarLocale(&'static str, TinyStr16),
}

#[cfg(feature = "std")]
impl std::error::Error for DateTimeFormatterError {}

impl From<PatternError> for DateTimeFormatterError {
    fn from(e: PatternError) -> Self {
        DateTimeFormatterError::Pattern(e)
    }
}

impl From<DataError> for DateTimeFormatterError {
    fn from(e: DataError) -> Self {
        DateTimeFormatterError::DataProvider(e)
    }
}

impl From<core::fmt::Error> for DateTimeFormatterError {
    fn from(e: core::fmt::Error) -> Self {
        DateTimeFormatterError::Format(e)
    }
}

impl From<SkeletonError> for DateTimeFormatterError {
    fn from(e: SkeletonError) -> Self {
        DateTimeFormatterError::Skeleton(e)
    }
}

impl From<PluralRulesError> for DateTimeFormatterError {
    fn from(e: PluralRulesError) -> Self {
        DateTimeFormatterError::PluralRules(e)
    }
}

impl From<DateTimeError> for DateTimeFormatterError {
    fn from(e: DateTimeError) -> Self {
        DateTimeFormatterError::DateTimeInput(e)
    }
}
