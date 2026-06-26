// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::{Alt, WithAlt};
use crate::displaynames::extract_names_for_zeromap_struct;

use icu::experimental::displaynames::provider::*;
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
    LocaleNamesLanguageMediumV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageShortV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Short),
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageLongV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(Alt::Long),
);

crate::displaynames::impl_displaynames_menu_v1!(
    LocaleNamesLanguageMenuMediumV1,
    LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
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
    use super::*;
    use icu::locale::{langid, subtags::language};

    #[test]
    fn test_basic_lang_display_names() {
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
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
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
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
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
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
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
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
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
    fn test_locale_names_language_medium() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesLanguageMediumV1> = provider
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

        let data: DataPayload<LocaleNamesLanguageShortV1> = provider
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

        let data: DataPayload<LocaleNamesLanguageLongV1> = provider
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

        let data: DataPayload<LocaleNamesLanguageMenuMediumV1> = provider
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
        let data_zh: DataPayload<LocaleNamesLanguageMenuMediumV1> = provider
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
}
