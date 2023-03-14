// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! HarfBuzz-specific error

use displaydoc::Display;
use icu_normalizer::NormalizerError;
use icu_properties::PropertiesError;

/// HarfBuzz-specific error
#[derive(Display, Debug)]
#[non_exhaustive]
pub enum HarfBuzzError {
    /// Error coming from the normalizer
    #[displaydoc("{0}")]
    Normalizer(NormalizerError),
    /// Error coming from properties
    #[displaydoc("{0}")]
    Properties(PropertiesError),
    /// Allocation failed within HarfBuzz itself
    Alloc,
}

#[cfg(feature = "std")]
impl std::error::Error for HarfBuzzError {}

impl From<PropertiesError> for HarfBuzzError {
    fn from(e: PropertiesError) -> Self {
        HarfBuzzError::Properties(e)
    }
}

impl From<NormalizerError> for HarfBuzzError {
    fn from(e: NormalizerError) -> Self {
        HarfBuzzError::Normalizer(e)
    }
}
