// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::CldrTimeZonesData;
use crate::cldr_serde;
use cldr_serde::time_zones::bcp47_tzid::Bcp47TzidAliasData;
use cldr_serde::time_zones::meta_zones::MetaLocationOrSubRegion;
use cldr_serde::time_zones::meta_zones::MetazoneAliasData;
use cldr_serde::time_zones::meta_zones::MetazoneForPeriod;
use cldr_serde::time_zones::meta_zones::ZonePeriod;
use cldr_serde::time_zones::time_zone_names::*;
use icu::calendar::DateTime;
use icu::calendar::Iso;
use icu::datetime::provider::time_zones::{
    ExemplarCitiesV1, MetazoneGenericNamesLongV1, MetazoneGenericNamesShortV1, MetazoneId,
    MetazoneSpecificNamesLongV1, MetazoneSpecificNamesShortV1, TimeZoneBcp47Id, TimeZoneFormatsV1,
};
use icu::timezone::provider::IsoMinutesSinceEpoch;
use icu::timezone::provider::MetazonePeriodV1;
use icu::timezone::provider::ZoneOffsetPeriodV1;
use icu::timezone::UtcOffset;
use icu::timezone::ZoneVariant;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::Saving;
use std::borrow::Cow;
use std::collections::BTreeMap;
use tinystr::tinystr;
use tinystr::TinyStr8;

/// Performs part 1 of type fallback as specified in the UTS-35 spec for TimeZone Goals:
/// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Goals
///
/// Part 2 of type fallback requires access to the IANA TimeZone Database
/// as well as a specific datetime context, so it is not relevant to DataProvider.
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

