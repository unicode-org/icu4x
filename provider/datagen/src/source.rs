// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::source::CldrCache;
use elsa::sync::FrozenMap;
pub use icu_codepointtrie::TrieType;
use icu_provider::DataError;
use std::any::Any;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

/// Bag of options for datagen source data.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SourceData {
    cldr_paths: Option<Arc<CldrCache>>,
    uprops_paths: Option<Arc<TomlCache>>,
    coll_paths: Option<Arc<TomlCache>>,
    segmenter_paths: Arc<TomlCache>,
    trie_type: TrieType,
}

impl Default for SourceData {
    fn default() -> Self {
        Self {
            cldr_paths: None,
            uprops_paths: None,
            coll_paths: None,
            segmenter_paths: Arc::new(TomlCache::new(
                PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data"),
            )),
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
            cldr_paths: Some(Arc::new(CldrCache::new(root, locale_subset))),
            ..self
        }
    }

    /// Adds Unicode Properties data to this `DataSource`. The path should
    /// point to a local `icuexportdata_uprops_full` directory (see
    /// [GitHub downloads](https://github.com/unicode-org/icu/releases)).
    pub fn with_uprops(self, root: PathBuf, trie_type: TrieType) -> Self {
        Self {
            uprops_paths: Some(Arc::new(TomlCache::new(
                root.join("icuexportdata_uprops_full")
                    .join(match trie_type {
                        TrieType::Fast => "fast",
                        TrieType::Small => "small",
                    }),
            ))),
            trie_type,
            ..self
        }
    }

    /// Adds collation data to this `DataSource`.
    pub fn with_coll(self, root: PathBuf) -> Self {
        Self {
            coll_paths: Some(Arc::new(TomlCache::new(root))),
            ..self
        }
    }

    #[cfg(test)]
    /// Create `SourceData` pointing to test data.
    pub(crate) fn for_test() -> Self {
        Self::default()
            .with_cldr(icu_testdata::paths::cldr_json_root(), "full".to_string())
            .with_uprops(icu_testdata::paths::uprops_toml_root(), TrieType::Small)
            .with_coll(icu_testdata::paths::coll_toml_root())
    }

    /// Paths to CLDR source data.
    pub(crate) fn get_cldr_paths(&self) -> Result<&CldrCache, DataError> {
        self.cldr_paths
            .as_deref()
            .ok_or(crate::error::MISSING_CLDR_ERROR)
    }

    /// Path to Unicode Properties source data.
    pub(crate) fn get_uprops_paths(&self) -> Result<&TomlCache, DataError> {
        self.uprops_paths
            .as_deref()
            .ok_or(crate::error::MISSING_UPROPS_ERROR)
    }

    /// Path to collation data.
    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub(crate) fn get_coll_paths(&self) -> Result<&TomlCache, DataError> {
        self.coll_paths
            .as_deref()
            .ok_or(crate::error::MISSING_COLLATION_ERROR)
    }

    /// Path to segmenter data.
    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub(crate) fn get_segmenter_paths(&self) -> Result<&TomlCache, DataError> {
        Ok(&self.segmenter_paths)
    }

    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub(crate) fn trie_type(&self) -> TrieType {
        self.trie_type
    }
}

pub(crate) struct TomlCache {
    root: PathBuf,
    cache: Arc<FrozenMap<PathBuf, Box<dyn Any + Send + Sync>>>,
}

impl Debug for TomlCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlCache")
            .field("root", &self.root)
            // skip formatting the cache
            .finish()
    }
}

impl TomlCache {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            cache: Arc::new(FrozenMap::new()),
        }
    }

    pub fn read_and_parse_toml<S, P: AsRef<Path>>(&self, path: P) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        let path = self.root.join(path);

        if self.cache.get(&path).is_none() {
            log::trace!("Reading: {:?}", &path);
            let file = std::fs::read_to_string(&path)
                .map_err(|e| DataError::from(e).with_path_context(&path))?;
            let file: S = toml::from_str(&file)
                .map_err(|e| crate::error::data_error_from_toml(e).with_path_context(&path))?;
            self.cache.insert(path.clone(), Box::new(file));
        }
        self.cache
            .get(&path)
            .unwrap()
            .downcast_ref::<S>()
            .ok_or_else(|| DataError::custom("Uprops TOML error").with_type_context::<S>())
    }

    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub fn list(&self) -> Result<impl Iterator<Item = PathBuf>, DataError> {
        let mut result = vec![];
        for entry in std::fs::read_dir(&self.root)
            .map_err(|e| DataError::from(e).with_path_context(&self.root))?
        {
            let entry = entry.map_err(|e| DataError::from(e).with_path_context(&self.root))?;
            let path = entry.path();
            result.push(path);
        }
        Ok(result.into_iter())
    }
}
