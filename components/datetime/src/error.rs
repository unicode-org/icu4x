// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::pattern;
use icu_data_provider::prelude::DataError;

/// A list of possible error outcomes for the [`DateTimeFormat`] struct.
///
/// [`DateTimeFormat`]: ./struct.DateTimeFormat.html
#[derive(Debug)]
pub enum DateTimeFormatError {
    /// An error coming from a pattern parsing
    Pattern(pattern::Error),
    /// An error originating from fmt::Write trait
    Format(std::fmt::Error),
    /// An error originating inside of the DataProvider
    DataProvider(DataError),
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
