// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::locale::provider::*;

use icu_provider::prelude::*;

impl DataProvider<ScriptDirectionV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ScriptDirectionV1Marker>, DataError> {
        self.check_req::<ScriptDirectionV1Marker>(req)?;
        let data: &cldr_serde::directionality::Resource =
            self.cldr()?.core().read_and_parse("scriptMetadata.json")?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ScriptDirectionV1::from(data)),
        })
    }
}

impl crate::IterableDataProviderCached<ScriptDirectionV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl From<&cldr_serde::directionality::Resource> for ScriptDirectionV1<'_> {
    fn from(other: &cldr_serde::directionality::Resource) -> Self {
        let mut rtl = vec![];
        let mut ltr = vec![];
        for (script, metadata) in &other.script_metadata {
            match metadata.rtl {
                cldr_serde::directionality::Rtl::Yes => rtl.push(script.to_unvalidated()),
                cldr_serde::directionality::Rtl::No => ltr.push(script.to_unvalidated()),
                // not storing, because it is the default return value for unknown markers downstream
                cldr_serde::directionality::Rtl::Unknown => (),
            }
        }
        rtl.sort_unstable();
        ltr.sort_unstable();
        Self {
            rtl: rtl.into_iter().collect(),
            ltr: ltr.into_iter().collect(),
        }
    }
}

#[test]
fn test_basic() {
    use icu::locale::subtags::script;

    let provider = SourceDataProvider::new_testing();
    let data: DataResponse<ScriptDirectionV1Marker> = provider.load(Default::default()).unwrap();

    assert!(data
        .payload
        .get()
        .rtl
        .binary_search(&script!("Avst").into_tinystr().to_unvalidated())
        .is_ok());
    assert!(data
        .payload
        .get()
        .ltr
        .binary_search(&script!("Avst").into_tinystr().to_unvalidated())
        .is_err());

    assert!(data
        .payload
        .get()
        .ltr
        .binary_search(&script!("Latn").into_tinystr().to_unvalidated())
        .is_ok());
    assert!(data
        .payload
        .get()
        .rtl
        .binary_search(&script!("Latn").into_tinystr().to_unvalidated())
        .is_err());

    assert!(data
        .payload
        .get()
        .ltr
        .binary_search(&script!("Zzzz").into_tinystr().to_unvalidated())
        .is_err());
    assert!(data
        .payload
        .get()
        .rtl
        .binary_search(&script!("Zzzz").into_tinystr().to_unvalidated())
        .is_err());
}
