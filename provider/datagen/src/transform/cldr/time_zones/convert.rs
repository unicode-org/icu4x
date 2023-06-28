// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::CldrTimeZonesData;
use crate::transform::cldr::cldr_serde;
use cldr_serde::time_zones::bcp47_tzid::Bcp47TzidAliasData;
use cldr_serde::time_zones::meta_zones::MetaLocationOrSubRegion;
use cldr_serde::time_zones::meta_zones::MetazoneAliasData;
use cldr_serde::time_zones::meta_zones::MetazoneForPeriod;
use cldr_serde::time_zones::meta_zones::ZonePeriod;
use cldr_serde::time_zones::time_zone_names::*;
use icu_calendar::DateTime;
use icu_datetime::provider::time_zones::{
    ExemplarCitiesV1, MetazoneGenericNamesLongV1, MetazoneGenericNamesShortV1, MetazoneId,
    MetazoneSpecificNamesLongV1, MetazoneSpecificNamesShortV1, TimeZoneBcp47Id, TimeZoneFormatsV1,
};
use icu_timezone::provider::MetazonePeriodV1;
use icu_timezone::ZoneVariant;
use std::borrow::Cow;
use std::collections::HashMap;
use tinystr::TinyStr8;

/// Performs part 1 of type fallback as specified in the UTS-35 spec for TimeZone Goals:
/// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Goals
///
/// Part 2 of type fallback requires access to the IANA TimeZone Database
/// as well as a specific datetime context, so it is not relevant to DataProvier.
fn type_fallback(zone_format: &ZoneFormat) -> Option<&String> {
    zone_format
        .0
        .get("generic")
        .or_else(|| zone_format.0.get("standard"))
}

fn parse_hour_format(hour_format: &str) -> (Cow<'static, str>, Cow<'static, str>) {
    // e.g. "+HH:mm;-HH:mm" -> ("+HH:mm", "-HH:mm")
    let index = hour_format.rfind(';').unwrap();
    let positive = hour_format[0..index].to_owned();
    let negative = hour_format[index + 1..].to_owned();
    (Cow::Owned(positive), Cow::Owned(negative))
}

