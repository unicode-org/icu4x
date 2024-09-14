// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::Field;
use crate::fields::FieldSymbol;
use crate::pattern::PatternError;
use displaydoc::Display;
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::types::MonthCode;
use icu_provider::DataError;

#[cfg(feature = "std")]
impl std::error::Error for DateTimeError {}

/// A list of error outcomes for various operations in this module.
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
    Skeleton(crate::skeleton::SkeletonError),
    /// An error originating from an unsupported field in a datetime format.
    #[displaydoc("Unsupported field: {0:?}")]
    UnsupportedField(FieldSymbol),
    /// An unsupported field with a field length.
    // TODO(#2856): Consider renaming `UnsupportedField` to `UnsupportedFieldSymbol` so that
    // this variant can be named `UnsupportedField`.
    #[displaydoc("Unsupported field: {0:?}")]
    UnsupportedFormattingField(Field),
    /// An error due to there being no patterns for the given options.
    #[displaydoc("Unsupported options")]
    UnsupportedOptions,
    /// An error originating from a missing weekday symbol in the data.
    #[displaydoc("Data file missing weekday symbol for weekday {0}")]
    MissingWeekdaySymbol(usize),
    /// An error originating from a missing month symbol in the data.
    #[displaydoc("Data file missing month symbol for month code {0}")]
    MissingMonthSymbol(MonthCode),
    /// The FixedDecimalFormatter is not loaded
    #[displaydoc("Missing FixedDecimalFormatter")]
    FixedDecimal,
    /// An error from mixing calendar types in a formatter.
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
    /// The names for the given field are not loaded
    #[displaydoc("Missing names for {0:?}")]
    MissingNames(Field),
    /// The same field occurs multiple times in a pattern or was loaded multiple times
    #[displaydoc("Duplicate field: {0:?}")]
    DuplicateField(Field),
}

/// An error from mixing calendar types in a formatter.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[displaydoc("DateTimeFormatter for {this_kind} calendar was given a {date_kind:?} calendar")]
#[non_exhaustive]
pub struct MismatchedCalendarError {
    /// The calendar kind of the target object (formatter).
    pub this_kind: AnyCalendarKind,
    /// The calendar kind of the input object (date being formatted).
    /// Can be `None` if the input calendar was not specified.
    pub date_kind: Option<AnyCalendarKind>,
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

impl From<MismatchedCalendarError> for DateTimeError {
    fn from(e: MismatchedCalendarError) -> Self {
        DateTimeError::MismatchedAnyCalendar(e.this_kind, e.date_kind)
    }
}
