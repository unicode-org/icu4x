// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use cldr_serde::time_zones::meta_zones::MetaLocationOrSubRegion;
use cldr_serde::time_zones::meta_zones::ZonePeriod;
use cldr_serde::time_zones::time_zone_names::*;
use core::cmp::Ordering;
use icu::calendar::Date;
use icu::calendar::Iso;
use icu::datetime::provider::time_zones::*;
use icu::locale::{langid, LanguageIdentifier};
use icu::timezone::provider::*;
use icu::timezone::Time;
use icu::timezone::UtcOffset;
use icu::timezone::ZoneVariant;
use icu_provider::prelude::*;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::Saving;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use writeable::Writeable;

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

        let offset_separator = self.load_duration_parts_internal(req)?.2.to_owned().into();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(TimeZoneEssentialsV1 {
                offset_separator,
                offset_pattern: Cow::Owned(time_zone_names.gmt_format.0.clone()),
                offset_zero: time_zone_names.gmt_zero_format.clone().into(),
                // TODO: get this from CLDR
                offset_unknown: time_zone_names
                    .gmt_format
                    .0
                    .interpolate(["+?"])
                    .write_to_string()
                    .into_owned()
                    .into(),
            }),
        })
    }
}

impl SourceDataProvider {
    #[allow(clippy::type_complexity)]
    fn calculate_locations(
        &self,
        req: DataRequest,
    ) -> Result<
        (
            BTreeMap<TimeZoneBcp47Id, String>,
            BTreeMap<TimeZoneBcp47Id, String>,
        ),
        DataError,
    > {
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

        let mut exemplar_cities = bcp47_tzids
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
                        let canonical_alias = aliases.split(' ').next().expect("split non-empty");
                        let mut alias_parts = canonical_alias.split('/');
                        (alias_parts.next() != Some("Etc")).then(|| {
                            alias_parts
                                .next_back()
                                .expect("split non-empty")
                                .replace('_', " ")
                        })
                    })
                    .map(|name| (*bcp47, name))
            })
            .collect::<BTreeMap<_, _>>();

        let primary_zones = self.primary_zones_map()?;

        let primary_zones_values = primary_zones.values().copied().collect::<BTreeSet<_>>();

        let region_display_names = if req.id.locale.is_default() {
            BTreeMap::default()
        } else {
            let regions = &self
                .cldr()?
                .displaynames()
                .read_and_parse::<cldr_serde::displaynames::region::Resource>(
                    req.id.locale,
                    "territories.json",
                )?
                .main
                .value
                .localedisplaynames
                .regions;
            regions
                .iter()
                .filter_map(|(region, value)| {
                    Some((
                        icu::locale::subtags::Region::try_from_str(region).ok()?,
                        value.as_str(),
                    ))
                })
                // Overwrite with short names, as we want to use those
                .chain(regions.iter().filter_map(|(region, value)| {
                    Some((
                        icu::locale::subtags::Region::try_from_str(
                            region.strip_suffix("-alt-short")?,
                        )
                        .ok()?,
                        value.as_str(),
                    ))
                }))
                .filter(|(r, _)| primary_zones_values.contains(r))
                .collect()
        };

        let find_endonym_or_en = |region: icu::locale::subtags::Region| -> Option<&str> {
            let expander = self.cldr().unwrap().extended_locale_expander().unwrap();
            let mut langid = LanguageIdentifier {
                region: Some(region),
                // `und` is `Latn`
                script: Some(icu::locale::subtags::script!("Latn")),
                ..Default::default()
            };
            expander.maximize(&mut langid);
            langid.region = None;
            expander.minimize(&mut langid);
            let locale = langid.into();

            // Avoid logging file-not-found errors
            let regions = &if self
                .cldr()
                .unwrap()
                .displaynames()
                .file_exists(&locale, "territories.json")
                != Ok(true)
            {
                self.cldr()
                    .unwrap()
                    .displaynames()
                    .read_and_parse::<cldr_serde::displaynames::region::Resource>(
                        &langid!("en").into(),
                        "territories.json",
                    )
                    .ok()?
            } else {
                self.cldr()
                    .unwrap()
                    .displaynames()
                    .read_and_parse::<cldr_serde::displaynames::region::Resource>(
                        &locale,
                        "territories.json",
                    )
                    .ok()?
            }
            .main
            .value
            .localedisplaynames
            .regions;

            regions
                .get(&format!("{region}-alt-short"))
                .or_else(|| regions.get(region.as_str()))
                .map(|x| x.as_str())
        };

        let mut locations = BTreeMap::new();

        exemplar_cities.retain(|&k, v| {
            if k.0 == "unk" {
                true
            } else if let Some(region) = primary_zones.get(&k) {
                let region_name = region_display_names
                    .get(region)
                    .copied()
                    .or_else(|| find_endonym_or_en(*region))
                    .unwrap_or(region.as_str())
                    .to_string();
                let retain = &region_name != v;
                locations.insert(k, region_name);
                retain
            } else {
                locations.insert(k, v.clone());
                false
            }
        });

        Ok((locations, exemplar_cities))
    }

    pub(super) fn metazone_period(&self) -> Result<&MetazonePeriodV1<'static>, DataError> {
        let metazones = &self
            .cldr()?
            .core()
            .read_and_parse::<cldr_serde::time_zones::meta_zones::Resource>(
                "supplemental/metaZones.json",
            )?
            .supplemental
            .meta_zones;
        let bcp47_tzid_data = self.iana_to_bcp47_map()?;
        let meta_zone_id_data = self.metazone_to_short_map()?;
        self.cldr()?
            .tz_caches
            .mz_period
            .get_or_init(|| {
                Ok(MetazonePeriodV1(
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
                            let id = bcp47_tzid_data.get(&iana).unwrap();
                        let mz_periods = periods
                            .iter()
                            .map(move |period| {
                                        (
                                        period
                                            .uses_meta_zone
                                            .from
                                            .as_deref()
                                            .map(parse_mzone_from)
                                            .map(|(date, time)| {
                                                (date.to_fixed() - EPOCH) as i32 * 24 * 60
                                                    + (time.hour.number() as i32 * 60
                                                        + time.minute.number() as i32)
                                            })
                                            .unwrap_or_default(),
                                        meta_zone_id_data.get(&period.uses_meta_zone.mzone).copied(),
                                    )
                            })
                                .collect::<Vec<_>>();
                        let offset_periods = self
                            .offset_period()
                            .unwrap()
                            .0
                            .get0(id)
                            .unwrap()
                            .iter1_copied()
                            .map(|(k, v)| {
                                use zerovec::ule::AsULE;
                                (i32::from_unaligned(*k), v)
                            })
                            .collect::<Vec<_>>();

                        let mut out = Vec::new();

                        let mut offset_i = 0;
                        let mut mz_i = 0;
                        let mut start = 0;
                        loop {
                            let has_dst = offset_periods[offset_i].1.1 != 0;
                            let mz_name = mz_periods[mz_i].1;

                            out.push((start, if has_dst { mz_name.map(|n| MetazoneId(n.0.to_ascii_uppercase()))} else { mz_name } ));

                            let next_offset_start = offset_periods.get(offset_i+1).map(|(start, _)| start);
                            let next_mz_start = mz_periods.get(mz_i + 1).map(|(start, _)| start);

                            match (next_offset_start, next_mz_start) {
                                (None, None) => break,
                                (Some(s), None) => {offset_i += 1; start = *s},
                                (None, Some(s)) => { mz_i += 1; start = *s},
                                (Some(s), Some(s2)) if s < s2 => {offset_i += 1; start = *s}
                                (Some(s2), Some(s)) if s < s2 => {mz_i += 1; start = *s}
                                (Some(s), Some(_s2)) /* if s == _s2 */ => {offset_i +=1; mz_i +=1; start = *s}
                            }
                        }

                        // Dedupe consecutive values, keeping lower start time.
                        out.dedup_by_key(|(_, mz)| *mz);

                        out.into_iter().map(move |(start, mz)| (id, start, mz))
                        })
                        .collect(),
                ))
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn offset_period(&self) -> Result<&ZoneOffsetPeriodV1<'static>, DataError> {
        let tzdb = self.tzdb()?.transitions()?;

        self.cldr()?
            .tz_caches
            .offset_period
            .get_or_init(|| {
                Ok(ZoneOffsetPeriodV1(
                    self.bcp47_to_canonical_iana_map()?
                        .iter()
                        .filter_map(|(bcp47, iana)| Some((bcp47, tzdb.get_zoneset(iana)?)))
                        .flat_map(|(bcp47, zoneset)| {
                            let mut data =
                                Vec::<(IsoMinutesSinceEpoch, (UtcOffset, UtcOffset))>::new();

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
                                    // WARNING: This produces a *local* timestamp, i.e. seconds since 1970-01-01 00:00:00 *wall time*,
                                    // even though the docs say that this is since the UNIX epoch (i.e. 1970-01-01 00:00:00 UTC).
                                    // This also assumes `t` uses the same offset as 1970-01-01 00:00:00.
                                    // While the local timestamps are what we want, the offset assumption probably needs fixing (TODO).
                                    .map(|t| (t.to_timestamp() / 60) as IsoMinutesSinceEpoch)
                                    .unwrap_or(IsoMinutesSinceEpoch::MAX);

                                if local_end_time <= 0 {
                                    continue;
                                }

                                match zone_info.saving {
                                    Saving::NoSaving => {
                                        store_offsets(
                                            &mut data,
                                            local_end_time,
                                            zone_info.offset,
                                            0,
                                        );
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

                            // Use start times instead of end times
                            data = data
                                .iter()
                                .copied()
                                .enumerate()
                                .map(|(i, (_, offsets))| {
                                    (data.get(i + 1).map(|d| d.0).unwrap_or(0), offsets)
                                })
                                .collect();

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
                ))
            })
            .as_ref()
            .map_err(|&e| e)
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

        let mut locations = self.calculate_locations(req)?.0;

        if *req.id.locale != DataLocale::default() {
            let und = self.calculate_locations(Default::default())?.0;
            locations.retain(|k, v| und.get(k) != Some(v));
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(LocationsV1 {
                locations: locations.into_iter().collect(),
                pattern_generic: Cow::Owned(time_zone_names.region_format.0.clone()),
                pattern_standard: Cow::Owned(time_zone_names.region_format_st.0.clone()),
                pattern_daylight: Cow::Owned(time_zone_names.region_format_dt.0.clone()),
                pattern_partial_location: Cow::Owned(time_zone_names.fallback_format.0.clone()),
            }),
        })
    }
}

impl DataProvider<ExemplarCitiesV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ExemplarCitiesV1Marker>, DataError> {
        self.check_req::<ExemplarCitiesV1Marker>(req)?;

        let mut exemplars = self.calculate_locations(req)?.1;

        if *req.id.locale != DataLocale::default() {
            let und = self.calculate_locations(Default::default())?.1;
            exemplars.retain(|k, v| und.get(k) != Some(v));
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ExemplarCitiesV1 {
                exemplars: exemplars.into_iter().collect(),
            }),
        })
    }
}

