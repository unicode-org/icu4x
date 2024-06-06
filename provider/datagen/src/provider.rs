// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use elsa::sync::FrozenMap;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use source::{AbstractFs, SerdeCache};
use std::borrow::Cow;
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
/// This provider covers all markers that are used by ICU4X. It is intended as the canonical
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
    cldr_paths: Option<Arc<CldrCache>>,
    icuexport_paths: Option<Arc<SerdeCache>>,
    segmenter_lstm_paths: Option<Arc<SerdeCache>>,
    trie_type: TrieType,
    collation_han_database: CollationHanDatabase,
    #[allow(clippy::type_complexity)] // not as complex as it appears
    supported_requests_cache: Arc<
        FrozenMap<
            DataMarkerInfo,
            Box<
                Result<
                    HashSet<(Cow<'static, DataLocale>, Cow<'static, DataMarkerAttributes>)>,
                    DataError,
                >,
            >,
        >,
    >,
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
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "45.0.0";

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
        static SINGLETON: std::sync::OnceLock<DatagenProvider> = std::sync::OnceLock::new();
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
            cldr_paths: None,
            icuexport_paths: None,
            segmenter_lstm_paths: None,
            trie_type: Default::default(),
            collation_han_database: Default::default(),
            supported_requests_cache: Default::default(),
        }
    }

    /// Adds CLDR source data to the provider. The root should point to a local
    /// `cldr-{tag}-json-full` directory or ZIP file (see
    /// [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    pub fn with_cldr(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            cldr_paths: Some(Arc::new(CldrCache::from_serde_cache(SerdeCache::new(
                AbstractFs::new(root)?,
            )))),
            ..self
        })
    }

    /// Adds ICU export source data to the provider. The path should point to a local
    /// `icuexportdata_{tag}` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
            ..self
        })
    }

    /// Adds segmenter LSTM source data to the provider. The path should point to a local
    /// `models` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/lstm_word_segmentation/releases)).
    pub fn with_segmenter_lstm(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            segmenter_lstm_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
            ..self
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
                cldr_paths: Some(Arc::new(CldrCache::from_serde_cache(SerdeCache::new(AbstractFs::new_from_url(format!(
                    "https://github.com/unicode-org/cldr-json/releases/download/{tag}/cldr-{tag}-json-full.zip",
                )))))),
                ..self
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
                icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new_from_url(format!(
                    "https://github.com/unicode-org/icu/releases/download/{tag}/icuexportdata_{}.zip",
                    tag.replace('/', "-")
                ))))),
                ..self
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
        Self {
            segmenter_lstm_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new_from_url(format!(
                "https://github.com/unicode-org/lstm_word_segmentation/releases/download/{tag}/models.zip"
            ))))),
            ..self
        }
    }

    const MISSING_CLDR_ERROR: DataError = DataErrorKind::MissingSourceData.with_str_context("cldr");

    const MISSING_ICUEXPORT_ERROR: DataError =
        DataErrorKind::MissingSourceData.with_str_context("icuexport");

    const MISSING_SEGMENTER_LSTM_ERROR: DataError =
        DataErrorKind::MissingSourceData.with_str_context("segmenter");

    /// Identifies errors that are due to missing CLDR data.
    pub fn is_missing_cldr_error(mut e: DataError) -> bool {
        e.marker = None;
        e == Self::MISSING_CLDR_ERROR
    }

    /// Identifies errors that are due to missing ICU export data.
    pub fn is_missing_icuexport_error(mut e: DataError) -> bool {
        e.marker = None;
        e == Self::MISSING_ICUEXPORT_ERROR
    }

    /// Identifies errors that are due to missing segmenter LSTM data.
    pub fn is_missing_segmenter_lstm_error(mut e: DataError) -> bool {
        e.marker = None;
        e == Self::MISSING_SEGMENTER_LSTM_ERROR
    }

    fn cldr(&self) -> Result<&CldrCache, DataError> {
        self.cldr_paths.as_deref().ok_or(Self::MISSING_CLDR_ERROR)
    }

    fn icuexport(&self) -> Result<&SerdeCache, DataError> {
        self.icuexport_paths
            .as_deref()
            .ok_or(Self::MISSING_ICUEXPORT_ERROR)
    }

    fn segmenter_lstm(&self) -> Result<&SerdeCache, DataError> {
        self.segmenter_lstm_paths
            .as_deref()
            .ok_or(Self::MISSING_SEGMENTER_LSTM_ERROR)
    }

    /// Set this to use tries optimized for speed instead of data size
    pub fn with_fast_tries(self) -> Self {
        Self {
            trie_type: TrieType::Fast,
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

    fn trie_type(&self) -> TrieType {
        self.trie_type
    }

    fn collation_han_database(&self) -> CollationHanDatabase {
        self.collation_han_database
    }

    /// List the locales for the given CLDR coverage levels
    pub fn locales_for_coverage_levels(
        &self,
        levels: impl IntoIterator<Item = CoverageLevel>,
    ) -> Result<impl IntoIterator<Item = icu_locale_core::LanguageIdentifier>, DataError> {
        self.cldr()?.locales(levels)
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
#[non_exhaustive]
enum TrieType {
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

trait IterableDataProviderCached<M: DataMarker>: DataProvider<M> {
    fn supported_requests_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError>;
}

impl DatagenProvider {
    #[allow(clippy::type_complexity)] // not as complex as it appears
    fn populate_supported_requests_cache<M: DataMarker>(
        &self,
    ) -> Result<&HashSet<(Cow<'static, DataLocale>, Cow<'static, DataMarkerAttributes>)>, DataError>
    where
        DatagenProvider: IterableDataProviderCached<M>,
    {
        self.supported_requests_cache
            .insert_with(M::INFO, || {
                Box::new(self.supported_requests_cached().map(|m| {
                    m.into_iter()
                        .map(|(k, v)| (Cow::Owned(k), Cow::Owned(v)))
                        .collect()
                }))
            })
            .as_ref()
            .map_err(|&e| e)
    }
}

impl<M: DataMarker> IterableDataProvider<M> for DatagenProvider
where
    DatagenProvider: IterableDataProviderCached<M>,
{
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        Ok(self
            .populate_supported_requests_cache()?
            .iter()
            .map(|(k, v)| (k.clone().into_owned(), v.clone().into_owned()))
            .collect())
    }

    fn supports_request(
        &self,
        locale: &DataLocale,
        marker_attributes: &DataMarkerAttributes,
    ) -> Result<bool, DataError> {
        Ok(self
            .populate_supported_requests_cache()?
            .contains(&(Cow::Borrowed(locale), Cow::Borrowed(marker_attributes))))
    }
}
