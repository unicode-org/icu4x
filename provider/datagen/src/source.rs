// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::source::CldrCache;
pub use crate::transform::cldr::source::LocaleSubset as CldrLocaleSubset;
use elsa::sync::FrozenMap;
use icu_provider::DataError;
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io::Cursor;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use zip::ZipArchive;

/// Bag of options for datagen source data.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SourceData {
    cldr_paths: Option<Arc<CldrCache>>,
    icuexport_paths: Option<Arc<SerdeCache>>,
    segmenter_paths: Arc<SerdeCache>,
    segmenter_lstm_paths: Arc<SerdeCache>,
    trie_type: IcuTrieType,
    collation_han_database: CollationHanDatabase,
    collations: Vec<String>,
}

impl Default for SourceData {
    fn default() -> Self {
        let segmenter_path =
            PathBuf::from(concat!(std::env!("CARGO_MANIFEST_DIR"), "/data/segmenter"));
        Self {
            cldr_paths: None,
            icuexport_paths: None,
            segmenter_paths: Arc::new(SerdeCache::new(
                AbstractFs::new(&segmenter_path).expect("valid dir"),
            )),
            segmenter_lstm_paths: Arc::new(SerdeCache::new(
                AbstractFs::new(segmenter_path.join("lstm")).expect("valid dir"),
            )),
            trie_type: IcuTrieType::Small,
            collation_han_database: CollationHanDatabase::Implicit,
            collations: vec![],
        }
    }
}

impl SourceData {
    /// The latest CLDR JSON tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "42.0.0";

    /// The latest ICU export tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str = "release-72-1";

    #[cfg(test)]
    pub fn for_test() -> Self {
        Self::default()
            .with_cldr(
                concat!(core::env!("CARGO_MANIFEST_DIR"), "/tests/data/cldr").into(),
                CldrLocaleSubset::Full,
            )
            .unwrap()
            .with_icuexport(
                concat!(core::env!("CARGO_MANIFEST_DIR"), "/tests/data/icuexport").into(),
            )
            .unwrap()
    }

    /// Adds CLDR data to this `DataSource`. The root should point to a local
    /// `cldr-{version}-json-{full, modern}.zip` directory or ZIP file (see
    /// [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    ///
    /// The `_locale_subset` variable is ignored.
    pub fn with_cldr(
        self,
        root: PathBuf,
        _locale_subset: CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        let root = AbstractFs::new(root)?;
        let locale_subset = if root.list("cldr-misc-full").is_ok() {
            CldrLocaleSubset::Full
        } else {
            CldrLocaleSubset::Modern
        };
        Ok(Self {
            cldr_paths: Some(Arc::new(CldrCache {
                cache: SerdeCache::new(root),
                locale_subset,
            })),
            ..self
        })
    }

