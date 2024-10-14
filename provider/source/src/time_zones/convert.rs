// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use cldr_serde::time_zones::meta_zones::MetaLocationOrSubRegion;
use cldr_serde::time_zones::meta_zones::ZonePeriod;
use cldr_serde::time_zones::time_zone_names::*;
use icu::calendar::DateTime;
use icu::calendar::Iso;
use icu::datetime::provider::time_zones::*;
use icu::timezone::provider::*;
use icu::timezone::UtcOffset;
use icu::timezone::ZoneVariant;
use icu_provider::prelude::*;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::Saving;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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

impl DataProvider<TimeZoneEssentialsV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimeZoneEssentialsV1Marker>, DataError> {
        self.check_req::<TimeZoneEssentialsV1Marker>(req)?;

        let time_zone_names = &self
            .cldr()?
            .dates("gregorian")
            .read_and_parse::<cldr_serde::time_zones::time_zone_names::Resource>(
                req.id.locale,
                "timeZoneNames.json",
            )?
            .main
            .value
            .dates
            .time_zone_names;

        // e.g. "+HH:mm;-hh:mm"
        let (offset_pattern_positive, offset_pattern_negative) =
            time_zone_names.hour_format.split_once(';').unwrap();

        // The single character before H/h
        let offset_positive_sign = offset_pattern_positive
            .split_once(['h', 'H'])
            .unwrap()
            .0
            .chars()
            .next()
            .unwrap_or('+');
        let offset_negative_sign = offset_pattern_negative
            .split_once(['h', 'H'])
            .unwrap()
            .0
            .chars()
            .next()
            .unwrap_or('-');
        // The single character before `mm`
        let offset_separator = offset_pattern_positive
            .split_once('m')
            .unwrap()
            .0
            .chars()
            .next_back()
            .unwrap_or(':');

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(TimeZoneEssentialsV1 {
                offset_negative_sign,
                offset_positive_sign,
                offset_separator,
                offset_pattern: Cow::Owned(time_zone_names.gmt_format.0.clone()),
                offset_zero: time_zone_names.gmt_zero_format.clone().into(),
            }),
        })
    }
}

impl DataProvider<LocationsV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LocationsV1Marker>, DataError> {
        self.check_req::<LocationsV1Marker>(req)?;

        let time_zone_names = &self
            .cldr()?
            .dates("gregorian")
            .read_and_parse::<cldr_serde::time_zones::time_zone_names::Resource>(
                req.id.locale,
                "timeZoneNames.json",
            )?
            .main
            .value
            .dates
            .time_zone_names;

        let primary_zones = self.compute_primary_zones()?;

        let primary_zones_values = primary_zones.values().copied().collect::<BTreeSet<_>>();

        let region_display_names = if req.id.locale.is_default() {
            BTreeMap::default()
        } else {
            self.cldr()?
                .displaynames()
                .read_and_parse::<cldr_serde::displaynames::region::Resource>(
                    req.id.locale,
                    "territories.json",
                )?
                .main
                .value
                .localedisplaynames
                .regions
                .iter()
                .filter(|&(k, _)| {
                    /// Substring used to denote alternative region names data variants for a given region. For example: "BA-alt-short", "TL-alt-variant".
                    const ALT_SUBSTRING: &str = "-alt-";
                    /// Substring used to denote short region display names data variants for a given region. For example: "BA-alt-short".
                    const SHORT_SUBSTRING: &str = "-alt-short";
                    !k.contains(ALT_SUBSTRING) && !k.contains(SHORT_SUBSTRING)
                })
                .filter_map(|(region, value)| {
                    Some((
                        icu::locale::subtags::Region::try_from_str(region).ok()?,
                        value.as_str(),
                    ))
                })
                .filter(|(r, _)| primary_zones_values.contains(r))
                .collect()
        };

        let bcp47_tzids = &self
            .cldr()?
            .bcp47()
            .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>("timezone.json")?
            .keyword
            .u
            .time_zones
            .values;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(LocationsV1 {
                locations: bcp47_tzids
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
                        if let Some(region) = primary_zones.get(bcp47) {
                            Some((
                                *bcp47,
                                region_display_names
                                    .get(region)
                                    .copied()
                                    .unwrap_or(region.as_str())
                                    .to_string(),
                            ))
                        } else {
                            aliases
                                .split(' ')
                                .filter_map(|alias| {
                                    let mut alias_parts = alias.split('/');
                                    let continent = alias_parts.next().expect("split non-empty");
                                    let location_or_subregion = alias_parts.next()?;
                                    let location_in_subregion = alias_parts.next();
                                    time_zone_names
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
                                        .and_then(|p| p.exemplar_city.clone())
                                })
                                .next()
                                .or_else(|| {
                                    let canonical_alias =
                                        aliases.split(' ').next().expect("split non-empty");
                                    let mut alias_parts = canonical_alias.split('/');
                                    (alias_parts.next() != Some("Etc")).then(|| {
                                        alias_parts
                                            .next_back()
                                            .expect("split non-empty")
                                            .replace('_', " ")
                                    })
                                })
                                .map(|name| (*bcp47, name))
                        }
                    })
                    .collect(),
                pattern_generic: Cow::Owned(time_zone_names.region_format.0.clone()),
                pattern_standard: Cow::Owned(time_zone_names.region_format_st.0.clone()),
                pattern_daylight: Cow::Owned(time_zone_names.region_format_dt.0.clone()),
                pattern_partial_location: Cow::Owned(time_zone_names.fallback_format.0.clone()),
            }),
        })
    }
}

