// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! The file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/units.json>

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
    /// Retrieves the unique identifier for a given unit name.
    ///
    /// This function searches for the specified unit name within the `convert_units`
    /// and returns its position as a `u16` identifier.
    ///
    /// # Arguments
    ///
    /// * `unit_name` - A string slice that holds the name of the unit to search for.
    ///
    /// # Returns
    ///
    /// * `Ok(u16)` - The unique identifier for the unit if found.
    /// * `Err(DataError)` - An error if the unit is not found or if the index is out of range for `u16`.
    pub fn unit_id(&self, unit_name: &str) -> Result<u16, DataError> {
        self.supplemental
            .convert_units
            .convert_units
            .iter()
            .enumerate()
            .find(|(_, (name, _))| name.as_str() == unit_name)
            .map(|(index, _)| index)
            .ok_or_else(|| DataError::custom("Unit not found"))
            .and_then(|index| {
                u16::try_from(index).map_err(|_| DataError::custom("Index out of range for u16"))
            })
    }

    /// Constructs a map of unit names to their corresponding unique identifiers.
    ///
    /// This function iterates over the units and assigns each a unique identifier
    /// based on its position in the list. The identifiers are represented as `u16`.
    ///
    /// # Errors
    ///
    /// Returns a `DataError` if the index cannot be converted to `u16`.
    pub fn unit_ids_map(&self) -> Result<BTreeMap<String, u16>, DataError> {
        self.supplemental
            .convert_units
            .convert_units
            .iter()
            .enumerate()
            .map(|(index, (unit_name, _))| {
                u16::try_from(index)
                    .map_err(|_| DataError::custom("Index out of range for u16"))
                    .map(|index_u16| (unit_name.clone(), index_u16))
            })
            .collect()
    }
}
