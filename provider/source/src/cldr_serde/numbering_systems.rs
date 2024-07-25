// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numberingSystem.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-core/supplemental/numberingSystems.json>

use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum NumberingSystemType {
    Numeric,
    Algorithmic,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct NumberingSystem {
    #[serde(rename = "_type")]
    pub(crate) nstype: NumberingSystemType,
    #[serde(rename = "_digits")]
    pub(crate) digits: Option<String>,
    #[serde(rename = "_rules")]
    pub(crate) rules: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct SupplementalData {
    #[serde(rename = "numberingSystems")]
    pub(crate) numbering_systems: HashMap<String, NumberingSystem>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: SupplementalData,
}
