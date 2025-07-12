// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON timzone.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/metaZones.json>

use icu::calendar::Date;
use icu::calendar::Iso;
use icu::time::{DateTime, Time};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct UsesMetazone {
    #[serde(rename = "_mzone")]
    pub(crate) mzone: String,
    #[serde(rename = "_from", default, deserialize_with = "deserialize_date")]
    pub(crate) from: Option<DateTime<Iso>>,
    #[serde(rename = "_to", default, deserialize_with = "deserialize_date")]
    pub(crate) to: Option<DateTime<Iso>>,
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
) -> Result<Option<DateTime<Iso>>, D::Error> {
    let Some(from) = Option::<String>::deserialize(deserializer)? else {
        return Ok(None);
    };
    // TODO(#2127): Ideally this parsing can move into a library function
    let mut parts = from.split(' ');
    let date = parts.next().unwrap();
    let time = parts.next().unwrap();
    let mut date_parts = date.split('-');
    let year = date_parts.next().unwrap().parse::<i32>().unwrap();
    let month = date_parts.next().unwrap().parse::<u8>().unwrap();
    let day = date_parts.next().unwrap().parse::<u8>().unwrap();
    let mut time_parts = time.split(':');
    let hour = time_parts.next().unwrap().parse::<u8>().unwrap();
    let minute = time_parts.next().unwrap().parse::<u8>().unwrap();

    Ok(Some(DateTime {
        date: Date::try_new_iso(year, month, day).unwrap(),
        time: Time::try_new(hour, minute, 0, 0).unwrap(),
    }))
}
