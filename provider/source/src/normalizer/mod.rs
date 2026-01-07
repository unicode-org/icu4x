// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::SourceDataProvider;
use crate::TrieType;
use icu::collections::char16trie::Char16Trie;
use icu::collections::char16trie::Char16TrieIterator;
use icu::collections::char16trie::TrieResult;
use icu::collections::codepointtrie::CodePointTrie;
use icu::normalizer::provider::*;
use icu_codepointtrie_builder::CodePointTrieBuilder;
use icu_codepointtrie_builder::CodePointTrieBuilderData;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use zerovec::ZeroVec;

mod normalizer_serde;

// These constants originate from page 143 of Unicode 14.0
/// Syllable base
const HANGUL_S_BASE: u32 = 0xAC00;
/// Lead jamo base
const HANGUL_L_BASE: u32 = 0x1100;
/// Lead jamo count
const HANGUL_L_COUNT: u32 = 19;
/// Trail jamo count (deliberately off by one to account for the absence of a trail)
const HANGUL_T_COUNT: u32 = 28;
/// Vowel jamo count times trail jamo count
const HANGUL_N_COUNT: u32 = 588;
/// Syllable count
const HANGUL_S_COUNT: u32 = 11172;
/// Trie value base corresponding for L
const HANGUL_L_TRIE_VAL_BASE: u16 = 0xD6A7;

// Marker for non-round-trip in NFC.
const NON_ROUND_TRIP_MARKER: u32 = 1 << 30;

macro_rules! normalization_provider {
    ($marker:ident, $serde_struct:ident, $file_name:literal, $conversion:expr, $toml_data:ident, $self:ident) => {
        use icu::normalizer::provider::$marker;

        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&$self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                $self.check_req::<$marker>(req)?;
                let $toml_data: &normalizer_serde::$serde_struct =
                    $self.icuexport()?.read_and_parse_toml(&format!(
                        "norm/{}/{}.toml",
                        if $file_name == "nfd" || $file_name == "nfkd" {
                            // Always use fast tries for these to unblock optimizations
                            // that depend being able to assume the fast trie type at compile
                            // time. See https://github.com/unicode-org/icu4x/pull/7222#issuecomment-3531679175
                            TrieType::Fast
                        } else {
                            $self.trie_type()
                        },
                        $file_name
                    ))?;

                $conversion
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    };
}

macro_rules! normalization_data_provider {
    ($marker:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            DecompositionData,
            $file_name,
            {
                let trie = CodePointTrie::<u32>::try_from(&toml_data.trie)
                    .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;

                let mut values = Vec::new();
                // Need to get the surrogate values, too, so iterating
                // by `u32`.
                for i in 0..=(char::MAX as u32) {
                    values.push(trie.get32(i))
                }

                // Mark lead jamo as non-passthrough. In principle,
                // they are passthrough, but we don't want to take the
                // passthrough path and then immediately fall off of it
                // due to the next character being a jamo V.
                for i in 0..HANGUL_L_COUNT {
                    values[(HANGUL_L_BASE + i) as usize] |= NON_ROUND_TRIP_MARKER;
                }

                let trie: CodePointTrie<u32> = CodePointTrieBuilder {
                    data: CodePointTrieBuilderData::ValuesByCodePoint(&values[..]),
                    default_value: 0,
                    error_value: 0,
                    trie_type: icu::collections::codepointtrie::TrieType::Fast,
                }
                .build();

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(DecompositionData {
                        trie,
                        passthrough_cap: toml_data.cap,
                    }),
                })
            },
            toml_data, // simply matches the identifier in the above block
            self
        );
    };
}

macro_rules! normalization_tables_provider {
    ($marker:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            DecompositionTables,
            $file_name,
            {
                let scalars24 = toml_data
                    .scalars32
                    .iter()
                    .map(|&u| {
                        u.try_into()
                            .map_err(|_| DataError::custom("scalars24 conversion"))
                    })
                    .collect::<Result<Vec<char>, DataError>>()?;
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(DecompositionTables {
                        scalars16: ZeroVec::alloc_from_slice(&toml_data.scalars16),
                        scalars24: ZeroVec::alloc_from_slice(&scalars24),
                    }),
                })
            },
            toml_data, // simply matches the identifier in the above block
            self
        );
    };
}

macro_rules! normalization_canonical_compositions_provider {
    ($marker:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            CanonicalCompositions,
            $file_name,
            {
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(CanonicalCompositions {
                        canonical_compositions: Char16Trie::new(ZeroVec::alloc_from_slice(
                            &toml_data.compositions,
                        )),
                    }),
                })
            },
            toml_data, // simply matches the identifier in the above block
            self
        );
    };
}