    /// Adds ICU export data to this `DataSource`. The path should point to a local
    /// `icuexportdata_uprops_full.zip` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
            ..self
        })
    }

    /// Adds CLDR data to this `DataSource`. The data will be downloaded from GitHub
    /// using the given tag (see [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    ///
    /// Also see: [`LATEST_TESTED_CLDR_TAG`](Self::LATEST_TESTED_CLDR_TAG)
    pub fn with_cldr_for_tag(
        self,
        tag: &str,
        locale_subset: CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        Ok(Self {
            cldr_paths: Some(Arc::new(CldrCache {
                cache: SerdeCache::new(AbstractFs::new_from_url(format!(
                    "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-{}.zip",
                    tag, tag, locale_subset
                ))),
                locale_subset,
            })),
            ..self
        })
    }

    /// Adds ICU export data to this `DataSource`. The data will be downloaded from GitHub
    /// using the given tag. (see [GitHub releases](https://github.com/unicode-org/icu/releases)).
    ///
    /// Also see: [`LATEST_TESTED_ICUEXPORT_TAG`](Self::LATEST_TESTED_ICUEXPORT_TAG)
    pub fn with_icuexport_for_tag(self, mut tag: &str) -> Result<Self, DataError> {
        if tag == "release-71-1" {
            tag = "icu4x/2022-08-17/71.x";
        }
        Ok(Self {
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new_from_url(
                format!(
                    "https://github.com/unicode-org/icu/releases/download/{}/icuexportdata_{}.zip",
                    tag,
                    tag.replace('/', "-")
                ),
            )))),
            ..self
        })
    }

    #[deprecated(
        since = "1.1.0",
        note = "Use `with_cldr_for_tag(SourceData::LATEST_TESTED_CLDR_TAG)`"
    )]
    /// Deprecated
    pub fn with_cldr_latest(self, locale_subset: CldrLocaleSubset) -> Result<Self, DataError> {
        self.with_cldr_for_tag(Self::LATEST_TESTED_CLDR_TAG, locale_subset)
    }

    #[deprecated(
        since = "1.1.0",
        note = "Use `with_icuexport_for_tag(SourceData::LATEST_TESTED_ICUEXPORT_TAG)`"
    )]
    /// Deprecated
    pub fn with_icuexport_latest(self) -> Result<Self, DataError> {
        self.with_icuexport_for_tag(Self::LATEST_TESTED_ICUEXPORT_TAG)
    }

    /// Set this to use tries optimized for speed instead of data size
    pub fn with_fast_tries(self) -> Self {
        Self {
            trie_type: IcuTrieType::Fast,
            ..self
        }
    }

    /// Set the [`CollationHanDatabase`] version.
    pub fn with_collation_han_database(self, collation_han_database: CollationHanDatabase) -> Self {
        Self {
            collation_han_database,
            ..self
        }
    }

    /// Set the list of BCP-47 collation IDs to include beyond the default set.
    ///
    /// If a list was already set, this function overwrites the previous list.
    ///
    /// The special string `"search*"` causes all search collation tables to be included.
    pub fn with_collations(self, collations: Vec<String>) -> Self {
        Self { collations, ..self }
    }

    /// Paths to CLDR source data.
    pub(crate) fn cldr(&self) -> Result<&CldrCache, DataError> {
        self.cldr_paths
            .as_deref()
            .ok_or(crate::error::MISSING_CLDR_ERROR)
    }

    /// Path to Unicode Properties source data.
    pub(crate) fn icuexport(&self) -> Result<&SerdeCache, DataError> {
        self.icuexport_paths
            .as_deref()
            .ok_or(crate::error::MISSING_ICUEXPORT_ERROR)
    }

    /// Path to segmenter data.
    pub(crate) fn segmenter(&self) -> Result<&SerdeCache, DataError> {
        Ok(&self.segmenter_paths)
    }

    pub(crate) fn segmenter_lstm(&self) -> Result<&SerdeCache, DataError> {
        Ok(&self.segmenter_lstm_paths)
    }

    pub(crate) fn trie_type(&self) -> IcuTrieType {
        self.trie_type
    }

    pub(crate) fn collation_han_database(&self) -> CollationHanDatabase {
        self.collation_han_database
    }

    pub(crate) fn collations(&self) -> &[String] {
        &self.collations
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum IcuTrieType {
    Fast,
    Small,
}

impl IcuTrieType {
    pub(crate) fn to_internal(self) -> icu_collections::codepointtrie::TrieType {
        match self {
            IcuTrieType::Fast => icu_collections::codepointtrie::TrieType::Fast,
            IcuTrieType::Small => icu_collections::codepointtrie::TrieType::Small,
        }
    }
}

impl std::fmt::Display for IcuTrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            IcuTrieType::Fast => write!(f, "fast"),
            IcuTrieType::Small => write!(f, "small"),
        }
    }
}

/// Specifies the collation Han database to use.
///
/// Unihan is more precise but significantly increases data size. See
/// <https://github.com/unicode-org/icu/blob/main/docs/userguide/icu_data/buildtool.md#collation-ucadata>
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum CollationHanDatabase {
    /// Implicit
    Implicit,
    /// Unihan
    Unihan,
}

impl std::fmt::Display for CollationHanDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CollationHanDatabase::Implicit => write!(f, "implicithan"),
            CollationHanDatabase::Unihan => write!(f, "unihan"),
        }
    }
}

pub(crate) struct SerdeCache {
    root: AbstractFs,
    cache: FrozenMap<String, Box<dyn Any + Send + Sync>>,
}

impl Debug for SerdeCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SerdeCache")
            .field("root", &self.root)
            // skip formatting the cache
            .finish()
    }
}

impl SerdeCache {
    pub fn new(root: AbstractFs) -> Self {
        Self {
            root,
            cache: FrozenMap::new(),
        }
    }

