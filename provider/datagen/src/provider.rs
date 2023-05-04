// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::source::*;
use crate::transform::cldr::source::CldrCache;
use crate::{CollationHanDatabase, CoverageLevel};
use icu_provider::prelude::*;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

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
    pub(crate) cldr_paths: Option<Arc<CldrCache>>,
    pub(crate) icuexport_paths: Option<Arc<SerdeCache>>,
    pub(crate) segmenter_lstm_paths: Option<Arc<SerdeCache>>,
    pub(crate) trie_type: TrieType,
    pub(crate) collation_han_database: CollationHanDatabase,
}

impl DatagenProvider {
    /// The latest CLDR JSON tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "44.0.0";

    /// The latest ICU export tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str = "release-74-1";

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

    #[cfg(test)]
    // This is equivalent for the files defined in `tools/testdata-scripts/globs.rs.data`.
    pub fn new_testing() -> Self {
        // Singleton so that all instantiations share the same cache.
        static SINGLETON: once_cell::sync::OnceCell<DatagenProvider> =
            once_cell::sync::OnceCell::new();
        SINGLETON
            .get_or_init(|| {
                Self::new_custom()
                    .with_cldr("tests/data/cldr".into())
                    .unwrap()
                    .with_icuexport("tests/data/icuexport".into())
                    .unwrap()
                    .with_segmenter_lstm("tests/data/lstm".into())
                    .unwrap()
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
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new_from_url(
                format!(
                "https://github.com/unicode-org/icu/releases/download/{tag}/icuexportdata_{}.zip",
                tag.replace('/', "-")
            ),
            )))),
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

    pub(crate) fn cldr(&self) -> Result<&CldrCache, DataError> {
        self.cldr_paths.as_deref().ok_or(Self::MISSING_CLDR_ERROR)
    }

    pub(crate) fn icuexport(&self) -> Result<&SerdeCache, DataError> {
        self.icuexport_paths
            .as_deref()
            .ok_or(Self::MISSING_ICUEXPORT_ERROR)
    }

    pub(crate) fn segmenter_lstm(&self) -> Result<&SerdeCache, DataError> {
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

    pub(crate) fn trie_type(&self) -> TrieType {
        self.trie_type
    }

    pub(crate) fn collation_han_database(&self) -> CollationHanDatabase {
        self.collation_han_database
    }

    /// List the locales for the given CLDR coverage levels
    pub fn locales_for_coverage_levels(
        &self,
        levels: impl IntoIterator<Item = CoverageLevel>,
    ) -> Result<impl IntoIterator<Item = icu_locid::LanguageIdentifier>, DataError> {
        self.cldr()?.locales(levels)
    }
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
