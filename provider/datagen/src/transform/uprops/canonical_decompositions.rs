// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use std::convert::TryFrom;
use zerovec::ZeroVec;

use crate::transform::reader::read_path_to_string;
use crate::transform::uprops::decompositions_serde::CanonicalDecompositionData;

use icu_normalizer::provider::CanonicalDecompositionDataV1;
use icu_normalizer::provider::CanonicalDecompositionDataV1Marker;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetBuilder;

use std::path::Path;
use std::sync::RwLock;

/// The provider struct holding the `SourceData` and the `RWLock`-wrapped
/// TOML data.
pub struct CanonicalDecompositionDataProvider {
    source: SourceData,
    data: RwLock<Option<CanonicalDecompositionData>>,
}

impl From<&SourceData> for CanonicalDecompositionDataProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
            data: RwLock::new(None),
        }
    }
}

impl ResourceProvider<CanonicalDecompositionDataV1Marker> for CanonicalDecompositionDataProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<CanonicalDecompositionDataV1Marker>, DataError> {
        if self.data.read().unwrap().is_none() {
            let path_buf = self.source.get_uprops_root()?.join("decompositions.toml");
            let path: &Path = &path_buf;
            let toml_str = read_path_to_string(path)?;
            let toml_obj: CanonicalDecompositionData = toml::from_str(&toml_str)
                .map_err(|e| crate::error::data_error_from_toml(e).with_path_context(path))?;
            *self.data.write().unwrap() = Some(toml_obj);
        }

        let guard = self.data.read().unwrap();

        let toml_data = guard.as_ref().unwrap();

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
