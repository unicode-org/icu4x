// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numberingSystem.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-core/supplemental/numberingSystems.json>

use litemap::LiteMap;
use serde::Deserialize;
use tinystr::{TinyStr8, TinyStrAuto};

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NumberingSystemType {
    Numeric,
    Algorithmic,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct NumberingSystem {
    #[serde(rename = "_type")]
    pub nstype: NumberingSystemType,
    #[serde(rename = "_digits")]
    pub digits: Option<String>,
    #[serde(rename = "_rules")]
    pub rules: Option<TinyStrAuto>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct SupplementalData {
    #[serde(rename = "numberingSystems")]
    pub numbering_systems: LiteMap<TinyStr8, NumberingSystem>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub supplemental: SupplementalData,
}
