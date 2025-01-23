// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::datetime::provider::time_zones::*;
use icu::timezone::provider::*;
use icu_locale_core::subtags::Region;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::sync::OnceLock;

mod convert;
mod names;
mod windows;

#[derive(Debug, Default)]
pub(crate) struct Caches {
    iana_to_bcp47: OnceLock<Result<BTreeMap<String, TimeZoneBcp47Id>, DataError>>,
    bcp47_to_canonical_iana: OnceLock<Result<BTreeMap<TimeZoneBcp47Id, String>, DataError>>,
    metazone_to_short: OnceLock<Result<BTreeMap<String, MetazoneId>, DataError>>,
    primary_zones: OnceLock<Result<BTreeMap<TimeZoneBcp47Id, Region>, DataError>>,
    mz_period: OnceLock<Result<MetazonePeriodV1<'static>, DataError>>,
    offset_period: OnceLock<Result<ZoneOffsetPeriodV1<'static>, DataError>>,
    reverse_metazones: OnceLock<Result<BTreeMap<MetazoneId, Vec<TimeZoneBcp47Id>>, DataError>>,
}

impl SourceDataProvider {
    fn reverse_metazones(&self) -> Result<&BTreeMap<MetazoneId, Vec<TimeZoneBcp47Id>>, DataError> {
        self.cldr()?
            .tz_caches
            .reverse_metazones
            .get_or_init(|| {
                let mz_period = self.metazone_period()?;
                let mut reverse_metazones = BTreeMap::<MetazoneId, Vec<TimeZoneBcp47Id>>::new();
                for cursor in mz_period.0.iter0() {
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
    fn iana_to_bcp47_map(&self) -> Result<&BTreeMap<String, TimeZoneBcp47Id>, DataError> {
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
                    if let Some(alias) = &bcp47_tzid_data.alias {
                        if bcp47_tzid.as_str() == "unk" {
                            // ignore the unknown time zone
                            continue;
                        }
                        for data_value in alias.split(' ') {
                            bcp47_tzids.insert(data_value.to_string(), *bcp47_tzid);
                        }
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
    fn bcp47_to_canonical_iana_map(&self) -> Result<&BTreeMap<TimeZoneBcp47Id, String>, DataError> {
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
                Ok(canonical_tzids)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// Returns a map from metazone long identifier to metazone BCP-47 ID.
    ///
    /// For example: "America_Central" to "amce"
    fn metazone_to_short_map(&self) -> Result<&BTreeMap<String, MetazoneId>, DataError> {
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

                let mut meta_zone_ids = BTreeMap::new();
                for (meta_zone_id, meta_zone_id_data) in meta_zone_ids_resource.iter() {
                    meta_zone_ids.insert(meta_zone_id_data.long_id.to_string(), *meta_zone_id);
                }
                Ok(meta_zone_ids)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn primary_zones_map(&self) -> Result<&BTreeMap<TimeZoneBcp47Id, Region>, DataError> {
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
                    // Montreal is meant to be deprecated, but pre-43 the deprecation
                    // fallback was not set, which is why it might show up here.
                    .filter(|(bcp47, _)| bcp47.0 != "camtr")
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
    TimeZoneEssentialsV1Marker,
    LocationsV1Marker,
    ExemplarCitiesV1Marker,
    MetazoneGenericNamesLongV1Marker,
    MetazoneGenericNamesShortV1Marker,
    MetazoneSpecificNamesLongV1Marker,
    MetazoneSpecificNamesShortV1Marker
);

impl IterableDataProviderCached<MetazonePeriodV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl IterableDataProviderCached<ZoneOffsetPeriodV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use icu::timezone::ZoneVariant;
    use tinystr::tinystr;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu::locale::langid;

        let provider = SourceDataProvider::new_testing();

        let time_zone_formats: DataResponse<TimeZoneEssentialsV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!("GMT", time_zone_formats.payload.get().offset_zero);
        assert_eq!("GMT+?", time_zone_formats.payload.get().offset_unknown);

        let locations_root: DataResponse<LocationsV1Marker> =
            provider.load(Default::default()).unwrap();
        assert_eq!(
            "Pohnpei",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZoneBcp47Id(tinystr!(8, "fmpni")))
                .unwrap()
        );
        assert_eq!(
            "Ireland",
            locations_root
                .payload
                .get()
                .locations
                .get(&TimeZoneBcp47Id(tinystr!(8, "iedub")))
                .unwrap()
        );

        let locations: DataResponse<LocationsV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("fr").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "Italie",
            locations
                .payload
                .get()
                .locations
                .get(&TimeZoneBcp47Id(tinystr!(8, "itrom")))
                .unwrap()
        );

        let generic_names_long: DataResponse<MetazoneGenericNamesLongV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "Australian Central Western Standard Time",
            generic_names_long
                .payload
                .get()
                .defaults
                .get(&MetazoneId(tinystr!(4, "aucw")))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            generic_names_long
                .payload
                .get()
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_long: DataResponse<MetazoneSpecificNamesLongV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            None, // deduplicated against generic
            specific_names_long
                .payload
                .get()
                .defaults
                .get(&(MetazoneId(tinystr!(4, "aucw")), ZoneVariant::Standard))
        );
        assert_eq!(
            None, // deduplicated against generic
            specific_names_long
                .payload
                .get()
                .overrides
                .get(&(TimeZoneBcp47Id(tinystr!(8, "utc")), ZoneVariant::Standard))
        );

        let generic_names_short: DataResponse<MetazoneGenericNamesShortV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "PT",
            generic_names_short
                .payload
                .get()
                .defaults
                .get(&MetazoneId(tinystr!(4, "AMPA")))
                .unwrap()
        );
        assert_eq!(
            "PST",
            generic_names_short
                .payload
                .get()
                .defaults
                .get(&MetazoneId(tinystr!(4, "ampa")))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            generic_names_short
                .payload
                .get()
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_short: DataResponse<MetazoneSpecificNamesShortV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "PDT",
            specific_names_short
                .payload
                .get()
                .defaults
                .get(&(MetazoneId(tinystr!(4, "ampa")), ZoneVariant::Daylight))
                .unwrap()
        );
        assert_eq!(
            None, // deduplicated against generic
            specific_names_short
                .payload
                .get()
                .overrides
                .get(&(TimeZoneBcp47Id(tinystr!(8, "utc")), ZoneVariant::Standard))
        );

        let metazone_period: DataResponse<MetazonePeriodV1Marker> =
            provider.load(Default::default()).unwrap();
        assert_eq!(
            Some(MetazoneId(tinystr!(4, "MGMT"))),
            metazone_period
                .payload
                .get()
                .0
                .get_copied_2d(&TimeZoneBcp47Id(tinystr!(8, "gblon")), &962040)
                .unwrap()
        );
    }
}
