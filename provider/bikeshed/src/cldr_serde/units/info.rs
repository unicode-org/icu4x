// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! The file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/units.json>

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
