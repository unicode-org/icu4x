// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_provider::DataError;

#[derive(Display, Debug, Copy, Clone)]
/// UnexpectedPropertyName
#[allow(clippy::exhaustive_structs)]
pub struct UnexpectedPropertyNameError;

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq)]
/// UnexpectedPropertyName or DataError
#[allow(clippy::exhaustive_enums)]
pub enum UnexpectedPropertyNameOrDataError {
    /// Unexpected property name
    UnexpectedPropertyName,
    /// Data error
    #[displaydoc("DataError({0})")]
    Data(DataError),
}

impl From<DataError> for UnexpectedPropertyNameOrDataError {
    fn from(value: DataError) -> Self {
        Self::Data(value)
    }
}