impl DataProvider<MetazonePeriodV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<MetazonePeriodV1Marker>, DataError> {
        self.check_req::<MetazonePeriodV1Marker>(req)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(self.metazone_period()?.clone()),
        })
    }
}

impl DataProvider<ZoneOffsetPeriodV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ZoneOffsetPeriodV1Marker>, DataError> {
        self.check_req::<ZoneOffsetPeriodV1Marker>(req)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(self.offset_period()?.clone()),
        })
    }
}

impl DataProvider<MetazoneGenericNamesLongV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<MetazoneGenericNamesLongV1Marker>, DataError> {
        self.check_req::<MetazoneGenericNamesLongV1Marker>(req)?;

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
        let bcp47_tzid_data = self.iana_to_bcp47_map()?;
        let meta_zone_id_data = self.metazone_to_short_map()?;
        let reverse_metazones = self.reverse_metazones()?;
        let locations = self.calculate_locations(req)?.0;

        let defaults = iter_mz_defaults(time_zone_names_resource, meta_zone_id_data, true)
            .flat_map(|(mz, zf)| zone_variant_fallback(zf).map(move |v| (mz, v)))
            .filter(|&(mz, v)| {
                let dst_tzs = reverse_metazones.get(&MetazoneId(mz.0.to_ascii_uppercase()));
                let std_tzs = reverse_metazones.get(&mz);
                if dst_tzs.is_none() && std_tzs.is_none() {
                    log::warn!("No tzs for {mz:?}");
                    return false;
                };
                dst_tzs
                    .iter()
                    .chain(std_tzs.iter())
                    .copied()
                    .flatten()
                    .any(|tz| {
                        let Some(location) = locations.get(tz) else {
                            return true;
                        };
                        writeable::cmp_utf8(
                            &time_zone_names_resource
                                .region_format
                                .interpolate([location]),
                            v.as_bytes(),
                        ) != Ordering::Equal
                    })
            })
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, true)
            .flat_map(|(tz, zf)| zone_variant_fallback(zf).map(move |v| (tz, v)))
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(MetazoneGenericNamesV1 {
                defaults,
                overrides,
            }),
        })
    }
}

