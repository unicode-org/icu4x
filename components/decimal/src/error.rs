// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Error types for decimal formatting.

use displaydoc::Display;

/// A list of possible error outcomes for operations in this crate.
#[derive(Display, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("error loading data: {0}")]
    Data(icu_provider::DataError),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<icu_provider::DataError> for Error {
    fn from(e: icu_provider::DataError) -> Self {
        Error::Data(e)
    }
}
