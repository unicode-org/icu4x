// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_locid_transform::provider::*;

use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;

impl DataProvider<ScriptDirectionV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ScriptDirectionV1Marker>, DataError> {
        self.check_req::<ScriptDirectionV1Marker>(req)?;
        let data: &cldr_serde::directionality::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("scriptMetadata.json")?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(ScriptDirectionV1::from(data))),
        })
    }
}

impl IterableDataProvider<ScriptDirectionV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
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
                // not storing, because it is the default return value for unknown keys downstream
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
    use icu_locid::subtags_script as script;

    let provider = crate::DatagenProvider::for_test();
    let data: DataPayload<ScriptDirectionV1Marker> = provider
        .load(Default::default())
        .unwrap()
        .take_payload()
        .unwrap();

    assert!(matches!(
        data.get()
            .rtl
            .binary_search(&script!("Avst").into_tinystr().to_unvalidated()),
        Ok(_)
    ));
    assert!(matches!(
        data.get()
            .ltr
            .binary_search(&script!("Avst").into_tinystr().to_unvalidated()),
        Err(_)
    ));

    assert!(matches!(
        data.get()
            .ltr
            .binary_search(&script!("Latn").into_tinystr().to_unvalidated()),
        Ok(_)
    ));
    assert!(matches!(
        data.get()
            .rtl
            .binary_search(&script!("Latn").into_tinystr().to_unvalidated()),
        Err(_)
    ));

    assert!(matches!(
        data.get()
            .ltr
            .binary_search(&script!("Zzzz").into_tinystr().to_unvalidated()),
        Err(_)
    ));
    assert!(matches!(
        data.get()
            .rtl
            .binary_search(&script!("Zzzz").into_tinystr().to_unvalidated()),
        Err(_)
    ));
}
