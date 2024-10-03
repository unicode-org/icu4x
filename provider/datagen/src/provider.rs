// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(deprecated)]

use elsa::sync::FrozenMap;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use source::{AbstractFs, SerdeCache};
use std::collections::HashSet;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use transform::cldr::source::CldrCache;

#[path = "transform/mod.rs"]
mod transform;

mod source;

#[cfg(test)]
mod tests;

/// An [`ExportableProvider`](icu_provider::datagen::ExportableProvider) backed by raw CLDR and ICU data.
///
/// This provider covers all keys that are used by ICU4X. It is intended as the canonical
/// provider for [`DatagenDriver::export`](crate::DatagenDriver::export).
///
/// If a required data source has not been set, `DataProvider::load` will
/// fail with the appropriate error:
/// * [`is_missing_cldr_error`](Self::is_missing_cldr_error)
/// * [`is_missing_icuexport_error`](Self::is_missing_icuexport_error)
/// * [`is_missing_segmenter_lstm_error`](Self::is_missing_segmenter_lstm_error)
#[allow(clippy::exhaustive_structs)] // any information will be added to SourceData
#[derive(Debug, Clone)]
pub struct DatagenProvider {
    #[doc(hidden)] // semver
    pub source: SourceData,
}

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        icu_provider::make_exportable_provider!(
            DatagenProvider,
            [
                icu_provider::hello_world::HelloWorldV1Marker,
                $(
                    $marker,
                )+
                $(
                    #[cfg(feature = "experimental_components")]
                    $emarker,
                )+
            ]
        );
    }
}
crate::registry!(cb);

icu_provider::impl_data_provider_never_marker!(DatagenProvider);

impl DatagenProvider {
    /// The latest CLDR JSON tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "46.0.0-BETA2";

    /// The latest ICU export tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str = "icu4x/2024-05-16/75.x";

    /// The latest segmentation LSTM model tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_SEGMENTER_LSTM_TAG: &'static str = "v0.1.0";

    /// A provider using the latest data that has been verified to work with this version of `icu_datagen`.
    ///
    /// See [`LATEST_TESTED_CLDR_TAG`](Self::LATEST_TESTED_CLDR_TAG),
    /// [`LATEST_TESTED_ICUEXPORT_TAG`](Self::LATEST_TESTED_ICUEXPORT_TAG),
    /// [`LATEST_TESTED_SEGMENTER_LSTM_TAG`](Self::LATEST_TESTED_SEGMENTER_LSTM_TAG).
    ///
    /// ✨ *Enabled with the `networking` Cargo feature.*
    #[cfg(feature = "networking")]
    pub fn new_latest_tested() -> Self {
        // Singleton so that all instantiations share the same cache.
        static SINGLETON: once_cell::sync::OnceCell<DatagenProvider> =
            once_cell::sync::OnceCell::new();
        SINGLETON
            .get_or_init(|| {
                Self::new_custom()
                    .with_cldr_for_tag(Self::LATEST_TESTED_CLDR_TAG)
                    .with_icuexport_for_tag(Self::LATEST_TESTED_ICUEXPORT_TAG)
                    .with_segmenter_lstm_for_tag(Self::LATEST_TESTED_SEGMENTER_LSTM_TAG)
            })
            .clone()
    }

    /// A provider with no source data. Without adding more sources, most `load` methods
    /// will return errors.
    ///
    /// Use [`with_cldr`](Self::with_cldr), [`with_icuexport`](Self::with_icuexport),
    /// [`with_segmenter_lstm`](Self::with_segmenter_lstm) to set data sources.
    pub fn new_custom() -> Self {
        Self {
            source: SourceData {
                cldr_paths: None,
                icuexport_paths: None,
                segmenter_lstm_paths: None,
                trie_type: Default::default(),
                collation_han_database: Default::default(),
                #[cfg(feature = "legacy_api")]
                icuexport_dictionary_fallback: None,
                #[cfg(feature = "legacy_api")]
                collations: Default::default(),
                supported_locales_cache: Default::default(),
            },
        }
    }

