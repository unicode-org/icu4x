// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::DateTimeError;
use crate::fields::FieldSymbol;
use crate::pattern::PatternError;
use crate::skeleton::SkeletonError;
use alloc::string::String;
use displaydoc::Display;
use icu_plurals::PluralRulesError;
use icu_provider::prelude::DataError;

/// A list of possible error outcomes for the [`DateTimeFormat`](crate::DateTimeFormat) struct.
#[derive(Display, Debug)]
pub enum DateTimeFormatError {
    /// An error originating from parsing a pattern.
    #[displaydoc("{0}")]
    Pattern(PatternError),
    /// An error originating from the [`Write`](std::fmt::Write) trait.
    #[displaydoc("{0}")]
    Format(core::fmt::Error),
    /// An error originating inside of the [`DataProvider`](icu_provider::DataProvider).
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
    MissingMonthSymbol(usize),
    /// An error originating from a missing era symbol in the data.
    #[displaydoc("Data file missing era symbol for era code {0}")]
    MissingEraSymbol(String),
}

#[cfg(feature = "std")]
impl std::error::Error for DateTimeFormatError {}

impl From<PatternError> for DateTimeFormatError {
    fn from(e: PatternError) -> Self {
        DateTimeFormatError::Pattern(e)
    }
}

impl From<DataError> for DateTimeFormatError {
    fn from(e: DataError) -> Self {
        DateTimeFormatError::DataProvider(e)
    }
}

impl From<core::fmt::Error> for DateTimeFormatError {
    fn from(e: core::fmt::Error) -> Self {
        DateTimeFormatError::Format(e)
    }
}

impl From<SkeletonError> for DateTimeFormatError {
    fn from(e: SkeletonError) -> Self {
        DateTimeFormatError::Skeleton(e)
    }
}

impl From<PluralRulesError> for DateTimeFormatError {
    fn from(e: PluralRulesError) -> Self {
        DateTimeFormatError::PluralRules(e)
    }
}

impl From<DateTimeError> for DateTimeFormatError {
    fn from(e: DateTimeError) -> Self {
        DateTimeFormatError::DateTimeInput(e)
    }
}
