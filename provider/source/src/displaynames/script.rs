// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::convert::TryFrom;
use icu::experimental::displaynames::provider::*;
use icu::locale::{subtags::Script, ParseError};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};

impl DataProvider<ScriptDisplayNamesV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ScriptDisplayNamesV1Marker>, DataError> {
        self.check_req::<ScriptDisplayNamesV1Marker>(req)?;

        let data: &cldr_serde::displaynames::script::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "scripts.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ScriptDisplayNamesV1::try_from(data).map_err(
                |e| DataError::custom("data for ScriptDisplayNames").with_display_context(&e),
            )?),
        })
    }
}

impl IterableDataProviderCached<ScriptDisplayNamesV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .displaynames()
            .list_locales()?
            .filter(|locale| {
                // The directory might exist without scripts.json
                self.cldr()
                    .unwrap()
                    .displaynames()
                    .file_exists(locale, "scripts.json")
                    .unwrap_or_default()
            })
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

/// Substring used to denote alternative display names data variants for a given script. For example: "BA-alt-short", "TL-alt-variant".
/// TODO(#3316): Distinguish stand-alone ("Traditional Han") from default ("Traditional")
const ALT_SUBSTRING: &str = "-alt-";
/// Substring used to denote short display names data variants for a given script. For example: "az-alt-short".
const ALT_SHORT_SUBSTRING: &str = "-alt-short";

impl TryFrom<&cldr_serde::displaynames::script::Resource> for ScriptDisplayNamesV1<'static> {
    type Error = ParseError;

    fn try_from(other: &cldr_serde::displaynames::script::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for entry in other.main.value.localedisplaynames.scripts.iter() {
            if let Some(script) = entry.0.strip_suffix(ALT_SHORT_SUBSTRING) {
                short_names.insert(
                    Script::try_from_str(script)?.into_tinystr(),
                    entry.1.as_str(),
                );
            } else if !entry.0.contains(ALT_SUBSTRING) {
                names.insert(
                    Script::try_from_str(entry.0)?.into_tinystr(),
                    entry.1.as_str(),
                );
            }
        }
        Ok(Self {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu::locale::{langid, subtags::script};

    #[test]
    fn test_basic_script_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<ScriptDisplayNamesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&script!("Cans").into_tinystr().to_unvalidated())
                .unwrap(),
            "Unified Canadian Aboriginal Syllabics"
        );
    }

    #[test]
    fn test_basic_script_short_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<ScriptDisplayNamesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .short_names
                .get(&script!("Cans").into_tinystr().to_unvalidated())
                .unwrap(),
            "UCAS"
        );
    }
}
