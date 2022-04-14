// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_provider::DataError;

#[cfg(feature = "std")]
impl std::error::Error for PropertiesError {}

#[derive(Display, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PropertiesError {
    /// An error occurred while loading data
    #[displaydoc("{0}")]
    PropDataLoad(DataError),
    /// An unknown value was used for the [`Script`] property
    #[displaydoc("Unknown script id: {0}")]
    UnknownScriptId(u16),
    /// An unknown value was used for the [`GeneralCategoryGroup`] property
    #[displaydoc("Unknown general category group: {0}")]
    UnknownGeneralCategoryGroup(u32),
}

impl From<DataError> for PropertiesError {
    fn from(e: DataError) -> Self {
        PropertiesError::PropDataLoad(e)
    }
}
