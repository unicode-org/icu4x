// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::options::Options;
use crate::transform::cldr::source::CldrCache;
pub use crate::transform::cldr::source::CoverageLevel;
use elsa::sync::FrozenMap;
use icu_provider::DataError;
use std::any::Any;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::io::Cursor;
use std::io::Read;
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
    // TODO: move this out when we decide we can break the exhaustiveness of DatagenProvider
    pub(crate) options: Options,
}

#[cfg(feature = "networking")]
/// The default [`SourceData`] downloads the latest supported data.
///
/// Requires `networking` Cargo feature.
impl Default for SourceData {
    fn default() -> Self {
        Self::offline()
            .with_cldr_for_tag(Self::LATEST_TESTED_CLDR_TAG, Default::default())
            .unwrap()
            .with_icuexport_for_tag(Self::LATEST_TESTED_ICUEXPORT_TAG)
            .unwrap()
    }
}

impl SourceData {
    /// The latest CLDR JSON tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "43.0.0";

    /// The latest ICU export tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str = "release-73-1";

    #[doc(hidden)]
    #[cfg(feature = "networking")]
    #[deprecated(since = "1.3.0", note = "use SourceData::default()")]
    pub fn latest_tested() -> Self {
        Self::default()
    }

    /// Creates a `SourceData` that does not have CLDR or ICU export sources set.
    ///
    /// Using this to generate keys that require the data will result in errors
    /// ([`is_missing_cldr_error`](crate::is_missing_cldr_error) /
    /// [`is_missing_icuexport_error`](crate::is_missing_icuexport_error)). Make sure to set
    /// local data sources using [`SourceData::with_cldr`] / [`SourceData::with_icuexport`].
    pub fn offline() -> Self {
        Self {
            cldr_paths: None,
            icuexport_paths: None,
            segmenter_paths: Arc::new(SerdeCache::new(AbstractFs::new_segmenter())),
            segmenter_lstm_paths: Arc::new(SerdeCache::new(AbstractFs::new_lstm())),
            options: Default::default(),
        }
    }

