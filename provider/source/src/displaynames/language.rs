// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::coverage_experimental::CoverageLevelForXPath;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::{Alt, WithAlt};
use crate::displaynames::extract_names_for_zeromap_struct;

use icu::experimental::displaynames::provider::{
    LanguageDisplayNames, LanguageDisplayNamesV1, LocaleDisplayNames, LocaleDisplayNamesV1,
    LocaleNamesLanguageLongHeavyV1, LocaleNamesLanguageLongLightV1,
    LocaleNamesLanguageMediumHeavyV1, LocaleNamesLanguageMediumLightV1,
    LocaleNamesLanguageMediumTinyV1, LocaleNamesLanguageMenuMediumHeavyV1,
    LocaleNamesLanguageMenuMediumLightV1, LocaleNamesLanguageShortHeavyV1,
    LocaleNamesLanguageShortLightV1, MenuNameParts,
};
use icu::locale::LanguageIdentifier;
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use std::collections::{BTreeMap, HashSet};
use tinystr::TinyAsciiStr;
use zerovec::VarZeroCow;

impl DataProvider<LanguageDisplayNamesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LanguageDisplayNamesV1>, DataError> {
        self.check_req::<LanguageDisplayNamesV1>(req)?;

        let data: &cldr_serde::displaynames::language::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "languages.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(LanguageDisplayNames::from(data)),
        })
    }
}
impl DataProvider<LocaleDisplayNamesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LocaleDisplayNamesV1>, DataError> {
        self.check_req::<LocaleDisplayNamesV1>(req)?;

        let data: &cldr_serde::displaynames::language::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "languages.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(LocaleDisplayNames::from(data)),
        })
    }
}

crate::displaynames::impl_displaynames_legacy_iter_v1!(LanguageDisplayNamesV1, "languages.json");
crate::displaynames::impl_displaynames_legacy_iter_v1!(LocaleDisplayNamesV1, "languages.json");

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageMediumTinyV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None,
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Basic | CoverageLevelForXPath::Core,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageMediumLightV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None,
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageMediumHeavyV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None,
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageShortLightV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Short),
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageShortHeavyV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Short),
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageLongLightV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Long),
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageLongHeavyV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Long),
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);
crate::displaynames::impl_displaynames_menu_v1!(
    LocaleNamesLanguageMenuMediumLightV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_menu_v1!(
    LocaleNamesLanguageMenuMediumHeavyV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    "//ldml/localeDisplayNames/languages/language",
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

impl From<&cldr_serde::displaynames::language::Resource> for LanguageDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::language::Resource) -> Self {
        let extracted = extract_names_for_zeromap_struct(
            &other.main.value.localedisplaynames.languages,
            &[Alt::Variant, Alt::Secondary, Alt::Official],
            "language",
            |langid| {
                // LanguageDisplayNames contains display names for language subtags without other subtags
                if langid.script.is_some() || langid.region.is_some() || !langid.variants.is_empty()
                {
                    None
                } else {
                    Some(langid.language.to_tinystr())
                }
            },
        );

        let to_zero_map = |map: BTreeMap<TinyAsciiStr<3>, &str>| {
            map.into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect()
        };

        Self {
            names: to_zero_map(extracted.names),
            short_names: to_zero_map(extracted.short_names),
            long_names: to_zero_map(extracted.long_names),
            menu_names: to_zero_map(extracted.menu_names),
        }
    }
}

impl From<&cldr_serde::displaynames::language::Resource> for LocaleDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::language::Resource) -> Self {
        let extracted = extract_names_for_zeromap_struct(
            &other.main.value.localedisplaynames.languages,
            &[Alt::Variant, Alt::Secondary, Alt::Official],
            "language",
            |langid| {
                // LocaleDisplayNames contains display names for languages with other subtags,
                // not duplicating the display names found in LanguageDisplayNames
                if langid.script.is_none() && langid.region.is_none() && langid.variants.is_empty()
                {
                    None
                } else {
                    Some(langid.to_string())
                }
            },
        );

        let to_zero_map = |map: BTreeMap<String, &str>| {
            map.iter()
                .map(|(k, v)| (PotentialUtf8::from_str(k), *v))
                .collect()
        };

