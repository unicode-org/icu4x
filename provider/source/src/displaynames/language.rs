// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_cache::CoverageLevelForXPath;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::{Alt, WithAlt};
use crate::displaynames::extract_names_for_zeromap_struct;

use icu::experimental::displaynames::provider::{
    LanguageDisplayNames, LanguageDisplayNamesV1, LocaleDisplayNames, LocaleDisplayNamesV1,
    LocaleNamesLanguageCoreLongV1, LocaleNamesLanguageCoreMediumV1, LocaleNamesLanguageCoreShortV1,
    LocaleNamesLanguageExtendedLongV1, LocaleNamesLanguageExtendedMediumV1,
    LocaleNamesLanguageExtendedShortV1, LocaleNamesLanguageMenuCoreMediumV1,
    LocaleNamesLanguageMenuExtendedMediumV1, LocaleNamesLanguageMinimalMediumV1, MenuNameParts,
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
    LocaleNamesLanguageMinimalMediumV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None,
    CoverageLevelForXPath::Basic | CoverageLevelForXPath::Core,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageCoreMediumV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None,
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageExtendedMediumV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None,
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageCoreShortV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Short),
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageExtendedShortV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Short),
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageCoreLongV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Long),
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageExtendedLongV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Long),
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);
crate::displaynames::impl_displaynames_menu_v1!(
    LocaleNamesLanguageMenuCoreMediumV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_menu_v1!(
    LocaleNamesLanguageMenuExtendedMediumV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
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
    use crate::displaynames::CheckAltCoverage;

    use super::*;
    use icu::locale::{langid, subtags::language};

    #[test]
    fn test_basic_lang_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("aa").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Afar");
    }

    #[test]
    fn test_basic_lang_short_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageExtendedShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("az").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Azeri");
    }

    #[test]
    fn test_basic_lang_long_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreLongV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("zh").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Mandarin Chinese");
    }

    #[test]
    fn test_basic_lang_menu_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMenuCoreMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("zh").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(data.get().core(), "Chinese, Mandarin");
    }

    #[test]
    fn test_basic_locale_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("de-CH").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Swiss High German");
    }

    #[test]
    fn test_locale_names_language_medium() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("aa").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Afar");
    }

    #[test]
    fn test_locale_names_language_short() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("en-GB").unwrap(),
                    &langid!("en").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "UK English");
    }

    #[test]
    fn test_locale_names_language_long() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreLongV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("zh").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Mandarin Chinese");
    }

    #[test]
    fn test_locale_names_language_menu_medium() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMenuExtendedMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("ku").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(data.get().core(), "Kurdish");
        assert_eq!(data.get().extension(), "Kurmanji");

        // Test fallback to alt-menu
        let data_zh: DataPayload<LocaleNamesLanguageMenuCoreMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("zh").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(data_zh.get().core(), "Chinese, Mandarin");
        assert_eq!(data_zh.get().extension(), "");
    }

    /// The cartesian product of Language x (Short | Medium | Long) x (Minimal | Core | Extended) x (Menu)
    /// contains some data markers that are uninhabited. This test ensures that every language display name
    /// key and coverage tier combination in CLDR is covered by an existing marker, so if future CLDR releases
    /// add data for uninhabited markers, we learn about it and can take action.
    #[test]
    fn test_empty_coverage_tiers_assert_no_data() {
        let provider = SourceDataProvider::new_testing();
        let cldr = provider.cldr().unwrap();

        crate::displaynames::for_each_cldr_key_and_tier(
            cldr,
            "languages.json",
            "languages",
            |res: &cldr_serde::displaynames::language::Resource| {
                &res.main.value.localedisplaynames.languages
            },
            |locale, key, tier| {
                if LocaleNamesLanguageMinimalMediumV1::contains_key(key, tier)
                    || LocaleNamesLanguageCoreMediumV1::contains_key(key, tier)
                    || LocaleNamesLanguageExtendedMediumV1::contains_key(key, tier)
                    || LocaleNamesLanguageCoreShortV1::contains_key(key, tier)
                    || LocaleNamesLanguageExtendedShortV1::contains_key(key, tier)
                    || LocaleNamesLanguageCoreLongV1::contains_key(key, tier)
                    || LocaleNamesLanguageExtendedLongV1::contains_key(key, tier)
                    || LocaleNamesLanguageMenuCoreMediumV1::contains_key(key, tier)
                    || LocaleNamesLanguageMenuExtendedMediumV1::contains_key(key, tier)
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
