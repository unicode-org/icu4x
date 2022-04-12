// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/metaZones.json>

use litemap::LiteMap;
use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZoneAliasData {
    #[serde(rename = "_longId")]
    pub long_id: String,
    #[serde(rename = "_since")]
    pub since: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct MetaZoneIds(pub LiteMap<String, MetaZoneAliasData>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct MetaZones {
    #[serde(rename = "metazoneIds")]
    pub meta_zone_ids: MetaZoneIds,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental {
    #[serde(rename = "metaZones")]
    pub meta_zones: MetaZones,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
