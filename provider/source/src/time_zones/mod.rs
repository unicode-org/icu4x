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
use tinystr::tinystr;

mod convert;
mod names;
mod windows;

// TODO: cache these
impl SourceDataProvider {
    /// Returns a map from time zone long identifier to time zone BCP-47 ID.
    ///
    /// For example: "America/Chicago" to "uschi"
    fn compute_bcp47_tzids_btreemap(&self) -> Result<BTreeMap<String, TimeZoneBcp47Id>, DataError> {
        let bcp47_tzids_resource = &self
            .cldr()?
            .bcp47()
            .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>("timezone.json")?
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
    }

    /// Returns a map from BCP-47 ID to a single canonical long identifier.
    ///
    /// For example: "inccu" to "Asia/Kolkata"
    fn compute_canonical_tzids_btreemap(
        &self,
    ) -> Result<BTreeMap<TimeZoneBcp47Id, String>, DataError> {
        let bcp47_tzids_resource = &self
            .cldr()?
            .bcp47()
            .read_and_parse::<cldr_serde::time_zones::bcp47_tzid::Resource>("timezone.json")?
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
    }

    /// Returns a map from metazone long identifier to metazone BCP-47 ID.
    ///
    /// For example: "America_Central" to "amce"
    fn compute_meta_zone_ids_btreemap(&self) -> Result<BTreeMap<String, MetazoneId>, DataError> {
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
        // TODO(#1781): Remove this special case once the short id is updated in CLDR
        meta_zone_ids.insert("Yukon".to_owned(), MetazoneId(tinystr!(4, "yuko")));
        Ok(meta_zone_ids)
    }

    fn compute_primary_zones(&self) -> Result<BTreeMap<TimeZoneBcp47Id, Region>, DataError> {
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

        let bcp47_tzids = self.compute_canonical_tzids_btreemap()?;

        let zone_tab = self.tzdb()?.zone_tab()?;
        let mut zones_per_region: BTreeMap<icu::locale::subtags::Region, usize> = BTreeMap::new();

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
            "Australian Central Western Time",
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
            "Australian Central Western Standard Time",
            specific_names_long
                .payload
                .get()
                .standard
                .defaults
                .get(&MetazoneId(tinystr!(4, "aucw")))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            specific_names_long
                .payload
                .get()
                .standard
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
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
                .daylight
                .defaults
                .get(&MetazoneId(tinystr!(4, "ampa")))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            specific_names_short
                .payload
                .get()
                .standard
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
        );

        let metazone_period: DataResponse<MetazonePeriodV1Marker> =
            provider.load(Default::default()).unwrap();
        assert_eq!(
            Some(MetazoneId(tinystr!(4, "mgmt"))),
            metazone_period
                .payload
                .get()
                .0
                .get_copied_2d(&TimeZoneBcp47Id(tinystr!(8, "gblon")), &962040)
                .unwrap()
        );
    }
}
