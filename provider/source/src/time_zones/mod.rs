// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::hash::Hash;
use core::hash::Hasher;
use icu::datetime::provider::time_zones::*;
use icu::time::provider::*;
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
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
    offset_period: Cache<ZoneOffsetPeriod<'static>>,
    reverse_metazones: Cache<BTreeMap<MetazoneId, Vec<TimeZone>>>,
}

impl SourceDataProvider {
    fn reverse_metazones(&self) -> Result<&BTreeMap<MetazoneId, Vec<TimeZone>>, DataError> {
        self.cldr()?
            .tz_caches
            .reverse_metazones
            .get_or_init(|| {
                let mz_period = self.metazone_period()?;
                let mut reverse_metazones = BTreeMap::<MetazoneId, Vec<TimeZone>>::new();
                for cursor in mz_period.list.iter0() {
                    let tz = *cursor.key0();
                    for mz in cursor.iter1_copied().flat_map(|(_, mz)| mz) {
                        reverse_metazones.entry(mz).or_default().push(tz);
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
                    } else if Some(true) == bcp47_tzid_data.deprecated {
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
                    } else if Some(true) == bcp47_tzid_data.deprecated {
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

                let bcp47_tzids = self.bcp47_to_canonical_iana_map()?;

                let zone_tab = self.tzdb()?.zone_tab()?;
                let mut zones_per_region: BTreeMap<icu::locale::subtags::Region, usize> =
                    BTreeMap::new();

                for &region in bcp47_tzids.values().flat_map(|iana| zone_tab.get(iana)) {
                    *zones_per_region.entry(region).or_default() += 1;
                }

                Ok(bcp47_tzids
                    .iter()
                    .filter_map(|(bcp47, canonical_iana)| {
                        Some((
                            *bcp47,
                            primary_zones.get(canonical_iana).copied().or_else(|| {
                                let region = *zone_tab.get(canonical_iana)?;
                                if zones_per_region.get(&region).copied().unwrap_or_default() > 1 {
                                    return None;
                                }
                                Some(region)
                            })?,
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
    TimeZoneEssentialsV1,
    LocationsV1,
    LocationsRootV1,
    ExemplarCitiesV1,
    ExemplarCitiesRootV1,
    MetazoneGenericNamesLongV1,
    MetazoneGenericNamesShortV1,
    MetazoneSpecificNamesLongV1,
    MetazoneSpecificNamesShortV1
);

impl IterableDataProviderCached<MetazonePeriodV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl IterableDataProviderCached<ZoneOffsetPeriodV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use icu::time::zone::TimeZoneVariant;
    use tinystr::tinystr;

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

        let time_zone_formats: DataResponse<TimeZoneEssentialsV1> = provider.load(en).unwrap();
        assert_eq!("GMT", time_zone_formats.payload.get().offset_zero);
        assert_eq!("GMT+?", time_zone_formats.payload.get().offset_unknown);

        let locations_root: DataResponse<LocationsRootV1> = provider.load(en).unwrap();
        assert_eq!(
            "Pohnpei",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZone(tinystr!(8, "fmpni")))
                .unwrap()
        );
        assert_eq!(
            "Ireland",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZone(tinystr!(8, "iedub")))
                .unwrap()
        );

        let locations: DataResponse<LocationsV1> = provider.load(fr).unwrap();
        assert_eq!(
            "Italie",
            locations
                .payload
                .get()
                .locations
                .get(&TimeZone(tinystr!(8, "itrom")))
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

        let generic_names_long: DataResponse<MetazoneGenericNamesLongV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "Australian Central Western Time",
            generic_names_long
                .payload
                .get()
                .defaults
                .get(&metazone_now(TimeZone(tinystr!(8, "aueuc"))))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            generic_names_long
                .payload
                .get()
                .overrides
                .get(&TimeZone(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_long: DataResponse<MetazoneSpecificNamesLongV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "Australian Central Western Standard Time",
            specific_names_long
                .payload
                .get()
                .defaults
                .get(&(
                    metazone_now(TimeZone(tinystr!(8, "aueuc"))),
                    TimeZoneVariant::Standard
                ))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            specific_names_long
                .payload
                .get()
                .overrides
                .get(&(TimeZone(tinystr!(8, "utc")), TimeZoneVariant::Standard))
                .unwrap()
        );

        let generic_names_short: DataResponse<MetazoneGenericNamesShortV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "PT",
            generic_names_short
                .payload
                .get()
                .defaults
                .get(&metazone_now(TimeZone(tinystr!(8, "uslax"))))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            generic_names_short
                .payload
                .get()
                .overrides
                .get(&TimeZone(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_short: DataResponse<MetazoneSpecificNamesShortV1> =
            provider.load(en).unwrap();
        assert_eq!(
            "PDT",
            specific_names_short
                .payload
                .get()
                .defaults
                .get(&(
                    metazone_now(TimeZone(tinystr!(8, "uslax"))),
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
                .get(&(TimeZone(tinystr!(8, "utc")), TimeZoneVariant::Standard))
                .unwrap()
        );
    }
}
