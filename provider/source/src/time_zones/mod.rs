// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::cmp::Ordering;
use core::hash::Hash;
use core::hash::Hasher;
use icu::datetime::provider::time_zones::*;
use icu::locale::subtags::region;
use icu::time::provider::*;
use icu::time::Time;
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::sync::OnceLock;
use twox_hash::XxHash64;

mod convert;
mod names;
mod windows;

type Cache<T> = OnceLock<Result<T, DataError>>;

#[derive(Debug, Default)]
pub(crate) struct Caches {
    iana_to_bcp47: Cache<BTreeMap<String, TimeZone>>,
    bcp47_to_canonical_iana: Cache<BTreeMap<TimeZone, String>>,
    metazone_to_short: Cache<(BTreeMap<String, MetazoneId>, u64)>,
    primary_zones: Cache<BTreeMap<TimeZone, Region>>,
    mz_period: Cache<MetazonePeriod<'static>>,
    offset_period: Cache<<TimeZoneOffsetsV1 as DynamicDataMarker>::DataStruct>,
    reverse_metazones: Cache<BTreeMap<(MetazoneId, MzMembership), Vec<TimeZone>>>,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
enum MzMembership {
    Any,
    Daylight,
}

impl SourceDataProvider {
    fn reverse_metazones(
        &self,
    ) -> Result<&BTreeMap<(MetazoneId, MzMembership), Vec<TimeZone>>, DataError> {
        self.cldr()?
            .tz_caches
            .reverse_metazones
            .get_or_init(|| {
                let mz_period = self.metazone_period()?;
                let offset_periods = self.offset_period()?;

                let mut reverse_metazones =
                    BTreeMap::<(MetazoneId, MzMembership), Vec<TimeZone>>::new();

                for c in mz_period.list.iter0() {
                    let &tz = c.key0();

                    use zerovec::ule::AsULE;
                    let mut mzs = c
                        .into_iter1_copied()
                        .map(|(k, v)| (MinutesSinceEpoch::from_unaligned(*k), v))
                        .peekable();
                    let mut offsets = offset_periods
                        .get0(&tz)
                        .unwrap()
                        .into_iter1_copied()
                        .map(|(k, v)| (MinutesSinceEpoch::from_unaligned(*k), v))
                        .peekable();

                    let mut curr_offset = offsets.next().unwrap();
                    let mut curr_mz = mzs.next().unwrap();

                    let horizon =
                        MinutesSinceEpoch::from((self.timezone_horizon, Time::start_of_day()));

                    while offsets.peek().is_some_and(|&(start, _)| start < horizon) {
                        curr_offset = offsets.next().unwrap();
                    }

                    loop {
                        if let Some(mz) = curr_mz.1 .0 {
                            reverse_metazones
                                .entry((mz, MzMembership::Any))
                                .or_default()
                                .push(tz);
                            // The daylight name is only required if a zone usign this metazone actually observes DST
                            if curr_offset.1 .1 != 0 {
                                reverse_metazones
                                    .entry((mz, MzMembership::Daylight))
                                    .or_default()
                                    .push(tz);
                            }
                        }

                        match (offsets.peek().copied(), mzs.peek().copied()) {
                            (None, None) => break,
                            (Some(_), None) => {
                                curr_offset = offsets.next().unwrap();
                            }
                            (None, Some(_)) => {
                                curr_mz = mzs.next().unwrap();
                            }
                            (Some(o), Some(m)) => match o.0.cmp(&m.0) {
                                Ordering::Less => {
                                    curr_offset = offsets.next().unwrap();
                                }
                                Ordering::Greater => {
                                    curr_mz = mzs.next().unwrap();
                                }
                                Ordering::Equal => {
                                    curr_offset = offsets.next().unwrap();
                                    curr_mz = mzs.next().unwrap();
                                }
                            },
                        }
                    }
                }
                Ok(reverse_metazones)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// Returns a map from time zone long identifier to time zone BCP-47 ID.
    ///
    /// For example: "America/Chicago" to "uschi"
    fn iana_to_bcp47_map(&self) -> Result<&BTreeMap<String, TimeZone>, DataError> {
        self.cldr()?
            .tz_caches
            .iana_to_bcp47
            .get_or_init(|| {
                let bcp47_tzids_resource = &self
                    .cldr()?
                    .bcp47()
                    .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>(
                        "timezone.json",
                    )?
                    .keyword
                    .u
                    .time_zones
                    .values;

                let mut bcp47_tzids = BTreeMap::new();
                for (bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource.iter() {
                    if bcp47_tzid.as_str() == "unk" {
                        // ignore the unknown time zone
                    } else if bcp47_tzid_data.deprecated == Some(true) {
                        // skip
                    } else if let Some(alias) =
                        bcp47_tzid_data.alias.as_ref().filter(|s| !s.is_empty())
                    {
                        for data_value in alias.split(' ') {
                            bcp47_tzids.insert(data_value.to_string(), *bcp47_tzid);
                        }
                    } else {
                        debug_assert!(
                            false,
                            "Could not find aliases for bcp47 time zone: {bcp47_tzid:?}"
                        );
                    }
                }
                Ok(bcp47_tzids)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// Returns a map from BCP-47 ID to a single canonical long identifier.
    ///
    /// For example: "inccu" to "Asia/Kolkata"
    fn bcp47_to_canonical_iana_map(&self) -> Result<&BTreeMap<TimeZone, String>, DataError> {
        self.cldr()?
            .tz_caches
            .bcp47_to_canonical_iana
            .get_or_init(|| {
                let bcp47_tzids_resource = &self
                    .cldr()?
                    .bcp47()
                    .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>(
                        "timezone.json",
                    )?
                    .keyword
                    .u
                    .time_zones
                    .values;

                let mut canonical_tzids = BTreeMap::new();
                for (bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource.iter() {
                    if bcp47_tzid.as_str() == "unk" {
                        // ignore the unknown time zone
                    } else if bcp47_tzid_data.deprecated == Some(true) {
                        // skip
                    } else if let Some(iana) = &bcp47_tzid_data.iana {
                        canonical_tzids.insert(*bcp47_tzid, iana.to_owned());
                    } else if let Some(alias) =
                        bcp47_tzid_data.alias.as_ref().filter(|s| !s.is_empty())
                    {
                        canonical_tzids
                            .insert(*bcp47_tzid, alias.split(' ').next().unwrap().to_owned());
                    } else {
                        debug_assert!(
                            false,
                            "Could not find canonical IANA for bcp47 time zone: {bcp47_tzid:?}"
                        );
                    }
                }
                Ok(canonical_tzids)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// Returns a map from metazone long identifier to metazone BCP-47 ID.
    ///
    /// For example: "America_Central" to "amce"
    fn metazone_to_id_map(&self) -> Result<(&BTreeMap<String, MetazoneId>, u64), DataError> {
        self.cldr()?
            .tz_caches
            .metazone_to_short
            .get_or_init(|| {
                let meta_zone_ids_resource = &self
                    .cldr()?
                    .core()
                    .read_and_parse::<cldr_serde::time_zones::meta_zones::Resource>(
                        "supplemental/metaZones.json",
                    )?
                    .supplemental
                    .meta_zones
                    .meta_zone_ids
                    .0;

                let mut hash = XxHash64::with_seed(0);
                meta_zone_ids_resource.len().hash(&mut hash);

                Ok((
                    meta_zone_ids_resource
                        .iter()
                        .enumerate()
                        .map(|(idx, (_short_id, alias))| {
                            alias.long_id.hash(&mut hash);
                            (
                                alias.long_id.clone(),
                                MetazoneId::new(idx as u8 + 1).unwrap(),
                            )
                        })
                        .collect(),
                    hash.finish(),
                ))
            })
            .as_ref()
            .map(|(map, checksum)| (map, *checksum))
            .map_err(|&e| e)
    }

    fn primary_zones_map(&self) -> Result<&BTreeMap<TimeZone, Region>, DataError> {
        self.cldr()?
            .tz_caches
            .primary_zones
            .get_or_init(|| {
                let primary_zones = self
                    .cldr()?
                    .core()
                    .read_and_parse::<cldr_serde::time_zones::primary_zones::Resource>(
                        "supplemental/primaryZones.json",
                    )?
                    .supplemental
                    .primary_zones
                    .iter()
                    .map(|(&region, iana)| (iana, region))
                    .collect::<BTreeMap<_, _>>();

                let bcp47_tzids_resource = &self
                    .cldr()?
                    .bcp47()
                    .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>(
                        "timezone.json",
                    )?
                    .keyword
                    .u
                    .time_zones
                    .values;

                let mut regions_to_zones: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();

                for (&bcp47_tzid, bcp47_tzid_data) in bcp47_tzids_resource {
                    regions_to_zones
                        .entry(match bcp47_tzid.as_str() {
                            // backfill since this data is not in 47 yet
                            "ancur" => region!("CW"),
                            "fimhq" => region!("AX"),
                            "gpmsb" => region!("MF"),
                            "gpsbh" => region!("BL"),
                            "gazastrp" | "hebron" => region!("PS"),
                            "jeruslm" => region!("IL"),
                            _ => {
                                if bcp47_tzid_data.deprecated == Some(true) {
                                    continue;
                                } else if let Some(region) = bcp47_tzid_data.region {
                                    region
                                } else if bcp47_tzid.0.len() != 5 {
                                    // Length-5 ID without override, no region
                                    continue;
                                } else {
                                    bcp47_tzid.as_str()[0..2].parse().unwrap()
                                }
                            }
                        })
                        .or_default()
                        .insert(bcp47_tzid);
                }

                let single_zone_regions = regions_to_zones
                    .iter()
                    .filter_map(|(region, zones)| {
                        (zones.len() == 1).then_some((zones.first().unwrap(), region))
                    })
                    .collect::<BTreeMap<_, _>>();

                Ok(self
                    .bcp47_to_canonical_iana_map()?
                    .iter()
                    .filter_map(|(bcp47, canonical_iana)| {
                        Some((
                            *bcp47,
                            primary_zones
                                .get(canonical_iana)
                                .copied()
                                .or_else(|| single_zone_regions.get(bcp47).copied().copied())?,
                        ))
                    })
                    .collect())
            })
            .as_ref()
            .map_err(|&e| e)
    }
}

macro_rules! impl_iterable_data_provider {
    ($($marker:ident),+) => {
        $(
            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    Ok(self
                        .cldr()?
                        .dates("gregorian")
                        .list_locales()?
                        .map(DataIdentifierCow::from_locale)
                        .collect())
                }
            }
        )+
    };
}

impl_iterable_data_provider!(
    TimezoneNamesEssentialsV1,
    TimezoneNamesLocationsOverrideV1,
    TimezoneNamesLocationsRootV1,
    TimezoneNamesCitiesOverrideV1,
    TimezoneNamesCitiesRootV1,
    TimezoneNamesGenericLongV1,
    TimezoneNamesStandardLongV1,
    TimezoneNamesGenericShortV1,
    TimezoneNamesSpecificLongV1,
    TimezoneNamesSpecificShortV1
);

impl IterableDataProviderCached<TimezoneMetazonePeriodsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl IterableDataProviderCached<TimeZoneOffsetsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use icu::locale::subtags::subtag;
    use icu::time::zone::TimeZoneVariant;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu::locale::langid;

        let provider = SourceDataProvider::new_testing();

        let en = langid!("en").into();
        let en = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&en),
            ..Default::default()
        };

        let fr = langid!("fr").into();
        let fr = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&fr),
            ..Default::default()
        };

        let time_zone_formats: DataResponse<TimezoneNamesEssentialsV1> = provider.load(en).unwrap();
        assert_eq!("GMT", time_zone_formats.payload.get().offset_zero);
        assert_eq!("GMT+?", time_zone_formats.payload.get().offset_unknown);

        let locations_root: DataResponse<TimezoneNamesLocationsRootV1> = provider.load(en).unwrap();
        assert_eq!(
            "Pohnpei",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZone(subtag!("fmpni")))
                .unwrap()
        );
        assert_eq!(
            "Ireland",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZone(subtag!("iedub")))
                .unwrap()
        );

        let locations: DataResponse<TimezoneNamesLocationsOverrideV1> = provider.load(fr).unwrap();
        assert_eq!(
            "Italie",
            locations
                .payload
                .get()
                .locations
                .get(&TimeZone(subtag!("itrom")))
                .unwrap()
        );

        let metazone_period = provider.metazone_period().unwrap();

        let metazone_now = |bcp47| {
            metazone_period
                .list
                .get0(&bcp47)
                .unwrap()
                .iter1_copied()
                .last()
                .unwrap()
                .1
                 .0
                .unwrap()
        };

        let generic_names_long: DataPayload<TimezoneNamesGenericLongV1> =
            provider.load(en).unwrap().payload;

        let generic_standard_names_long: DataPayload<TimezoneNamesStandardLongV1> =
            provider.load(en).unwrap().payload;

        assert_eq!(
            "Australian Central Western Time",
            generic_names_long
                .get()
                .defaults
                .get(&metazone_now(TimeZone(subtag!("aueuc"))))
                .or_else(|| generic_standard_names_long
                    .get()
                    .defaults
                    .get(&metazone_now(TimeZone(subtag!("aueuc")))))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            generic_names_long
                .get()
                .overrides
                .get(&TimeZone(subtag!("utc")))
                .or_else(|| generic_standard_names_long
                    .get()
                    .overrides
                    .get(&TimeZone(subtag!("utc"))))
                .unwrap()
        );

        let specific_names_long: DataPayload<TimezoneNamesSpecificLongV1> =
            provider.load(en).unwrap().payload;
        assert_eq!(
            "Australian Central Western Standard Time",
            specific_names_long
                .get()
                .defaults
                .get(&(
                    metazone_now(TimeZone(subtag!("aueuc"))),
                    TimeZoneVariant::Standard
                ))
                .or_else(|| generic_standard_names_long
                    .get()
                    .defaults
                    .get(&metazone_now(TimeZone(subtag!("aueuc")))))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            specific_names_long
                .get()
                .overrides
                .get(&(TimeZone(subtag!("utc")), TimeZoneVariant::Standard))
                .or_else(|| generic_standard_names_long
                    .get()
                    .overrides
                    .get(&TimeZone(subtag!("utc"))))
                .unwrap()
        );

        let generic_names_short: DataResponse<TimezoneNamesGenericShortV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "PT",
            generic_names_short
                .payload
                .get()
                .defaults
                .get(&metazone_now(TimeZone(subtag!("uslax"))))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            generic_names_short
                .payload
                .get()
                .overrides
                .get(&TimeZone(subtag!("utc")))
                .unwrap()
        );

        let specific_names_short: DataResponse<TimezoneNamesSpecificShortV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "PDT",
            specific_names_short
                .payload
                .get()
                .defaults
                .get(&(
                    metazone_now(TimeZone(subtag!("uslax"))),
                    TimeZoneVariant::Daylight
                ))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            specific_names_short
                .payload
                .get()
                .overrides
                .get(&(TimeZone(subtag!("utc")), TimeZoneVariant::Standard))
                .unwrap()
        );
    }
}
