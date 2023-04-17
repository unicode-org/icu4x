// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use core::convert::TryFrom;
use icu_displaynames::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use tinystr::{TinyAsciiStr, TinyStrError};

impl DataProvider<ScriptDisplayNamesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ScriptDisplayNamesV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let data: &cldr_serde::script_displaynames::Resource =
            self.source
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
/// TODO(#3316): Distinguish stand-alone ("Traditional Han") from default ("Traditional")
const ALT_SUBSTRING: &str = "-alt-";
/// Substring used to denote short display names data variants for a given script. For example: "az-alt-short".
const ALT_SHORT_SUBSTRING: &str = "-alt-short";

impl TryFrom<&cldr_serde::script_displaynames::Resource> for ScriptDisplayNamesV1<'static> {
    type Error = TinyStrError;

    fn try_from(other: &cldr_serde::script_displaynames::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.scripts.iter() {
                if let Some(script) = entry.0.strip_suffix(ALT_SHORT_SUBSTRING) {
                    short_names.insert(TinyAsciiStr::from_str(script)?, entry.1.as_ref());
                } else if !entry.0.contains(ALT_SUBSTRING) {
                    names.insert(TinyAsciiStr::from_str(entry.0)?, entry.1.as_ref());
                }
            }
        }
        Ok(Self {
            // Old CLDR versions may contain trivial entries, so filter
            names: names.into_iter().filter(|&(k, v)| k != v).collect(),
            short_names: short_names.into_iter().filter(|&(k, v)| k != v).collect(),
        })
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
                .get(&TinyAsciiStr::from_str("Cans").unwrap())
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
                .get(&TinyAsciiStr::from_str("Cans").unwrap())
                .unwrap(),
            "UCAS"
        );
    }
}