impl DataProvider<MetazoneSpecificNamesLongV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<MetazoneSpecificNamesLongV1Marker>, DataError> {
        self.check_req::<MetazoneSpecificNamesLongV1Marker>(req)?;

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

        let bcp47_tzid_data = self.iana_to_bcp47_map()?;
        let meta_zone_id_data = self.metazone_to_short_map()?;
        let reverse_metazones = self.reverse_metazones()?;
        let locations = self.calculate_locations(req)?.0;

        let defaults = iter_mz_defaults(time_zone_names_resource, meta_zone_id_data, true)
            .flat_map(|(mz, zf)| zone_variant_convert(zf).map(move |(zv, v)| ((mz, zv), v)))
            .filter(|&((mz, zv), v)| {
                let dst_tzs = reverse_metazones.get(&MetazoneId(mz.0.to_ascii_uppercase()));
                let std_tzs = reverse_metazones.get(&mz);

                if dst_tzs.is_none() && std_tzs.is_none() {
                    log::warn!("No tzs for {mz:?}");
                    return false;
                };
                if dst_tzs.is_none() && zv == ZoneVariant::Daylight {
                    return false;
                }
                dst_tzs
                    .iter()
                    .chain(std_tzs.iter())
                    .copied()
                    .flatten()
                    .any(|tz| {
                        let Some(location) = locations.get(tz) else {
                            return true;
                        };
                        if zv == ZoneVariant::Daylight {
                            writeable::cmp_utf8(
                                &time_zone_names_resource
                                    .region_format_dt
                                    .0
                                    .interpolate([location]),
                                v.as_bytes(),
                            ) != Ordering::Equal
                        } else if zv == ZoneVariant::Standard {
                            writeable::cmp_utf8(
                                &time_zone_names_resource
                                    .region_format_st
                                    .0
                                    .interpolate([location]),
                                v.as_bytes(),
                            ) != Ordering::Equal
                        } else {
                            // tilde dep on icu_timezone
                            unreachable!()
                        }
                    })
            })
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, true)
            .flat_map(|(tz, zf)| zone_variant_convert(zf).map(move |(zv, v)| ((tz, zv), v)))
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(MetazoneSpecificNamesV1 {
                defaults,
                overrides,
            }),
        })
    }
}

