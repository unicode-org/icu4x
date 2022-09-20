// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/metaZones.json>

use icu_datetime::provider::time_zones::MetazoneId;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetazoneAliasData {
    #[serde(rename = "_longId")]
    pub long_id: String,
    #[serde(rename = "_since")]
    pub since: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetazoneIds(pub HashMap<MetazoneId, MetazoneAliasData>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct UsesMetazone {
    #[serde(rename = "_mzone")]
    pub mzone: String,
    #[serde(rename = "_from")]
    pub from: Option<String>,
    #[serde(rename = "_to")]
    pub to: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetazoneForPeriod {
    #[serde(rename = "usesMetazone")]
    pub uses_meta_zone: UsesMetazone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MetaLocationOrSubRegion {
    Location(Vec<MetazoneForPeriod>),
    SubRegion(HashMap<String, Vec<MetazoneForPeriod>>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ZonePeriod {
    Region(Vec<MetazoneForPeriod>),
    LocationOrSubRegion(HashMap<String, MetaLocationOrSubRegion>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct TimeZonePeriod(pub HashMap<String, ZonePeriod>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetazoneInfo {
    #[serde(rename = "timezone")]
    pub time_zone: TimeZonePeriod,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MapZone {
    #[serde(rename = "_other")]
    pub other: String,
    #[serde(rename = "_type")]
    pub zone_type: String,
    #[serde(rename = "_territory")]
    pub territory: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetazoneTerritory {
    #[serde(rename = "mapZone")]
    pub map_zone: MapZone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetazonesTerritory(pub Vec<MetazoneTerritory>);

#[derive(Debug, Clone, Deserialize)]
pub struct Metazones {
    #[serde(rename = "metazoneInfo")]
    pub meta_zone_info: MetazoneInfo,
    #[serde(rename = "metazones")]
    pub meta_zones_territory: MetazonesTerritory,
    #[serde(rename = "metazoneIds")]
    pub meta_zone_ids: MetazoneIds,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Supplemental {
    #[serde(rename = "metaZones")]
    pub meta_zones: Metazones,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
