// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use icu_normalizer::provider::CanonicalDecompositionDataV1;
use icu_normalizer::provider::CanonicalDecompositionDataV1Marker;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetBuilder;
use std::convert::TryFrom;
use zerovec::ZeroVec;
use icu_normalizer::u24::U24;

macro_rules! normalization_provider {
    ($marker:ident, $provider:ident, $serde_struct:ident, $file_name:literal, $conversion:expr, $toml_data:ident) => {
        use icu_normalizer::provider::$marker;

        /// The provider struct holding the `SourceData` and the `RWLock`-wrapped
        /// TOML data.
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

        impl ResourceProvider<$marker> for $provider {
            fn load_resource(
                &self,
                _req: &DataRequest,
            ) -> Result<DataResponse<$marker>, DataError> {
                let $toml_data: $serde_struct = self.source.get_uprops_paths.read_and_parse_toml($file_name)?;

                $conversion
            }
        }

        icu_provider::make_exportable_provider!($provider, [$marker,]);

        impl IterableResourceProvider<$marker> for $provider {
            fn supported_options(&self) -> Result<Vec<ResourceOptions>, DataError> {
                Ok(vec![ResourceOptions::default()])
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
            toml_data
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
            toml_data
        );
    };
}

normalization_data_provider!(
    CanonicalDecompositionDataV1Marker,
    CanonicalDecompositionDataProvider,
    "nfd.toml"
);

normalization_data_provider!(
    CompatibilityDecompositionDataV1Marker,
    CompatibilityDecompositionDataProvider,
    "nfkd.toml"
);

normalization_data_provider!(
    Uts46DecompositionDataV1Marker,
    Uts46DecompositionDataProvider,
    "uts46d.toml"
);

normalization_tables_provider!(
    CanonicalDecompositionTablesV1Marker,
    CanonicalDecompositionTablesProvider,
    "nfdex.toml"
);

normalization_tables_provider!(
    CompatibilityDecompositionTablesV1Marker,
    CompatibilityDecompositionTablesProvider,
    "nfkdex.toml"
);

// No uts46dex.toml, because that data is also in nfkdex.toml.