/// Performs (non-Hangul) canonical composition on a pair of characters
/// or returns `None` if these characters don't compose.
fn compose_non_hangul(mut iter: Char16TrieIterator, starter: char, second: char) -> Option<char> {
    // To make the trie smaller, the pairs are stored second character first.
    // Given how this method is used in ways where it's known that `second`
    // is or isn't a starter. We could potentially split the trie into two
    // tries depending on whether `second` is a starter.
    match iter.next(second) {
        TrieResult::NoMatch => None,
        TrieResult::NoValue => match iter.next(starter) {
            TrieResult::NoMatch => None,
            TrieResult::FinalValue(i) => {
                if let Some(c) = char::from_u32(i as u32) {
                    Some(c)
                } else {
                    // GIGO case
                    panic!("Bad canonical composition data");
                }
            }
            TrieResult::NoValue | TrieResult::Intermediate(_) => {
                // GIGO case
                panic!("Bad canonical composition data");
            }
        },
        TrieResult::FinalValue(_) | TrieResult::Intermediate(_) => {
            // GIGO case
            panic!("Bad canonical composition data");
        }
    }
}

// This is just for memory locality within `linear`.
// Probably not worthwhile to put more effort into
// this.
fn rank_primary(primary: char) -> u32 {
    match primary {
        '\u{0300}'..='\u{036F}' => 0,
        'a'..='z' => 1,
        'A'..='Z' => 2,
        'à'..='ÿ' => 3,
        'À'..='Ý' => 4,
        '\u{0100}'..='\u{02FF}' => 5,
        '\u{1E00}'..='\u{1EFF}' => 6,
        '\u{0370}'..='\u{03FF}' => 7,
        '\u{1F00}'..='\u{1FFF}' => 6,
        _ => u32::MAX,
    }
}

// Fallback when there is no frequency data
fn rank_secondary(secondary: char) -> u8 {
    match secondary {
        'e' => 0,
        'a' => 1,
        'o' => 2,
        'i' => 3,
        'u' => 4,
        'E' => 5,
        'A' => 6,
        'O' => 7,
        'I' => 8,
        'U' => 9,
        '\u{0301}' => 10,
        _ => u8::MAX,
    }
}

fn bmp(mappings: &[(char, char, u64)]) -> bool {
    for (secondary, composed, _) in mappings {
        if *secondary > '\u{FFFF}' {
            return false;
        }
        if *composed > '\u{FFFF}' {
            return false;
        }
    }
    true
}

