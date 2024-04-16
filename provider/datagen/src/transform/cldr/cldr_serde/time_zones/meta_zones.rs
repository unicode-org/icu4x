// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/metaZones.json>

use icu_datetime::provider::time_zones::MetazoneId;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct MetazoneAliasData {
    #[serde(rename = "_longId")]
    pub(in crate::provider) long_id: String,
    #[serde(rename = "_since")]
    pub(in crate::provider) since: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(in crate::provider) struct MetazoneIds(
    pub(in crate::provider) BTreeMap<MetazoneId, MetazoneAliasData>,
);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct UsesMetazone {
    #[serde(rename = "_mzone")]
    pub(in crate::provider) mzone: String,
    #[serde(rename = "_from")]
    pub(in crate::provider) from: Option<String>,
    #[serde(rename = "_to")]
    pub(in crate::provider) to: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct MetazoneForPeriod {
    #[serde(rename = "usesMetazone")]
    pub(in crate::provider) uses_meta_zone: UsesMetazone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(in crate::provider) enum MetaLocationOrSubRegion {
    Location(Vec<MetazoneForPeriod>),
    SubRegion(BTreeMap<String, Vec<MetazoneForPeriod>>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(in crate::provider) enum ZonePeriod {
    Region(Vec<MetazoneForPeriod>),
    LocationOrSubRegion(BTreeMap<String, MetaLocationOrSubRegion>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct TimeZonePeriod(pub(in crate::provider) BTreeMap<String, ZonePeriod>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct MetazoneInfo {
    #[serde(rename = "timezone")]
    pub(in crate::provider) time_zone: TimeZonePeriod,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct MapZone {
    #[serde(rename = "_other")]
    pub(in crate::provider) other: String,
    #[serde(rename = "_type")]
    pub(in crate::provider) zone_type: String,
    #[serde(rename = "_territory")]
    pub(in crate::provider) territory: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct MetazoneTerritory {
    #[serde(rename = "mapZone")]
    pub(in crate::provider) map_zone: MapZone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(in crate::provider) struct MetazonesTerritory(pub(in crate::provider) Vec<MetazoneTerritory>);

#[derive(Debug, Clone, Deserialize)]
pub(in crate::provider) struct Metazones {
    #[serde(rename = "metazoneInfo")]
    pub(in crate::provider) meta_zone_info: MetazoneInfo,
    #[serde(rename = "metazones")]
    pub(in crate::provider) _meta_zones_territory: MetazonesTerritory,
    #[serde(rename = "metazoneIds")]
    pub(in crate::provider) meta_zone_ids: MetazoneIds,
}

#[derive(Debug, Clone, Deserialize)]
pub(in crate::provider) struct Supplemental {
    #[serde(rename = "metaZones")]
    pub(in crate::provider) meta_zones: Metazones,
}

#[derive(Debug, Clone, Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) supplemental: Supplemental,
}
