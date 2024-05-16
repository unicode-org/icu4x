// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! The file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/supplemental/units.json>

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Constant {
    #[serde(rename = "_value")]
    pub(in crate::provider) value: String,

    #[serde(rename = "_status")]
    pub(in crate::provider) status: Option<String>,

    #[serde(rename = "_description")]
    pub(in crate::provider) description: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Quantity {
    #[serde(rename = "_quantity")]
    pub(in crate::provider) quantity: String,

    #[serde(rename = "_status")]
    pub(in crate::provider) status: Option<String>,

    #[serde(rename = "_description")]
    pub(in crate::provider) description: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct ConvertUnit {
    #[serde(rename = "_baseUnit")]
    pub(in crate::provider) base_unit: String,

    #[serde(rename = "_factor")]
    pub(in crate::provider) factor: Option<String>,

    #[serde(rename = "_offset")]
    pub(in crate::provider) offset: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct UnitConstants {
    #[serde(flatten)]
    pub(in crate::provider) constants: BTreeMap<String, Constant>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct UnitQuantities {
    #[serde(flatten)]
    pub(in crate::provider) quantities: BTreeMap<String, Quantity>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct ConvertUnits {
    #[serde(flatten)]
    pub(in crate::provider) convert_units: BTreeMap<String, ConvertUnit>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Supplemental {
    #[serde(rename = "unitConstants")]
    pub(in crate::provider) unit_constants: UnitConstants,

    #[serde(rename = "unitQuantities")]
    pub(in crate::provider) unit_quantities: UnitQuantities,

    #[serde(rename = "convertUnits")]
    pub(in crate::provider) convert_units: ConvertUnits,
}

#[derive(Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) supplemental: Supplemental,
}
