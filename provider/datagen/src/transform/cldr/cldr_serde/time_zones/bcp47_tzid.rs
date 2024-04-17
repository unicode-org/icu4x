// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-bcp47/bcp47/timezone.json>

use icu_datetime::provider::time_zones::TimeZoneBcp47Id;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct Bcp47TzidAliasData {
    #[serde(rename = "_deprecated")]
    pub(in crate::provider) deprecated: Option<bool>,
    #[serde(rename = "_preferred")]
    pub(in crate::provider) preferred: Option<TimeZoneBcp47Id>,
    #[serde(rename = "_description")]
    pub(in crate::provider) description: String,
    #[serde(rename = "_alias")]
    pub(in crate::provider) alias: Option<String>,
    #[serde(rename = "_since")]
    pub(in crate::provider) since: Option<String>,
    #[serde(rename = "_iana")]
    pub(in crate::provider) iana: Option<String>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Bcp47TimeZoneIds {
    pub(in crate::provider) _alias: String,
    pub(in crate::provider) _description: String,
    #[serde(flatten)]
    pub(in crate::provider) values: BTreeMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct U {
    #[serde(rename = "tz")]
    pub(in crate::provider) time_zones: Bcp47TimeZoneIds,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Keyword {
    pub(in crate::provider) u: U,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) keyword: Keyword,
}
