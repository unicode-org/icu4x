// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderCached;
use icu_locale_core::langid;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_segmenter::provider::*;
use std::collections::HashSet;
use std::fmt::Debug;
use zerovec::ZeroVec;

#[derive(serde::Deserialize, Debug)]
struct SegmenterDictionaryData {
    trie_data: Vec<u16>,
}

impl DatagenProvider {
    fn load_dictionary_data(
        &self,
        req: DataRequest,
    ) -> Result<UCharDictionaryBreakDataV1<'static>, DataError> {
        let filename = format!(
            "segmenter/dictionary/{}.toml",
            req.marker_attributes as &str
        );

        let toml_data = self
            .icuexport()
            .and_then(|e| e.read_and_parse_toml::<SegmenterDictionaryData>(&filename));

        Ok(UCharDictionaryBreakDataV1 {
            trie_data: ZeroVec::alloc_from_slice(&toml_data?.trie_data),
        })
    }
}

macro_rules! implement {
    ($marker:ident, $supported:expr) => {
        impl DataProvider<$marker> for DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let data = self.load_dictionary_data(req)?;
                Ok(DataResponse {
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(data)),
                })
            }
        }

        impl IterableDataProviderCached<$marker> for DatagenProvider {
            fn supported_requests_cached(
                &self,
            ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
                Ok($supported
                    .into_iter()
                    .map(|m| (Default::default(), m.parse().unwrap()))
                    .collect())
            }
        }
    };
}

implement!(DictionaryForWordOnlyAutoV1Marker, ["cjdict"]);
implement!(
    DictionaryForWordLineExtendedV1Marker,
    ["khmerdict", "laodict", "burmesedict", "thaidict"]
);
