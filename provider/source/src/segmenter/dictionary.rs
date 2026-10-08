// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use alloc::collections::BTreeSet;
use icu::segmenter::provider::DictionaryBreakData;
use icu::segmenter::provider::SegmenterDictionaryAutoV2;
use icu::segmenter::provider::SegmenterDictionaryExtendedV2;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ZeroVec;

impl SourceDataProvider {
    fn load_dictionary_data(
        &self,
        req: DataRequest,
    ) -> Result<DictionaryBreakData<'static>, DataError> {
        let dict = &self.icuexport()?.root.read_to_string(&format!(
            "segmenter/dictionary/{}.txt",
            req.id.marker_attributes as &str
        ))?;
        let dict = dict.strip_prefix('\u{FEFF}').unwrap_or(dict);

        let mut words = BTreeMap::new();
        let mut alphabet = BTreeSet::new();

        for line in dict.lines() {
            let line = line.split_once('#').unwrap_or((line, "")).0.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_ascii_whitespace();
            let word = parts.next().unwrap();
            let value = parts.next().unwrap_or("0").parse::<i32>().unwrap();
            words.insert(word, value);
            alphabet.extend(word.chars());
        }

        let trie = if alphabet.len() <= 128 {
            let alphabet: ZeroVec<'static, char> = alphabet.into_iter().collect();

            DictionaryBreakData::ZeroTrie {
                trie: ZeroTrieSimpleAscii::try_from_btree_map_str(
                    &words
                        .into_iter()
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
            DictionaryBreakData::Char16Trie(words.into_iter().collect())
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