impl DataProvider<MetazonePeriodV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<MetazonePeriodV1Marker>, DataError> {
        self.check_req::<MetazonePeriodV1Marker>(req)?;

        let metazones = &self
            .cldr()?
            .core()
            .read_and_parse::<cldr_serde::time_zones::meta_zones::Resource>(
                "supplemental/metaZones.json",
            )?
            .supplemental
            .meta_zones;

        let bcp47_tzid_data = &self.compute_bcp47_tzids_btreemap()?;
        let meta_zone_id_data = &self.compute_meta_zone_ids_btreemap()?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(MetazonePeriodV1(
                metazones
                    .meta_zone_info
                    .time_zone
                    .0
                    .iter()
                    .flat_map(|(key, zone)| match zone {
                        ZonePeriod::Region(periods) => vec![(key.to_string(), periods)],
                        ZonePeriod::LocationOrSubRegion(place) => place
                            .iter()
                            .flat_map(move |(key2, location_or_subregion)| {
                                match location_or_subregion {
                                    MetaLocationOrSubRegion::Location(periods) => {
                                        vec![(format!("{key}/{key2}"), periods)]
                                    }
                                    MetaLocationOrSubRegion::SubRegion(subregion) => subregion
                                        .iter()
                                        .flat_map(move |(key3, periods)| {
                                            vec![(format!("{key}/{key2}/{key3}"), periods)]
                                        })
                                        .collect::<Vec<_>>(),
                                }
                            })
                            .collect::<Vec<_>>(),
                    })
                    .flat_map(|(iana, periods)| {
                        periods.iter().flat_map(move |period| {
                            Some((
                                bcp47_tzid_data.get(&iana)?,
                                period
                                    .uses_meta_zone
                                    .from
                                    .as_deref()
                                    .map(parse_mzone_from)
                                    .unwrap_or(
                                        DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0)
                                            .unwrap(),
                                    )
                                    .minutes_since_local_unix_epoch(),
                                meta_zone_id_data.get(&period.uses_meta_zone.mzone).copied(),
                            ))
                        })
                    })
                    .collect(),
            )),
        })
    }
}

impl DataProvider<ZoneOffsetPeriodV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ZoneOffsetPeriodV1Marker>, DataError> {
        self.check_req::<ZoneOffsetPeriodV1Marker>(req)?;

        let tzdb = self.tzdb()?.transitions()?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ZoneOffsetPeriodV1(
                self.compute_canonical_tzids_btreemap()?
                    .iter()
                    .filter_map(|(bcp47, iana)| Some((bcp47, tzdb.get_zoneset(iana)?)))
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
                                    UtcOffset::try_from_seconds(utc_offset as i32).unwrap(),
                                    UtcOffset::try_from_seconds(dst_offset_relative as i32)
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
                                    store_offsets(
                                        &mut data,
                                        local_end_time,
                                        zone_info.offset,
                                        amount,
                                    );
                                }

                                Saving::Multiple(ref rules) => {
                                    let mut rules = tzdb.rulesets[rules]
                                        .iter()
                                        // Only want transitions into DST
                                        .filter(|rule| rule.time_to_add != 0)
                                        .filter(|rule| {
                                            // Use all rules (from year 1800) until the potential end time of the zone_info (or year 2500)
                                            (1800..zone_info
                                                .end_time
                                                .map(|e| e.year())
                                                .unwrap_or(2500))
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
                                        store_offsets(
                                            &mut data,
                                            local_end_time,
                                            zone_info.offset,
                                            0,
                                        );
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

                        data.into_iter().map(
                            move |(end_time, (utc_offset, dst_offset_relative))| {
                                (
                                    bcp47,
                                    end_time,
                                    (
                                        utc_offset.to_eighths_of_hour(),
                                        dst_offset_relative.to_eighths_of_hour(),
                                    ),
                                )
                            },
                        )
                    })
                    .collect(),
            )),
        })
    }
}

