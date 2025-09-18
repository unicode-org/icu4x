// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use icu::experimental::displaynames::provider::*;
use icu::locale::subtags::Language;
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use std::collections::{BTreeMap, HashSet};

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

impl IterableDataProviderCached<LanguageDisplayNamesV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .displaynames()
            .list_locales()?
            .filter(|locale| {
                // The directory might exist without languages.json
                self.cldr()
                    .unwrap()
                    .displaynames()
                    .file_exists(locale, "languages.json")
                    .unwrap_or_default()
            })
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

impl IterableDataProviderCached<LocaleDisplayNamesV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .displaynames()
            .list_locales()?
            .filter(|locale| {
                // The directory might exist without languages.json
                self.cldr()
                    .unwrap()
                    .displaynames()
                    .file_exists(locale, "languages.json")
                    .unwrap_or_default()
            })
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

/// Substring used to denote alternative region names data variants for a given region. For example: "BA-alt-short", "TL-alt-variant".
const ALT_SUBSTRING: &str = "-alt-";
/// Substring used to denote short display names data variants for a given language. For example: "az-alt-short".
const ALT_SHORT_SUBSTRING: &str = "-alt-short";
/// Substring used to denote long display names data variants for a given language. For example: "az-alt-long".
const ALT_LONG_SUBSTRING: &str = "-alt-long";
/// Substring used to denote menu display names data variants for a given language. For example: "az-alt-menu".
const ALT_MENU_SUBSTRING: &str = "-alt-menu";

impl From<&cldr_serde::displaynames::language::Resource> for LanguageDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::language::Resource) -> Self {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        let mut long_names = BTreeMap::new();
        let mut menu_names = BTreeMap::new();
        for (key, value) in other.main.value.localedisplaynames.languages.iter() {
            if let Some(lang) = key.strip_suffix(ALT_SHORT_SUBSTRING) {
                if let Ok(lang) = lang.parse::<Language>() {
                    short_names.insert(lang.to_tinystr(), value.as_ref());
                }
            } else if let Some(lang) = key.strip_suffix(ALT_LONG_SUBSTRING) {
                if let Ok(lang) = lang.parse::<Language>() {
                    long_names.insert(lang.to_tinystr(), value.as_ref());
                }
            } else if let Some(lang) = key.strip_suffix(ALT_MENU_SUBSTRING) {
                if let Ok(lang) = lang.parse::<Language>() {
                    menu_names.insert(lang.to_tinystr(), value.as_ref());
                }
            } else if let Ok(lang) = key.parse::<Language>() {
                names.insert(lang.to_tinystr(), value.as_ref());
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
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        let mut long_names = BTreeMap::new();
        let mut menu_names = BTreeMap::new();
        for (key, value) in other.main.value.localedisplaynames.languages.iter() {
            if key.contains("-menu-") {
                // TODO: handle -menu-core and -menu-extension
                continue;
            }
            #[expect(clippy::collapsible_if)] // consistency
            if let Some(locale) = key.strip_suffix(ALT_SHORT_SUBSTRING) {
                if locale.contains('-') {
                    short_names.insert(locale, value.as_ref());
                }
            } else if let Some(locale) = key.strip_suffix(ALT_LONG_SUBSTRING) {
                if locale.contains('-') {
                    long_names.insert(locale, value.as_ref());
                }
            } else if let Some(locale) = key.strip_suffix(ALT_MENU_SUBSTRING) {
                if locale.contains('-') {
                    menu_names.insert(locale, value.as_ref());
                }
            } else if !key.contains(ALT_SUBSTRING) {
                if key.contains('-') {
                    names.insert(key, value.as_ref());
                }
            }
        }
        Self {
            names: names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            short_names: short_names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            long_names: long_names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            menu_names: menu_names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (PotentialUtf8::from_str(k), v))
                .collect(),
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
}
