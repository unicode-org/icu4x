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
pub struct Constant {
    #[serde(rename = "_value")]
    pub value: String,

    #[serde(rename = "_status")]
    pub status: Option<String>,

    #[serde(rename = "_description")]
    pub description: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct UnitConstants {
    #[serde(flatten)]
    pub constants: BTreeMap<String, Constant>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental {
    #[serde(rename = "unitConstants")]
    pub unit_constants: UnitConstants,
}

#[derive(Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
