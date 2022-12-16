// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "experimental")]

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

/// Substring used to denote alternative region names data variants for a given region. For example: "BA-alt-short", "TL-alt-variant".
const ALT_SUBSTRING: &str = "-alt-";

impl From<&cldr_serde::language_displaynames::Resource> for LanguageDisplayNamesV1<'static> {
    fn from(other: &cldr_serde::language_displaynames::Resource) -> Self {
        let mut names = ZeroMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.languages.iter() {
                let region = String::from(entry.0);
                if !region.contains(ALT_SUBSTRING) {
                    let key = UnvalidatedStr::from_str(&region);
                    names.insert(key, entry.1.as_ref());
                }
            }
        }
        Self { names }
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
}