    /// Adds CLDR source data to the provider. The root should point to a local
    /// `cldr-{tag}-json-full` directory or ZIP file (see
    /// [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    pub fn with_cldr(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            source: SourceData {
                cldr_paths: Some(Arc::new(CldrCache::from_serde_cache(SerdeCache::new(
                    AbstractFs::new(root)?,
                )))),
                ..self.source
            },
        })
    }

    /// Adds ICU export source data to the provider. The path should point to a local
    /// `icuexportdata_{tag}` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            source: SourceData {
                icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
                ..self.source
            },
        })
    }

    /// Adds segmenter LSTM source data to the provider. The path should point to a local
    /// `models` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/lstm_word_segmentation/releases)).
    pub fn with_segmenter_lstm(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            source: SourceData {
                segmenter_lstm_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
                ..self.source
            },
        })
    }

    /// Adds CLDR source data to the provider. The data will be downloaded from GitHub
    /// using the given tag (see [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    ///
    /// Also see: [`LATEST_TESTED_CLDR_TAG`](Self::LATEST_TESTED_CLDR_TAG)
    ///
    /// ✨ *Enabled with the `networking` Cargo feature.*
    #[cfg(feature = "networking")]
    pub fn with_cldr_for_tag(self, tag: &str) -> Self {
        Self {
            source: SourceData {
                cldr_paths: Some(Arc::new(CldrCache::from_serde_cache(SerdeCache::new(AbstractFs::new_from_url(format!(
                    "https://github.com/unicode-org/cldr-json/releases/download/{tag}/cldr-{tag}-json-full.zip",
                )))))),
                ..self.source
            }
        }
    }

    /// Adds ICU export source data to the provider. The data will be downloaded from GitHub
    /// using the given tag. (see [GitHub releases](https://github.com/unicode-org/icu/releases)).
    ///
    /// Also see: [`LATEST_TESTED_ICUEXPORT_TAG`](Self::LATEST_TESTED_ICUEXPORT_TAG)
    ///
    /// ✨ *Enabled with the `networking` Cargo feature.*
    #[cfg(feature = "networking")]
    pub fn with_icuexport_for_tag(self, mut tag: &str) -> Self {
        if tag == "release-71-1" {
            tag = "icu4x/2022-08-17/71.x";
        }
        Self {
            source: SourceData {
                icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new_from_url(format!(
                    "https://github.com/unicode-org/icu/releases/download/{tag}/icuexportdata_{}.zip",
                    tag.replace('/', "-")
                ))))),
                ..self.source
            }
        }
    }

    /// Adds segmenter LSTM source data to the provider. The data will be downloaded from GitHub
    /// using the given tag. (see [GitHub releases](https://github.com/unicode-org/lstm_word_segmentation/releases)).
    ///
    /// Also see: [`LATEST_TESTED_SEGMENTER_LSTM_TAG`](Self::LATEST_TESTED_SEGMENTER_LSTM_TAG)
    ///
    /// ✨ *Enabled with the `networking` Cargo feature.*
    #[cfg(feature = "networking")]
    pub fn with_segmenter_lstm_for_tag(self, tag: &str) -> Self {
        Self { source: SourceData {
            segmenter_lstm_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new_from_url(format!(
                "https://github.com/unicode-org/lstm_word_segmentation/releases/download/{tag}/models.zip"
            ))))),
            ..self.source }
        }
    }

    const MISSING_CLDR_ERROR: DataError = DataErrorKind::MissingSourceData.with_str_context("cldr");

    const MISSING_ICUEXPORT_ERROR: DataError =
        DataErrorKind::MissingSourceData.with_str_context("icuexport");

    const MISSING_SEGMENTER_LSTM_ERROR: DataError =
        DataErrorKind::MissingSourceData.with_str_context("segmenter");

    /// Identifies errors that are due to missing CLDR data.
    pub fn is_missing_cldr_error(mut e: DataError) -> bool {
        e.key = None;
        e == Self::MISSING_CLDR_ERROR
    }

    /// Identifies errors that are due to missing ICU export data.
    pub fn is_missing_icuexport_error(mut e: DataError) -> bool {
        e.key = None;
        e == Self::MISSING_ICUEXPORT_ERROR
    }

    /// Identifies errors that are due to missing segmenter LSTM data.
    pub fn is_missing_segmenter_lstm_error(mut e: DataError) -> bool {
        e.key = None;
        e == Self::MISSING_SEGMENTER_LSTM_ERROR
    }

    fn cldr(&self) -> Result<&CldrCache, DataError> {
        self.source
            .cldr_paths
            .as_deref()
            .ok_or(Self::MISSING_CLDR_ERROR)
    }

    fn icuexport(&self) -> Result<&SerdeCache, DataError> {
        self.source
            .icuexport_paths
            .as_deref()
            .ok_or(Self::MISSING_ICUEXPORT_ERROR)
    }

    fn segmenter_lstm(&self) -> Result<&SerdeCache, DataError> {
        self.source
            .segmenter_lstm_paths
            .as_deref()
            .ok_or(Self::MISSING_SEGMENTER_LSTM_ERROR)
    }

    /// Set this to use tries optimized for speed instead of data size
    pub fn with_fast_tries(self) -> Self {
        Self {
            source: SourceData {
                trie_type: TrieType::Fast,
                ..self.source
            },
        }
    }

    /// Set the [`CollationHanDatabase`] version.
    pub fn with_collation_han_database(self, collation_han_database: CollationHanDatabase) -> Self {
        Self {
            source: SourceData {
                collation_han_database,
                ..self.source
            },
        }
    }

    fn trie_type(&self) -> TrieType {
        self.source.trie_type
    }

    fn collation_han_database(&self) -> CollationHanDatabase {
        self.source.collation_han_database
    }

    /// List the locales for the given CLDR coverage levels
    pub fn locales_for_coverage_levels(
        &self,
        levels: impl IntoIterator<Item = CoverageLevel>,
    ) -> Result<impl IntoIterator<Item = icu_locid::LanguageIdentifier>, DataError> {
        self.cldr()?.locales(levels)
    }

    fn supported_locales_set<M>(&self) -> Result<&HashSet<DataLocale>, DataError>
    where
        M: KeyedDataMarker,
        Self: IterableDataProviderInternal<M>,
    {
        #[allow(deprecated)] // SourceData
        self.source
            .supported_locales_cache
            .insert_with(M::KEY, || Box::new(self.supported_locales_impl()))
            .as_ref()
            .map_err(|e| *e)
    }
}

