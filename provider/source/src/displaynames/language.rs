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
    fn test_basic_locale_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LanguageDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&language!("es").to_tinystr().to_unvalidated())
                .unwrap(),
            "Spanish"
        );
    }

    #[test]
    fn test_locale_names_language_minimal() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMinimalMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("en").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "English");
    }

    #[test]
    fn test_locale_names_language_medium() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("es").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Spanish");
    }

    #[test]
    fn test_locale_names_language_short() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("en-US").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "US English");
    }

    #[test]
    fn test_locale_names_language_long() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageCoreLongV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("zh-Hans").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Simplified Mandarin Chinese");
    }

    #[test]
    fn test_locale_names_language_menu_medium() {
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

    /// The cartesian product of (Language|Script|Region|Variant)x(Short|Medium|Long)x(Minimal|Core|Extended)
    /// contains some data markers that are uninhabited. This test ensures that if future CLDRs add data in
    /// one of these markers, we learn about it and can take action.
    #[test]
    fn test_empty_coverage_tiers_assert_no_data() {
        use crate::SourceDataProvider;
        let provider = SourceDataProvider::new_testing();
        let cldr = provider.cldr().unwrap();
        let fallbacker = cldr.locale_fallbacker().unwrap();
        let coverage_cldr = crate::cldr_cache::coverage_cldr_cache();

        let displaynames_dir = cldr.displaynames();
        let locales = displaynames_dir.list_locales().unwrap();

        for locale in locales {
            // 1. Languages
            if displaynames_dir
                .file_exists(&locale, "languages.json")
                .unwrap_or(false)
                && let Ok(res) = displaynames_dir
                    .read_and_parse::<cldr_serde::displaynames::language::Resource>(
                        &locale,
                        "languages.json",
                    )
            {
                for key in res.main.value.localedisplaynames.languages.keys() {
                    let xpath = crate::displaynames::construct_xpath(
                        "languages",
                        &key.subtag,
                        key.alt,
                        key.menu,
                    );
                    let tier = coverage_cldr
                        .coverage_tier(fallbacker, &locale, &xpath)
                        .unwrap();

                    if LocaleNamesLanguageMinimalMediumV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageCoreMediumV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageExtendedMediumV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageCoreShortV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageExtendedShortV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageCoreLongV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageExtendedLongV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageMenuCoreMediumV1::check_coverage(&key, tier) {
                        continue;
                    }
                    if LocaleNamesLanguageMenuExtendedMediumV1::check_coverage(&key, tier) {
                        continue;
                    }

                    panic!(
                        "Found unexpected alt, menu, and tier combination for language: {key:?} in locale: {locale:?} and tier: {tier:?}"
                    );
                }
            }

            // 2. Scripts
            if displaynames_dir
                .file_exists(&locale, "scripts.json")
                .unwrap_or(false)
                && let Ok(res) = displaynames_dir
                    .read_and_parse::<cldr_serde::displaynames::script::Resource>(
                        &locale,
                        "scripts.json",
                    )
            {
                for key in res.main.value.localedisplaynames.scripts.keys() {
                    if key.alt != Some(Alt::Short) {
                        continue;
                    }

                    let xpath = crate::displaynames::construct_xpath(
                        "scripts",
                        &key.subtag,
                        key.alt,
                        key.menu,
                    );
                    let tier = coverage_cldr
                        .coverage_tier(fallbacker, &locale, &xpath)
                        .unwrap();

                    // 4. Script Minimal Short (Basic) & 5. Script Core Short (Moderate)
                    assert!(
                        tier != CoverageLevelForXPath::Basic
                            && tier != CoverageLevelForXPath::Moderate,
                        "Found unexpected script short display name in tier {:?} for locale {:?}, key {:?}",
                        tier,
                        locale,
                        key
                    );
                }
            }

            // 3. Variants
            if displaynames_dir
                .file_exists(&locale, "variants.json")
                .unwrap_or(false)
                && let Ok(res) = displaynames_dir
                    .read_and_parse::<cldr_serde::displaynames::variant::Resource>(
                        &locale,
                        "variants.json",
                    )
            {
                for key in res.main.value.localedisplaynames.variants.keys() {
                    if key.alt.is_some() || key.menu.is_some() {
                        continue;
                    }

                    let xpath = crate::displaynames::construct_xpath(
                        "variants",
                        &key.subtag,
                        key.alt,
                        key.menu,
                    );
                    let tier = coverage_cldr
                        .coverage_tier(fallbacker, &locale, &xpath)
                        .unwrap();

                    // 6. Variant Minimal Medium (Basic) & 7. Variant Core Medium (Moderate)
                    assert!(
                        tier != CoverageLevelForXPath::Basic
                            && tier != CoverageLevelForXPath::Moderate,
                        "Found unexpected variant display name in tier {:?} for locale {:?}, key {:?}",
                        tier,
                        locale,
                        key
                    );
                }
            }

            // 4. Regions / Territories
            if displaynames_dir
                .file_exists(&locale, "territories.json")
                .unwrap_or(false)
                && let Ok(res) = displaynames_dir
                    .read_and_parse::<cldr_serde::displaynames::region::Resource>(
                        &locale,
                        "territories.json",
                    )
            {
                for key in res.main.value.localedisplaynames.regions.keys() {
                    if key.alt.is_some() || key.menu.is_some() {
                        continue;
                    }

                    let xpath = crate::displaynames::construct_xpath(
                        "regions",
                        &key.subtag,
                        key.alt,
                        key.menu,
                    );
                    let tier = coverage_cldr
                        .coverage_tier(fallbacker, &locale, &xpath)
                        .unwrap();

                    // 8. Region Extended Medium (Modern)
                    assert!(
                        tier != CoverageLevelForXPath::Modern,
                        "Found unexpected region medium display name in tier {:?} for locale {:?}, key {:?}",
                        tier,
                        locale,
                        key
                    );
                }
            }
        }
    }
}
