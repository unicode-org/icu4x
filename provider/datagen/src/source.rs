// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use icu_codepointtrie::TrieType;
use icu_provider::DataError;
use std::path::{Path, PathBuf};

/// Bag of options for datagen source data.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SourceData {
    cldr_paths: Option<CldrPaths>,
    uprops_root: Option<PathBuf>,
    trie_type: TrieType,
}

impl Default for SourceData {
    fn default() -> Self {
        Self {
            cldr_paths: None,
            uprops_root: None,
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
            cldr_paths: Some(CldrPaths {
                root,
                locale_subset,
            }),
            ..self
        }
    }

    /// Adds Unicode Properties data to this `DataSource`. The path should
    /// point to a local `icuexportdata_uprops_full` directory (see
    /// [GitHub downloads](https://github.com/unicode-org/icu/releases)).
    pub fn with_uprops(self, uprops_root: PathBuf) -> Self {
        Self {
            uprops_root: Some(uprops_root),
            ..self
        }
    }

    /// Sets the [`TrieType`] to be used when generating data.
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
    pub(crate) fn get_uprops_root(&self) -> Result<&Path, DataError> {
        Ok(self
            .uprops_root
            .as_deref()
            .ok_or(DatagenError::MissingUpropsPath)?)
    }

    /// Path to segmenter data.
    #[cfg(feature = "experimental")]
    pub(crate) fn get_segmenter_data_root(&self) -> Result<PathBuf, DataError> {
        Ok(PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data"))
    }

    pub(crate) fn trie_type(&self) -> TrieType {
        self.trie_type
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct CldrPaths {
    pub root: PathBuf,
    pub locale_subset: String,
}

impl CldrPaths {
    pub(crate) fn cldr_core(&self) -> PathBuf {
        self.root.join("cldr-core")
    }

    pub(crate) fn cldr_numbers(&self) -> PathBuf {
        self.root
            .join(format!("cldr-numbers-{}", self.locale_subset))
    }

    pub(crate) fn cldr_misc(&self) -> PathBuf {
        self.root.join(format!("cldr-misc-{}", self.locale_subset))
    }

    pub(crate) fn cldr_bcp47(&self) -> PathBuf {
        self.root.join("cldr-bcp47")
    }

    pub(crate) fn cldr_dates(&self, cal: &str) -> PathBuf {
        if cal == "gregorian" {
            self.root.join(format!("cldr-dates-{}", self.locale_subset))
        } else {
            self.root
                .join(format!("cldr-cal-{}-{}", cal, self.locale_subset))
        }
    }
}