impl DataProvider<MetazoneGenericNamesShortV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<MetazoneGenericNamesShortV1Marker>, DataError> {
        self.check_req::<MetazoneGenericNamesShortV1Marker>(req)?;

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
        let bcp47_tzid_data = self.iana_to_bcp47_map()?;
        let meta_zone_id_data = self.metazone_to_short_map()?;

        let defaults = iter_mz_defaults(time_zone_names_resource, meta_zone_id_data, false)
            .flat_map(|(mz, zf)| zone_variant_fallback(zf).map(move |v| (mz, v)))
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, false)
            .flat_map(|(tz, zf)| zone_variant_fallback(zf).map(move |v| (tz, v)))
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(MetazoneGenericNamesV1 {
                defaults,
                overrides,
            }),
        })
    }
}
impl DataProvider<MetazoneSpecificNamesShortV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<MetazoneSpecificNamesShortV1Marker>, DataError> {
        self.check_req::<MetazoneSpecificNamesShortV1Marker>(req)?;

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

        let bcp47_tzid_data = self.iana_to_bcp47_map()?;
        let meta_zone_id_data = self.metazone_to_short_map()?;
        let reverse_metazones = self.reverse_metazones()?;

        let defaults = iter_mz_defaults(time_zone_names_resource, meta_zone_id_data, false)
            .flat_map(|(mz, zf)| zone_variant_convert(zf).map(move |(zv, v)| ((mz, zv), v)))
            .filter(|&((mz, zv), _)| {
                let dst_tzs = reverse_metazones.get(&MetazoneId(mz.0.to_ascii_uppercase()));
                let std_tzs = reverse_metazones.get(&mz);

                if dst_tzs.is_none() && std_tzs.is_none() {
                    log::warn!("No tzs for {mz:?}");
                    return false;
                };
                if dst_tzs.is_none() && zv == ZoneVariant::Daylight {
                    return false;
                }
                true
            })
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, false)
            .flat_map(|(tz, zf)| zone_variant_convert(zf).map(move |(zv, v)| ((tz, zv), v)))
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(MetazoneSpecificNamesV1 {
                defaults,
                overrides,
            }),
        })
    }
}

