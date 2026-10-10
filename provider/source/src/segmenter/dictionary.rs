// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::collections::char16trie::Char16Trie;
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::segmenter::provider::DictionaryBreakData;
use icu::segmenter::provider::SegmenterDictionaryAutoV2;
use icu::segmenter::provider::SegmenterDictionaryExtendedV2;
use icu_provider::prelude::*;
use itertools::Itertools;
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

        let mut alphabet_builder = CodePointInversionListBuilder::new();

        for (word, _) in dict.iter2() {
            for c in word.chars() {
                alphabet_builder.add_char(c);
            }
        }

        let alphabet = alphabet_builder.clone().build();

        let trie = if alphabet.size() <= 128 {
            let mut minimal_alphabet = alphabet_builder;
            let mut size = alphabet.size();
            for gap in alphabet
                .iter_ranges_complemented()
                .sorted_by_key(|r| r.end() - r.start())
            {
                size += (gap.end() - gap.start() + 1) as usize;
                if size > 128 {
                    break;
                }
                minimal_alphabet.add_range32(gap);
            }

            let alphabet = minimal_alphabet.build();

            DictionaryBreakData::ZeroTrie {
                trie: ZeroTrieSimpleAscii::try_from_btree_map_str(
                    &dict
                        .iter2()
                        .map(|(k, v)| {
                            (
                                k.chars()
                                    .map(|c| alphabet.position(c).unwrap() as u8 as char)
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