/// Specifies the collation Han database to use.
///
/// Unihan is more precise but significantly increases data size. See
/// <https://github.com/unicode-org/icu/blob/main/docs/userguide/icu_data/buildtool.md#collation-ucadata>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CollationHanDatabase {
    /// Implicit
    #[serde(rename = "implicit")]
    #[default]
    Implicit,
    /// Unihan
    #[serde(rename = "unihan")]
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

/// A language's CLDR coverage level.
///
/// In ICU4X, these are disjoint sets: a language belongs to a single coverage level. This
/// contrasts with CLDR usage, where these levels are understood to be additive (i.e., "basic"
/// includes all language with "basic", or better coverage). The ICU4X semantics allow
/// generating different data files for different coverage levels without duplicating data.
/// However, the data itself is still additive (e.g. for fallback to work correctly), so data
/// for moderate (basic) languages should only be loaded if modern (modern and moderate) data
/// is already present.
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum CoverageLevel {
    /// Locales listed as modern coverage targets by the CLDR subcomittee.
    ///
    /// This is the highest level of coverage.
    Modern,
    /// Locales listed as moderate, but not modern, coverage targets by the CLDR subcomittee.
    ///
    /// This is a medium level of coverage.
    Moderate,
    /// Locales listed as basic, but not moderate or modern, coverage targets by the CLDR subcomittee.
    ///
    /// This is the lowest level of coverage.
    Basic,
}

/// Specifies the trie type to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[doc(hidden)]
#[non_exhaustive]
pub enum TrieType {
    /// Fast tries are optimized for speed
    #[serde(rename = "fast")]
    Fast,
    /// Small tries are optimized for size
    #[serde(rename = "small")]
    #[default]
    Small,
}

impl std::fmt::Display for TrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TrieType::Fast => write!(f, "fast"),
            TrieType::Small => write!(f, "small"),
        }
    }
}

trait IterableDataProviderInternal<M: KeyedDataMarker>: DataProvider<M> {
    fn supported_locales_impl(&self) -> Result<HashSet<DataLocale>, DataError>;
}

impl<M: KeyedDataMarker> IterableDataProvider<M> for DatagenProvider
where
    DatagenProvider: IterableDataProviderInternal<M>,
{
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        self.supported_locales_set()
            .map(|v| v.iter().cloned().collect())
    }

    fn supports_locale(&self, locale: &DataLocale) -> Result<bool, DataError> {
        self.supported_locales_set().map(|v| v.contains(locale))
    }
}

