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

/// The provider struct holding the `SourceData` and the `RWLock`-wrapped
/// TOML data.
pub struct CanonicalDecompositionDataProvider {
    source: SourceData,
}

impl From<&SourceData> for CanonicalDecompositionDataProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

impl ResourceProvider<CanonicalDecompositionDataV1Marker> for CanonicalDecompositionDataProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<CanonicalDecompositionDataV1Marker>, DataError> {
        let toml_data = self.source.get_uprops_paths()?.get_decompositions()?;

        let mut builder = UnicodeSetBuilder::new();
        for range in &toml_data.ranges {
            builder.add_range_u32(&(range.0..=range.1));
        }
        let uniset = builder.build();

        let trie = CodePointTrie::<u32>::try_from(&toml_data.trie)
            .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(CanonicalDecompositionDataV1 {
                trie,
                scalars16: ZeroVec::alloc_from_slice(&toml_data.scalars16),
                scalars32: ZeroVec::alloc_from_slice(&toml_data.scalars32),
                decomposition_starts_with_non_starter: uniset,
            })),
        })
    }
}

icu_provider::make_exportable_provider!(
    CanonicalDecompositionDataProvider,
    [CanonicalDecompositionDataV1Marker,]
);

impl IterableResourceProvider<CanonicalDecompositionDataV1Marker>
    for CanonicalDecompositionDataProvider
{
    fn supported_options(&self) -> Result<Vec<ResourceOptions>, DataError> {
        Ok(vec![ResourceOptions::default()])
    }
}
