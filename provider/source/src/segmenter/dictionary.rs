// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use alloc::collections::BTreeMap;
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::segmenter::provider::DictionaryBreakData;
use icu::segmenter::provider::SegmenterDictionaryAutoV2;
use icu::segmenter::provider::SegmenterDictionaryExtendedV2;
use icu_provider::prelude::*;
use itertools::Itertools;
use std::collections::HashSet;
use zerotrie::ZeroTrieSimpleAscii;

impl SourceDataProvider {
    fn load_dictionary_data(
        &self,
        req: DataRequest,
    ) -> Result<DictionaryBreakData<'static>, DataError> {
        let dict = &self.icuexport()?.read_and_parse(
            &format!(
                "segmenter/dictionary/{}.txt",
                req.id.marker_attributes as &str
            ),
            |dict| {
                let dict =
                    String::from_utf8(dict.into_vec()).map_err(|_| DataError::custom("UTF-8"))?;

                let dict = dict.strip_prefix('\u{FEFF}').unwrap_or(&dict);

                Ok(dict
                    .lines()
                    .filter_map(|line| {
                        let line = line.split_once('#').unwrap_or((line, "")).0.trim();
                        if line.is_empty() {
                            return None;
                        }
                        let mut parts = line.split_ascii_whitespace();
                        let word = parts.next().unwrap();
                        let value = parts.next().unwrap_or("0").parse::<i32>().unwrap();
                        Some((word.to_string(), value))
                    })
                    .collect::<BTreeMap<_, _>>())
            },
        )?;

        let mut alphabet_builder = CodePointInversionListBuilder::new();

        for word in dict.keys() {
            for c in word.chars() {
                alphabet_builder.add_char(c);
            }
        }

        let alphabet = alphabet_builder.clone().build();

        let trie = if alphabet.size() <= 128 {
            // Build a smaller alphabet CPIL. The alphabet might be fratured, and every
            // block in the CPIL increases runtime cost (binary search). So we fill in
            // as many gaps as we can, while keeping total size < 128. For Thai and Lao
            // this results in single blocks (which is equivalent to just storing an
            // offset). Burmese and Khmer use ZWJ/ZWNJ, which are too far away from the
            // respective scripts, so those alphabets contain 2/3 blocks (which should
            // still be fast enough to look up).
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

            if alphabet.iter_ranges().count() > 3 {
                log::warn!(
                    "Fractured alphabet for {}: {alphabet:?}",
                    req.id.marker_attributes as &str
                )
            }

            DictionaryBreakData::ZeroTrie {
                trie: ZeroTrieSimpleAscii::try_from_btree_map_str(
                    &dict
                        .iter()
                        .map(|(k, &v)| {
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
            DictionaryBreakData::Char16Trie(dict.iter().map(|(k, &v)| (k, v)).collect())
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
