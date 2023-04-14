// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "experimental")]
use crate::rules::reference::parser::ParserError;
use displaydoc::Display;
use icu_provider::DataError;

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum PluralsError {
    /// A parsing error for the plural rules.
    #[cfg(feature = "experimental")]
    #[displaydoc("Parser error: {0}")]
    Parser(ParserError),
    /// An error originating from [`icu_provider`].
    #[displaydoc("Data provider error: {0}")]
    Data(DataError),
}

#[cfg(feature = "std")]
impl std::error::Error for PluralsError {}

#[cfg(feature = "experimental")]
impl From<ParserError> for PluralsError {
    fn from(e: ParserError) -> Self {
        PluralsError::Parser(e)
    }
}

impl From<DataError> for PluralsError {
    fn from(e: DataError) -> Self {
        PluralsError::Data(e)
    }
}
