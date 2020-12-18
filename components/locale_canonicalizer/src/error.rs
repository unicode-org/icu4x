// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_provider::prelude::*;

#[derive(Debug)]
pub enum LocaleCanonicalizerError {
    NotMatched,
    ProviderDataError(DataError),
}

impl From<DataError> for LocaleCanonicalizerError {
    fn from(data_error: DataError) -> Self {
        Self::ProviderDataError(data_error)
    }
}