        Self {
            names: to_zero_map(extracted.names),
            short_names: to_zero_map(extracted.short_names),
            long_names: to_zero_map(extracted.long_names),
            menu_names: to_zero_map(extracted.menu_names),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::coverage_experimental::CheckAltCoverage;
    use super::*;
    use icu::locale::{data_locale, subtags::language};

    #[test]
    fn test_basic_lang_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LanguageDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&language!("aa").to_tinystr().to_unvalidated())
                .unwrap(),
            "Afar"
        );
    }

    #[test]
    fn test_basic_lang_short_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LanguageDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .short_names
                .get(&language!("az").to_tinystr().to_unvalidated())
                .unwrap(),
            "Azeri"
        );
    }

    #[test]
    fn test_basic_lang_long_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LanguageDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .long_names
                .get(&language!("zh").to_tinystr().to_unvalidated())
                .unwrap(),
            "Mandarin Chinese"
        );
    }

    #[test]
    fn test_basic_lang_menu_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LanguageDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .menu_names
                .get(&language!("zh").to_tinystr().to_unvalidated())
                .unwrap(),
            "Chinese, Mandarin"
        );
    }

    #[test]
    fn test_basic_locale_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(PotentialUtf8::from_str("de-CH"))
                .unwrap(),
            "Swiss High German"
        );
    }

    #[test]
    fn test_locale_names_language_medium_light() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMediumLightV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("fr").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "French");
    }

    #[test]
    fn test_locale_names_language_medium_tiny() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMediumTinyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("en").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "English");
    }

    #[test]
    fn test_locale_names_language_medium_heavy() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMediumHeavyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("aa").unwrap(),
                    &data_locale!("en-001"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Afar");
    }

    #[test]
    fn test_locale_names_language_short_light() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageShortLightV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("en-GB").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "UK English");
    }

    #[test]
    fn test_locale_names_language_short_heavy() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageShortHeavyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("az").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Azeri");
    }

    #[test]
    fn test_locale_names_language_long_light() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageLongLightV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("zh").unwrap(),
                    &data_locale!("en-001"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Mandarin Chinese");
    }

    #[test]
    fn test_locale_names_language_long_heavy() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageLongHeavyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("cr").unwrap(),
                    &data_locale!("en"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Woods Cree");
    }

    #[test]
    fn test_locale_names_language_menu_medium_heavy() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMenuMediumHeavyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("ku").unwrap(),
                    &data_locale!("en-001"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(data.get().core(), "Kurdish");
        assert_eq!(data.get().extension(), "Kurmanji");
    }

    #[test]
    fn test_locale_names_language_menu_medium_light() {
        let provider = SourceDataProvider::new_testing();

        // Test fallback to alt-menu
        let data: DataPayload<LocaleNamesLanguageMenuMediumLightV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("zh").unwrap(),
                    &data_locale!("en-001"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(data.get().core(), "Chinese, Mandarin");
        assert_eq!(data.get().extension(), "");
    }

    /// The cartesian product of Language x (Short | Medium | Long) x (Minimal | Core | Extended) x (Menu)
    /// contains some data markers that are uninhabited. This test ensures that every language display name
    /// key and coverage tier combination in CLDR is covered by an existing marker, so if future CLDR releases
    /// add data for uninhabited markers, we learn about it and can take action.
    #[test]
    fn test_empty_coverage_tiers_assert_no_data() {
        let provider = SourceDataProvider::new_testing();
        let cldr = provider.cldr().unwrap();

        crate::displaynames::coverage_experimental::for_each_cldr_key_and_tier(
            cldr,
            "languages.json",
            "//ldml/localeDisplayNames/languages/language",
            |res: &cldr_serde::displaynames::language::Resource| {
                &res.main.value.localedisplaynames.languages
            },
            |locale, key, tier| {
                if LocaleNamesLanguageMediumTinyV1::contains_key(key, tier)
                    || LocaleNamesLanguageMediumLightV1::contains_key(key, tier)
                    || LocaleNamesLanguageMediumHeavyV1::contains_key(key, tier)
                    || LocaleNamesLanguageShortLightV1::contains_key(key, tier)
                    || LocaleNamesLanguageShortHeavyV1::contains_key(key, tier)
                    || LocaleNamesLanguageLongLightV1::contains_key(key, tier)
                    || LocaleNamesLanguageLongHeavyV1::contains_key(key, tier)
                    || LocaleNamesLanguageMenuMediumLightV1::contains_key(key, tier)
                    || LocaleNamesLanguageMenuMediumHeavyV1::contains_key(key, tier)
                {
                    return;
                }

                panic!(
                    "Found unexpected alt, menu, and tier combination for language: {key:?} in locale: {locale:?} and tier: {tier:?}"
                );
            },
        );
    }
}
