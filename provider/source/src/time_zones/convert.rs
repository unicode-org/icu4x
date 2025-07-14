// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::MzMembership;
use crate::cldr_serde;
use crate::SourceDataProvider;
use cldr_serde::time_zones::time_zone_names::*;
use core::cmp::Ordering;
use icu::datetime::provider::time_zones::*;
use icu::locale::LanguageIdentifier;
use icu::time::provider::*;
use icu::time::zone::TimeZoneVariant;
use icu::time::zone::UtcOffset;
use icu::time::zone::VariantOffsets;
use icu::time::zone::ZoneNameTimestamp;
use icu::time::ZonedDateTime;
use icu_provider::prelude::icu_locale_core::subtags::Language;
use icu_provider::prelude::*;
use parse_zoneinfo::line::ChangeTime;
use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::Saving;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use writeable::Writeable;
use zerovec::ule::NichedOption;

impl DataProvider<TimezoneNamesEssentialsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<TimezoneNamesEssentialsV1>, DataError> {
        self.check_req::<TimezoneNamesEssentialsV1>(req)?;

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
            payload: DataPayload::from_owned(TimeZoneEssentials {
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
    #[expect(clippy::type_complexity)]
    fn calculate_locations(
        &self,
        locale: &DataLocale,
    ) -> Result<(BTreeMap<TimeZone, String>, BTreeMap<TimeZone, String>), DataError> {
        let time_zone_names = &self
            .cldr()?
            .dates("gregorian")
            .read_and_parse::<cldr_serde::time_zones::time_zone_names::Resource>(
                locale,
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
            .filter_map(|(&bcp47, bcp47_tzid_data)| {
                Some((
                    bcp47,
                    bcp47_tzid_data
                        .alias
                        .as_ref()?
                        .split(' ')
                        .next()
                        .unwrap()
                        .to_owned(),
                ))
            })
            .chain(self.future_zones()?.map(|(a, b)| (b, a)))
            .filter_map(|(bcp47, canonical_alias)| {
                // Etc zones don't have locations, with the exception of Unknown, which we still want to skip in root
                if canonical_alias.starts_with("Etc/")
                    && (canonical_alias != "Etc/Unknown" || locale.is_unknown())
                {
                    return None;
                }

                let mut alias_parts = canonical_alias.split('/');
                let exemplar = time_zone_names
                    .zone
                    .0
                    .get(alias_parts.next().expect("split non-empty"))
                    .and_then(|x| x.0.get(alias_parts.next()?))
                    .and_then(|x| match x {
                        LocationOrSubRegion::Location(place) => place.exemplar_city.clone(),
                        LocationOrSubRegion::SubRegion(region) => {
                            region.get(alias_parts.next()?)?.exemplar_city.clone()
                        }
                    })
                    .unwrap_or_else(|| {
                        canonical_alias
                            .split('/')
                            .next_back()
                            .expect("split non-empty")
                            .replace('_', " ")
                    });
                Some((bcp47, exemplar))
            })
            .collect::<BTreeMap<_, _>>();

        let primary_zones = self.primary_zones_map()?;

        let primary_zones_values = primary_zones.values().copied().collect::<BTreeSet<_>>();

        let region_display_names = if !self
            .cldr()?
            .displaynames()
            .file_exists(locale, "territories.json")?
        {
            BTreeMap::default()
        } else {
            let regions = &self
                .cldr()?
                .displaynames()
                .read_and_parse::<cldr_serde::displaynames::region::Resource>(
                    locale,
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

        let mut locations = BTreeMap::new();

        exemplar_cities.retain(|&k, v| {
            if k.is_unknown() {
                true
            } else if let Some(region) = primary_zones.get(&k) {
                if let Some(region_name) = region_display_names.get(region) {
                    locations.insert(k, region_name.to_string());
                    region_name != v
                } else {
                    locations.insert(k, v.clone());
                    false
                }
            } else {
                locations.insert(k, v.clone());
                false
            }
        });

        Ok((locations, exemplar_cities))
    }

    pub(crate) fn offset_period(
        &self,
    ) -> Result<&<TimezoneVariantsOffsetsV1 as DynamicDataMarker>::DataStruct, DataError> {
        let tzdb = self.tzdb()?.transitions()?;

        self.cldr()?
            .tz_caches
            .offset_period
            .get_or_init(|| {
                Ok(self
                    .bcp47_to_canonical_iana_map()?
                    .iter()
                    .filter_map(|(bcp47, iana)| Some((bcp47, tzdb.get_zoneset(iana)?)))
                    .flat_map(|(bcp47, zoneset)| {
                        let mut data = Vec::<(ZoneNameTimestamp, VariantOffsets)>::new();

                        fn store_offsets(
                            data: &mut Vec<(ZoneNameTimestamp, VariantOffsets)>,
                            end_time: ZoneNameTimestamp,
                            mut utc_offset: i64,
                            dst_offset_relative: i64,
                        ) {
                            // Africa/Monrovia used -00:44:30 pre-1972. We cannot represent this, so we set it to -00:45
                            if utc_offset == -2670 {
                                utc_offset = -2700
                            }

                            data.push((
                                end_time,
                                {
                                    let mut vs = VariantOffsets::from_standard(UtcOffset::from_seconds_unchecked(utc_offset as i32));
                                    if dst_offset_relative != 0 {
                                        vs.daylight = Some(UtcOffset::from_seconds_unchecked(utc_offset as i32 + dst_offset_relative as i32));
                                    }
                                    vs
                                }
                            ));
                        }

                        for zone_info in zoneset.iter() {
                            let local_end_time = match zone_info.end_time {
                                // Skip transitions before the UNIX Epoch
                                Some(t) if t.to_timestamp(zone_info.offset, 0) < 0 => continue,
                                // None means the rules are in effect "until the end of time"
                                None => ZoneNameTimestamp::far_in_future(),
                                // This assumes DST is not active at the zone change
                                Some(t) => {
                                    let epoch_seconds = t.to_timestamp(zone_info.offset, 0);
                                    ZoneNameTimestamp::from_zoned_date_time_iso(ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
                                        epoch_seconds * 1000,
                                        UtcOffset::zero(),
                                    ))
                                }
                            };

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
                                    let ruleset = &tzdb.rulesets[rules];
                                    let mut rules = ruleset
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
                                                        let epoch_seconds = rule.absolute_datetime(y, zone_info.offset, rule.time_to_add);
                                                        ZoneNameTimestamp::from_zoned_date_time_iso(ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
                                                            epoch_seconds * 1000,
                                                            UtcOffset::zero()
                                                        ))
                                                    }
                                                },
                                                rule.time_to_add,
                                            )
                                        })
                                        .filter(|&(rule_end_time, _)| {
                                            // Discard rules from before this zoneinfo (or before the epoch)
                                            rule_end_time
                                                > data
                                                    .last()
                                                    .map(|&(prev_end_time, _)| prev_end_time)
                                                    .unwrap_or(ZoneNameTimestamp::far_in_past())
                                        })
                                        .collect::<Vec<_>>();

                                    // Dedup consecutive rules, keeping higher end time
                                    rules.sort_by_key(|(end_time, _)| core::cmp::Reverse(*end_time));
                                    rules.dedup_by_key(|(_, offsets)| *offsets);
                                    rules.reverse();

                                    if rules.is_empty() {
                                        // No rule applies
                                        store_offsets(
                                            &mut data,
                                            local_end_time,
                                            zone_info.offset,
                                            0,
                                        );
                                    } else {
                                        for &(rule_end_time, dst_offset_relative) in &rules {
                                            store_offsets(
                                                &mut data,
                                                // use the rule until the end of the zone info, or the end of the rule
                                                // whichever is earlier
                                                core::cmp::min(rule_end_time, local_end_time),
                                                zone_info.offset,
                                                dst_offset_relative,
                                            );
                                        }

                                        let end_year = match zone_info.end_time {
                                            None => i64::MAX,
                                            Some(
                                                ChangeTime::UntilYear(Year::Number(y))
                                                | ChangeTime::UntilMonth(Year::Number(y), Month::January)
                                                | ChangeTime::UntilDay(Year::Number(y), Month::January, DaySpec::Ordinal(1))) => y - 1,
                                            Some(c) => c.year(),
                                        };

                                        if !ruleset.iter().any(|r| r.applies_to_year(end_year)) {
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
                        data.sort_by_key(|(end_time, _)| core::cmp::Reverse(*end_time));
                        // Dedup consecutive offsets, keeping higher end time
                        data.dedup_by_key(|(_, offsets)| *offsets);

                        // Use start times instead of end times
                        data
                            .iter()
                            .copied()
                            .enumerate()
                            .map(|(i, (_, offsets))| {
                                (bcp47, data.get(i + 1).map(|d| d.0).unwrap_or(ZoneNameTimestamp::far_in_past()), offsets)
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                    })
                    .collect())
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn dedupe_group(&self, locale: DataLocale) -> Result<DataLocale, DataError> {
        let mut group = LanguageIdentifier::from((locale.language, locale.script, locale.region));
        self.cldr()?
            .extended_locale_expander()?
            .maximize(&mut group);
        group.language = Language::UNKNOWN;
        group.region = Default::default();
        self.cldr()?
            .extended_locale_expander()?
            .maximize(&mut group);
        self.cldr()?
            .extended_locale_expander()?
            .minimize_favor_script(&mut group);
        let group = DataLocale::from(group);
        if self
            .cldr()
            .unwrap()
            .displaynames()
            .file_exists(&group, "territories.json")
            != Ok(true)
            || self
                .cldr()
                .unwrap()
                .dates("gregorian")
                .file_exists(&group, "timeZoneNames.json")
                != Ok(true)
        {
            Ok(locale)
        } else {
            Ok(group)
        }
    }
}

impl DataProvider<TimezoneNamesLocationsOverrideV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesLocationsOverrideV1>, DataError> {
        self.check_req::<TimezoneNamesLocationsOverrideV1>(req)?;

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

        let mut locations = self.calculate_locations(req.id.locale)?.0;

        let base = DataProvider::<TimezoneNamesLocationsRootV1>::load(&self, req)?.payload;

        locations.retain(|k, v| base.get().locations.get(k) != Some(v));

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(Locations {
                locations: locations.into_iter().collect(),
                pattern_generic: Cow::Owned(time_zone_names.region_format.0.clone()),
                pattern_standard: Cow::Owned(time_zone_names.region_format_st.0.clone()),
                pattern_daylight: Cow::Owned(time_zone_names.region_format_dt.0.clone()),
                pattern_partial_location: Cow::Owned(time_zone_names.fallback_format.0.clone()),
            }),
        })
    }
}

impl DataProvider<TimezoneNamesLocationsRootV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesLocationsRootV1>, DataError> {
        self.check_req::<TimezoneNamesLocationsOverrideV1>(req)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(Locations {
                locations: self
                    .calculate_locations(&self.dedupe_group(*req.id.locale)?)?
                    .0
                    .into_iter()
                    .collect(),
                pattern_generic: Default::default(),
                pattern_standard: Default::default(),
                pattern_daylight: Default::default(),
                pattern_partial_location: Default::default(),
            }),
        })
    }
}

