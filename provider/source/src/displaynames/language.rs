// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::ModifiedSubtag;
use crate::displaynames::{
    ALT_LONG, ALT_MENU, ALT_OFFICIAL, ALT_SECONDARY, ALT_SHORT, ALT_VARIANT,
};

use icu::experimental::displaynames::provider::*;
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use std::collections::{BTreeMap, HashSet};
use zerovec::{VarZeroCow, ZeroMap};

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
    icu::locale::LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    None::<&str>,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageShortV1,
    icu::locale::LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(ALT_SHORT),
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesLanguageLongV1,
    icu::locale::LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
    Some(ALT_LONG),
);

crate::displaynames::impl_displaynames_menu_v1!(
    LocaleNamesLanguageMenuMediumV1,
    icu::locale::LanguageIdentifier,
    cldr_serde::displaynames::language::Resource,
    "languages.json",
    languages,
);

impl From<&cldr_serde::displaynames::language::Resource> for LanguageDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::language::Resource) -> Self {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        let mut long_names = BTreeMap::new();
        let mut menu_names = BTreeMap::new();
        for (key, value) in other.main.value.localedisplaynames.languages.iter() {
            if key.menu_variant.is_some() {
                continue;
            }

            let langid = &key.subtag;
            if langid.script.is_some() || langid.region.is_some() || !langid.variants.is_empty() {
                continue;
            }
            let lang = langid.language;

            match key.alt_variant.as_deref() {
                Some(ALT_SHORT) => {
                    short_names.insert(lang.to_tinystr(), value.as_ref());
                }
                Some(ALT_LONG) => {
                    long_names.insert(lang.to_tinystr(), value.as_ref());
                }
                Some(ALT_MENU) => {
                    menu_names.insert(lang.to_tinystr(), value.as_ref());
                }
                None => {
                    names.insert(lang.to_tinystr(), value.as_ref());
                }
                Some(ALT_VARIANT) | Some(ALT_SECONDARY) | Some(ALT_OFFICIAL) => {
                    // TODO(#8012): Handle preference-specific alt variants.
                }
                Some(alt) => {
                    log::warn!("Unknown alt variant for language: {}", alt);
                }
            }
        }
        Self {
            // Old CLDR versions may contain trivial entries, so filter
            names: names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            short_names: short_names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            long_names: long_names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            menu_names: menu_names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
        }
    }
}

impl From<&cldr_serde::displaynames::language::Resource> for LocaleDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::language::Resource) -> Self {
        let mut names = ZeroMap::new();
        let mut short_names = ZeroMap::new();
        let mut long_names = ZeroMap::new();
        let mut menu_names = ZeroMap::new();
        for (key, value) in other.main.value.localedisplaynames.languages.iter() {
            if key.menu_variant.is_some() {
                // Note: we don't handle -menu-core and -menu-extension here,
                // but we handle them in the new LocaleNames markers.
                continue;
            }

            let langid = &key.subtag;
            if langid.script.is_none() && langid.region.is_none() && langid.variants.is_empty() {
                continue;
            }

            let locale_str = langid.to_string();
            let val_str = value.as_str();
            if locale_str == val_str {
                continue;
            }

            let pot_utf8 = PotentialUtf8::from_str(&locale_str);

            match key.alt_variant.as_deref() {
                Some(ALT_SHORT) => {
                    short_names.insert(pot_utf8, val_str);
                }
                Some(ALT_LONG) => {
                    long_names.insert(pot_utf8, val_str);
                }
                Some(ALT_MENU) => {
                    menu_names.insert(pot_utf8, val_str);
                }
                None => {
                    names.insert(pot_utf8, val_str);
                }
                _ => {}
            }
        }
        Self {
            names,
            short_names,
            long_names,
            menu_names,
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
