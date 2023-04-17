// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Error types for decimal formatting.

use displaydoc::Display;

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum DecimalError {
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("error loading data: {0}")]
    Data(icu_provider::DataError),
}

#[cfg(feature = "std")]
impl std::error::Error for DecimalError {}

impl From<icu_provider::DataError> for DecimalError {
    fn from(e: icu_provider::DataError) -> Self {
        DecimalError::Data(e)
    }
}