fn compute_bcp47_tzids_hashmap(
    bcp47_tzids_resource: &HashMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
) -> HashMap<String, TimeZoneBcp47Id> {
    let mut bcp47_tzids = HashMap::new();
    for (bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource.iter() {
        if let Some(alias) = &bcp47_tzid_data.alias {
            for data_value in alias.split(' ') {
                bcp47_tzids.insert(data_value.to_string(), *bcp47_tzid);
            }
        }
    }
    bcp47_tzids
}

fn compute_meta_zone_ids_hashmap(
    meta_zone_ids_resource: &HashMap<MetazoneId, MetazoneAliasData>,
) -> HashMap<String, MetazoneId> {
    let mut meta_zone_ids = HashMap::new();
    for (meta_zone_id, meta_zone_id_data) in meta_zone_ids_resource.iter() {
        meta_zone_ids.insert(meta_zone_id_data.long_id.to_string(), *meta_zone_id);
    }
    meta_zone_ids
}

impl From<CldrTimeZonesData<'_>> for TimeZoneFormatsV1<'static> {
    fn from(other: CldrTimeZonesData<'_>) -> Self {
        let data = other.time_zone_names_resource;
        Self {
            hour_format: parse_hour_format(&data.hour_format),
            gmt_format: data.gmt_format.clone().into(),
            gmt_zero_format: data.gmt_zero_format.clone().into(),
            region_format: data.region_format.clone().into(),
            region_format_variants: data
                .region_format_variants
                .iter()
                .map(|(key, value)| {
                    (
                        key.parse::<TinyStr8>()
                            .expect("Time-zone variant was not compatible with TinyStr8"),
                        value.clone(),
                    )
                })
                .collect(),
            fallback_format: data.fallback_format.clone().into(),
            // TODO(#2256): Have a better timezone offset_fallback.
            gmt_offset_fallback: "GMT+?".to_string().into(),
        }
    }
}

impl Location {
    fn exemplar_city(&self) -> Option<String> {
        match self {
            Self::City(place) => Some(place.exemplar_city.clone()),
            Self::Long(place) => place.exemplar_city.clone(),
            Self::Short(place) => place.exemplar_city.clone(),
        }
    }

    fn long_metazone_names(&self) -> Option<ZoneFormat> {
        match self {
            Self::City(place) => place.long.clone(),
            Self::Long(place) => Some(place.long.clone()),
            Self::Short(place) => place.long.clone(),
        }
    }

    fn short_metazone_names(&self) -> Option<ZoneFormat> {
        match self {
            Self::City(place) => place.short.clone(),
            Self::Long(place) => place.short.clone(),
            Self::Short(place) => Some(place.short.clone()),
        }
    }
}

impl From<CldrTimeZonesData<'_>> for ExemplarCitiesV1<'static> {
    fn from(other: CldrTimeZonesData<'_>) -> Self {
        Self(
            other
                .bcp47_tzids_resource
                .iter()
                .filter_map(|(bcp47, bcp47_tzid_data)| {
                    bcp47_tzid_data
                        .alias
                        .as_ref()
                        .map(|aliases| (bcp47, aliases))
                })
                // Montreal is meant to be deprecated, but pre-43 the deprecation
                // fallback was not set, which is why it might show up here.
                .filter(|(bcp47, _)| bcp47.0 != "camtr")
                .filter_map(|(bcp47, aliases)| {
                    let alias = aliases.split(' ').next().expect("split non-empty");
                    let mut alias_parts = alias.split('/');
                    let continent = alias_parts.next().expect("split non-empty");
                    let location_or_subregion = alias_parts.next()?;
                    let location_in_subregion = alias_parts.next();

                    Some((
                        bcp47,
                        other
                            .time_zone_names_resource
                            .zone
                            .0
                            .get(continent)
                            .and_then(|x| x.0.get(location_or_subregion))
                            .and_then(|x| match x {
                                LocationOrSubRegion::Location(place) => Some(place),
                                LocationOrSubRegion::SubRegion(region) => {
                                    region.get(location_in_subregion?)
                                }
                            })
                            .and_then(|p| p.exemplar_city())
                            .or_else(|| {
                                (continent != "Etc").then(|| {
                                    alias
                                        .split('/')
                                        .next_back()
                                        .expect("split non-empty")
                                        .replace('_', " ")
                                })
                            })?,
                    ))
                })
                .collect(),
        )
    }
}

impl From<CldrTimeZonesData<'_>> for MetazonePeriodV1<'static> {
    fn from(other: CldrTimeZonesData<'_>) -> Self {
        let data = other.meta_zone_periods_resource;
        let bcp47_tzid_data = &compute_bcp47_tzids_hashmap(other.bcp47_tzids_resource);
        let meta_zone_id_data = &compute_meta_zone_ids_hashmap(other.meta_zone_ids_resource);
        Self(
            data.iter()
                .flat_map(|(key, zone)| {
                    let key = key;
                    match zone {
                        ZonePeriod::Region(periods) => match bcp47_tzid_data.get(key) {
                            Some(bcp47) => {
                                vec![(*bcp47, periods.clone(), meta_zone_id_data.clone())]
                            }
                            None => panic!("Cannot find bcp47 for {key:?}."),
                        },
                        ZonePeriod::LocationOrSubRegion(place) => place
                            .iter()
                            .flat_map(move |(inner_key, location_or_subregion)| {
                                let mut key = key.clone();
                                key.push('/');
                                key.push_str(inner_key);
                                match location_or_subregion {
                                    MetaLocationOrSubRegion::Location(periods) => {
                                        match bcp47_tzid_data.get(&key) {
                                            Some(bcp47) => {
                                                vec![(
                                                    *bcp47,
                                                    periods.clone(),
                                                    meta_zone_id_data.clone(),
                                                )]
                                            }
                                            None => panic!("Cannot find bcp47 for {key:?}."),
                                        }
                                    }
                                    MetaLocationOrSubRegion::SubRegion(subregion) => subregion
                                        .iter()
                                        .flat_map(move |(inner_inner_key, periods)| {
                                            let mut key = key.clone();
                                            key.push('/');
                                            key.push_str(inner_inner_key);
                                            match bcp47_tzid_data.get(&key) {
                                                Some(bcp47) => {
                                                    vec![(
                                                        *bcp47,
                                                        periods.clone(),
                                                        meta_zone_id_data.clone(),
                                                    )]
                                                }
                                                None => panic!("Cannot find bcp47 for {key:?}."),
                                            }
                                        })
                                        .collect::<Vec<_>>(),
                                }
                            })
                            .collect::<Vec<_>>(),
                    }
                })
                .flat_map(metazone_periods_iter)
                .collect(),
        )
    }
}

macro_rules! long_short_impls {
    ($generic:ty, $specific:ty, $field:ident, $metazones_name:ident) => {
        impl From<CldrTimeZonesData<'_>> for $generic {
            fn from(other: CldrTimeZonesData<'_>) -> Self {
                let data = other.time_zone_names_resource;
                let bcp47_tzid_data = &compute_bcp47_tzids_hashmap(other.bcp47_tzids_resource);
                let meta_zone_id_data =
                    &compute_meta_zone_ids_hashmap(other.meta_zone_ids_resource);
                Self {
                    defaults: match &data.metazone {
                        None => Default::default(),
                        Some(metazones) => metazones
                            .0
                            .iter()
                            .filter_map(|(key, metazone)| {
                                match meta_zone_id_data.get(key) {
                                    Some(meta_zone_short_id) => {
                                        metazone.$field.as_ref().and_then(type_fallback).map(
                                            |format| (meta_zone_short_id.clone(), format.clone()),
                                        )
                                    }
                                    None => {
                                        // TODO(#1781): Remove this special case once the short id is updated in CLDR
                                        if key == "Yukon" {
                                            metazone.$field.as_ref().and_then(type_fallback).map(
                                                |format| {
                                                    const TINYSTR_YUKO: tinystr::TinyAsciiStr<4> =
                                                        tinystr::tinystr!(4, "yuko");
                                                    (MetazoneId(TINYSTR_YUKO), format.clone())
                                                },
                                            )
                                        } else {
                                            panic!(
                                                "Cannot find short id of meta zone for {key:?}."
                                            )
                                        }
                                    }
                                }
                            })
                            .collect(),
                    },
                    overrides: data
                        .zone
                        .0
                        .iter()
                        .flat_map(|(key, region)| {
                            region
                                .0
                                .iter()
                                .flat_map(move |(inner_key, place_or_region)| {
                                    let mut key = key.clone();
                                    key.push('/');
                                    key.push_str(&inner_key);
                                    match place_or_region {
                                        LocationOrSubRegion::Location(place) => {
                                            match bcp47_tzid_data.get(&key) {
                                                Some(bcp47) => place
                                                    .$metazones_name()
                                                    .and_then(|zf| type_fallback(&zf).cloned())
                                                    .map(|format| vec![(bcp47, format)])
                                                    .unwrap_or_default(),
                                                None => panic!("Cannot find bcp47 for {key:?}."),
                                            }
                                        }
                                        LocationOrSubRegion::SubRegion(region) => region
                                            .iter()
                                            .filter_map(|(inner_key, place)| {
                                                let mut key = key.clone();
                                                key.push('/');
                                                key.push_str(&inner_key);
                                                match bcp47_tzid_data.get(&key) {
                                                    Some(bcp47) => place
                                                        .$metazones_name()
                                                        .and_then(|zf| type_fallback(&zf).cloned())
                                                        .map(|format| (bcp47, format)),
                                                    None => {
                                                        panic!("Cannot find bcp47 for {key:?}.")
                                                    }
                                                }
                                            })
                                            .collect::<Vec<_>>(),
                                    }
                                })
                        })
                        .collect(),
                }
            }
        }

        impl From<CldrTimeZonesData<'_>> for $specific {
            fn from(other: CldrTimeZonesData<'_>) -> Self {
                let data = other.time_zone_names_resource;
                let bcp47_tzid_data = &compute_bcp47_tzids_hashmap(other.bcp47_tzids_resource);
                let meta_zone_id_data =
                    &compute_meta_zone_ids_hashmap(other.meta_zone_ids_resource);
                Self {
                    defaults: match &data.metazone {
                        None => Default::default(),
                        Some(metazones) => metazones
                            .0
                            .iter()
                            .filter_map(|(key, metazone)| {
                                match meta_zone_id_data.get(key) {
                                    Some(meta_zone_short_id) => metazone
                                        .$field
                                        .as_ref()
                                        .map(|value| (meta_zone_short_id.clone(), value.clone())),
                                    None => {
                                        // TODO(#1781): Remove this special case once the short id is updated in CLDR
                                        if key == "Yukon" {
                                            metazone.$field.as_ref().map(|value| {
                                                const TINYSTR_YUKO: tinystr::TinyAsciiStr<4> =
                                                    tinystr::tinystr!(4, "yuko");
                                                (MetazoneId(TINYSTR_YUKO), value.clone())
                                            })
                                        } else {
                                            panic!(
                                                "Cannot find short id of meta zone for {key:?}."
                                            )
                                        }
                                    }
                                }
                            })
                            .flat_map(iterate_zone_format_for_meta_zone_id)
                            .collect(),
                    },
                    overrides: data
                        .zone
                        .0
                        .iter()
                        .flat_map(|(key, region)| {
                            region
                                .0
                                .iter()
                                .flat_map(move |(inner_key, place_or_region)| {
                                    let mut key = key.clone();
                                    key.push('/');
                                    key.push_str(&inner_key);
                                    match place_or_region {
                                        LocationOrSubRegion::Location(place) => {
                                            match bcp47_tzid_data.get(&key) {
                                                Some(bcp47) => vec![place]
                                                    .into_iter()
                                                    .filter_map(|inner_place| {
                                                        inner_place
                                                            .$metazones_name()
                                                            .map(|format| (bcp47.clone(), format))
                                                    })
                                                    .collect::<Vec<_>>(),
                                                None => panic!("Cannot find bcp47 for {key:?}."),
                                            }
                                        }
                                        LocationOrSubRegion::SubRegion(region) => region
                                            .iter()
                                            .filter_map(|(inner_key, place)| {
                                                let mut key = key.clone();
                                                key.push('/');
                                                key.push_str(&inner_key);
                                                match bcp47_tzid_data.get(&key) {
                                                    Some(bcp47) => place
                                                        .$metazones_name()
                                                        .map(|format| (bcp47.clone(), format)),
                                                    None => {
                                                        panic!("Cannot find bcp47 for {key:?}.")
                                                    }
                                                }
                                            })
                                            .collect::<Vec<_>>(),
                                    }
                                })
                        })
                        .flat_map(iterate_zone_format_for_time_zone_id)
                        .collect(),
                }
            }
        }
    };
}