    fn read_and_parse<S>(
        &self,
        path: &str,
        parser: fn(&[u8]) -> Result<S, DataError>,
    ) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        match self.cache.get(path) {
            Some(x) => x,
            None => self.cache.insert(
                path.to_string(),
                Box::new(
                    parser(&self.root.read_to_buf(path)?)
                        .map_err(|e| e.with_path_context(&path))?,
                ),
            ),
        }
        .downcast_ref::<S>()
        .ok_or_else(|| DataError::custom("Cache error").with_type_context::<S>())
    }

    pub fn read_and_parse_json<S>(&self, path: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.read_and_parse(path, |bytes| {
            serde_json::from_slice(bytes).map_err(DataError::from)
        })
    }

    pub fn read_and_parse_toml<S>(&self, path: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.read_and_parse(path, |bytes| {
            toml::from_slice(bytes).map_err(crate::error::data_error_from_toml)
        })
    }

    pub fn list(&self, path: &str) -> Result<impl Iterator<Item = PathBuf>, DataError> {
        self.root.list(path)
    }
}

pub(crate) enum AbstractFs {
    Fs(PathBuf),
    Zip(RwLock<Result<ZipArchive<Cursor<Vec<u8>>>, String>>),
}

impl Debug for AbstractFs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbstractFs").finish()
    }
}

impl AbstractFs {
    fn new<P: AsRef<Path>>(root: P) -> Result<Self, DataError> {
        if std::fs::metadata(root.as_ref())
            .map_err(|e| DataError::from(e).with_path_context(root.as_ref()))?
            .is_dir()
        {
            Ok(Self::Fs(root.as_ref().to_path_buf()))
        } else {
            Ok(Self::Zip(RwLock::new(Ok(ZipArchive::new(Cursor::new(
                std::fs::read(&root)?,
            ))
            .map_err(|e| {
                DataError::custom("Zip").with_display_context(&e)
            })?))))
        }
    }

    fn new_from_url(path: String) -> Self {
        Self::Zip(RwLock::new(Err(path)))
    }

    fn init(&self) -> Result<(), DataError> {
        match self {
            Self::Zip(lock) => {
                if lock.read().expect("poison").is_ok() {
                    return Ok(());
                }
                let mut lock = lock.write().expect("poison");
                let resource = if let Err(resource) = lock.deref() {
                    resource
                } else {
                    return Ok(());
                };
                lazy_static::lazy_static! {
                    static ref CACHE: cached_path::Cache = cached_path::CacheBuilder::new()
                        .freshness_lifetime(u64::MAX)
                        .progress_bar(None)
                        .build()
                        .unwrap();
                }
                let root = CACHE
                    .cached_path(resource)
                    .map_err(|e| DataError::custom("Download").with_display_context(&e))?;
                *lock = Ok(ZipArchive::new(Cursor::new(std::fs::read(&root)?))
                    .map_err(|e| DataError::custom("Zip").with_display_context(&e))?);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn read_to_buf(&self, path: &str) -> Result<Vec<u8>, DataError> {
        self.init()?;
        match self {
            Self::Fs(root) => {
                log::trace!("Reading: {}/{}", root.display(), path);
                std::fs::read(&root.join(path))
                    .map_err(|e| DataError::from(e).with_path_context(&root.join(path)))
            }
            Self::Zip(zip) => {
                log::trace!("Reading: <zip>/{}", path);
                let mut buf = Vec::new();
                zip.write()
                    .expect("poison")
                    .as_mut()
                    .ok()
                    .unwrap() // init called
                    .by_name(path)
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

    fn list(&self, path: &str) -> Result<impl Iterator<Item = PathBuf>, DataError> {
        self.init()?;
        Ok(match self {
            Self::Fs(root) => std::fs::read_dir(&root.join(path))
                .map_err(|e| DataError::from(e).with_display_context(path))?
                .map(|e| -> Result<_, DataError> { Ok(PathBuf::from(e?.file_name())) })
                .collect::<Result<HashSet<_>, DataError>>()
                .map(HashSet::into_iter)?,
            Self::Zip(zip) => zip
                .read()
                .expect("poison")
                .as_ref()
                .ok()
                .unwrap() // init called
                .file_names()
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| suffix.split('/').find(|s| !s.is_empty()))
                .map(PathBuf::from)
                .collect::<HashSet<_>>()
                .into_iter(),
        })
    }
}
