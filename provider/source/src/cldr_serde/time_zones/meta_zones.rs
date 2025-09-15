// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/metaZones.json>

use crate::time_zones::Timestamp;
use icu::locale::subtags::Region;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct UsesMetazone {
    #[serde(rename = "_mzone")]
    pub(crate) mzone: Option<String>,
    #[serde(rename = "_from", default, deserialize_with = "deserialize_date")]
    pub(crate) from: Option<Timestamp>,
    #[serde(rename = "_to", default, deserialize_with = "deserialize_date")]
    pub(crate) to: Option<Timestamp>,
    #[serde(rename = "_stdOffset")]
    pub(crate) std_offset: Option<String>,
    #[serde(rename = "_dstOffset")]
    pub(crate) dst_offset: Option<String>,
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
    pub(crate) metazone: String,
    #[serde(rename = "_type")]
    pub(crate) time_zone: String,
    #[serde(rename = "_territory")]
    pub(crate) territory: Region,
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
    pub(crate) meta_zones_territory: MetazonesTerritory,
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

impl TimeZonePeriod {
    pub(crate) fn iter(&self) -> impl Iterator<Item = (String, &Vec<MetazoneForPeriod>)> + '_ {
        self.0.iter().flat_map(|(key, zone)| match zone {
            ZonePeriod::Region(periods) => vec![(key.to_string(), periods)],
            ZonePeriod::LocationOrSubRegion(place) => place
                .iter()
                .flat_map(
                    move |(key2, location_or_subregion)| match location_or_subregion {
                        MetaLocationOrSubRegion::Location(periods) => {
                            vec![(format!("{key}/{key2}"), periods)]
                        }
                        MetaLocationOrSubRegion::SubRegion(subregion) => subregion
                            .iter()
                            .flat_map(move |(key3, periods)| {
                                vec![(format!("{key}/{key2}/{key3}"), periods)]
                            })
                            .collect::<Vec<_>>(),
                    },
                )
                .collect::<Vec<_>>(),
        })
    }
}

fn deserialize_date<'de, D: serde::de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Timestamp>, D::Error> {
    use icu::calendar::Iso;
    use icu::time::zone::UtcOffset;
    use icu::time::DateTime;
    use serde::de::Error;

    let Some(timestamp) = Option::<String>::deserialize(deserializer)? else {
        return Ok(None);
    };

    let DateTime { date, time } = DateTime::try_from_str(&timestamp, Iso)
        .map_err(|_| D::Error::custom("Invalid metazone timestamp"))?;

    Ok(Some(Timestamp {
        date,
        time,
        zone: UtcOffset::zero(),
    }))
}
