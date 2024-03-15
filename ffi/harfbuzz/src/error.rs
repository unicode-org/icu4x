// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! HarfBuzz-specific error

use displaydoc::Display;
use icu_provider::DataError;

#[derive(Display, Debug)]
pub struct HarfBuzzAllocError;

#[derive(Display, Debug)]
pub enum HarfBuzzAllocOrDataError {
    /// Error coming from the data provider
    #[displaydoc("{0}")]
    Data(DataError),
    /// Allocation failed within HarfBuzz itself
    Alloc,
}

impl From<DataError> for HarfBuzzAllocOrDataError {
    fn from(e: DataError) -> Self {
        HarfBuzzAllocOrDataError::Data(e)
    }
}