/// Returns a map from time zone long identifier to time zone BCP-47 ID.
///
/// For example: "America/Chicago" to "uschi"
pub(crate) fn compute_bcp47_tzids_btreemap(
    bcp47_tzids_resource: &BTreeMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
) -> BTreeMap<String, TimeZoneBcp47Id> {
    let mut bcp47_tzids = BTreeMap::new();
    for (bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource.iter() {
        if let Some(alias) = &bcp47_tzid_data.alias {
            for data_value in alias.split(' ') {
                bcp47_tzids.insert(data_value.to_string(), *bcp47_tzid);
            }
        }
    }
    bcp47_tzids
}

/// Returns a map from BCP-47 ID to a single canonical long identifier.
///
/// For example: "inccu" to "Asia/Kolkata"
pub(crate) fn compute_canonical_tzids_btreemap(
    bcp47_tzids_resource: &BTreeMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
) -> BTreeMap<TimeZoneBcp47Id, String> {
    let mut canonical_tzids = BTreeMap::new();
    for (bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource.iter() {
        if Some(true) == bcp47_tzid_data.deprecated {
            // skip
        } else if let Some(iana) = &bcp47_tzid_data.iana {
            canonical_tzids.insert(*bcp47_tzid, iana.clone());
        } else if let Some(iana) = &bcp47_tzid_data
            .alias
            .as_ref()
            .and_then(|s| s.split(' ').next())
        {
            canonical_tzids.insert(*bcp47_tzid, String::from(*iana));
        } else {
            debug_assert!(
                false,
                "Could not find canonical IANA for bcp47 time zone: {bcp47_tzid:?}"
            );
        }
    }
    canonical_tzids
}

/// Returns a map from metazone long identifier to metazone BCP-47 ID.
///
/// For example: "America_Central" to "amce"
fn compute_meta_zone_ids_btreemap(
    meta_zone_ids_resource: &BTreeMap<MetazoneId, MetazoneAliasData>,
) -> BTreeMap<String, MetazoneId> {
    let mut meta_zone_ids = BTreeMap::new();
    for (meta_zone_id, meta_zone_id_data) in meta_zone_ids_resource.iter() {
        meta_zone_ids.insert(meta_zone_id_data.long_id.to_string(), *meta_zone_id);
    }
    // TODO(#1781): Remove this special case once the short id is updated in CLDR
    meta_zone_ids.insert("Yukon".to_owned(), MetazoneId(tinystr!(4, "yuko")));
    meta_zone_ids
}

impl From<CldrTimeZonesData<'_>> for TimeZoneFormatsV1<'static> {
    fn from(other: CldrTimeZonesData<'_>) -> Self {
        let data = other.time_zone_names_resource;
        Self {
            hour_format: parse_hour_format(&data.hour_format),
            offset_format: data.gmt_format.clone().into(),
            offset_zero_format: data.gmt_zero_format.clone().into(),
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
        }
    }
}

impl Location {
    fn exemplar_city(&self) -> Option<String> {
        self.exemplar_city.clone()
    }

    fn long_metazone_names(&self) -> Option<ZoneFormat> {
        self.long.clone()
    }

    fn short_metazone_names(&self) -> Option<ZoneFormat> {
        self.short.clone()
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

fn iter_meta_zone_periods(
    data: &BTreeMap<String, ZonePeriod>,
) -> impl Iterator<Item = (String, &Vec<MetazoneForPeriod>)> {
    data.iter().flat_map(|(key, zone)| match zone {
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

impl From<CldrTimeZonesData<'_>> for MetazonePeriodV1<'static> {
    fn from(other: CldrTimeZonesData<'_>) -> Self {
        let bcp47_tzid_data = &compute_bcp47_tzids_btreemap(other.bcp47_tzids_resource);
        let meta_zone_id_data = &compute_meta_zone_ids_btreemap(other.meta_zone_ids_resource);
        Self(
            iter_meta_zone_periods(other.meta_zone_periods_resource)
                .flat_map(|(iana, periods)| match bcp47_tzid_data.get(&iana) {
                    Some(&bcp47) => metazone_periods_iter(bcp47, periods, meta_zone_id_data),
                    None => panic!("Cannot find bcp47 for {iana:?}."),
                })
                .collect(),
        )
    }
}

impl From<CldrTimeZonesData<'_>> for ZoneOffsetPeriodV1<'static> {
    fn from(other: CldrTimeZonesData<'_>) -> Self {
        Self(
            compute_canonical_tzids_btreemap(other.bcp47_tzids_resource)
                .into_iter()
                .filter_map(|(bcp47, iana)| Some((bcp47, other.tzdb.get_zoneset(&iana)?)))
                .flat_map(|(bcp47, zoneset)| {
                    let mut data = Vec::<(IsoMinutesSinceEpoch, (UtcOffset, UtcOffset))>::new();

                    fn store_offsets(
                        data: &mut Vec<(IsoMinutesSinceEpoch, (UtcOffset, UtcOffset))>,
                        end_time: IsoMinutesSinceEpoch,
                        mut utc_offset: i64,
                        mut dst_offset_relative: i64,
                    ) {
                        // TZDB uses negative DST offsets (i.e. DST in the winter for some zones,
                        // such as `Europe/Dublin`. In ICU4X, we normalize all time zones to have
                        // positive DST offsets, during the summer.
                        if dst_offset_relative < 0 {
                            utc_offset += dst_offset_relative;
                            dst_offset_relative = -dst_offset_relative;
                        }
                        data.push((
                            end_time,
                            (
                                UtcOffset::try_from_offset_seconds(utc_offset as i32).unwrap(),
                                UtcOffset::try_from_offset_seconds(dst_offset_relative as i32)
                                    .unwrap(),
                            ),
                        ));
                    }

                    for zone_info in zoneset.iter() {
                        let local_end_time = zone_info
                            .end_time
                            .map(|t| (t.to_timestamp() / 60) as IsoMinutesSinceEpoch)
                            .unwrap_or(IsoMinutesSinceEpoch::MAX);

                        if local_end_time <= 0 {
                            continue;
                        }

                        match zone_info.saving {
                            Saving::NoSaving => {
                                store_offsets(&mut data, local_end_time, zone_info.offset, 0);
                            }

                            Saving::OneOff(amount) => {
                                store_offsets(&mut data, local_end_time, zone_info.offset, amount);
                            }

                            Saving::Multiple(ref rules) => {
                                let mut rules = other.tzdb.rulesets[rules]
                                    .iter()
                                    // Only want transitions into DST
                                    .filter(|rule| rule.time_to_add != 0)
                                    .filter(|rule| {
                                        // Use all rules (from year 1800) until the potential end time of the zone_info (or year 2500) 
                                        (1800..zone_info.end_time.map(|e| e.year()).unwrap_or(2500))
                                            .any(|y| rule.applies_to_year(y))
                                    })
                                    .map(|rule| {
                                        (
                                            match rule.to_year.unwrap_or(rule.from_year) {
                                                Year::Minimum => unreachable!(),
                                                Year::Maximum => local_end_time,
                                                Year::Number(y) => {
                                                    (rule.absolute_datetime(
                                                        y,
                                                        zone_info.offset,
                                                        rule.time_to_add,
                                                    ) / 60)
                                                        as IsoMinutesSinceEpoch
                                                }
                                            },
                                            rule.time_to_add,
                                        )
                                    })
                                    .filter(|&(rule_local_end_time, _)| {
                                        // Discard rules from before this zoneinfo (or before the epoch)
                                        rule_local_end_time
                                            > data
                                                .last()
                                                .map(|&(prev_end_time, _)| prev_end_time)
                                                .unwrap_or(0)
                                    })
                                    .collect::<Vec<_>>();

                                rules.sort_by_key(|&(local_end_time, _)| local_end_time);

                                if rules.is_empty() {
                                    // No rule applies
                                    store_offsets(&mut data, local_end_time, zone_info.offset, 0);
                                } else {
                                    for &(local_end_time, dst_offset_relative) in &rules {
                                        store_offsets(
                                            &mut data,
                                            local_end_time,
                                            zone_info.offset,
                                            dst_offset_relative,
                                        );
                                    }

                                    if rules.last().unwrap().0 < local_end_time {
                                        // rules end before zoneinfo ends, continue without offset
                                        store_offsets(
                                            &mut data,
                                            local_end_time,
                                            zone_info.offset,
                                            0,
                                        );
                                    }
                                }
                            }
                        }
                    }

                    // Sort descending
                    data.sort_by_key(|(end_time, _)| -*end_time);
                    // Dedup consecutive offsets, keeping higher end time
                    data.dedup_by_key(|(_, offsets)| *offsets);

                    data.into_iter()
                        .map(move |(end_time, (utc_offset, dst_offset_relative))| {
                            (
                                bcp47,
                                end_time,
                                (
                                    utc_offset.offset_eighths_of_hour(),
                                    dst_offset_relative.offset_eighths_of_hour(),
                                ),
                            )
                        })
                })
                .collect(),
        )
    }
}

macro_rules! long_short_impls {
    ($generic:ty, $specific:ty, $field:ident, $metazones_name:ident) => {
        impl From<CldrTimeZonesData<'_>> for $generic {
            fn from(other: CldrTimeZonesData<'_>) -> Self {
                let data = other.time_zone_names_resource;
                let bcp47_tzid_data = &compute_bcp47_tzids_btreemap(other.bcp47_tzids_resource);
                let meta_zone_id_data =
                    &compute_meta_zone_ids_btreemap(other.meta_zone_ids_resource);
                Self {
                    defaults: match &data.metazone {
                        None => Default::default(),
                        Some(metazones) => metazones
                            .0
                            .iter()
                            .filter_map(|(key, metazone)| match meta_zone_id_data.get(key) {
                                Some(meta_zone_short_id) => metazone
                                    .$field
                                    .as_ref()
                                    .and_then(type_fallback)
                                    .map(|format| (meta_zone_short_id.clone(), format.clone())),
                                None => panic!("Cannot find short id of meta zone for {key:?}."),
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
                                    let iana = format!("{key}/{inner_key}");
                                    match place_or_region {
                                        LocationOrSubRegion::Location(place) => {
                                            match bcp47_tzid_data.get(&iana) {
                                                Some(bcp47) => place
                                                    .$metazones_name()
                                                    .and_then(|zf| type_fallback(&zf).cloned())
                                                    .map(|format| vec![(bcp47, format)])
                                                    .unwrap_or_default(),
                                                None => panic!("Cannot find bcp47 for {iana:?}."),
                                            }
                                        }
                                        LocationOrSubRegion::SubRegion(region) => region
                                            .iter()
                                            .filter_map(|(inner_key, place)| {
                                                let iana = format!("{iana}/{inner_key}");
                                                match bcp47_tzid_data.get(&iana) {
                                                    Some(bcp47) => place
                                                        .$metazones_name()
                                                        .and_then(|zf| type_fallback(&zf).cloned())
                                                        .map(|format| (bcp47, format)),
                                                    None => {
                                                        panic!("Cannot find bcp47 for {iana:?}.")
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
                let bcp47_tzid_data = &compute_bcp47_tzids_btreemap(other.bcp47_tzids_resource);
                let meta_zone_id_data =
                    &compute_meta_zone_ids_btreemap(other.meta_zone_ids_resource);
                Self {
                    defaults: match &data.metazone {
                        None => Default::default(),
                        Some(metazones) => metazones
                            .0
                            .iter()
                            .filter_map(|(key, metazone)| match meta_zone_id_data.get(key) {
                                Some(&meta_zone_short_id) => metazone
                                    .$field
                                    .as_ref()
                                    .map(|zone_format| (meta_zone_short_id, zone_format.clone())),
                                None => panic!("Cannot find short id of meta zone for {key:?}."),
                            })
                            .flat_map(|(meta, zone_format)| {
                                convert_cldr_zone_variant(zone_format.0)
                                    .map(move |(variant, value)| (meta, variant, value))
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
                                    let iana = format!("{key}/{inner_key}");
                                    match place_or_region {
                                        LocationOrSubRegion::Location(place) => {
                                            match bcp47_tzid_data.get(&iana) {
                                                Some(&bcp47) => [place]
                                                    .into_iter()
                                                    .filter_map(|place| {
                                                        place
                                                            .$metazones_name()
                                                            .map(|zone_format| (bcp47, zone_format))
                                                    })
                                                    .collect::<Vec<_>>(),
                                                None => panic!("Cannot find bcp47 for {iana:?}."),
                                            }
                                        }
                                        LocationOrSubRegion::SubRegion(region) => region
                                            .iter()
                                            .filter_map(|(inner_key, place)| {
                                                let iana = format!("{iana}/{inner_key}");
                                                match bcp47_tzid_data.get(&iana) {
                                                    Some(&bcp47) => place
                                                        .$metazones_name()
                                                        .map(|format| (bcp47, format)),
                                                    None => {
                                                        panic!("Cannot find bcp47 for {iana:?}.")
                                                    }
                                                }
                                            })
                                            .collect::<Vec<_>>(),
                                    }
                                })
                        })
                        .flat_map(|(bcp47, zone_format)| {
                            convert_cldr_zone_variant(zone_format.0)
                                .map(move |(variant, value)| (bcp47, variant, value))
                        })
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

fn convert_cldr_zone_variant(
    kvs: BTreeMap<String, String>,
) -> impl Iterator<Item = (ZoneVariant, String)> {
    kvs.into_iter()
        .filter(|(variant, _)| variant != "generic")
        .map(move |(variant, value)| {
            (
                match variant.as_str() {
                    "standard" => ZoneVariant::Standard,
                    "daylight" => ZoneVariant::Daylight,
                    _ => panic!("Time-zone variant was not compatible with ZoneVariant: {variant}"),
                },
                value,
            )
        })
}

fn parse_mzone_from(from: &str) -> DateTime<Iso> {
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
    DateTime::try_new_iso_datetime(year, month, day, hour, minute, 0).unwrap()
}

fn metazone_periods_iter<'a>(
    time_zone_key: TimeZoneBcp47Id,
    periods: &'a [MetazoneForPeriod],
    meta_zone_id_data: &'a BTreeMap<String, MetazoneId>,
) -> impl Iterator<Item = (TimeZoneBcp47Id, i32, Option<MetazoneId>)> + 'a {
    periods
        .iter()
        .map(move |period| match &period.uses_meta_zone.from {
            Some(from) => (
                time_zone_key,
                parse_mzone_from(from).minutes_since_local_unix_epoch(),
                meta_zone_id_data.get(&period.uses_meta_zone.mzone).copied(),
            ),
            None => (
                time_zone_key,
                DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0)
                    .unwrap()
                    .minutes_since_local_unix_epoch(),
                meta_zone_id_data.get(&period.uses_meta_zone.mzone).copied(),
            ),
        })
}
