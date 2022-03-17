// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_properties::error::PropertiesError;
use icu_provider::prelude::DataError;

#[derive(Display, Debug)]
pub enum NormalizerError {
    #[displaydoc("{0}")]
    DataProvider(DataError),
}

#[cfg(feature = "std")]
impl std::error::Error for NormalizerError {}

impl From<DataError> for NormalizerError {
    fn from(e: DataError) -> Self {
        NormalizerError::DataProvider(e)
    }
}

impl From<PropertiesError> for NormalizerError {
    fn from(e: PropertiesError) -> Self {
        match e {
            PropertiesError::PropDataLoad(d) => NormalizerError::DataProvider(d),
            _ => unreachable!("Shouldn't have non-Data PropertiesError"),
        }
    }
}
