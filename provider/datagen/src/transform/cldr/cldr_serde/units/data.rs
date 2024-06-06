// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON units.json file.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-units-full/main/en/units.json>

use serde::Deserialize;
use std::collections::BTreeMap;

// TODO(youneis): Check if there are more cases.
#[derive(PartialEq, Debug, Deserialize)]
pub struct Patterns {
    #[serde(rename = "unitPattern-count-zero")]
    pub zero: Option<String>,

    #[serde(rename = "unitPattern-count-one")]
    pub one: Option<String>,

    #[serde(rename = "unitPattern-count-two")]
    pub two: Option<String>,

    #[serde(rename = "unitPattern-count-few")]
    pub few: Option<String>,

    #[serde(rename = "unitPattern-count-many")]
    pub many: Option<String>,

    #[serde(rename = "unitPattern-count-other")]
    pub other: Option<String>,
}

// TODO: replace Value with specific structs
#[derive(PartialEq, Debug, Deserialize)]
pub struct UnitsData {
    pub long: BTreeMap<String, Patterns>,

    pub short: BTreeMap<String, Patterns>,

    pub narrow: BTreeMap<String, Patterns>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangUnits {
    pub units: UnitsData,
}

pub type Resource = super::super::LocaleResource<LangUnits>;
