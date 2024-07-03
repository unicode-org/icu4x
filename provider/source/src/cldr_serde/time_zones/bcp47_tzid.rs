// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-bcp47/bcp47/timezone.json>

use icu::datetime::provider::time_zones::TimeZoneBcp47Id;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Bcp47TzidAliasData {
    #[serde(rename = "_deprecated")]
    pub(crate) deprecated: Option<bool>,
    #[serde(rename = "_preferred")]
    pub(crate) preferred: Option<TimeZoneBcp47Id>,
    #[serde(rename = "_description")]
    pub(crate) description: String,
    #[serde(rename = "_alias")]
    pub(crate) alias: Option<String>,
    #[serde(rename = "_since")]
    pub(crate) since: Option<String>,
    #[serde(rename = "_iana")]
    pub(crate) iana: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Bcp47TimeZoneIds {
    pub(crate) _alias: String,
    pub(crate) _description: String,
    #[serde(flatten)]
    pub(crate) values: BTreeMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct U {
    #[serde(rename = "tz")]
    pub(crate) time_zones: Bcp47TimeZoneIds,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Keyword {
    pub(crate) u: U,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) keyword: Keyword,
}
