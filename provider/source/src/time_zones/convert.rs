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
use icu::timezone::provider::MetazonePeriodV1;
use icu::timezone::UtcOffset;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

#[test]
fn test() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    let provider = crate::SourceDataProvider::new_latest_tested();

    compute_meta_zone_offsets(
        &provider
            .cldr()
            .unwrap()
            .core()
            .read_and_parse::<cldr_serde::time_zones::meta_zones::Resource>(
                "supplemental/metaZones.json",
            )
            .unwrap()
            .supplemental
            .meta_zones
            .meta_zone_info
            .time_zone
            .0,
        provider.tzdb().get().unwrap(),
    );
}

fn compute_meta_zone_offsets(
    meta_zone_periods: &BTreeMap<String, ZonePeriod>,
    tzdb: &parse_zoneinfo::table::Table,
) -> BTreeMap<String, (UtcOffset, Option<UtcOffset>)> {
    let mut iana_since: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();

    for (iana, periods) in iter_meta_zone_periods(meta_zone_periods) {
        for period in periods {
            let from = period.uses_meta_zone.from.as_deref();
            let to = period.uses_meta_zone.to.as_deref();
            iana_since
                .entry(&period.uses_meta_zone.mzone)
                .or_default()
                .insert(((from, to), iana.clone()));
        }
    }

    iana_since.into_iter().map(|(meta, ianas)| {
        let (mut standard, mut daylight) = (None, None);
        let mut first = ("", None, None);
        for ((from, to), iana) in &ianas {
            let (mut new_standard, mut new_daylight) = offsets_for_iana(iana, tzdb, *from, *to);
            if let Some(d) = new_daylight.as_mut() {
                // Some time zones, like Europe/Dublin use DST the wrong way
                // around. Don't let this weirdness leak into meta zones.
                if d.offset_seconds() < new_standard.offset_seconds() {
                    core::mem::swap(d, &mut new_standard);
                }
            }
            if let Some(standard) = standard {
                match (standard, daylight, new_standard, new_daylight) {
                    // Same standard times, no DST, allowed
                    (s1, None, s2, None) if s1 == s2 => {}
                    // Same standard times, one DST, allowed
                    (s1, Some(_), s2, None) | (s1, None, s2, Some(_)) if s1 == s2 => {}
                    // Same standard times, same DST times, allowed
                    (s1, Some(d1), s2, Some(d2)) if s1 == s2 && d1 == d2 => {}
                    // Same standard times, different DST times, weird but allowed
                    (s1, _, s2, _) if s1 == s2 => {
                        log::trace!(
                            "{meta:?}: {iana:?} ({from} - {to}) differs from {first_iana:?} ({first_from} - {first_to}): standard {standard:?} vs {new_standard:?}, daylight {daylight:?} vs {new_daylight:?}",
                            from = from.unwrap_or("-∞"),
                            to = to.unwrap_or("∞"),
                            first_iana = first.0,
                            first_from = first.1.unwrap_or("-∞"),
                            first_to = first.2.unwrap_or("∞"),
                        )
                    }
                    // all other cases
                    _ => {
                        log::warn!(
                            "{meta:?}: {iana:?} ({from} - {to}) differs from {first_iana:?} ({first_from} - {first_to}): {standard:?} vs {new_standard:?}",
                            from = from.unwrap_or("-∞"),
                            to = to.unwrap_or("∞"),
                            first_iana = first.0,
                            first_from = first.1.unwrap_or("-∞"),
                            first_to = first.2.unwrap_or("∞"),
                        )
                    }
                }
                if daylight.is_none() && standard == new_standard {
                    // Only set daylight if standards agree
                    daylight = new_daylight;
                }
            } else {
                first = (iana, *from, *to);
                standard = Some(new_standard);
                daylight = new_daylight;
            }
        }
        (meta.clone(), (standard.unwrap(), daylight))
    })
    .collect()
}

