// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Error types for list formatting.

use alloc::string::String;
use displaydoc::Display;
use icu_provider::DataError;

#[derive(Display, Debug)]
pub enum Error {
    #[displaydoc("error loading data: {0}")]
    Data(icu_provider::DataError),
    #[displaydoc("Illegal pattern: {0}")]
    IllegalPattern(String),
    #[cfg(any(
        feature = "provider_transform_internals",
        feature = "icu4x_human_readable_de"
    ))]
    #[displaydoc("Illegal condition: {0}")]
    IllegalCondition(regex_automata::dfa::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<DataError> for Error {
    fn from(e: icu_provider::DataError) -> Self {
        Error::Data(e)
    }
}