impl DataProvider<TimezoneNamesCitiesOverrideV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesCitiesOverrideV1>, DataError> {
        self.check_req::<TimezoneNamesCitiesOverrideV1>(req)?;

        let mut exemplars = self.calculate_locations(req.id.locale)?.1;

        let base = DataProvider::<TimezoneNamesCitiesRootV1>::load(&self, req)?.payload;

        exemplars.retain(|k, v| base.get().exemplars.get(k) != Some(v));

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ExemplarCities {
                exemplars: exemplars.into_iter().collect(),
            }),
        })
    }
}

impl DataProvider<TimezoneNamesCitiesRootV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<TimezoneNamesCitiesRootV1>, DataError> {
        self.check_req::<TimezoneNamesCitiesRootV1>(req)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ExemplarCities {
                exemplars: self
                    .calculate_locations(&self.dedupe_group(*req.id.locale)?)?
                    .1
                    .into_iter()
                    .collect(),
            }),
        })
    }
}

impl DataProvider<TimezoneMetazonePeriodsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<TimezoneMetazonePeriodsV1>, DataError> {
        self.check_req::<TimezoneMetazonePeriodsV1>(req)?;

        let metazones = self.metazones()?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(metazones.checksum),
            payload: DataPayload::from_owned(MetazonePeriod {
                list: self
                    .metazones()?
                    .periods
                    .iter()
                    .flat_map(|(tz, ps)| ps.iter().map(move |&(t, mz)| (tz, t, NichedOption(mz))))
                    .collect(),
            }),
        })
    }
}

impl DataProvider<TimezoneVariantsOffsetsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<TimezoneVariantsOffsetsV1>, DataError> {
        self.check_req::<TimezoneVariantsOffsetsV1>(req)?;

        // Collapse periods a bit more. This cannot be done in offset_period, as it extends DST usage to periods
        // that didn't actually use DST, which interferes with excluding display names beyond the metazone horizon.
        let data = self
            .offset_period()?
            .iter0()
            .flat_map(|c| {
                let tz = *c.key0();
                use zerovec::ule::AsULE;
                let mut offsets = c
                    .iter1_copied()
                    .map(|(k, v)| (ZoneNameTimestamp::from_unaligned(*k), v))
                    .collect::<Vec<_>>();
                offsets.dedup_by(|(_, a), (_, b)| {
                    if a.standard != b.standard {
                        return false;
                    }
                    match (a.daylight, b.daylight) {
                        (None, None) => true,
                        (Some(a), Some(b)) => a == b,
                        // It's fine if one period doesn't use DST,
                        (Some(a), None) => {
                            b.daylight = Some(a);
                            true
                        }
                        (None, Some(b)) => {
                            a.daylight = Some(b);
                            true
                        }
                    }
                });
                offsets.into_iter().map(move |(t, o)| (tz, t, o))
            })
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl DataProvider<TimezoneNamesGenericLongV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesGenericLongV1>, DataError> {
        self.check_req::<TimezoneNamesGenericLongV1>(req)?;

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
        let metazones = self.metazones()?;
        let locations = self.calculate_locations(req.id.locale)?.0;

        let defaults = iter_mz_defaults(time_zone_names_resource, &metazones.ids, true)
            .filter_map(|(mz, zf)| {
                let v = zf.0.get("generic")?.as_str();

                // The generic name will be used for all zones using this metazone
                let tzs = metazones.reverse.get(&(mz, MzMembership::Any))?;

                let same_as_location = tzs.iter().all(|tz| {
                    let Some(location) = locations.get(tz) else {
                        return false;
                    };
                    writeable::cmp_utf8(
                        &time_zone_names_resource
                            .region_format
                            .interpolate([location]),
                        v.as_bytes(),
                    ) == Ordering::Equal
                });

                if same_as_location {
                    None
                } else {
                    Some((mz, v))
                }
            })
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, true)
            .filter_map(|(tz, zf)| Some((tz, zf.0.get("generic")?.as_str())))
            .collect();

        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(metazones.checksum),
            payload: DataPayload::from_owned(MetazoneGenericNames {
                defaults,
                overrides,
            }),
        })
    }
}

