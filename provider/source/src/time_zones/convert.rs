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

        fn parse_hour_format(hour_format: &str) -> (Cow<'static, str>, Cow<'static, str>) {
            // e.g. "+HH:mm;-hh:mm" -> ("+H:m", "-h:m")
            let index = hour_format.rfind(';').unwrap();
            let positive = hour_format[0..index]
                .replace("H", "h")
                .replace("hh", "H")
                .replace("mm", "m");
            let negative = hour_format[index + 1..]
                .replace("H", "h")
                .replace("hh", "H")
                .replace("mm", "m");
            (Cow::Owned(positive), Cow::Owned(negative))
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(TimeZoneEssentialsV1 {
                hour_format: parse_hour_format(&time_zone_names.hour_format),
                offset_format: Cow::Owned(time_zone_names.gmt_format.0.clone()),
                offset_zero_format: time_zone_names.gmt_zero_format.clone().into(),
                region_format: Cow::Owned(time_zone_names.region_format.0.clone()),
                region_format_variants: time_zone_names
                    .region_format_variants
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.parse::<TinyStr8>()
                                .expect("Time-zone variant was not compatible with TinyStr8"),
                            Cow::Owned(value.0.clone()),
                        )
                    })
                    .collect(),
                fallback_format: Cow::Owned(time_zone_names.fallback_format.0.clone()),
            }),
        })
    }
}

impl DataProvider<ExemplarCitiesV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ExemplarCitiesV1Marker>, DataError> {
        self.check_req::<ExemplarCitiesV1Marker>(req)?;

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
            payload: DataPayload::from_owned(ExemplarCitiesV1(
                bcp47_tzids
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
                                .and_then(|p| {
                                    let this = &p;
                                    this.exemplar_city.clone()
                                })
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
            )),
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
                    .flat_map(|(iana, periods)| match bcp47_tzid_data.get(&iana) {
                        Some(&bcp47) => periods.iter().map(move |period| {
                            (
                                bcp47,
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
                            )
                        }),
                        None => panic!("Cannot find bcp47 for {iana:?}."),
                    })
                    .collect(),
            )),
        })
    }
}

impl DataProvider<ZoneOffsetPeriodV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ZoneOffsetPeriodV1Marker>, DataError> {
        self.check_req::<ZoneOffsetPeriodV1Marker>(req)?;

        let tzdb = self.tzdb()?.get()?;

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
                                        utc_offset.offset_eighths_of_hour(),
                                        dst_offset_relative.offset_eighths_of_hour(),
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
                    .read_and_parse::<cldr_serde::time_zones::time_zone_names::Resource>(req.id.locale, "timeZoneNames.json")?
                    .main
                    .value
                    .dates
                    .time_zone_names;

                let bcp47_tzid_data = &self.compute_bcp47_tzids_btreemap()?;
                let meta_zone_id_data = &self.compute_meta_zone_ids_btreemap()?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(
                        MetazoneGenericNamesV1 {
                            defaults: match &time_zone_names_resource.metazone {
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
                                                LocationOrSubRegion::Location(place) => {
                                                    match bcp47_tzid_data.get(&iana) {
                                                        Some(bcp47) => place
                                                            .$field
                                                            .clone()
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
                                                                .$field
                                                                .clone()
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
                        },
                    ),
                })
            }
        }

        impl DataProvider<$specific> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$specific>, DataError> {
                self.check_req::<$specific>(req)?;

                let time_zone_names_resource = &self
                    .cldr()?
                    .dates("gregorian")
                    .read_and_parse::<cldr_serde::time_zones::time_zone_names::Resource>(req.id.locale, "timeZoneNames.json")?
                    .main
                    .value
                    .dates
                    .time_zone_names;

                let bcp47_tzid_data = &self.compute_bcp47_tzids_btreemap()?;
                let meta_zone_id_data = &self.compute_meta_zone_ids_btreemap()?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(
                        MetazoneSpecificNamesV1 {
                            defaults: match &time_zone_names_resource.metazone {
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
                                                LocationOrSubRegion::Location(place) => {
                                                    match bcp47_tzid_data.get(&iana) {
                                                        Some(&bcp47) => [place]
                                                            .into_iter()
                                                            .filter_map(|place| {
                                                                place
                                                                    .$field
                                                                    .clone()
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
                                                                .$field
                                                                .clone()
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
                    ),
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
        .map(move |(variant, value)| {
            (
                match variant.as_str() {
                    "standard" => ZoneVariant::standard(),
                    "daylight" => ZoneVariant::daylight(),
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
