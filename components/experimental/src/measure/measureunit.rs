// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use smallvec::SmallVec;

use super::provider::single_unit::SingleUnit;

// TODO NOTE: the MeasureUnitParser takes the trie and the ConverterFactory takes the full payload and an instance of MeasureUnitParser.
/// MeasureUnit is a struct that contains a processed CLDR unit.
///     For example, "meter-per-second".
/// NOTE:
///   - To construct a MeasureUnit from a cldr unit identifier, use the `MeasureUnitParser`.
#[derive(Debug)]
pub struct MeasureUnit {
    /// Contains the processed units.
    pub(crate) single_units: SmallVec<[SingleUnit; 8]>,
}

impl MeasureUnit {
    /// Returns a reference to the single units of the measure unit.
    pub fn get_single_units(&self) -> &SmallVec<[SingleUnit; 8]> {
        &self.single_units
    }
}
