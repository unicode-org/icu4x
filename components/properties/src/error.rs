// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_provider::DataError;
use tinystr::{tinystr, TinyStr16};

#[cfg(doc)]
use crate::GeneralCategoryGroup;
#[cfg(doc)]
use crate::Script;

#[cfg(feature = "std")]
impl std::error::Error for PropertiesError {}

/// A list of error outcomes for various operations in the `icu_properties` crate.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PropertiesError {
    /// An error occurred while loading data
    #[displaydoc("{0}")]
    PropDataLoad(DataError),
    /// An unknown value was used for the [`Script`](crate::Script) property
    #[displaydoc("Unknown script id: {0}")]
    UnknownScriptId(u16),
    /// An unknown value was used for the [`GeneralCategoryGroup`](crate::GeneralCategoryGroup) property
    #[displaydoc("Unknown general category group: {0}")]
    UnknownGeneralCategoryGroup(u32),
    /// An unknown or unexpected property was used for an API dealing with properties at runtime
    #[displaydoc("Unexpected or unknown property {0}")]
    UnexpectedProperty(TinyStr16),
}

impl From<DataError> for PropertiesError {
    fn from(e: DataError) -> Self {
        PropertiesError::PropDataLoad(e)
    }
}

impl PropertiesError {
    pub(crate) fn unexpected_property(name: &str) -> Self {
        let tiny = name
            .get(0..16)
            .and_then(|x| TinyStr16::from_str(x).ok())
            .unwrap_or(tinystr!(16, "invalid"));
        Self::UnexpectedProperty(tiny)
    }
}
