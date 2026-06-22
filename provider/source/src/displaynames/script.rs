// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::WithAlt;
use crate::displaynames::{ALT_SECONDARY, ALT_SHORT, ALT_STANDALONE, ALT_VARIANT};
use icu::experimental::displaynames::provider::*;
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
            payload: DataPayload::from_owned(ScriptDisplayNames::from(data)),
        })
    }
}

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptMediumV1,
    icu::locale::subtags::Script,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    None::<&str>,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptShortV1,
    icu::locale::subtags::Script,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    Some(ALT_SHORT),
);

crate::displaynames::impl_displaynames_legacy_iter_v1!(ScriptDisplayNamesV1, "scripts.json");

impl From<&cldr_serde::displaynames::script::Resource> for ScriptDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::script::Resource) -> Self {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for (key, value) in other.main.value.localedisplaynames.scripts.iter() {
            if key.menu.is_some() {
                continue;
            }

            let script = key.subtag;

            match key.alt.as_deref() {
                Some(ALT_SHORT) => {
                    short_names.insert(script.to_tinystr(), value.as_str());
                }
                None => {
                    names.insert(script.to_tinystr(), value.as_str());
                }
                Some(ALT_VARIANT) | Some(ALT_SECONDARY) => {
                    // TODO(#8012): Handle this with datagen alt flags.
                }
                Some(ALT_STANDALONE) => {
                    // TODO(#8011): Support standalone display names.
                }
                Some(alt) => {
                    log::warn!("Unknown alt variant for script: {}", alt);
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
        }
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
    fn test_locale_names_script_medium() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesScriptMediumV1> = provider
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
