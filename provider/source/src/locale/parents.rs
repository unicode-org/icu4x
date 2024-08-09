// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;

use icu::locale::provider::*;
use icu::locale::{
    subtags::{Language, Region, Script},
    LanguageIdentifier,
};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use writeable::Writeable;
use zerovec::ule::UnvalidatedStr;

impl DataProvider<ParentsV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ParentsV1Marker>, DataError> {
        self.check_req::<ParentsV1Marker>(req)?;
        let parents_data: &cldr_serde::parent_locales::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/parentLocales.json")?;

        let metadata = DataResponseMetadata::default();
        Ok(DataResponse {
            metadata,
            payload: DataPayload::from_owned(parents_data.into()),
        })
    }
}

impl crate::IterableDataProviderCached<ParentsV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl From<&cldr_serde::parent_locales::Resource> for ParentsV1<'static> {
    fn from(source_data: &cldr_serde::parent_locales::Resource) -> Self {
        let mut parents = BTreeMap::<_, (Language, Option<Script>, Option<Region>)>::new();

        for (source, target) in source_data.supplemental.parent_locales.parent_locale.iter() {
            assert!(!source.language.is_empty());
            if source.script.is_some()
                && source.region.is_none()
                && target == &LanguageIdentifier::UND
            {
                // We always fall back from language-script to und
                continue;
            }
            parents.insert(source.write_to_string(), target.into());
        }

        ParentsV1 {
            parents: parents
                .iter()
                .map(|(k, v)| (<&UnvalidatedStr>::from(k.as_ref()), v))
                .collect(),
        }
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    let parents: DataResponse<ParentsV1Marker> = provider.load(Default::default()).unwrap();

    assert_eq!(
        parents
            .payload
            .get()
            .parents
            .get_copied("zh-Hant-MO".into())
            .map(LanguageIdentifier::from),
        Some(langid!("zh-Hant-HK"))
    );
}
