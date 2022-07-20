// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::SourceData;
use icu_char16trie::char16trie::Char16Trie;
use icu_codepointtrie::CodePointTrie;
use icu_normalizer::provider::*;
use icu_normalizer::u24::U24;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetBuilder;
use std::convert::TryFrom;
use zerovec::ZeroVec;

mod normalizer_serde;

macro_rules! normalization_provider {
    ($marker:ident, $provider:ident, $serde_struct:ident, $file_name:literal, $conversion:expr, $toml_data:ident) => {
        use icu_normalizer::provider::$marker;

        /// A data provider reading from TOML files produced by the ICU4C icuexportdata tool.
        pub struct $provider {
            source: SourceData,
        }

        impl From<&SourceData> for $provider {
            fn from(source: &SourceData) -> Self {
                Self {
                    source: source.clone(),
                }
            }
        }

        impl DataProvider<$marker> for $provider {
            fn load(&self, _req: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                let $toml_data: &normalizer_serde::$serde_struct =
                    self.source.icuexport()?.read_and_parse_toml(&format!(
                        "norm/{}/{}.toml",
                        self.source.trie_type(),
                        $file_name
                    ))?;

                $conversion
            }
        }

        icu_provider::make_exportable_provider!($provider, [$marker,]);

        impl IterableDataProvider<$marker> for $provider {
            fn supported_options(&self) -> Result<Vec<DataOptions>, DataError> {
                Ok(vec![DataOptions::default()])
            }
        }
    };
}

macro_rules! normalization_data_provider {
    ($marker:ident, $provider:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            $provider,
            DecompositionData,
            $file_name,
            {
                let mut builder = UnicodeSetBuilder::new();
                for range in &toml_data.ranges {
                    builder.add_range_u32(&(range.0..=range.1));
                }
                let uniset = builder.build();

                let trie = CodePointTrie::<u32>::try_from(&toml_data.trie)
                    .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;

                Ok(DataResponse {
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(DecompositionDataV1 {
                        trie,
                        decomposition_starts_with_non_starter: uniset,
                    })),
                })
            },
            toml_data // simply matches the identifier in the above block
        );
    };
}

macro_rules! normalization_supplement_provider {
    ($marker:ident, $provider:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            $provider,
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
                    })),
                })
            },
            toml_data // simply matches the identifier in the above block
        );
    };
}

macro_rules! normalization_tables_provider {
    ($marker:ident, $provider:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            $provider,
            DecompositionTables,
            $file_name,
            {
                let mut scalars24: Vec<U24> = Vec::new();
                for &u in toml_data.scalars32.iter() {
                    scalars24.push(
                        u.try_into()
                            .map_err(|_| DataError::custom("scalars24 conversion"))?,
                    );
                }
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

macro_rules! normalization_passthrough_provider {
    ($marker:ident, $provider:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            $provider,
            CompositionPassthrough,
            $file_name,
            {
                let mut builder = UnicodeSetBuilder::new();
                for range in &toml_data.ranges {
                    builder.add_range_u32(&(range.0..=range.1));
                }
                let uniset = builder.build();

                Ok(DataResponse {
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(CompositionPassthroughV1 {
                        potential_passthrough_and_not_backward_combining: uniset,
                    })),
                })
            },
            toml_data // simply matches the identifier in the above block
        );
    };
}

macro_rules! normalization_canonical_compositions_provider {
    ($marker:ident, $provider:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            $provider,
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
    ($marker:ident, $provider:ident, $file_name:literal) => {
        normalization_provider!(
            $marker,
            $provider,
            NonRecursiveDecompositionSupplement,
            $file_name,
            {
                let trie = CodePointTrie::<u32>::try_from(&toml_data.trie)
                    .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;
                let mut scalars24: Vec<U24> = Vec::new();
                for &u in toml_data.scalars32.iter() {
                    scalars24.push(
                        u.try_into()
                            .map_err(|_| DataError::custom("scalars24 conversion"))?,
                    );
                }

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

normalization_data_provider!(
    CanonicalDecompositionDataV1Marker,
    CanonicalDecompositionDataProvider,
    "nfd"
);

normalization_supplement_provider!(
    CompatibilityDecompositionSupplementV1Marker,
    CompatibilityDecompositionSupplementProvider,
    "nfkd"
);

normalization_supplement_provider!(
    Uts46DecompositionSupplementV1Marker,
    Uts46DecompositionSupplementProvider,
    "uts46d"
);

normalization_tables_provider!(
    CanonicalDecompositionTablesV1Marker,
    CanonicalDecompositionTablesProvider,
    "nfdex"
);

normalization_tables_provider!(
    CompatibilityDecompositionTablesV1Marker,
    CompatibilityDecompositionTablesProvider,
    "nfkdex"
);

// No uts46dex, because that data is also in nfkdex.

normalization_passthrough_provider!(
    CanonicalCompositionPassthroughV1Marker,
    CanonicalCompositionPassthroughProvider,
    // nfkc.toml is close enough that we could provide an option
    // to use nfkc.toml here so that it would get deduplicated
    // with the meant-for-NFKC case below for data size at the
    // expense of pessimizing the performance for the characters
    // that have compatibility decompositions that matter for
    // NFKC but that could be passed through in NFC.
    // This optimization only makes sense if the application
    // is known to use both NFC and NFKC, and it's likely
    // enough for an app to want NFC and for it to be maximally
    // performant that it doesn't make sense to default to
    // this size optimization.
    "nfc"
);

normalization_passthrough_provider!(
    CompatibilityCompositionPassthroughV1Marker,
    CompatibilityCompositionPassthroughProvider,
    "nfkc"
);

normalization_passthrough_provider!(
    Uts46CompositionPassthroughV1Marker,
    Uts46CompositionPassthroughProvider,
    "uts46"
);

normalization_canonical_compositions_provider!(
    CanonicalCompositionsV1Marker,
    CanonicalCompositionsProvider,
    "compositions"
);

normalization_non_recursive_decomposition_supplement_provider!(
    NonRecursiveDecompositionSupplementV1Marker,
    NonRecursiveDecompositionSupplementProvider,
    "decompositionex"
);