// SEMVER GRAVEYARD

/// Bag of options for [`datagen`](crate::datagen).
///
/// Warning: this includes hardcoded segmentation data for backwards compatibility.
/// It is strongly discouraged to keep using this API, instead use [`DatagenProvider`]
/// and set segmentation data explicitly.
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
#[derive(Clone, Debug)]
#[non_exhaustive]
#[deprecated(since = "1.3.0", note = "use `DatagenProvider`")]
pub struct SourceData {
    cldr_paths: Option<Arc<CldrCache>>,
    icuexport_paths: Option<Arc<SerdeCache>>,
    segmenter_lstm_paths: Option<Arc<SerdeCache>>,
    trie_type: TrieType,
    collation_han_database: CollationHanDatabase,
    #[cfg(feature = "legacy_api")]
    // populated if constructed through `SourceData` constructor only
    icuexport_dictionary_fallback: Option<Arc<SerdeCache>>,
    #[cfg(feature = "legacy_api")]
    pub(crate) collations: Vec<String>,
    #[allow(clippy::type_complexity)] // not as complex as it appears
    supported_locales_cache: Arc<FrozenMap<DataKey, Box<Result<HashSet<DataLocale>, DataError>>>>,
}

#[cfg(feature = "legacy_api")]
impl Default for SourceData {
    fn default() -> Self {
        Self {
            icuexport_dictionary_fallback: Some(Arc::new(SerdeCache::new(AbstractFs::Memory(
                [
                    (
                        "segmenter/dictionary/cjdict.toml",
                        include_bytes!("../tests/data/icuexport/segmenter/dictionary/cjdict.toml").as_slice(),
                    ),
                    (
                        "segmenter/dictionary/khmerdict.toml",
                        include_bytes!("../tests/data/icuexport/segmenter/dictionary/khmerdict.toml").as_slice(),
                    ),
                    (
                        "segmenter/dictionary/laodict.toml",
                        include_bytes!("../tests/data/icuexport/segmenter/dictionary/laodict.toml").as_slice(),
                    ),
                    (
                        "segmenter/dictionary/burmesedict.toml",
                        include_bytes!("../tests/data/icuexport/segmenter/dictionary/burmesedict.toml").as_slice(),
                    ),
                    (
                        "segmenter/dictionary/thaidict.toml",
                        include_bytes!("../tests/data/icuexport/segmenter/dictionary/thaidict.toml").as_slice(),
                    ),
                ]
                .into_iter()
                .collect(),
            )))),
            segmenter_lstm_paths: Some(Arc::new(SerdeCache::new(AbstractFs::Memory(
                [
                    (
                        "Khmer_codepoints_exclusive_model4_heavy/weights.json",
                        include_bytes!(
                            "../tests/data/lstm/Khmer_codepoints_exclusive_model4_heavy/weights.json"
                        )
                        .as_slice(),
                    ),
                    (
                        "Lao_codepoints_exclusive_model4_heavy/weights.json",
                        include_bytes!(
                            "../tests/data/lstm/Lao_codepoints_exclusive_model4_heavy/weights.json"
                        )
                        .as_slice(),
                    ),
                    (
                        "Burmese_codepoints_exclusive_model4_heavy/weights.json",
                        include_bytes!(
                            "../tests/data/lstm/Burmese_codepoints_exclusive_model4_heavy/weights.json"
                        )
                        .as_slice(),
                    ),
                    (
                        "Thai_codepoints_exclusive_model4_heavy/weights.json",
                        include_bytes!(
                            "../tests/data/lstm/Thai_codepoints_exclusive_model4_heavy/weights.json"
                        )
                        .as_slice(),
                    ),
                    (
                        "Thai_graphclust_model4_heavy/weights.json",
                        include_bytes!("../tests/data/lstm/Thai_graphclust_model4_heavy/weights.json")
                            .as_slice(),
                    ),
                ]
                .into_iter()
                .collect(),
            )))),
            ..DatagenProvider::new_custom().source
        }
    }
}

#[cfg(feature = "legacy_api")]
impl SourceData {
    /// See [`DatagenProvider::LATEST_TESTED_CLDR_TAG`]
    pub const LATEST_TESTED_CLDR_TAG: &'static str = DatagenProvider::LATEST_TESTED_CLDR_TAG;

