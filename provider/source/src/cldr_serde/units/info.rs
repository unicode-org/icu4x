// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! The file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/units.json>

use icu::experimental::measure::parser::ids::CLDR_IDS_TRIE;
use icu::experimental::measure::provider::single_unit::UnitID;
use icu_provider::DataError;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Constant {
    #[serde(rename = "_value")]
    pub(crate) value: String,

    #[serde(rename = "_status")]
    pub(crate) status: Option<String>,

    #[serde(rename = "_description")]
    pub(crate) description: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Quantity {
    #[serde(rename = "_quantity")]
    pub(crate) quantity: String,

    #[serde(rename = "_status")]
    pub(crate) status: Option<String>,

    #[serde(rename = "_description")]
    pub(crate) description: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct ConvertUnit {
    #[serde(rename = "_baseUnit")]
    pub(crate) base_unit: String,

    #[serde(rename = "_factor")]
    pub(crate) factor: Option<String>,

    #[serde(rename = "_offset")]
    pub(crate) offset: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct UnitConstants {
    #[serde(flatten)]
    pub(crate) constants: BTreeMap<String, Constant>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct UnitQuantities {
    #[serde(flatten)]
    pub(crate) quantities: BTreeMap<String, Quantity>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct ConvertUnits {
    #[serde(flatten)]
    pub(crate) convert_units: BTreeMap<String, ConvertUnit>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "unitConstants")]
    pub(crate) unit_constants: UnitConstants,

    #[serde(rename = "unitQuantities")]
    pub(crate) unit_quantities: UnitQuantities,

    #[serde(rename = "convertUnits")]
    pub(crate) convert_units: ConvertUnits,
}

#[derive(Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}

impl Resource {
    /// Retrieves the unique identifier for a given unit name which is the unit's index in the list.
    ///
    ///
    /// # Returns
    ///
    /// * `Ok(u16)` - The unique identifier for the unit if found.
    /// * `Err(DataError)` - An error if the unit is not found or if the index is out of range for `u16`.
    pub fn unit_id(&self, unit_name: &str) -> Result<UnitID, DataError> {
        CLDR_IDS_TRIE
            .get(unit_name)
            .ok_or_else(|| DataError::custom("Unit not found"))
            .and_then(|value| {
                UnitID::try_from(value).map_err(|_| DataError::custom("Value out of range for u16"))
            })
    }

    /// Constructs a map of unit names to their unique identifiers, which are the unit's indices in the list.
    ///
    /// # Errors
    ///
    /// Returns a `DataError` if the index cannot be converted to `u16`.
    pub fn unit_ids_map(&self) -> Result<BTreeMap<String, UnitID>, DataError> {
        CLDR_IDS_TRIE
            .iter()
            .map(|(unit_name, unit_id)| {
                UnitID::try_from(unit_id)
                    .map(|id| (unit_name.to_string(), id))
                    .map_err(|_| DataError::custom("Value out of range for u16"))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()
    }
}
