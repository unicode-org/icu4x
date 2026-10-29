// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::coverage_experimental::CoverageLevelForXPath;
use crate::AltVariantKind;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::alt::{Alt, WithAlt};
use crate::displaynames::{
    extract_names_for_zeromap_struct, extract_names_for_zeromap_struct_with_overrides,
};
use icu::experimental::displaynames::provider::*;
use icu::locale::provider::names::*;
use icu::locale::subtags::{Region, region};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use zerovec::VarZeroCow;

impl SourceDataProvider {
    fn region_default_alt(&self, region: Region) -> Option<Alt> {
        if region == region!("HK")
            && self
                .alt_variants
                .contains(&AltVariantKind::LocaleNamesHkShort)
        {
            return Some(Alt::Short);
        }
        if region == region!("MO")
            && self
                .alt_variants
                .contains(&AltVariantKind::LocaleNamesMoShort)
        {
            return Some(Alt::Short);
        }
        if region == region!("PS")
            && self
                .alt_variants
                .contains(&AltVariantKind::LocaleNamesPsShort)
        {
            return Some(Alt::Short);
        }
        if region == region!("FK")
            && self
                .alt_variants
                .contains(&AltVariantKind::LocaleNamesFkVariant)
        {
            return Some(Alt::Variant);
        }
        None
    }

    fn region_display_names_from_cldr(
        &self,
        other: &cldr_serde::displaynames::region::Resource,
    ) -> RegionDisplayNames<'static> {
        let extracted = extract_names_for_zeromap_struct_with_overrides(
            &other.main.value.localedisplaynames.regions,
            &[Alt::Variant, Alt::Chagos, Alt::Biot],
            "region",
            |region| Some(region.to_tinystr()),
            |&region| self.region_default_alt(region),
        );

        let to_zero_map = |map: BTreeMap<tinystr::TinyAsciiStr<3>, &str>| {
            map.into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect()
        };

        RegionDisplayNames {
            names: to_zero_map(extracted.names),
            short_names: to_zero_map(extracted.short_names),
        }
    }
}

impl DataProvider<LocaleNamesRegionV0> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LocaleNamesRegionV0>, DataError> {
        self.check_req::<LocaleNamesRegionV0>(req)?;

        let data: &cldr_serde::displaynames::region::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "territories.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(self.region_display_names_from_cldr(data)),
        })
    }
}

crate::displaynames::impl_displaynames_legacy_iter_v1!(LocaleNamesRegionV0, "territories.json");

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionMediumTinyV1,
    Region,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    None,
    territory,
    CoverageLevelForXPath::Basic | CoverageLevelForXPath::Core,
    Self::region_default_alt,
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
    Self::region_default_alt,
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
    use icu::locale::{data_locale, subtags::region};

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesRegionV0> = provider
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

        let data: DataPayload<LocaleNamesRegionV0> = provider
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

    #[test]
    fn test_region_alt_variants() {
        let default_provider = SourceDataProvider::new_testing();
        let alt_provider = SourceDataProvider::new_testing().with_alt_variants([
            AltVariantKind::LocaleNamesHkShort,
            AltVariantKind::LocaleNamesMoShort,
            AltVariantKind::LocaleNamesFkVariant,
            AltVariantKind::LocaleNamesPsShort,
        ]);

        let load_tiny = |provider: &SourceDataProvider, code: &str| -> String {
            let payload: DataPayload<LocaleNamesRegionMediumTinyV1> = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::try_from_str(code).unwrap(),
                        &data_locale!("en"),
                    ),
                    ..Default::default()
                })
                .unwrap()
                .payload;
            payload.get().to_string()
        };
        let load_light = |provider: &SourceDataProvider, code: &str| -> String {
            let payload: DataPayload<LocaleNamesRegionMediumLightV1> = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::try_from_str(code).unwrap(),
                        &data_locale!("en"),
                    ),
                    ..Default::default()
                })
                .unwrap()
                .payload;
            payload.get().to_string()
        };

        assert_eq!(load_tiny(&default_provider, "HK"), "Hong Kong SAR China");
        assert_eq!(load_tiny(&default_provider, "MO"), "Macao SAR China");
        assert_eq!(
            load_light(&default_provider, "PS"),
            "Palestinian Territories"
        );
        assert_eq!(load_tiny(&default_provider, "FK"), "Falkland Islands");

        assert_eq!(load_tiny(&alt_provider, "HK"), "Hong Kong");
        assert_eq!(load_tiny(&alt_provider, "MO"), "Macao");
        assert_eq!(load_light(&alt_provider, "PS"), "Palestine");
        assert_eq!(
            load_tiny(&alt_provider, "FK"),
            "Falkland Islands (Islas Malvinas)"
        );

        let v0_default: DataPayload<LocaleNamesRegionV0> = default_provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en")),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let v0_alt: DataPayload<LocaleNamesRegionV0> = alt_provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            v0_default
                .get()
                .names
                .get(&region!("HK").to_tinystr().to_unvalidated()),
            Some("Hong Kong SAR China")
        );
        assert_eq!(
            v0_alt
                .get()
                .names
                .get(&region!("HK").to_tinystr().to_unvalidated()),
            Some("Hong Kong")
        );
        assert_eq!(
            v0_alt
                .get()
                .names
                .get(&region!("MO").to_tinystr().to_unvalidated()),
            Some("Macao")
        );
        assert_eq!(
            v0_alt
                .get()
                .names
                .get(&region!("PS").to_tinystr().to_unvalidated()),
            Some("Palestine")
        );
        assert_eq!(
            v0_alt
                .get()
                .names
                .get(&region!("FK").to_tinystr().to_unvalidated()),
            Some("Falkland Islands (Islas Malvinas)")
        );
    }

    /// The cartesian product of Region x (Short | Medium) x (Minimal | Core | Extended)
    /// contains some data markers that are uninhabited. This test ensures that every region display name
    /// key and coverage tier combination in CLDR is covered by an existing marker, so if future CLDR releases
    /// add data for uninhabited markers, we learn about it and can take action.
    #[test]
    #[cfg(feature = "networking")]
    fn test_empty_coverage_tiers_assert_no_data() {
        use crate::displaynames::coverage_experimental::CheckAltCoverage;

        let provider = SourceDataProvider::new();
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
