// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-bcp47/bcp47/timezone.json>

use icu_datetime::provider::time_zones::TimeZoneBcp47Id;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Bcp47TzidAliasData {
    #[serde(rename = "_deprecated")]
    pub deprecated: Option<bool>,
    #[serde(rename = "_preferred")]
    pub preferred: Option<String>,
    #[serde(rename = "_description")]
    pub description: String,
    #[serde(rename = "_alias")]
    pub alias: Option<String>,
    #[serde(rename = "_since")]
    pub since: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Bcp47TimeZoneIds {
    pub _alias: String,
    pub _description: String,
    #[serde(flatten)]
    pub values: HashMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct U {
    #[serde(rename = "tz")]
    pub time_zones: Bcp47TimeZoneIds,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Keyword {
    pub u: U,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub keyword: Keyword,
}
