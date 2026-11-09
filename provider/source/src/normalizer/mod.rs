// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::SourceDataProvider;
use crate::TrieType;
use icu::collections::char16trie::Char16Trie;
use icu::collections::char16trie::Char16TrieCursor;
use icu::collections::codepointtrie::CodePointTrie;
use icu::collections::codepointtrie::CodePointTrieBuilder;
use icu::normalizer::provider::*;
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

                let mut builder = CodePointTrieBuilder::new(
                    0,
                    0,
                    icu::collections::codepointtrie::TrieType::Fast,
                );

                for i in 0..HANGUL_L_BASE {
                    builder.set_value(i, trie.get32(i));
                }
                for i in HANGUL_L_BASE..HANGUL_L_BASE + HANGUL_L_COUNT {
                    builder.set_value(i, NON_ROUND_TRIP_MARKER | trie.get32(i));
                }
                for i in HANGUL_L_BASE + HANGUL_L_COUNT..=(char::MAX as u32) {
                    builder.set_value(i, trie.get32(i));
                }

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(DecompositionData {
                        trie: builder.build(),
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
fn compose_non_hangul(mut cursor: Char16TrieCursor, starter: char, second: char) -> Option<char> {
    // To make the trie smaller, the pairs are stored second character first.
    // Given how this method is used in ways where it's known that `second`
    // is or isn't a starter. We could potentially split the trie into two
    // tries depending on whether `second` is a starter.
    cursor.step(second);
    if cursor.is_empty() {
        // GIGO case
        assert!(cursor.value().is_none(), "Bad canonical composition data");
        return None;
    }
    if cursor.value().is_some() {
        // GIGO case
        panic!("Bad canonical composition data");
    }
    cursor.step(starter);
    match (cursor.value(), cursor.is_empty()) {
        (None, true) => None,
        (Some(i), true) => {
            if let Some(c) = char::from_u32(i as u32) {
                Some(c)
            } else {
                // GIGO case
                panic!("Bad canonical composition data");
            }
        }
        (_, false) => {
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
                let rankings: HashMap<char, u64> = serde_json::from_str(include_str!(
                    "../../data/nfc_statistics.json"
                ))
                .map_err(|e| DataError::custom("JSON deserialize").with_display_context(&e))?;

                let decomp_owned =
                    icu::normalizer::properties::CanonicalDecomposition::try_new_unstable(self)?;
                let decomp: icu::normalizer::properties::CanonicalDecompositionBorrowed =
                    decomp_owned.as_borrowed();

                let old_style = Char16Trie::new(ZeroVec::alloc_from_slice(&toml_data.compositions));

                let mut hash: HashMap<char, Vec<(char, char, u64)>> = HashMap::new();
                for c in '\u{0}'..=char::MAX {
                    if ('\u{AC00}'..='\u{D7AF}').contains(&c) {
                        continue;
                    }
                    if let icu::normalizer::properties::Decomposed::Expansion(first, second) =
                        decomp.decompose(c)
                    {
                        if let Some(composed) =
                            compose_non_hangul(old_style.cursor(), first, second)
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

                let mut builder = CodePointTrieBuilder::new(
                    default_value,
                    error_value,
                    icu::collections::codepointtrie::TrieType::Fast,
                );

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
                    v.sort_by_key(|(secondary, _, rank)| {
                        (*rank, rank_secondary(*secondary), *secondary)
                    });

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
                        builder.set_value(u32::from(primary), tagged_start_and_len as u16);
                    } else {
                        builder.set_value(u32::from(primary), start_and_len as u16);
                    }
                }

                // The lookup code distinguishes the special trie values from
                // real entries by the special values decoding to slices that
                // are out of bounds. Check that the arrays don't grow long enough
                // to make those slices in bounds.
                //
                // `default_value` decodes to a linear16 slice.
                assert!(
                    linear16.len()
                        < usize::from(default_value >> 4) + usize::from(default_value & 0b1111)
                );
                // `lv_marker` decodes to an empty linear24 slice. (Hangul lead
                // jamo values decode to linear24 slices that start even later.)
                assert!(linear24.len() < usize::from((lv_marker >> 4) & 0b1_11111_11111));

                for l in 0..HANGUL_L_COUNT {
                    builder.set_value(
                        HANGUL_L_BASE + l,
                        HANGUL_L_TRIE_VAL_BASE + (l * HANGUL_N_COUNT) as u16,
                    );
                }

                // According to databake, the cost of marking the lv syllables vs. marking
                // the full syllable range is 96 bytes. It seems well worthwhile to pay
                // 96 bytes in size in order to avoid costlier math at normalization time.
                let mut lv = HANGUL_S_BASE;
                while lv < HANGUL_S_BASE + HANGUL_S_COUNT {
                    builder.set_value(lv, lv_marker);
                    lv += HANGUL_T_COUNT;
                }

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(CanonicalCompositionsNew {
                        trie: builder.build(),
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

// These macros implement ICU4C-internal properties that we have accidentally exposed (#7892).
// They are slated for removal, but the code might be useful for a future ICU4C-independent
// normalization implementation, which is why they live in this file.

macro_rules! impl_decomposition_inert_property {
    ($marker:ident, $try_new_decomp:ident) => {
        impl DataProvider<icu::properties::provider::$marker> for SourceDataProvider {
            fn load(
                &self,
                _req: DataRequest,
            ) -> Result<DataResponse<icu::properties::provider::$marker>, DataError> {
                use icu::collections::codepointinvlist::CodePointInversionListBuilder;
                use icu::normalizer::DecomposingNormalizer;
                use icu::properties::{
                    CodePointMapData, props::CanonicalCombiningClass,
                    provider::PropertyCodePointSet,
                };

                let decomp = DecomposingNormalizer::$try_new_decomp(self)?;
                let decomp = decomp.as_borrowed();
                let ccc = CodePointMapData::<CanonicalCombiningClass>::try_new_unstable(self)?;
                let ccc = ccc.as_borrowed();

                let mut builder = CodePointInversionListBuilder::new();
                // Add all code points that are starters and are not decomposable,
                // including surrogates.
                for cp in 0..=(char::MAX as u32) {
                    let Some(ch) = char::from_u32(cp) else {
                        builder.add32(cp);
                        continue;
                    };

                    if ccc.get(ch) == CanonicalCombiningClass::NotReordered
                        && decomp.is_normalized(ch.encode_utf8(&mut [0; 4]))
                    {
                        builder.add32(cp);
                    }
                }
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(
                        builder.build(),
                    )),
                })
            }
        }
        impl crate::IterableDataProviderCached<icu::properties::provider::$marker>
            for SourceDataProvider
        {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    };
}

macro_rules! impl_composition_inert_property {
    ($marker:ident, $try_new_decomp:ident, $try_new_comp:ident) => {
        impl DataProvider<icu::properties::provider::$marker> for SourceDataProvider {
            fn load(
                &self,
                _req: DataRequest,
            ) -> Result<DataResponse<icu::properties::provider::$marker>, DataError> {
                use icu::collections::codepointinvlist::CodePointInversionListBuilder;
                use icu::normalizer::properties::{
                    CanonicalComposition, CanonicalDecomposition, Decomposed,
                };
                use icu::normalizer::{ComposingNormalizer, DecomposingNormalizer};
                use icu::properties::{
                    CodePointMapData, props::CanonicalCombiningClass,
                    provider::PropertyCodePointSet,
                };

                let composing_normalizer = ComposingNormalizer::$try_new_comp(self)?;
                let composing_normalizer = composing_normalizer.as_borrowed();
                let decomposing_normalizer = DecomposingNormalizer::$try_new_decomp(self)?;
                let decomposing_normalizer = decomposing_normalizer.as_borrowed();
                let nfd = DecomposingNormalizer::try_new_nfd_unstable(self)?;
                let nfd = nfd.as_borrowed();

                let canonical_comp = CanonicalComposition::try_new_unstable(self)?;
                let canonical_comp = canonical_comp.as_borrowed();
                let canonical_decomp = CanonicalDecomposition::try_new_unstable(self)?;
                let canonical_decomp = canonical_decomp.as_borrowed();
                let ccc = CodePointMapData::<CanonicalCombiningClass>::try_new_unstable(self)?;
                let ccc = ccc.as_borrowed();

                let mut combines_forwards = HashSet::new();
                let mut canonical_comp_seconds = HashSet::new();
                let mut composes_with_lowest_reordered_ccc = HashMap::new();

                // Compute `combines_forwards`, `canonical_comp_seconds`, and
                // `composes_with_lowest_reordered_ccc` from the canonical
                // decompositions of all primary composites.
                for ch in (0..=char::MAX as u32).filter_map(char::from_u32) {
                    if let Decomposed::Expansion(starter, second) = canonical_decomp.decompose(ch)
                        && canonical_comp.compose(starter, second) == Some(ch)
                    {
                        combines_forwards.insert(starter);
                        canonical_comp_seconds.insert(second);
                        let ccc = ccc.get(second);
                        if ccc > CanonicalCombiningClass::NotReordered {
                            composes_with_lowest_reordered_ccc
                                .entry(starter)
                                .and_modify(|c| *c = std::cmp::min(*c, ccc))
                                .or_insert(ccc);
                        }
                    }
                }

                let mut combines_backwards = HashSet::new();
                for ch in (0..=char::MAX as u32).filter_map(char::from_u32) {
                    let nfd_first = nfd.normalize_iter([ch].into_iter()).next().unwrap();
                    if canonical_comp_seconds.contains(&nfd_first) {
                        combines_backwards.insert(ch);
                    }
                }

                let mut builder = CodePointInversionListBuilder::new();
                'cp: for cp in 0..=(char::MAX as u32) {
                    let Some(ch) = char::from_u32(cp) else {
                        builder.add32(cp);
                        continue;
                    };

                    if ccc.get(ch) != CanonicalCombiningClass::NotReordered {
                        continue;
                    }

                    if combines_forwards.contains(&ch) {
                        continue;
                    }

                    if !composing_normalizer.is_normalized(ch.encode_utf8(&mut [0; 4])) {
                        continue;
                    }

                    let mut decomposed = decomposing_normalizer.normalize_iter([ch].into_iter());

                    let mut starter = decomposed.next().unwrap();

                    if combines_backwards.contains(&starter) {
                        continue;
                    }

                    for follow in decomposed {
                        if let Some(&lowest_ccc) = composes_with_lowest_reordered_ccc.get(&starter)
                            && lowest_ccc < ccc.get(follow)
                        {
                            // There exists a character C (with
                            // ccc=`lowest_ccc`) which composes with `starter`
                            // and which is not blocked from `starter` in the
                            // sequence <`starter`, `follow`, C>, nor therefore in
                            // <`starter`, (rest of `decomposed`), C>,
                            // thus the decomposition of <`ch`, C> would compose
                            // to something different and `ch` is not inert.
                            continue 'cp;
                        }

                        starter = canonical_comp.compose(starter, follow).unwrap();
                    }

                    builder.add32(cp);
                }
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(
                        builder.build(),
                    )),
                })
            }
        }
        impl crate::IterableDataProviderCached<icu::properties::provider::$marker>
            for SourceDataProvider
        {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    };
}

impl_decomposition_inert_property!(PropertyBinaryNfdInertV1, try_new_nfd_unstable);
impl_decomposition_inert_property!(PropertyBinaryNfkdInertV1, try_new_nfkd_unstable);
impl_composition_inert_property!(
    PropertyBinaryNfcInertV1,
    try_new_nfd_unstable,
    try_new_nfc_unstable
);
impl_composition_inert_property!(
    PropertyBinaryNfkcInertV1,
    try_new_nfkd_unstable,
    try_new_nfkc_unstable
);