long_short_impls!(
    MetazoneGenericNamesLongV1<'static>,
    MetazoneSpecificNamesLongV1<'static>,
    long,
    long_metazone_names
);

long_short_impls!(
    MetazoneGenericNamesShortV1<'static>,
    MetazoneSpecificNamesShortV1<'static>,
    short,
    short_metazone_names
);

fn convert_cldr_zone_variant(cldr_zone_variant: &str) -> ZoneVariant {
    match cldr_zone_variant {
        "standard" => ZoneVariant::standard(),
        "daylight" => ZoneVariant::daylight(),
        _ => panic!("Time-zone variant was not compatible with ZoneVariant: {cldr_zone_variant}"),
    }
}

fn iterate_zone_format_for_meta_zone_id(
    pair: (MetazoneId, ZoneFormat),
) -> impl Iterator<Item = (MetazoneId, ZoneVariant, String)> {
    let (key1, zf) = pair;
    zf.0.into_iter()
        .filter(|(key, _)| !key.eq("generic"))
        .map(move |(key, value)| (key1, convert_cldr_zone_variant(&key), value))
}

fn iterate_zone_format_for_time_zone_id(
    pair: (TimeZoneBcp47Id, ZoneFormat),
) -> impl Iterator<Item = (TimeZoneBcp47Id, ZoneVariant, String)> {
    let (key1, zf) = pair;
    zf.0.into_iter()
        .filter(|(key, _)| !key.eq("generic"))
        .map(move |(key, value)| (key1, convert_cldr_zone_variant(&key), value))
}

