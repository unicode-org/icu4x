// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::source::CldrCache;
use elsa::sync::FrozenMap;
pub use icu_codepointtrie::TrieType;
use icu_provider::DataError;
use std::any::Any;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
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
                AbstractFs::new(
                    PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data"),
                    String::new(),
                )
                .expect("valid dir"),
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
    pub fn with_cldr(self, root: PathBuf, locale_subset: String) -> Result<Self, DataError> {
        Ok(Self {
            cldr_paths: Some(Arc::new(CldrCache::new(
                AbstractFs::new(root, String::new())?,
                locale_subset,
            ))),
            ..self
        })
    }

    /// Adds Unicode Properties data to this `DataSource`. The path should
    /// point to a local `icuexportdata_uprops_full` directory (see
    /// [GitHub downloads](https://github.com/unicode-org/icu/releases)).
    pub fn with_uprops(self, root: PathBuf, trie_type: TrieType) -> Result<Self, DataError> {
        Ok(Self {
            uprops_paths: Some(Arc::new(TomlCache::new(AbstractFs::new(
                root,
                format!(
                    "icuexportdata_uprops_full/{}/",
                    match trie_type {
                        TrieType::Fast => "fast",
                        TrieType::Small => "small",
                    }
                ),
            )?))),
            trie_type,
            ..self
        })
    }

    /// Adds collation data to this `DataSource`.
    pub fn with_coll(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            coll_paths: Some(Arc::new(TomlCache::new(AbstractFs::new(
                root,
                String::new(),
            )?))),
            ..self
        })
    }

    #[cfg(test)]
    /// Create `SourceData` pointing to test data.
    pub(crate) fn for_test() -> Self {
        Self::default()
            .with_cldr(icu_testdata::paths::cldr_json_root(), "full".to_string())
            .expect("testdata is valid")
            .with_uprops(icu_testdata::paths::uprops_toml_root(), TrieType::Small)
            .expect("testdata is valid")
            .with_coll(icu_testdata::paths::coll_toml_root())
            .expect("testdata is valid")
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
    root: AbstractFs,
    cache: Arc<FrozenMap<String, Box<dyn Any + Send + Sync>>>,
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
    pub fn new(root: AbstractFs) -> Self {
        Self {
            root,
            cache: Arc::new(FrozenMap::new()),
        }
    }

    pub fn read_and_parse_toml<S>(&self, path: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        match self.cache.get(path) {
            Some(x) => x,
            None => {
                let file = self.root.read_to_buf(path)?;
                let file: S = toml::from_slice(&file).map_err(|e| {
                    crate::error::data_error_from_toml(e).with_display_context(&path)
                })?;
                self.cache.insert(path.to_string(), Box::new(file))
            }
        }
        .downcast_ref::<S>()
        .ok_or_else(|| DataError::custom("Uprops TOML error").with_type_context::<S>())
    }

    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub fn list(&self) -> Result<impl Iterator<Item = PathBuf>, DataError> {
        self.root.list("").map(Vec::into_iter)
    }
}

#[derive(Debug)]
pub(crate) enum AbstractFs {
    Fs(PathBuf),
    // The PB points to a valid ZIP file, the String is a prefix inside the ZIP
    Zip(PathBuf, String),
}

impl AbstractFs {
    pub fn new<P: AsRef<Path>>(root: P, prefix: String) -> Result<Self, DataError> {
        if std::fs::metadata(root.as_ref())?.is_dir() {
            Ok(Self::Fs(root.as_ref().join(prefix)))
        } else {
            zip::ZipArchive::new(File::open(&root)?)
                .map_err(|e| DataError::custom("Zip").with_display_context(&e))?;
            Ok(Self::Zip(root.as_ref().into(), prefix))
        }
    }

    pub fn read_to_buf(&self, path: &str) -> Result<Vec<u8>, DataError> {
        match self {
            Self::Fs(root) => {
                log::trace!("Reading: {}/{}", root.display(), path);
                std::fs::read(&root.join(path))
                    .map_err(|e| DataError::from(e).with_path_context(&root.join(path)))
            }
            Self::Zip(root, prefix) => {
                log::trace!("Reading: {}/{}", root.join(prefix).display(), path);
                let mut buf = Vec::new();
                zip::ZipArchive::new(File::open(root)?)
                    .unwrap("validated in constructor")
                    .by_name(&format!("{}{}", prefix, path))
                    .map_err(|e| {
                        DataError::custom("Zip")
                            .with_display_context(&e)
                            .with_display_context(path)
                    })?
                    .read_to_end(&mut buf)?;
                Ok(buf)
            }
        }
    }

    pub fn list(&self, path: &str) -> Result<Vec<PathBuf>, DataError> {
        Ok(match self {
            Self::Fs(root) => std::fs::read_dir(&root.join(path))?
                .map(|e| -> Result<_, DataError> { Ok(PathBuf::from(e?.file_name())) })
                .collect::<Result<_, DataError>>()?,
            Self::Zip(root, prefix) => zip::ZipArchive::new(File::open(root)?)
                .expect("validated in constructor")
                .file_names()
                .filter_map(|p| p.strip_prefix(prefix))
                .filter_map(|p| p.strip_prefix('/'))
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| suffix.split('/').next())
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect(),
        })
    }
}
