// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/metaZones.json>

use icu_datetime::provider::time_zones::MetaZoneId;
use litemap::LiteMap;
use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZoneAliasData {
    #[serde(rename = "_longId")]
    pub long_id: String,
    #[serde(rename = "_since")]
    pub since: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZoneIds(pub LiteMap<MetaZoneId, MetaZoneAliasData>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct UsesMetaZone {
    #[serde(rename = "_mzone")]
    pub mzone: String,
    #[serde(rename = "_from")]
    pub from: Option<String>,
    #[serde(rename = "_to")]
    pub to: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZoneForPeriod {
    #[serde(rename = "usesMetazone")]
    pub uses_meta_zone: UsesMetaZone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(clippy::enum_variant_names)]
pub enum MetaLocationOrSubRegion {
    Location(Vec<MetaZoneForPeriod>),
    SubRegion(LiteMap<String, Vec<MetaZoneForPeriod>>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(clippy::enum_variant_names)]
pub enum ZonePeriod {
    Region(Vec<MetaZoneForPeriod>),
    LocationOrSubRegion(LiteMap<String, MetaLocationOrSubRegion>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct TimeZonePeriod(pub LiteMap<String, ZonePeriod>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZoneInfo {
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
pub struct MetaZoneTerritory {
    #[serde(rename = "mapZone")]
    pub map_zone: MapZone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZonesTerritory(pub Vec<MetaZoneTerritory>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct MetaZones {
    #[serde(rename = "metazoneInfo")]
    pub meta_zone_info: MetaZoneInfo,
    #[serde(rename = "metazones")]
    pub meta_zones_territory: MetaZonesTerritory,
    #[serde(rename = "metazoneIds")]
    pub meta_zone_ids: MetaZoneIds,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Supplemental {
    #[serde(rename = "metaZones")]
    pub meta_zones: MetaZones,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
