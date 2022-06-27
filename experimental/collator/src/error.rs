// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collator-specific error

use displaydoc::Display;
use icu_properties::PropertiesError;
use icu_provider::prelude::DataError;

/// Error returned by the constructor of `Collator`
#[derive(Display, Debug)]
pub enum CollatorError {
    /// The requested collation does not exist
    NotFound,
    /// Requested data was found but was malformed
    MalformedData,
    /// An error originating inside of the data provider.
    #[displaydoc("{0}")]
    DataProvider(DataError),
}

#[cfg(feature = "std")]
impl std::error::Error for CollatorError {}

impl From<DataError> for CollatorError {
    fn from(e: DataError) -> Self {
        CollatorError::DataProvider(e)
    }
}

impl From<PropertiesError> for CollatorError {
    fn from(e: PropertiesError) -> Self {
        match e {
            PropertiesError::PropDataLoad(d) => CollatorError::DataProvider(d),
            _ => unreachable!("Shouldn't have non-Data PropertiesError"),
        }
    }
}
