// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use crate::transform::reader;
use icu_provider::prelude::*;
use std::any::Any;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Bag of options for datagen source data.
#[derive(Clone)]
#[non_exhaustive]
pub struct SourceData {
    cldr_paths: Option<CldrPaths>,
    uprops_root: Option<PathBuf>,
    cache: moka::sync::Cache<PathBuf, Arc<dyn Any + Send + Sync>>,
}

impl Debug for SourceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceData")
            .field("cldr_paths", &self.cldr_paths)
            .field("uprops_root", &self.uprops_root)
            // skip formatting the cache
            .finish()
    }
}

impl Default for SourceData {
    fn default() -> Self {
        let cache = moka::sync::Cache::builder().max_capacity(1000).build();
        SourceData {
            cldr_paths: None,
            uprops_root: None,
            cache,
        }
    }
}

impl SourceData {
    /// Adds CLDR data to this `DataSource`. The root should point to
    /// a local `cldr-{version}-json-full.zip` directory (see [GitHub downloads](
    /// https://github.com/unicode-org/cldr-json/releases)), and `locale_subset`
    /// is a valid locale subset (currently either "full" or "modern").
    pub fn with_cldr(mut self, root: PathBuf, locale_subset: String) -> Self {
        self.cldr_paths = Some(CldrPaths {
            root,
            locale_subset,
        });
        self
    }

    /// Adds Unicode Properties data to this `DataSource`. The path should
    /// point to a local `icuexportdata_uprops_full` directory (see
    /// [GitHub downloads](https://github.com/unicode-org/icu/releases)).
    pub fn with_uprops(mut self, uprops_root: PathBuf) -> Self {
        self.uprops_root = Some(uprops_root);
        self
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

    pub(crate) fn get_uprops_dir_contents(&self) -> Result<Vec<PathBuf>, DatagenError> {
        let root = self.get_uprops_root()?;
        reader::get_dir_contents(root)
    }

    pub(crate) fn load<T>(
        &self,
        path: &Path,
        init: impl FnOnce(&[u8]) -> Result<T, DatagenError>,
    ) -> Result<Arc<T>, DatagenError>
    where
        T: Any + Send + Sync,
    {
        self.cache
            .try_get_with::<_, DatagenError>(path.to_path_buf(), move || {
                let contents = reader::read_path_to_string(path)?;
                let t = init(contents.as_bytes())?;
                Ok(Arc::new(t))
            })
            .map_err(|arc_err| DatagenError::Custom(arc_err.to_string(), None))
            .map(Arc::downcast)
            .and_then(|result| {
                result.map_err(|_| {
                    DatagenError::MismatchedType(
                        std::any::type_name::<T>(),
                        Some(path.to_path_buf()),
                    )
                })
            })
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
