// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::collections::char16trie::Char16Trie;
use icu::segmenter::provider::DictionaryBreakData;
use icu::segmenter::provider::SegmenterDictionaryAutoV1;
use icu::segmenter::provider::SegmenterDictionaryExtendedV1;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::fmt::Debug;
use zerovec::ZeroVec;

#[derive(serde::Deserialize, Debug)]
struct SegmenterDictionaryData {
    trie_data: Vec<u16>,
}

impl SourceDataProvider {
    fn load_dictionary_data(
        &self,
        req: DataRequest,
    ) -> Result<DictionaryBreakData<'static>, DataError> {
        let filename = format!(
            "segmenter/dictionary/{}.toml",
            req.id.marker_attributes as &str
        );

        let toml_data = self
            .icuexport()?
            .read_and_parse_toml::<SegmenterDictionaryData>(&filename)?;

        let trie = Char16Trie {
            data: ZeroVec::alloc_from_slice(&toml_data.trie_data),
        };

        Ok(DictionaryBreakData(trie))
    }
}

macro_rules! implement {
    ($marker:ident, [$($supported:expr),*]) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let data = self.load_dictionary_data(req)?;
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(data),
                })
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(
                &self,
            ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                const SUPPORTED: &[&DataMarkerAttributes] = &[$(DataMarkerAttributes::from_str_or_panic($supported)),*];
                Ok(SUPPORTED
                    .iter()
                    .copied()
                    .map(DataIdentifierCow::from_marker_attributes)
                    .collect())
            }
        }
    };
}

implement!(SegmenterDictionaryAutoV1, ["cjdict"]);
implement!(
    SegmenterDictionaryExtendedV1,
    ["khmerdict", "laodict", "burmesedict", "thaidict"]
);