impl DataProvider<TimezoneNamesStandardLongV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesStandardLongV1>, DataError> {
        self.check_req::<TimezoneNamesGenericLongV1>(req)?;

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
        let metazones = self.metazones()?;
        let locations = self.calculate_locations(req.id.locale)?.0;

        let defaults = iter_mz_defaults(time_zone_names_resource, &metazones.ids, true)
            .filter_map(|(mz, zf)| {
                // Add the standard name if the generic name does not exist
                let v = (!zf.0.contains_key("generic"))
                    .then(|| zf.0.get("standard"))
                    .flatten()?
                    .as_str();

                // The standard name will be used for all zones using this metazone
                let tzs = metazones.reverse.get(&(mz, MzMembership::Any))?;

                let same_as_location = tzs.iter().all(|tz| {
                    let Some(location) = locations.get(tz) else {
                        return false;
                    };
                    writeable::cmp_utf8(
                        &time_zone_names_resource
                            .region_format
                            .interpolate([location]),
                        v.as_bytes(),
                    ) == Ordering::Equal
                });

                if same_as_location {
                    None
                } else {
                    Some((mz, v))
                }
            })
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, true)
            .filter_map(|(tz, zf)| Some((tz, zf.0.get("standard")?.as_str())))
            .collect();

        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(metazones.checksum),
            payload: DataPayload::from_owned(MetazoneGenericNames {
                defaults,
                overrides,
            }),
        })
    }
}

impl DataProvider<TimezoneNamesSpecificLongV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesSpecificLongV1>, DataError> {
        self.check_req::<TimezoneNamesSpecificLongV1>(req)?;

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
        let metazones = self.metazones()?;
        let locations = &self.calculate_locations(req.id.locale)?.0;