    /// See [`DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG`]
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str =
        DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG;

    #[cfg(feature = "networking")]
    /// See [`DatagenProvider::new_latest_tested`]
    pub fn latest_tested() -> Self {
        DatagenProvider::new_latest_tested().source
    }

    /// See [`DatagenProvider::with_cldr`]
    pub fn with_cldr(
        self,
        root: PathBuf,
        _use_default_here: CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        Ok(DatagenProvider { source: self }.with_cldr(root)?.source)
    }

    /// See [`DatagenProvider::with_icuexport`]
    pub fn with_icuexport(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(DatagenProvider { source: self }
            .with_icuexport(root)?
            .source)
    }

    #[cfg(feature = "networking")]
    /// See [`DatagenProvider::with_cldr_for_tag`]
    pub fn with_cldr_for_tag(
        self,
        tag: &str,
        _use_default_here: CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        Ok(DatagenProvider { source: self }
            .with_cldr_for_tag(tag)
            .source)
    }

    #[cfg(feature = "networking")]
    /// See [`DatagenProvider::with_icuexport_for_tag`]
    pub fn with_icuexport_for_tag(self, tag: &str) -> Result<Self, DataError> {
        Ok(DatagenProvider { source: self }
            .with_icuexport_for_tag(tag)
            .source)
    }

    #[deprecated(
        since = "1.1.0",
        note = "Use `DatagenProvider::with_cldr_for_tag(DatagenProvider::LATEST_TESTED_CLDR_TAG)`"
    )]
    #[cfg(feature = "networking")]
    /// See [`DatagenProvider::with_cldr_for_tag`]
    pub fn with_cldr_latest(self, _use_default_here: CldrLocaleSubset) -> Result<Self, DataError> {
        self.with_cldr_for_tag(Self::LATEST_TESTED_CLDR_TAG, Default::default())
    }

    #[deprecated(
        since = "1.1.0",
        note = "Use `DatagenProvider::with_icuexport_for_tag(DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG)`"
    )]
    #[cfg(feature = "networking")]
    /// See [`DatagenProvider::with_icuexport_for_tag`]
    pub fn with_icuexport_latest(self) -> Result<Self, DataError> {
        self.with_icuexport_for_tag(Self::LATEST_TESTED_ICUEXPORT_TAG)
    }

    /// See [`DatagenProvider::with_fast_tries`]
    pub fn with_fast_tries(self) -> Self {
        DatagenProvider { source: self }.with_fast_tries().source
    }

    /// See [`DatagenProvider::with_collation_han_database`]
    pub fn with_collation_han_database(self, collation_han_database: CollationHanDatabase) -> Self {
        DatagenProvider { source: self }
            .with_collation_han_database(collation_han_database)
            .source
    }

    #[cfg(feature = "legacy_api")]
    /// See [`DatagenDriver::with_additional_collations`](crate::DatagenDriver::with_additional_collations)
    pub fn with_collations(self, collations: Vec<String>) -> Self {
        Self { collations, ..self }
    }

    /// List the locales for the given CLDR coverage levels
    pub fn locales(
        &self,
        levels: &[CoverageLevel],
    ) -> Result<Vec<icu_locid::LanguageIdentifier>, DataError> {
        self.cldr_paths
            .as_deref()
            .ok_or(DatagenProvider::MISSING_CLDR_ERROR)?
            .locales(levels.iter().copied())
    }
}

#[allow(clippy::exhaustive_enums)] // exists for backwards compatibility
#[doc(hidden)]
#[derive(Debug)]
#[cfg(feature = "legacy_api")]
pub enum CldrLocaleSubset {
    Ignored,
}

#[cfg(feature = "legacy_api")]
impl Default for CldrLocaleSubset {
    fn default() -> Self {
        Self::Ignored
    }
}

#[cfg(feature = "legacy_api")]
impl CldrLocaleSubset {
    #[allow(non_upper_case_globals)]
    pub const Full: Self = Self::Ignored;
    #[allow(non_upper_case_globals)]
    pub const Modern: Self = Self::Ignored;
}

#[cfg(feature = "legacy_api")]
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
impl AnyProvider for DatagenProvider {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        self.as_any_provider().load_any(key, req)
    }
}
