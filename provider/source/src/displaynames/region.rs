// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::coverage_experimental::CoverageLevelForXPath;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::{Alt, WithAlt};
use crate::displaynames::extract_names_for_zeromap_struct;
use icu::experimental::displaynames::provider::*;
use icu::locale::provider::names::*;
use icu::locale::subtags::Region;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use zerovec::VarZeroCow;

impl DataProvider<RegionDisplayNamesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<RegionDisplayNamesV1>, DataError> {
        self.check_req::<RegionDisplayNamesV1>(req)?;

        let data: &cldr_serde::displaynames::region::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "territories.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(RegionDisplayNames::from(data)),
        })
    }
}

crate::displaynames::impl_displaynames_legacy_iter_v1!(RegionDisplayNamesV1, "territories.json");

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionMediumTinyV1,
    Region,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    None,
    territory,
    CoverageLevelForXPath::Basic | CoverageLevelForXPath::Core,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionMediumLightV1,
    Region,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    None,
    territory,
    CoverageLevelForXPath::Moderate,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionShortTinyV1,
    Region,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    Some(Alt::Short),
    territory,
    CoverageLevelForXPath::Basic,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionShortLightV1,
    Region,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    Some(Alt::Short),
    territory,
    CoverageLevelForXPath::Moderate,
);

impl From<&cldr_serde::displaynames::region::Resource> for RegionDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::region::Resource) -> Self {
        let extracted = extract_names_for_zeromap_struct(
            &other.main.value.localedisplaynames.regions,
            // TODO(#8012): Handle preference-specific alt variants, perhaps with datagen alt flags.
            &[Alt::Variant, Alt::Chagos, Alt::Biot],
            "region",
            |region| Some(region.to_tinystr()),
        );

        let to_zero_map = |map: BTreeMap<tinystr::TinyAsciiStr<3>, &str>| {
            map.into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect()
        };

        Self {
            names: to_zero_map(extracted.names),
            short_names: to_zero_map(extracted.short_names),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::displaynames::coverage_experimental::CheckAltCoverage;
    use icu::locale::{data_locale, subtags::region};

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<RegionDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&region!("AE").to_tinystr().to_unvalidated())
                .unwrap(),
            "United Arab Emirates"
        );
    }

    #[test]
    fn test_basic_short_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<RegionDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .short_names
                .get(&region!("BA").to_tinystr().to_unvalidated())
                .unwrap(),
            "Bosnia"
        );
    }

    #[test]
    fn test_locale_names_region_medium_light() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesRegionMediumLightV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("AF").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Afghanistan");
    }

    #[test]
    fn test_locale_names_region_short_light() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesRegionShortLightV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("BA").unwrap(),
                    &data_locale!("en-001"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Bosnia");
    }

    #[test]
    fn test_locale_names_region_medium_tiny() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesRegionMediumTinyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("US").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "United States");
    }

    #[test]
    fn test_locale_names_region_short_tiny() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesRegionShortTinyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("GB").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "UK");
    }

    /// The cartesian product of Region x (Short | Medium) x (Minimal | Core | Extended)
    /// contains some data markers that are uninhabited. This test ensures that every region display name
    /// key and coverage tier combination in CLDR is covered by an existing marker, so if future CLDR releases
    /// add data for uninhabited markers, we learn about it and can take action.
    #[test]
    fn test_empty_coverage_tiers_assert_no_data() {
        let provider = SourceDataProvider::new_testing();
        let cldr = provider.cldr().unwrap();

        crate::displaynames::coverage_experimental::for_each_cldr_key_and_tier(
            cldr,
            "territories.json",
            // TODO(#8012): Handle preference-specific alt variants, perhaps with datagen alt flags.
            &[Alt::Variant, Alt::Chagos, Alt::Biot],
            |l| &l.territory,
            |res: &cldr_serde::displaynames::region::Resource| {
                &res.main.value.localedisplaynames.regions
            },
            |locale, key, tier| {
                if LocaleNamesRegionMediumTinyV1::contains_key(key, tier)
                    || LocaleNamesRegionMediumLightV1::contains_key(key, tier)
                    || LocaleNamesRegionShortTinyV1::contains_key(key, tier)
                    || LocaleNamesRegionShortLightV1::contains_key(key, tier)
                {
                    return;
                }

                panic!(
                    "Found unexpected alt, menu, and tier combination for region: {key:?} in locale: {locale:?} and tier: {tier:?}"
                );
            },
        );
    }

    #[test]
    #[cfg(feature = "networking")]
    fn test_modern_locales_have_maximized_region_display_names() {
        use crate::CoverageLevel;
        use icu::locale::LocaleExpander;

        let provider = SourceDataProvider::new();
        let cldr = provider.cldr().unwrap();
        let modern_locales = cldr.locales([CoverageLevel::Modern]).unwrap();

        let expander = LocaleExpander::try_new_extended_unstable(&provider).unwrap();

        let tiny_region_ids =
            IterableDataProvider::<LocaleNamesRegionMediumTinyV1>::iter_ids(&provider).unwrap();

        for data_locale in modern_locales {
            if data_locale.is_unknown() {
                continue;
            }
            let mut langid = data_locale.into_locale().id;
            expander.maximize(&mut langid);
            let region = langid.region.unwrap();

            let data_id = DataIdentifierCow::from_borrowed_and_owned(
                DataMarkerAttributes::from_str_or_panic(region.as_str()),
                data_locale,
            );

            // Assert that all modern locales contain a region displayname for their maximized region in the tiny slice
            assert!(tiny_region_ids.contains(&data_id), "{data_locale}");
        }
    }
}