macro_rules! normalization_canonical_compositions_provider_new {
    ($marker:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            CanonicalCompositions,
            $file_name,
            {
                let ranking_json_string = std::fs::read_to_string("/home/hsivonen/Downloads/out.json").unwrap();
                let rankings: HashMap<char, u64> = serde_json::from_str(&ranking_json_string).unwrap();

                let decomp_owned =
                    icu::normalizer::properties::CanonicalDecomposition::try_new_unstable(self)
                        .unwrap();
                let decomp: icu::normalizer::properties::CanonicalDecompositionBorrowed =
                    decomp_owned.as_borrowed();

                let old_style = Char16Trie::new(ZeroVec::alloc_from_slice(&toml_data.compositions));

                let mut hash: HashMap<char, Vec<(char, char, u64)>> = HashMap::new();
                for c in '\u{0}'..=char::MAX {
                    if c >= '\u{AC00}' && c <= '\u{D7AF}' {
                        continue;
                    }
                    if let icu::normalizer::properties::Decomposed::Expansion(first, second) =
                        decomp.decompose(c)
                    {
                        if let Some(composed) = compose_non_hangul(old_style.iter(), first, second)
                        {
                            assert_eq!(c, composed);
                            let (primary, secondary) = match first {
                                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                                    // This special case balances out the max length of entries
                                    // in `linear` so that no entry exceeds 10 items as of Unicode 17.
                                    (second, first)
                                }
                                _ => (first, second),
                            };
                            let entry = hash.entry(primary).or_insert(Vec::new());
                            entry.push((secondary, c, u64::MAX - *rankings.get(&c).unwrap_or(&0)));
                        }
                    }
                }
                let mut keys: Vec<char> = hash.keys().copied().collect();
                // The sort order can be changed freely here without
                // changing the lookup code or affecting correctness.
                // However, the order chosen here affects the memory
                // locality of accesses to `linear`.
                keys.sort_by_key(|&primary| (rank_primary(primary), primary));

                let default_value = 0b11111_11111_11111; // 15 bits set
                let error_value = 0b11111_11111_11111;

                let mut index = vec![default_value; char::MAX as usize];

                let mut linear16: Vec<(u16, u16)> = Vec::new();
                let mut linear24: Vec<(char, char)> = Vec::new();

                // Could be at most 0xD6A6, but let's have a nice binary-round number for more obvious debugging
                // and perhaps compressibility.
                let lv_marker = 0xD600u16;

                for primary in keys.into_iter() {
                    let v = hash.get_mut(&primary).unwrap();
                    // The sort order can be changed freely here without
                    // changing the lookup code or affecting correctness.
                    // However, the order chosen here affects how quickly
                    // the linear search terminates when a match exists,
                    // so considering the relative frequences of the possible
                    // `secondary` values _given_ `primary`, the possible
                    // `secondary` values should be sorted from most frequent
                    // to least frequent.
                    v.sort_by_key(|(secondary, _, rank)| (*rank, rank_secondary(*secondary), *secondary));

                    // Trie value format:
                    //
                    // HANGUL_L_TRIE_VAL_BASE through and inclusive 0xFFFF mark Hangul
                    // lead jamo such that the value is
                    // `((c - `HANGUL_L_BASE`) * HANGUL_N_COUNT) + HANGUL_L_TRIE_VAL_BASE`.
                    // (Only every 588th value is used. The above always have
                    // the most-significant bit set.)
                    //
                    // Otherwise:
                    // Bit 15 is 1 if lookup is from linear24 and 0 if from
                    // linear16.
                    //
                    // Bits 0..3 are slice length. (We need to be encode 10,
                    // and this allows us to encode 15. No need to spend one
                    // instruction upon decode to account for never having to
                    // encode 0.)
                    //
                    // The other bits are the length.
                    //
                    // Any trie value that has bit 15 set but the index bits
                    // are out of bounds for linear24 is considered to mark
                    // a Hangul LV syllable.
                    let (supplementary, start, len) = if bmp(&v) {
                        let start = linear16.len();
                        for (secondary, composed, _) in v {
                            linear16.push((*secondary as u16, *composed as u16));
                        }
                        let len = linear16.len() - start;
                        (false, start, len)
                    } else {
                        let start = linear24.len();
                        for (secondary, composed, _) in v {
                            linear24.push((*secondary, *composed));
                        }
                        let len = linear24.len() - start;
                        (true, start, len)
                    };
                    assert!(len < 16);
                    let start_and_len = start << 4 | len;
                    assert!(start_and_len < default_value as usize);
                    if supplementary {
                        let tagged_start_and_len = start_and_len | (1 << 15);
                        assert!(tagged_start_and_len < lv_marker as usize);
                        index[primary as usize] = tagged_start_and_len as u16;
                    } else {
                        index[primary as usize] = start_and_len as u16;
                    }
                }

                for l in 0..HANGUL_L_COUNT {
                    index[(HANGUL_L_BASE + l) as usize] = HANGUL_L_TRIE_VAL_BASE + (l * HANGUL_N_COUNT) as u16;
                }

                // According to databake, the cost of marking the lv syllables vs. marking
                // the full syllable range is 96 bytes. It seems well worthwhile to pay
                // 96 bytes in size in order to avoid costlier math at normalization time.
                let mut lv = HANGUL_S_BASE;
                while lv < HANGUL_S_BASE + HANGUL_S_COUNT {
                    index[lv as usize] = lv_marker;
                    lv += HANGUL_T_COUNT;
                }

                let trie: CodePointTrie<u16> = CodePointTrieBuilder {
                    data: CodePointTrieBuilderData::ValuesByCodePoint(&index[..]),
                    default_value,
                    error_value,
                    trie_type: icu::collections::codepointtrie::TrieType::Fast,
                }
                .build();

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(CanonicalCompositionsNew {
                        trie,
                        linear16: ZeroVec::alloc_from_slice(&linear16),
                        linear24: ZeroVec::alloc_from_slice(&linear24),
                    }),
                })
            },
            toml_data, // simply matches the identifier in the above block
            self
        );
    };
}

macro_rules! normalization_non_recursive_decomposition_supplement_provider {
    ($marker:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            NonRecursiveDecompositionSupplement,
            $file_name,
            {
                let trie = CodePointTrie::<u32>::try_from(&toml_data.trie)
                    .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;
                let scalars24 = toml_data
                    .scalars32
                    .iter()
                    .map(|&u| {
                        u.try_into()
                            .map_err(|_| DataError::custom("scalars24 conversion"))
                    })
                    .collect::<Result<Vec<char>, DataError>>()?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(NonRecursiveDecompositionSupplement {
                        trie,
                        scalars24: ZeroVec::alloc_from_slice(&scalars24),
                    }),
                })
            },
            toml_data, // simply matches the identifier in the above block
            self
        );
    };
}

normalization_data_provider!(NormalizerNfdDataV1, "nfd");

normalization_data_provider!(NormalizerNfkdDataV1, "nfkd");

normalization_data_provider!(NormalizerUts46DataV1, "uts46d");

normalization_tables_provider!(NormalizerNfdTablesV1, "nfdex");

normalization_tables_provider!(NormalizerNfkdTablesV1, "nfkdex");

// No uts46dex, because that data is also in nfkdex.

normalization_canonical_compositions_provider!(NormalizerNfcV1, "compositions");

normalization_canonical_compositions_provider_new!(NormalizerNfcV2, "compositions");

normalization_non_recursive_decomposition_supplement_provider!(
    NormalizerNfdSupplementV1,
    "decompositionex"
);
