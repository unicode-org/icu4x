// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::SourceDataProvider;
use crate::TrieType;
use icu::collections::char16trie::Char16Trie;
use icu::collections::codepointtrie::CodePointTrie;
use icu::normalizer::provider::*;
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use zerovec::ZeroVec;

mod normalizer_serde;

macro_rules! normalization_provider {
    ($marker:ident, $serde_struct:ident, $file_name:literal, $conversion:expr, $toml_data:ident) => {
        use icu::normalizer::provider::$marker;

        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let $toml_data: &normalizer_serde::$serde_struct =
                    self.icuexport()?.read_and_parse_toml(&format!(
                        "norm/{}/{}.toml",
                        if $file_name == "nfd" || $file_name == "nfkd" {
                            // Always use fast tries for these to unblock optimizations
                            // that depend being able to assume the fast trie type at compile
                            // time. See https://github.com/unicode-org/icu4x/pull/7222#issuecomment-3531679175
                            TrieType::Fast
                        } else {
                            self.trie_type()
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

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(DecompositionData {
                        trie,
                        passthrough_cap: toml_data.cap,
                    }),
                })
            },
            toml_data // simply matches the identifier in the above block
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
            toml_data // simply matches the identifier in the above block
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
            toml_data // simply matches the identifier in the above block
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
            toml_data // simply matches the identifier in the above block
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
                let mut potential_seconds = HashSet::new();
                let mut composes_with_lowest_reordered_ccc = HashMap::new();

                for ch in (0..=char::MAX as u32).filter_map(char::from_u32) {
                    if let Decomposed::Expansion(starter, second) = canonical_decomp.decompose(ch)
                        && canonical_comp.compose(starter, second) == Some(ch)
                    {
                        combines_forwards.insert(starter);
                        potential_seconds.insert(second);
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
                    let starter = nfd.normalize_iter([ch].into_iter()).next().unwrap();
                    if potential_seconds.contains(&starter) {
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