fn iter_mz_defaults<'a>(
    time_zone_names_resource: &'a TimeZoneNames,
    meta_zone_id_data: &'a BTreeMap<String, MetazoneId>,
    is_long: bool,
) -> impl Iterator<Item = (MetazoneId, &'a ZoneFormat)> + 'a {
    time_zone_names_resource
        .metazone
        .as_ref()
        .map(|m| &m.0)
        .unwrap_or({
            static EMPTY: BTreeMap<String, Metazone> = BTreeMap::new();
            &EMPTY
        })
        .iter()
        .filter_map(move |(key, metazone)| {
            Some((*meta_zone_id_data.get(key)?, metazone.long_short(is_long)?))
        })
}

fn iter_mz_overrides<'a>(
    time_zone_names_resource: &'a TimeZoneNames,
    bcp47_tzid_data: &'a BTreeMap<String, TimeZoneBcp47Id>,
    is_long: bool,
) -> impl Iterator<Item = (TimeZoneBcp47Id, &'a ZoneFormat)> {
    time_zone_names_resource
        .zone
        .0
        .iter()
        .flat_map(move |(key, region)| {
            region
                .0
                .iter()
                .flat_map(move |(inner_key, place_or_region)| {
                    let iana = format!("{key}/{inner_key}");
                    let Some(&tz) = bcp47_tzid_data.get(&iana) else {
                        return Default::default();
                    };
                    match place_or_region {
                        LocationOrSubRegion::Location(place) => place
                            .long_short(is_long)
                            .map(|zf| (tz, zf))
                            .into_iter()
                            .collect::<Vec<_>>(),
                        LocationOrSubRegion::SubRegion(region) => region
                            .iter()
                            .filter_map(|(inner_key, place)| {
                                let iana = format!("{iana}/{inner_key}");
                                let Some(&tz) = bcp47_tzid_data.get(&iana) else {
                                    return Default::default();
                                };
                                place.long_short(is_long).map(|zf| (tz, zf))
                            })
                            .collect(),
                    }
                })
        })
}

/// Performs part 1 of type fallback as specified in the UTS-35 spec for TimeZone Goals:
/// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Goals
///
/// Part 2 of type fallback requires access to the IANA TimeZone Database
/// as well as a specific datetime context, so it is not relevant to DataProvider.
fn zone_variant_fallback(zone_format: &ZoneFormat) -> Option<&str> {
    zone_format
        .0
        .get("generic")
        .or_else(|| zone_format.0.get("standard"))
        .map(|s| s.as_str())
}

fn zone_variant_convert(zone_format: &ZoneFormat) -> impl Iterator<Item = (ZoneVariant, &str)> {
    zone_format
        .0
        .iter()
        .filter(|&(variant, _)| variant != "generic")
        .flat_map(move |(variant, value)| {
            Some((
                match variant.as_str() {
                    "standard" => ZoneVariant::Standard,
                    "daylight" => ZoneVariant::Daylight,
                    _ => return None,
                },
                value.as_str(),
            ))
        })
}

fn parse_mzone_from(from: &str) -> (Date<Iso>, Time) {
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
    (
        Date::try_new_iso(year, month, day).unwrap(),
        Time::try_new(hour, minute, 0, 0).unwrap(),
    )
}