    /// Adds CLDR data to this `SourceData`. The root should point to a local
    /// `cldr-{version}-json-{full, modern}.zip` directory or ZIP file (see
    /// [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    pub fn with_cldr(
        self,
        root: PathBuf,
        _use_default_here: crate::CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        let root = AbstractFs::new(root)?;
        Ok(Self {
            cldr_paths: Some(Arc::new(CldrCache(SerdeCache::new(root)))),
            ..self
        })
    }

    /// Adds ICU export data to this `SourceData`. The path should point to a local
    /// `icuexportdata_uprops_full.zip` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
            ..self
        })
    }

    /// Adds CLDR data to this `SourceData`. The data will be downloaded from GitHub
    /// using the given tag (see [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    ///
    /// Also see: [`LATEST_TESTED_CLDR_TAG`](Self::LATEST_TESTED_CLDR_TAG)
    ///
    /// Requires `networking` Cargo feature.
    #[cfg(feature = "networking")]
    pub fn with_cldr_for_tag(
        self,
        tag: &str,
        _use_default_here: crate::CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        Ok(Self {
            cldr_paths: Some(Arc::new(CldrCache(SerdeCache::new(AbstractFs::new_from_url(format!(
                    "https://github.com/unicode-org/cldr-json/releases/download/{tag}/cldr-{tag}-json-full.zip",
                )))
            ))),
            ..self
        })
    }

    /// Adds ICU export data to this `SourceData`. The data will be downloaded from GitHub
    /// using the given tag. (see [GitHub releases](https://github.com/unicode-org/icu/releases)).
    ///
    /// Also see: [`LATEST_TESTED_ICUEXPORT_TAG`](Self::LATEST_TESTED_ICUEXPORT_TAG)
    ///
    /// Requires `networking` Cargo feature.
    #[cfg(feature = "networking")]
    pub fn with_icuexport_for_tag(self, mut tag: &str) -> Result<Self, DataError> {
        if tag == "release-71-1" {
            tag = "icu4x/2022-08-17/71.x";
        }
        Ok(Self {
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new_from_url(
                format!(
                    "https://github.com/unicode-org/icu/releases/download/{tag}/icuexportdata_{}.zip",
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
    #[cfg(feature = "networking")]
    #[doc(hidden)]
    pub fn with_cldr_latest(
        self,
        _use_default_here: crate::CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        self.with_cldr_for_tag(Self::LATEST_TESTED_CLDR_TAG, Default::default())
    }

    #[deprecated(
        since = "1.1.0",
        note = "Use `with_icuexport_for_tag(SourceData::LATEST_TESTED_ICUEXPORT_TAG)`"
    )]
    #[cfg(feature = "networking")]
    #[doc(hidden)]
    pub fn with_icuexport_latest(self) -> Result<Self, DataError> {
        self.with_icuexport_for_tag(Self::LATEST_TESTED_ICUEXPORT_TAG)
    }

    #[deprecated(note = "use crate::Options", since = "1.3.0")]
    #[doc(hidden)]
    pub fn with_fast_tries(self) -> Self {
        Self {
            options: Options {
                trie_type: crate::options::TrieType::Fast,
                ..self.options
            },
            ..self
        }
    }

    #[deprecated(note = "use crate::Options", since = "1.3.0")]
    #[doc(hidden)]
    pub fn with_collation_han_database(self, collation_han_database: CollationHanDatabase) -> Self {
        Self {
            options: Options {
                collation_han_database,
                ..self.options
            },
            ..self
        }
    }

    #[deprecated(note = "use crate::Options", since = "1.3.0")]
    #[doc(hidden)]
    pub fn with_collations(self, collations: Vec<String>) -> Self {
        Self {
            options: Options {
                collations: collations.into_iter().collect(),
                ..self.options
            },
            ..self
        }
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

    /// List the locales for the given CLDR coverage levels
    pub fn locales(
        &self,
        levels: &[CoverageLevel],
    ) -> Result<Vec<icu_locid::LanguageIdentifier>, DataError> {
        self.cldr()?.locales(levels)
    }
}

#[doc(hidden)]
pub use crate::options::CollationHanDatabase;

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

    pub fn list(&self, path: &str) -> Result<impl Iterator<Item = String>, DataError> {
        self.root.list(path)
    }

    pub fn file_exists(&self, path: &str) -> Result<bool, DataError> {
        self.root.file_exists(path)
    }
}

pub(crate) enum AbstractFs {
    Fs(PathBuf),
    Zip(RwLock<Result<ZipArchive<Cursor<Vec<u8>>>, String>>),
    Memory(BTreeMap<&'static str, &'static [u8]>),
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

    fn new_segmenter() -> Self {
        const SEGMENTER: &[(&str, &[u8])] = &[
            (
                "grapheme.toml",
                include_bytes!("../data/segmenter/grapheme.toml"),
            ),
            ("word.toml", include_bytes!("../data/segmenter/word.toml")),
            ("line.toml", include_bytes!("../data/segmenter/line.toml")),
            (
                "sentence.toml",
                include_bytes!("../data/segmenter/sentence.toml"),
            ),
            (
                "dictionary_cj.toml",
                include_bytes!("../data/segmenter/dictionary_cj.toml"),
            ),
            (
                "dictionary_km.toml",
                include_bytes!("../data/segmenter/dictionary_km.toml"),
            ),
            (
                "dictionary_lo.toml",
                include_bytes!("../data/segmenter/dictionary_lo.toml"),
            ),
            (
                "dictionary_my.toml",
                include_bytes!("../data/segmenter/dictionary_my.toml"),
            ),
            (
                "dictionary_th.toml",
                include_bytes!("../data/segmenter/dictionary_th.toml"),
            ),
        ];

        Self::Memory(SEGMENTER.iter().copied().collect())
    }

    fn new_lstm() -> Self {
        const LSTM: &[(&str, &[u8])] = &[
            (
                "lstm_km.json",
                include_bytes!("../data/segmenter/lstm/lstm_km.json"),
            ),
            (
                "lstm_lo.json",
                include_bytes!("../data/segmenter/lstm/lstm_lo.json"),
            ),
            (
                "lstm_my.json",
                include_bytes!("../data/segmenter/lstm/lstm_my.json"),
            ),
            (
                "lstm_th.json",
                include_bytes!("../data/segmenter/lstm/lstm_th.json"),
            ),
        ];

        Self::Memory(LSTM.iter().copied().collect())
    }
    #[cfg(feature = "networking")]
    fn new_from_url(path: String) -> Self {
        Self::Zip(RwLock::new(Err(path)))
    }

    fn init(&self) -> Result<(), DataError> {
        #[cfg(feature = "networking")]
        if let Self::Zip(lock) = self {
            if lock.read().expect("poison").is_ok() {
                return Ok(());
            }
            let mut lock = lock.write().expect("poison");
            let resource = if let Err(resource) = &*lock {
                resource
            } else {
                return Ok(());
            };

            let root = {
                lazy_static::lazy_static! {
                    static ref CACHE: cached_path::Cache = cached_path::CacheBuilder::new()
                        .freshness_lifetime(u64::MAX)
                        .progress_bar(None)
                        .build()
                        .unwrap();
                }

                CACHE
                    .cached_path(resource)
                    .map_err(|e| DataError::custom("Download").with_display_context(&e))?
            };
            *lock = Ok(ZipArchive::new(Cursor::new(std::fs::read(root)?))
                .map_err(|e| DataError::custom("Zip").with_display_context(&e))?);
        }
        Ok(())
    }

    fn read_to_buf(&self, path: &str) -> Result<Vec<u8>, DataError> {
        self.init()?;
        match self {
            Self::Fs(root) => {
                log::debug!("Reading: {}/{}", root.display(), path);
                std::fs::read(root.join(path))
                    .map_err(|e| DataError::from(e).with_path_context(&root.join(path)))
            }
            Self::Zip(zip) => {
                log::debug!("Reading: <zip>/{}", path);
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
            Self::Memory(map) => map.get(path).copied().map(Vec::from).ok_or_else(|| {
                DataError::custom("Not found in icu4x-datagen's data/").with_display_context(path)
            }),
        }
    }

    fn list(&self, path: &str) -> Result<impl Iterator<Item = String>, DataError> {
        self.init()?;
        Ok(match self {
            Self::Fs(root) => std::fs::read_dir(root.join(path))
                .map_err(|e| DataError::from(e).with_display_context(path))?
                .map(|e| -> Result<_, DataError> { Ok(e?.file_name().into_string().unwrap()) })
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
                .map(String::from)
                .collect::<HashSet<_>>()
                .into_iter(),
            Self::Memory(map) => map
                .keys()
                .copied()
                .map(String::from)
                .collect::<HashSet<_>>()
                .into_iter(),
        })
    }

    fn file_exists(&self, path: &str) -> Result<bool, DataError> {
        self.init()?;
        Ok(match self {
            Self::Fs(root) => root.join(path).is_file(),
            Self::Zip(zip) => zip
                .read()
                .expect("poison")
                .as_ref()
                .ok()
                .unwrap() // init called
                .file_names()
                .any(|p| p == path),
            Self::Memory(map) => map.contains_key(path),
        })
    }
}