        let mut defaults = iter_mz_defaults(time_zone_names_resource, &metazones.ids, true)
            .flat_map(move |(mz, zf)| {
                variant_convert(zf).flat_map(move |(zv, v)| {
                    let tzs = metazones.reverse.get(&(
                        mz,
                        if zv == TimeZoneVariant::Daylight {
                            // The daylight name will only be used by zones that use DST
                            MzMembership::Daylight
                        } else {
                            // The standard name will be used by all zones
                            MzMembership::Any
                        },
                    ))?;

                    let same_as_specific_location = tzs.iter().all(|tz| {
                        let Some(location) = locations.get(tz) else {
                            return false;
                        };
                        writeable::cmp_utf8(
                            &if zv == TimeZoneVariant::Daylight {
                                &time_zone_names_resource.region_format_dt
                            } else {
                                &time_zone_names_resource.region_format_st
                            }
                            .0
                            .interpolate([location]),
                            v.as_bytes(),
                        ) == Ordering::Equal
                    });
                    if same_as_specific_location {
                        // Deduplicate against specific location format
                        None
                    } else if zv == TimeZoneVariant::Standard && !zf.0.contains_key("generic") {
                        // Deduplicate against GenericStandard
                        Some(((mz, zv), ""))
                    } else {
                        Some(((mz, zv), v))
                    }
                })
            })
            .collect::<Vec<_>>();
        let mut use_standard = BTreeSet::new();
        defaults.retain(|&((mz, zv), v)| {
            if v.is_empty() && zv == TimeZoneVariant::Standard {
                use_standard.insert(mz);
                false
            } else {
                true
            }
        });
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, true)
            .flat_map(|(tz, zf)| variant_convert(zf).map(move |(zv, v)| ((tz, zv), v)))
            .collect();

        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(metazones.checksum),
            payload: DataPayload::from_owned(MetazoneSpecificNames {
                defaults: defaults.into_iter().collect(),
                overrides,
                use_standard: use_standard.into_iter().collect(),
            }),
        })
    }
}

impl DataProvider<TimezoneNamesGenericShortV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesGenericShortV1>, DataError> {
        self.check_req::<TimezoneNamesGenericShortV1>(req)?;

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
        let metazones = self.metazones()?;

        let defaults = iter_mz_defaults(time_zone_names_resource, &metazones.ids, false)
            .flat_map(|(mz, zf)| variant_fallback(zf).map(move |v| (mz, v)))
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, false)
            .flat_map(|(tz, zf)| variant_fallback(zf).map(move |v| (tz, v)))
            .collect();

        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(metazones.checksum),
            payload: DataPayload::from_owned(MetazoneGenericNames {
                defaults,
                overrides,
            }),
        })
    }
}
impl DataProvider<TimezoneNamesSpecificShortV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TimezoneNamesSpecificShortV1>, DataError> {
        self.check_req::<TimezoneNamesSpecificShortV1>(req)?;

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
        let metazones = self.metazones()?;

        let defaults = iter_mz_defaults(time_zone_names_resource, &metazones.ids, false)
            .flat_map(|(mz, zf)| variant_convert(zf).map(move |(zv, v)| ((mz, zv), v)))
            .collect();
        let overrides = iter_mz_overrides(time_zone_names_resource, bcp47_tzid_data, false)
            .flat_map(|(tz, zf)| variant_convert(zf).map(move |(zv, v)| ((tz, zv), v)))
            .collect();

        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(metazones.checksum),
            payload: DataPayload::from_owned(MetazoneSpecificNames {
                defaults,
                overrides,
                use_standard: Default::default(),
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
    bcp47_tzid_data: &'a BTreeMap<String, TimeZone>,
    is_long: bool,
) -> impl Iterator<Item = (TimeZone, &'a ZoneFormat)> {
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
fn variant_fallback(zone_format: &ZoneFormat) -> Option<&str> {
    zone_format
        .0
        .get("generic")
        .or_else(|| zone_format.0.get("standard"))
        .map(|s| s.as_str())
}

fn variant_convert(zone_format: &ZoneFormat) -> impl Iterator<Item = (TimeZoneVariant, &str)> {
    zone_format
        .0
        .iter()
        .filter(|&(variant, _)| variant != "generic")
        .flat_map(move |(variant, value)| {
            Some((
                match variant.as_str() {
                    "standard" => TimeZoneVariant::Standard,
                    "daylight" => TimeZoneVariant::Daylight,
                    _ => return None,
                },
                value.as_str(),
            ))
        })
}
