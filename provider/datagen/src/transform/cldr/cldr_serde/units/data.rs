// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-units-full/main/en/units.json>

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Deserialize)]
pub struct UnitsData {
    long: BTreeMap<String, String>,
    short: BTreeMap<String, String>,
    narrow: BTreeMap<String, String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Units {
    pub units: UnitsData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangUnits {
    pub units: Units,
}

// uncomment when LocaleResource is used
// pub type Resource = super::super::LocaleResource<LangUnits>;
