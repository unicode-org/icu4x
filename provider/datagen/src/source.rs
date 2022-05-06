// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use icu_codepointtrie::TrieType;
use crate::transform::cldr::source::CldrPaths;
use crate::transform::uprops::source::UpropsPaths;
use icu_provider::DataError;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

/// Bag of options for datagen source data.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SourceData {
    cldr_paths: Option<Arc<CldrPaths>>,
    uprops_paths: Option<Arc<UpropsPaths>>,
    trie_type: TrieType,
}

impl Default for SourceData {
    fn default() -> Self {
        Self {
            cldr_paths: None,
            uprops_paths: None,
            trie_type: TrieType::Small,
        }
    }
}

impl SourceData {
    /// Adds CLDR data to this `DataSource`. The root should point to
    /// a local `cldr-{version}-json-full.zip` directory (see [GitHub downloads](
    /// https://github.com/unicode-org/cldr-json/releases)), and `locale_subset`
    /// is a valid locale subset (currently either "full" or "modern").
    pub fn with_cldr(self, root: PathBuf, locale_subset: String) -> Self {
        Self {
            cldr_paths: Some(Arc::new(CldrPaths::new(root, locale_subset))),
            ..self
        }
    }

    /// Adds Unicode Properties data to this `DataSource`. The path should
    /// point to a local `icuexportdata_uprops_full` directory (see
    /// [GitHub downloads](https://github.com/unicode-org/icu/releases)).
    pub fn with_uprops(self, root: PathBuf) -> Self {
        Self {
            uprops_paths: Some(Arc::new(UpropsPaths::new(root))),
            ..self
        }
    }

    /// Sets the [`TrieType`] to be used when generating data, including rule-based
    /// segmentation data.
    ///
    /// For Unicode Properties data, the TrieType is implicitly set when selecting the
    /// root directory (either "small" or "fast").
    pub fn with_trie_type(self, trie_type: TrieType) -> Self {
        Self { trie_type, ..self }
    }

    #[cfg(test)]
    /// Create `SourceData` pointing to test data.
    pub(crate) fn for_test() -> Self {
        Self::default()
            .with_cldr(icu_testdata::paths::cldr_json_root(), "full".to_string())
            .with_uprops(icu_testdata::paths::uprops_toml_root())
    }

    /// Paths to CLDR source data.
    pub(crate) fn get_cldr_paths(&self) -> Result<&CldrPaths, DataError> {
        Ok(self
            .cldr_paths
            .as_ref()
            .ok_or(DatagenError::MissingCldrPaths)?)
    }

    /// Path to Unicode Properties source data.
    pub(crate) fn get_uprops_paths(&self) -> Result<&UpropsPaths, DataError> {
        Ok(self
            .uprops_paths
            .as_ref()
            .ok_or(DatagenError::MissingUpropsPath)?)
    }

    /// Path to segmenter data.
    #[cfg(feature = "experimental")]
    pub(crate) fn get_segmenter_data_root(&self) -> Result<PathBuf, DataError> {
        Ok(PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data"))
    }

    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub(crate) fn trie_type(&self) -> TrieType {
        self.trie_type
    }
}