fn offsets_for_iana(
    iana: &str,
    tzdb: &parse_zoneinfo::table::Table,
    from: Option<&str>,
    to: Option<&str>,
) -> (UtcOffset, Option<UtcOffset>) {
    use parse_zoneinfo::transitions::TableTransitions;

    let timespans = tzdb.timespans(iana).unwrap();

    let mut timespans_chronological = [(i64::MIN, &timespans.first)]
        .into_iter()
        .chain(timespans.rest.iter().filter_map(|(since, ts)| (*since > 0).then_some((*since, ts))))
        .map(|(since, ts)| {
            (
                since,
                UtcOffset::try_from_offset_seconds(ts.utc_offset as i32).unwrap(),
                (ts.dst_offset != 0).then(|| {
                    UtcOffset::try_from_offset_seconds((ts.utc_offset + ts.dst_offset) as i32)
                        .unwrap()
                }),
            )
        })
        .collect::<Vec<_>>();

    timespans_chronological.sort_by_key(|(since, _, _)| *since);

    let first_timespan_index = match timespans_chronological.binary_search_by_key(
        &from
            // TODO: these timestamps are in local time :'(
            .map(|s| parse_mzone_from(s).minutes_since_local_unix_epoch() as i64 * 60)
            .unwrap_or(i64::MIN),
        |(since, _, _)| *since,
    ) {
        Ok(i) => i, // from date is inclusive
        Err(i) => i - 1,
    };

    let last_timespan_index = match timespans_chronological.binary_search_by_key(
        &to
            // TODO: these timestamps are in local time :'(
            .map(|s| parse_mzone_from(s).minutes_since_local_unix_epoch() as i64 * 60)
            .unwrap_or(i64::MAX),
        |(since, _, _)| *since,
    ) {
        Ok(i) => i - 1, // to date is exclusive
        Err(i) => i - 1,
    };

    let current = timespans_chronological[last_timespan_index].1;

    for (_,  previous, _) in &timespans_chronological[first_timespan_index..last_timespan_index] {
        if *previous != current {
            log::trace!("{iana} observed different offsets without changing metazone {current:?}, {previous:?}")
        }
    }

    let last_dst = timespans_chronological[first_timespan_index..=last_timespan_index]
        .iter()
        .rev()
        .flat_map(|(_, _, dst)| *dst)
        .next();

    (current, last_dst)
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
        ZonePeriod::Region(periods) => vec![(format!("{key}"), periods)],
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
                let bcp47_tzid_data = &compute_bcp47_tzids_btreemap(other.bcp47_tzids_resource);
                let meta_zone_id_data =
                    &compute_meta_zone_ids_btreemap(other.meta_zone_ids_resource);
                let meta_zone_offsets =
                    &compute_meta_zone_offsets(other.meta_zone_periods_resource, other.tzdb);
                Self {
                    defaults: match &data.metazone {
                        None => Default::default(),
                        Some(metazones) => metazones
                            .0
                            .iter()
                            .filter_map(|(key, metazone)| match meta_zone_id_data.get(key) {
                                Some(meta_zone_short_id) => metazone.$field.as_ref().map(|value| {
                                    (
                                        meta_zone_short_id.clone(),
                                        key.clone(),
                                        meta_zone_offsets.get(key).cloned().unwrap_or_else(||  {println!("Missing offsets for {meta_zone_short_id:?}"); (UtcOffset::zero(), None)}),
                                        value.clone(),
                                    )
                                }),
                                None => panic!("Cannot find short id of meta zone for {key:?}."),
                            })
                            .flat_map(|(z, dbg, os, zf)| {
                                convert_cldr_zone_variant(zf.0, &dbg, os)
                                    .map(move |(o, v)| (z, o.offset_eighths_of_hour(), v))
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
                                                Some(bcp47) => [place]
                                                    .into_iter()
                                                    .filter_map(|inner_place| {
                                                        inner_place.$metazones_name().map(
                                                            |format| {
                                                                (
                                                                    bcp47.clone(),
                                                                    key.clone(),
                                                                    offsets_for_iana(
                                                                        &key,
                                                                        &other.tzdb,
                                                                        None,
                                                                        None,
                                                                    ),
                                                                    format,
                                                                )
                                                            },
                                                        )
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
                                                    Some(bcp47) => {
                                                        place.$metazones_name().map(|format| {
                                                            (
                                                                bcp47.clone(),
                                                                key.clone(),
                                                                offsets_for_iana(
                                                                    &key,
                                                                    &other.tzdb,
                                                                    None,
                                                                    None,
                                                                ),
                                                                format,
                                                            )
                                                        })
                                                    }
                                                    None => {
                                                        panic!("Cannot find bcp47 for {key:?}.")
                                                    }
                                                }
                                            })
                                            .collect::<Vec<_>>(),
                                    }
                                })
                        })
                        .flat_map(|(z, dbg, os, zf)| {
                            convert_cldr_zone_variant(zf.0, &dbg, os)
                                .map(move |(o, v)| (z, o.offset_eighths_of_hour(), v))
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
    debug_name: &impl std::fmt::Debug,
    (std_offset, mut dst_offset): (UtcOffset, Option<UtcOffset>),
) -> impl Iterator<Item = (UtcOffset, String)> {
    if dst_offset.is_none() && kvs.contains_key("daylight") {
        log::warn!("{debug_name:?} has a DST display name, but the time zone has never used DST");
    }
    if dst_offset.is_some() && !kvs.contains_key("daylight") {
        log::warn!("{debug_name:?} has used DST ({}), in the past (or still is), but doesn't have a DST display name", dst_offset.unwrap().offset_seconds() as f32 / 3600.0);
    }
    if dst_offset == Some(std_offset) && kvs.contains_key("daylight") {
        log::warn!("{debug_name:?}'s last historical DST offset is the same as its current standard offset ({}), so it cannot be stored", std_offset.offset_seconds() as f32 / 3600.0);
        dst_offset = None;
    }
    kvs.into_iter()
        .filter(|(variant, _)| variant != "generic")
        .filter_map(move |(variant, value)| {
            Some(match variant.as_str() {
                "standard" => (std_offset, value),
                "daylight" => (dst_offset?, value),
                _ => panic!("Unknown time-zone variant: {variant}"),
            })
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
    periods: &'a Vec<MetazoneForPeriod>,
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
