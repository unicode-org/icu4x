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
use std::collections::HashSet;
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