macro_rules! long_short_impls {
    ($generic:ty, $specific:ty, $field:ident,) => {
        impl DataProvider<$generic> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$generic>, DataError> {
                self.check_req::<$generic>(req)?;

                let time_zone_names_resource = &self
                    .cldr()?
                    .dates("gregorian")
                    .read_and_parse::<cldr_serde::time_zones::time_zone_names::Resource>(
                        req.id.locale,
                        "timeZoneNames.json",
                    )?
                    .main
                    .value
                    .dates
                    .time_zone_names;

                let bcp47_tzid_data = &self.compute_bcp47_tzids_btreemap()?;
                let meta_zone_id_data = &self.compute_meta_zone_ids_btreemap()?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(MetazoneGenericNamesV1 {
                        defaults: match &time_zone_names_resource.metazone {
                            None => Default::default(),
                            Some(metazones) => metazones
                                .0
                                .iter()
                                .filter_map(|(key, metazone)| {
                                    metazone.$field.as_ref().and_then(type_fallback).and_then(
                                        |format| {
                                            Some((
                                                meta_zone_id_data.get(key)?.clone(),
                                                format.clone(),
                                            ))
                                        },
                                    )
                                })
                                .collect(),
                        },
                        overrides: time_zone_names_resource
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
                                            LocationOrSubRegion::Location(place) => place
                                                .$field
                                                .clone()
                                                .and_then(|zf| type_fallback(&zf).cloned())
                                                .and_then(|format| {
                                                    Some(vec![(
                                                        bcp47_tzid_data.get(&iana)?,
                                                        format,
                                                    )])
                                                })
                                                .unwrap_or_default(),
                                            LocationOrSubRegion::SubRegion(region) => region
                                                .iter()
                                                .filter_map(|(inner_key, place)| {
                                                    let iana = format!("{iana}/{inner_key}");
                                                    place
                                                        .$field
                                                        .clone()
                                                        .and_then(|zf| type_fallback(&zf).cloned())
                                                        .and_then(|format| {
                                                            Some((
                                                                bcp47_tzid_data.get(&iana)?,
                                                                format,
                                                            ))
                                                        })
                                                })
                                                .collect::<Vec<_>>(),
                                        }
                                    })
                            })
                            .collect(),
                    }),
                })
            }
        }

        impl DataProvider<$specific> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$specific>, DataError> {
                self.check_req::<$specific>(req)?;

                let time_zone_names_resource = &self
                    .cldr()?
                    .dates("gregorian")
                    .read_and_parse::<cldr_serde::time_zones::time_zone_names::Resource>(
                        req.id.locale,
                        "timeZoneNames.json",
                    )?
                    .main
                    .value
                    .dates
                    .time_zone_names;

                let bcp47_tzid_data = &self.compute_bcp47_tzids_btreemap()?;
                let meta_zone_id_data = &self.compute_meta_zone_ids_btreemap()?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(MetazoneSpecificNamesV1 {
                        defaults: match &time_zone_names_resource.metazone {
                            None => Default::default(),
                            Some(metazones) => metazones
                                .0
                                .iter()
                                .filter_map(|(key, metazone)| {
                                    metazone.$field.as_ref().and_then(|zone_format| {
                                        Some((meta_zone_id_data.get(key)?, zone_format.clone()))
                                    })
                                })
                                .flat_map(|(meta, zone_format)| {
                                    convert_cldr_zone_variant(zone_format.0)
                                        .map(move |(variant, value)| (meta, variant, value))
                                })
                                .collect(),
                        },
                        overrides: time_zone_names_resource
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
                                            LocationOrSubRegion::Location(place) => [place]
                                                .into_iter()
                                                .filter_map(|place| {
                                                    place.$field.clone().and_then(|zone_format| {
                                                        Some((
                                                            bcp47_tzid_data.get(&iana)?,
                                                            zone_format,
                                                        ))
                                                    })
                                                })
                                                .collect::<Vec<_>>(),
                                            LocationOrSubRegion::SubRegion(region) => region
                                                .iter()
                                                .filter_map(|(inner_key, place)| {
                                                    let iana = format!("{iana}/{inner_key}");
                                                    place.$field.clone().and_then(|format| {
                                                        Some((bcp47_tzid_data.get(&iana)?, format))
                                                    })
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
                    }),
                })
            }
        }
    };
}

long_short_impls!(
    MetazoneGenericNamesLongV1Marker,
    MetazoneSpecificNamesLongV1Marker,
    long,
);

long_short_impls!(
    MetazoneGenericNamesShortV1Marker,
    MetazoneSpecificNamesShortV1Marker,
    short,
);

fn convert_cldr_zone_variant(
    kvs: BTreeMap<String, String>,
) -> impl Iterator<Item = (ZoneVariant, String)> {
    kvs.into_iter()
        .filter(|(variant, _)| variant != "generic")
        .flat_map(move |(variant, value)| {
            Some((
                match variant.as_str() {
                    "standard" => ZoneVariant::standard(),
                    "daylight" => ZoneVariant::daylight(),
                    _ => return None,
                },
                value,
            ))
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
