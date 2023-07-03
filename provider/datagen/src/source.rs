// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::options::Options;
use crate::transform::cldr::source::CldrCache;
pub use crate::transform::cldr::source::CoverageLevel;
use elsa::sync::FrozenMap;
use icu_provider::prelude::*;
use std::any::Any;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
#[cfg(feature = "networking")]
use std::fs::File;
#[cfg(feature = "networking")]
use std::io::BufWriter;
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
    icuexport_fallback_paths: Arc<SerdeCache>,
    segmenter_lstm_paths: Arc<SerdeCache>,
    // TODO: move this out when we decide we can break the exhaustiveness of DatagenProvider
    pub(crate) options: Options,
    pub(crate) fallbacker: Option<icu_locid_transform::fallback::LocaleFallbacker>,
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
            .with_segmenter_lstm_for_tag(Self::LATEST_TESTED_SEGMENTER_LSTM_TAG)
            .unwrap()
    }
}

impl SourceData {
    /// The latest CLDR JSON tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "43.0.0";

    /// The latest ICU export tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str = "icu4x/2023-05-02/73.x";

    /// The latest segmentation LSTM model tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_SEGMENTER_LSTM_TAG: &'static str = "v0.1.0";

    #[doc(hidden)]
    #[cfg(feature = "networking")]
    #[deprecated(since = "1.3.0", note = "use SourceData::default()")]
    pub fn latest_tested() -> Self {
        Self::default()
    }

    /// Creates a `SourceData` that does not have CLDR or ICU export sources set.
    pub fn offline() -> Self {
        Self {
            cldr_paths: None,
            icuexport_paths: None,
            icuexport_fallback_paths: Arc::new(SerdeCache::new(
                AbstractFs::new_icuexport_fallback(),
            )),
            segmenter_lstm_paths: Arc::new(SerdeCache::new(AbstractFs::new_lstm_fallback())),
            options: Default::default(),
            fallbacker: None,
        }
    }

    /// Adds CLDR data to this `SourceData`. The root should point to a local
    /// `cldr-{tag}-json-full.zip` directory or ZIP file (see
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
    /// `icuexportdata_{tag}.zip` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
            ..self
        })
    }

    /// Adds segmenter LSTM data to this `SourceData`. The path should point to a local
    /// `models.zip` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/lstm_word_segmentation/releases)).
    pub fn with_segmenter_lstm(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            segmenter_lstm_paths: Arc::new(SerdeCache::new(AbstractFs::new(root)?)),
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

    /// Adds segmenter LSTM data to this `SourceData`. The data will be downloaded from GitHub
    /// using the given tag. (see [GitHub releases](https://github.com/unicode-org/lstm_word_segmentation/releases)).
    ///
    /// Also see: [`LATEST_TESTED_SEGMENTER_LSTM_TAG`](Self::LATEST_TESTED_SEGMENTER_LSTM_TAG)
    ///
    /// Requires `networking` Cargo feature.
    #[cfg(feature = "networking")]
    pub fn with_segmenter_lstm_for_tag(self, tag: &str) -> Result<Self, DataError> {
        Ok(Self {
            segmenter_lstm_paths: Arc::new(SerdeCache::new(AbstractFs::new_from_url(format!(
                "https://github.com/unicode-org/lstm_word_segmentation/releases/download/{tag}/models.zip"
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

    pub(crate) fn cldr(&self) -> Result<&CldrCache, DataError> {
        self.cldr_paths
            .as_deref()
            .ok_or(crate::error::MISSING_CLDR_ERROR)
    }

    pub(crate) fn icuexport(&self) -> Result<&SerdeCache, DataError> {
        self.icuexport_paths
            .as_deref()
            .ok_or(crate::error::MISSING_ICUEXPORT_ERROR)
    }

    pub(crate) fn icuexport_fallback(&self) -> &SerdeCache {
        &self.icuexport_fallback_paths
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
            serde_json::from_slice(bytes)
                .map_err(|e| DataError::custom("JSON deserialize").with_display_context(&e))
        })
    }

    pub fn read_and_parse_toml<S>(&self, path: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.read_and_parse(path, |bytes| {
            toml::from_slice(bytes)
                .map_err(|e| DataError::custom("TOML deserialize").with_display_context(&e))
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
                DataError::custom("Invalid ZIP file")
                    .with_display_context(&e)
                    .with_path_context(&root)
            })?))))
        }
    }

    fn new_icuexport_fallback() -> Self {
        Self::Memory(
            [
                (
                    "segmenter/dictionary/cjdict.toml",
                    include_bytes!("../data/segmenter/dictionary/cjdict.toml").as_slice(),
                ),
                (
                    "segmenter/dictionary/khmerdict.toml",
                    include_bytes!("../data/segmenter/dictionary/khmerdict.toml").as_slice(),
                ),
                (
                    "segmenter/dictionary/laodict.toml",
                    include_bytes!("../data/segmenter/dictionary/laodict.toml").as_slice(),
                ),
                (
                    "segmenter/dictionary/burmesedict.toml",
                    include_bytes!("../data/segmenter/dictionary/burmesedict.toml").as_slice(),
                ),
                (
                    "segmenter/dictionary/thaidict.toml",
                    include_bytes!("../data/segmenter/dictionary/thaidict.toml").as_slice(),
                ),
            ]
            .into_iter()
            .collect(),
        )
    }

    fn new_lstm_fallback() -> Self {
        Self::Memory(
            [
                (
                    "Khmer_codepoints_exclusive_model4_heavy/weights.json",
                    include_bytes!(
                        "../data/lstm/Khmer_codepoints_exclusive_model4_heavy/weights.json"
                    )
                    .as_slice(),
                ),
                (
                    "Lao_codepoints_exclusive_model4_heavy/weights.json",
                    include_bytes!(
                        "../data/lstm/Lao_codepoints_exclusive_model4_heavy/weights.json"
                    )
                    .as_slice(),
                ),
                (
                    "Burmese_codepoints_exclusive_model4_heavy/weights.json",
                    include_bytes!(
                        "../data/lstm/Burmese_codepoints_exclusive_model4_heavy/weights.json"
                    )
                    .as_slice(),
                ),
                (
                    "Thai_codepoints_exclusive_model4_heavy/weights.json",
                    include_bytes!(
                        "../data/lstm/Thai_codepoints_exclusive_model4_heavy/weights.json"
                    )
                    .as_slice(),
                ),
            ]
            .into_iter()
            .collect(),
        )
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

            let root = std::env::var_os("ICU4X_SOURCE_CACHE")
                .map(PathBuf::from)
                .unwrap_or_else(|| std::env::temp_dir().join("icu4x-source-cache/"))
                .join(resource.rsplit("//").next().unwrap());

            if !root.exists() {
                log::info!("Downloading {resource}");
                std::fs::create_dir_all(root.parent().unwrap())?;
                std::io::copy(
                    &mut ureq::get(resource)
                        .call()
                        .map_err(|e| DataError::custom("Download").with_display_context(&e))?
                        .into_reader(),
                    &mut BufWriter::new(File::create(&root)?),
                )?;
            }

            *lock = Ok(
                ZipArchive::new(Cursor::new(std::fs::read(&root)?)).map_err(|e| {
                    DataError::custom("Invalid ZIP file")
                        .with_display_context(&e)
                        .with_path_context(&root)
                })?,
            );
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
                        DataErrorKind::Io(std::io::ErrorKind::NotFound)
                            .into_error()
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
