// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use core::convert::TryFrom;
use icu_displaynames::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use zerovec::ule::UnvalidatedStr;
use zerovec::ZeroMap;

impl DataProvider<LanguageDisplayNamesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LanguageDisplayNamesV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let data: &cldr_serde::language_displaynames::Resource = self
            .source
            .cldr()?
            .displaynames()
            .read_and_parse(&langid, "languages.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(
                LanguageDisplayNamesV1::try_from(data).map_err(|e| {
                    DataError::custom("data for LanguageDisplayNames").with_display_context(&e)
                })?,
            )),
        })
    }
}

impl IterableDataProvider<LanguageDisplayNamesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .displaynames()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}

/// Substring used to denote short display names data variants for a given language. For example: "az-alt-short".
const ALT_SHORT_SUBSTRING: &str = "-alt-short";
/// Substring used to denote long display names data variants for a given language. For example: "az-alt-long".
const ALT_LONG_SUBSTRING: &str = "-alt-long";
/// Substring used to denote menu display names data variants for a given language. For example: "az-alt-menu".
const ALT_MENU_SUBSTRING: &str = "-alt-menu";

impl From<&cldr_serde::language_displaynames::Resource> for LanguageDisplayNamesV1<'static> {
    fn from(other: &cldr_serde::language_displaynames::Resource) -> Self {
        let mut names = ZeroMap::new();
        let mut short_names = ZeroMap::new();
        let mut long_names = ZeroMap::new();
        let mut menu_names = ZeroMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.languages.iter() {
                let mut region = String::from(entry.0);
                if region.contains(ALT_SHORT_SUBSTRING) {
                    region.truncate(region.find(ALT_SHORT_SUBSTRING).unwrap());
                    let key = UnvalidatedStr::from_str(&region);
                    short_names.insert(key, entry.1.as_ref());
                } else if region.contains(ALT_LONG_SUBSTRING) {
                    region.truncate(region.find(ALT_LONG_SUBSTRING).unwrap());
                    let key = UnvalidatedStr::from_str(&region);
                    long_names.insert(key, entry.1.as_ref());
                } else if region.contains(ALT_MENU_SUBSTRING) {
                    region.truncate(region.find(ALT_MENU_SUBSTRING).unwrap());
                    let key = UnvalidatedStr::from_str(&region);
                    menu_names.insert(key, entry.1.as_ref());
                } else {
                    let key = UnvalidatedStr::from_str(&region);
                    names.insert(key, entry.1.as_ref());
                }
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
    use icu_locid::locale;

    #[test]
    fn test_basic_lang_display_names() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<LanguageDisplayNamesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en-001").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            data.get()
                .names
                .get(UnvalidatedStr::from_str("aa"))
                .unwrap(),
            "Afar"
        );
    }

    #[test]
    fn test_basic_lang_short_display_names() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<LanguageDisplayNamesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en-001").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            data.get()
                .short_names
                .get(UnvalidatedStr::from_str("az"))
                .unwrap(),
            "Azeri"
        );
    }

    #[test]
    fn test_basic_lang_long_display_names() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<LanguageDisplayNamesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en-001").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            data.get()
                .long_names
                .get(UnvalidatedStr::from_str("zh"))
                .unwrap(),
            "Mandarin Chinese"
        );
    }

    #[test]
    fn test_basic_lang_menu_display_names() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<LanguageDisplayNamesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en-001").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            data.get()
                .menu_names
                .get(UnvalidatedStr::from_str("zh"))
                .unwrap(),
            "Chinese, Mandarin"
        );
    }
}
