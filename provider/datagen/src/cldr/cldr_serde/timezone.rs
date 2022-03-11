// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-bcp47/bcp47/timezone.json>

use litemap::LiteMap;
use serde::Deserialize;

/*
#[derive(PartialEq, Debug, Deserialize)]
pub struct Alias {
    #[serde(rename = "_alias")]
    pub alias: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct TimeZones{
    pub tz: LiteMap<String, String>,
}


#[derive(PartialEq, Debug, Deserialize)]
pub struct TimeZones{
    pub time_zone: LiteMap<String, Alias>,
}

 */

#[derive(PartialEq, Debug, Deserialize)]
pub struct Alias {
    #[serde(rename = "_alias")]
    pub alias: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct TimeZones (pub LiteMap<String, Alias>);


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
