// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/metaZones.json>

use icu::datetime::provider::time_zones::MetazoneId;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct MetazoneAliasData {
    #[serde(rename = "_longId")]
    pub(crate) long_id: String,
    #[serde(rename = "_since")]
    pub(crate) since: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct MetazoneIds(pub(crate) BTreeMap<MetazoneId, MetazoneAliasData>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct UsesMetazone {
    #[serde(rename = "_mzone")]
    pub(crate) mzone: String,
    #[serde(rename = "_from")]
    pub(crate) from: Option<String>,
    #[serde(rename = "_to")]
    pub(crate) to: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct MetazoneForPeriod {
    #[serde(rename = "usesMetazone")]
    pub(crate) uses_meta_zone: UsesMetazone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum MetaLocationOrSubRegion {
    Location(Vec<MetazoneForPeriod>),
    SubRegion(BTreeMap<String, Vec<MetazoneForPeriod>>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum ZonePeriod {
    Region(Vec<MetazoneForPeriod>),
    LocationOrSubRegion(BTreeMap<String, MetaLocationOrSubRegion>),
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct TimeZonePeriod(pub(crate) BTreeMap<String, ZonePeriod>);

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct MetazoneInfo {
    #[serde(rename = "timezone")]
    pub(crate) time_zone: TimeZonePeriod,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct MapZone {
    #[serde(rename = "_other")]
    pub(crate) other: String,
    #[serde(rename = "_type")]
    pub(crate) zone_type: String,
    #[serde(rename = "_territory")]
    pub(crate) territory: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct MetazoneTerritory {
    #[serde(rename = "mapZone")]
    pub(crate) map_zone: MapZone,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct MetazonesTerritory(pub(crate) Vec<MetazoneTerritory>);

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Metazones {
    #[serde(rename = "metazoneInfo")]
    pub(crate) meta_zone_info: MetazoneInfo,
    #[serde(rename = "metazones")]
    pub(crate) _meta_zones_territory: MetazonesTerritory,
    #[serde(rename = "metazoneIds")]
    pub(crate) meta_zone_ids: MetazoneIds,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "metaZones")]
    pub(crate) meta_zones: Metazones,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
