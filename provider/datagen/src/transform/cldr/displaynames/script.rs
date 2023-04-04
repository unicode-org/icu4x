// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use core::convert::TryFrom;
use icu_displaynames::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use zerovec::ule::UnvalidatedStr;

impl DataProvider<ScriptDisplayNamesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ScriptDisplayNamesV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let data: &cldr_serde::script_displaynames::Resource = self
            .source
            .cldr()?
            .displaynames()
            .read_and_parse(&langid, "scripts.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(
                ScriptDisplayNamesV1::try_from(data).map_err(|e| {
                    DataError::custom("data for ScriptDisplayNames").with_display_context(&e)
                })?,
            )),
        })
    }
}

impl IterableDataProvider<ScriptDisplayNamesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .displaynames()
            .list_langs()?
            .filter(|langid| {
                // The directory might exist without scripts.json
                self.source
                    .cldr()
                    .unwrap()
                    .displaynames()
                    .file_exists(langid, "scripts.json")
                    .unwrap_or_default()
            })
            .map(DataLocale::from)
            .collect())
    }
}

/// Substring used to denote alternative display names data variants for a given script. For example: "BA-alt-short", "TL-alt-variant".
const ALT_SUBSTRING: &str = "-alt-";
/// Substring used to denote short display names data variants for a given script. For example: "az-alt-short".
const ALT_SHORT_SUBSTRING: &str = "-alt-short";

impl From<&cldr_serde::script_displaynames::Resource> for ScriptDisplayNamesV1<'static> {
    fn from(other: &cldr_serde::script_displaynames::Resource) -> Self {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.scripts.iter() {
                if let Some(script) = entry.0.strip_suffix(ALT_SHORT_SUBSTRING) {
                    let key = UnvalidatedStr::from_str(script);
                    short_names.insert(key, entry.1.as_ref());
                } else if !entry.0.contains(ALT_SUBSTRING) {
                    let key = UnvalidatedStr::from_str(entry.0);
                    names.insert(key, entry.1.as_ref());
                }
            }
        }
        Self {
            names: names.into_iter().collect(),
            short_names: short_names.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;

    #[test]
    fn test_basic_script_display_names() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<ScriptDisplayNamesV1Marker> = provider
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
                .get(UnvalidatedStr::from_str("Cans"))
                .unwrap(),
            "Unified Canadian Aboriginal Syllabics"
        );
    }

    #[test]
    fn test_basic_script_short_display_names() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<ScriptDisplayNamesV1Marker> = provider
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
                .get(UnvalidatedStr::from_str("Cans"))
                .unwrap(),
            "UCAS"
        );
    }
}
