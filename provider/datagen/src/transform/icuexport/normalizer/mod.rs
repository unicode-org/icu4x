// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use icu_collections::char16trie::Char16Trie;
use icu_collections::codepointtrie::CodePointTrie;
use icu_normalizer::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use zerovec::ZeroVec;

mod normalizer_serde;

macro_rules! normalization_provider {
    ($marker:ident, $serde_struct:ident, $file_name:literal, $conversion:expr, $toml_data:ident) => {
        use icu_normalizer::provider::$marker;

        impl DataProvider<$marker> for crate::DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let $toml_data: &normalizer_serde::$serde_struct =
                    self.source.icuexport()?.read_and_parse_toml(&format!(
                        "norm/{}/{}.toml",
                        self.source.options.trie_type, $file_name
                    ))?;

                $conversion
            }
        }

        impl IterableDataProvider<$marker> for crate::DatagenProvider {
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                Ok(vec![DataLocale::default()])
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
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(DecompositionDataV1 { trie })),
                })
            },
            toml_data // simply matches the identifier in the above block
        );
    };
}

macro_rules! normalization_supplement_provider {
    ($marker:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            DecompositionSupplement,
            $file_name,
            {
                let trie = CodePointTrie::<u32>::try_from(&toml_data.trie)
                    .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;

                Ok(DataResponse {
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(DecompositionSupplementV1 {
                        trie,
                        flags: toml_data.flags,
                        passthrough_cap: toml_data.cap,
                    })),
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
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(DecompositionTablesV1 {
                        scalars16: ZeroVec::alloc_from_slice(&toml_data.scalars16),
                        scalars24: ZeroVec::alloc_from_slice(&scalars24),
                    })),
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
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(CanonicalCompositionsV1 {
                        canonical_compositions: Char16Trie::new(ZeroVec::alloc_from_slice(
                            &toml_data.compositions,
                        )),
                    })),
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
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(
                        NonRecursiveDecompositionSupplementV1 {
                            trie,
                            scalars24: ZeroVec::alloc_from_slice(&scalars24),
                        },
                    )),
                })
            },
            toml_data // simply matches the identifier in the above block
        );
    };
}

normalization_data_provider!(CanonicalDecompositionDataV1Marker, "nfd");

normalization_supplement_provider!(CompatibilityDecompositionSupplementV1Marker, "nfkd");

normalization_supplement_provider!(Uts46DecompositionSupplementV1Marker, "uts46d");

normalization_tables_provider!(CanonicalDecompositionTablesV1Marker, "nfdex");

normalization_tables_provider!(CompatibilityDecompositionTablesV1Marker, "nfkdex");

// No uts46dex, because that data is also in nfkdex.

normalization_canonical_compositions_provider!(CanonicalCompositionsV1Marker, "compositions");

normalization_non_recursive_decomposition_supplement_provider!(
    NonRecursiveDecompositionSupplementV1Marker,
    "decompositionex"
);
