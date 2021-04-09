// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::FieldSymbol;
use crate::pattern;
use crate::skeleton::SkeletonError;
use icu_provider::prelude::DataError;

/// A list of possible error outcomes for the [`DateTimeFormat`](crate::DateTimeFormat) struct.
///
#[derive(Debug)]
pub enum DateTimeFormatError {
    /// An error coming from a pattern parsing
    Pattern(pattern::Error),
    /// An error originating from fmt::Write trait
    Format(std::fmt::Error),
    /// An error originating inside of the DataProvider
    DataProvider(DataError),
    /// Missing field in date time input
    /// TODO: How can we return which field was missing?
    MissingInputField,
    /// An error from skeleton matching,
    Skeleton(SkeletonError),
    /// Field unsupported for this type of date time format
    UnsupportedField(FieldSymbol),
}

impl From<DataError> for DateTimeFormatError {
    fn from(err: DataError) -> Self {
        Self::DataProvider(err)
    }
}

impl From<pattern::Error> for DateTimeFormatError {
    fn from(err: pattern::Error) -> Self {
        Self::Pattern(err)
    }
}

impl From<std::fmt::Error> for DateTimeFormatError {
    fn from(err: std::fmt::Error) -> Self {
        Self::Format(err)
    }
}
