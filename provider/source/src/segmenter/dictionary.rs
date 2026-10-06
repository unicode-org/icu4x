// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use alloc::collections::BTreeSet;
use icu::collections::char16trie::Char16Trie;
use icu::segmenter::provider::DictionaryBreakData;
use icu::segmenter::provider::SegmenterDictionaryAutoV2;
use icu::segmenter::provider::SegmenterDictionaryExtendedV2;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerotrie::ZeroTrieSimpleAscii;
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
        // TODO: Read {}.txt instead
        let dict = Char16Trie::new(
            ZeroVec::from_slice_or_alloc(
                &self
                    .icuexport()?
                    .read_and_parse_toml::<SegmenterDictionaryData>(&format!(
                        "segmenter/dictionary/{}.toml",
                        req.id.marker_attributes as &str
                    ))?
                    .trie_data,
            )
            .into_owned(),
        );

        let mut alphabet = BTreeSet::new();

        for (word, _) in dict.iter2() {
            alphabet.extend(word.chars());
            if alphabet.len() > 128 {
                break;
            }
        }

        let trie = if alphabet.len() <= 128 {
            let alphabet: ZeroVec<'static, char> = alphabet.into_iter().collect();

            DictionaryBreakData::ZeroTrie {
                trie: ZeroTrieSimpleAscii::try_from_btree_map_str(
                    &dict
                        .iter2()
                        .map(|(k, v)| {
                            (
                                k.chars()
                                    .map(|c| alphabet.binary_search(&c).unwrap() as u8 as char)
                                    .collect::<String>(),
                                v as u32 as usize,
                            )
                        })
                        .collect(),
                )
                .unwrap()
                .convert_store(),
                alphabet,
            }
        } else {
            DictionaryBreakData::Char16Trie(dict)
        };

        Ok(trie)
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

implement!(SegmenterDictionaryAutoV2, ["cjdict"]);
implement!(
    SegmenterDictionaryExtendedV2,
    ["khmerdict", "laodict", "burmesedict", "thaidict"]
);
