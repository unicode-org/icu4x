// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-units-full/main/en/units.json>

use serde::Deserialize;
use std::collections::BTreeMap;
use serde_json::Value;


// TODO: replace Value with specific structs
#[derive(PartialEq, Debug, Deserialize)]
pub struct UnitsData {
    pub long: BTreeMap<String, Value>,

    pub short: BTreeMap<String, Value>,

    pub narrow: BTreeMap<String, Value>,

    #[serde(flatten)]
    extra: Value,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangUnits {
    pub units: UnitsData,
}

pub type Resource = super::super::LocaleResource<LangUnits>;
