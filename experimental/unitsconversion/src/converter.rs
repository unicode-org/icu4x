// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ConversionError;

/// Represents the possible cases for the convertibility between two units.
pub enum Convertibility {
    Convertible,
    Reciprocal,
    NotConvertible,
}

/// A factory for creating a converter.
pub struct ConverterFactory {}

impl ConverterFactory {
    /// Extract the convertibility from the given units in the form of CLDR identifiers.
    pub fn extract_convertibility(
        &self,
        unit1: &str,
        unit2: &str,
    ) -> Result<Convertibility, ConversionError> {
        todo!();
    }
}