fn metazone_periods_iter(
    pair: (
        TimeZoneBcp47Id,
        Vec<MetazoneForPeriod>,
        HashMap<String, MetazoneId>,
    ),
) -> impl Iterator<Item = (TimeZoneBcp47Id, i32, Option<MetazoneId>)> {
    let (time_zone_key, periods, meta_zone_id_data) = pair;
    periods
        .into_iter()
        .map(move |period| match &period.uses_meta_zone.from {
            Some(from) => {
                // TODO(#2127): Ideally this parsing can move into a library function
                let parts: Vec<String> = from.split(' ').map(|s| s.to_string()).collect();
                let date = &parts[0];
                let time = &parts[1];
                let date_parts: Vec<String> = date.split('-').map(|s| s.to_string()).collect();
                let year = date_parts[0].parse::<i32>().unwrap();
                let month = date_parts[1].parse::<u8>().unwrap();
                let day = date_parts[2].parse::<u8>().unwrap();
                let time_parts: Vec<String> = time.split(':').map(|s| s.to_string()).collect();
                let hour = time_parts[0].parse::<u8>().unwrap();
                let minute = time_parts[1].parse::<u8>().unwrap();
                let iso =
                    DateTime::try_new_iso_datetime(year, month, day, hour, minute, 0).unwrap();
                let minutes = iso.minutes_since_local_unix_epoch();

                match meta_zone_id_data.get(&period.uses_meta_zone.mzone) {
                    Some(meta_zone_short_id) => (time_zone_key, minutes, Some(*meta_zone_short_id)),
                    None => {
                        // TODO(#1781): Remove this special case once the short id is updated in CLDR
                        if &period.uses_meta_zone.mzone == "Yukon" {
                            (
                                time_zone_key,
                                minutes,
                                Some(MetazoneId(tinystr::tinystr!(4, "yuko"))),
                            )
                        } else {
                            (time_zone_key, minutes, None)
                        }
                    }
                }
            }
            None => {
                let iso = DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0).unwrap();
                let minutes = iso.minutes_since_local_unix_epoch();
                match meta_zone_id_data.get(&period.uses_meta_zone.mzone) {
                    Some(meta_zone_short_id) => (time_zone_key, minutes, Some(*meta_zone_short_id)),
                    None => {
                        // TODO(#1781): Remove this special case once the short id is updated in CLDR
                        if &period.uses_meta_zone.mzone == "Yukon" {
                            (
                                time_zone_key,
                                minutes,
                                Some(MetazoneId(tinystr::tinystr!(4, "yuko"))),
                            )
                        } else {
                            (time_zone_key, minutes, None)
                        }
                    }
                }
            }
        })
}
