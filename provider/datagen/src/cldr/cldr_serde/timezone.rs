// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-bcp47/bcp47/timezone.json>

use litemap::LiteMap;
use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ValueWithDeprecatedDescriptionAlias {
    #[serde(rename = "_deprecated")]
    pub deprecated: bool,
    pub preferred: Option<String>,
    #[serde(rename = "_description")]
    pub description: String,
    #[serde(rename = "_alias")]
    pub alias: String,
    pub since: Option<String>,
}


#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ValueWithDescriptionAliasSince {
    pub deprecated: Option<bool>,
    pub preferred: Option<String>,
    #[serde(rename = "_description")]
    pub description: String,
    #[serde(rename = "_alias")]
    pub alias: String,
    #[serde(rename = "_since")]
    pub since: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ValueWithDeprecatedDescriptionPreferred {
    #[serde(rename = "_deprecated")]
    pub deprecated: bool,
    #[serde(rename = "_preferred")]
    pub preferred: String,
    #[serde(rename = "_description")]
    pub description: String,
    pub alias: Option<String>,
    pub since: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct ValueWithDescriptionAlias {
    pub deprecated: Option<bool>,
    pub preferred: Option<String>,
    #[serde(rename = "_description")]
    pub description: String,
    #[serde(rename = "_alias")]
    pub alias: String,
    pub since: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(clippy::enum_variant_names)]
pub enum TimeZoneValues {
    ValueWithDeprecatedDescriptionAlias(ValueWithDeprecatedDescriptionAlias),
    ValueWithDescriptionAliasSince(ValueWithDescriptionAliasSince),
    ValueWithDescriptionAlias(ValueWithDescriptionAlias),
    ValueWithDeprecatedDescriptionPreferred(ValueWithDeprecatedDescriptionPreferred),
    ValueString(String),
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct TimeZones {
    pub _alias: String,
    pub _description: String,
    #[serde(flatten)]
    pub values: LiteMap<String, TimeZoneValues>
}


#[derive(PartialEq, Debug, Deserialize)]
pub struct U{
    #[serde(rename = "tz")]
    pub time_zones: TimeZones,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Keyword {
    pub u: U
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub keyword: Keyword,
}
