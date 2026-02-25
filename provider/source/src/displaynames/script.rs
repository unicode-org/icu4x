// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::displaynames::{ALT_SHORT_SUBSTRING, ALT_SUBSTRING};
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::convert::TryFrom;
use icu::experimental::displaynames::provider::*;
use icu::locale::{subtags::Script, ParseError};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use zerovec::VarZeroCow;

impl DataProvider<ScriptDisplayNamesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ScriptDisplayNamesV1>, DataError> {
        self.check_req::<ScriptDisplayNamesV1>(req)?;

        let data: &cldr_serde::displaynames::script::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "scripts.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ScriptDisplayNames::try_from(data).map_err(|e| {
                DataError::custom("data for ScriptDisplayNames").with_display_context(&e)
            })?),
        })
    }
}

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptLongV1,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    None::<&str>,
    "ScriptDisplayNames"
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptShortV1,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    Some(ALT_SHORT_SUBSTRING),
    "ScriptDisplayNames"
);

crate::displaynames::impl_displaynames_main_iter_v1!(ScriptDisplayNamesV1, "scripts.json");

impl TryFrom<&cldr_serde::displaynames::script::Resource> for ScriptDisplayNames<'static> {
    type Error = ParseError;

    fn try_from(other: &cldr_serde::displaynames::script::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for entry in other.main.value.localedisplaynames.scripts.iter() {
            if let Some(script) = entry.0.strip_suffix(ALT_SHORT_SUBSTRING) {
                short_names.insert(Script::try_from_str(script)?.to_tinystr(), entry.1.as_str());
            } else if !entry.0.contains(ALT_SUBSTRING) {
                names.insert(
                    Script::try_from_str(entry.0)?.to_tinystr(),
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

        let data: DataPayload<ScriptDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&script!("Cans").to_tinystr().to_unvalidated())
                .unwrap(),
            "Unified Canadian Aboriginal Syllabics"
        );
    }

    #[test]
    fn test_basic_script_short_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<ScriptDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .short_names
                .get(&script!("Cans").to_tinystr().to_unvalidated())
                .unwrap(),
            "UCAS"
        );
    }

    #[test]
    fn test_locale_names_script_long() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesScriptLongV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("Latn").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Latin");
    }

    #[test]
    fn test_locale_names_script_short() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesScriptShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("Cans").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "UCAS");
    }
}
