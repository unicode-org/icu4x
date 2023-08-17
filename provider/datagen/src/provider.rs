// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(deprecated)]

use crate::source::*;
use crate::transform::cldr::source::CldrCache;
use crate::{CollationHanDatabase, CoverageLevel};
use icu_provider::prelude::*;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

/// A [`DataProvider`] backed by raw CLDR and ICU data.
///
/// This provider covers all keys that are used by ICU4X. It is intended as the canonical
/// provider for [`DatagenDriver::export`](crate::DatagenDriver::export).
///
/// If a specific data source has not been set, `DataProvider::load` will
/// error ([`is_missing_cldr_error`](crate::is_missing_cldr_error) /
/// [`is_missing_icuexport_error`](crate::is_missing_icuexport_error)) /
/// [`is_missing_segmenter_lstm_error`](crate::is_missing_segmenter_lstm_error))
/// if the data is required for that key.
#[allow(clippy::exhaustive_structs)] // any information will be added to SourceData
#[derive(Debug, Clone)]
pub struct DatagenProvider {
    #[doc(hidden)] // semver
    pub source: SourceData,
}

impl Default for DatagenProvider {
    fn default() -> Self {
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
            },
        }
    }
}

impl DatagenProvider {
    /// The latest CLDR JSON tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "43.1.0";

    /// The latest ICU export tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str = "icu4x/2023-05-02/73.x";

    /// The latest segmentation LSTM model tag that has been verified to work with this version of `icu_datagen`.
    pub const LATEST_TESTED_SEGMENTER_LSTM_TAG: &'static str = "v0.1.0";

    /// A provider using the latest data that has been verified to work with this version of `icu_datagen`.
    ///
    /// See [`DatagenProvider::LATEST_TESTED_CLDR_TAG`], [`DatagenProvider::LATEST_TESTED_ICUEXPORT_TAG`],
    /// [`DatagenProvider::LATEST_TESTED_SEGMENTER_LSTM_TAG`].
    ///
    /// ✨ *Enabled with the `networking` Cargo feature.*
    #[cfg(feature = "networking")]
    pub fn latest_tested() -> Self {
        // Singleton so that all instantiations share the same cache.
        static SINGLETON: once_cell::sync::OnceCell<DatagenProvider> =
            once_cell::sync::OnceCell::new();
        SINGLETON
            .get_or_init(|| {
                Self::default()
                    .with_cldr_for_tag(Self::LATEST_TESTED_CLDR_TAG)
                    .with_icuexport_for_tag(Self::LATEST_TESTED_ICUEXPORT_TAG)
                    .with_segmenter_lstm_for_tag(Self::LATEST_TESTED_SEGMENTER_LSTM_TAG)
            })
            .clone()
    }

    #[cfg(test)]
    pub fn latest_tested_offline_subset() -> Self {
        // Singleton so that all instantiations share the same cache.
        static SINGLETON: once_cell::sync::OnceCell<DatagenProvider> =
            once_cell::sync::OnceCell::new();
        SINGLETON
            .get_or_init(|| {
                // This is equivalent for the files defined in `tools/testdata-scripts/globs.rs.data`.
                let data_root =
                    std::path::Path::new(core::env!("CARGO_MANIFEST_DIR")).join("tests/data");
                Self::default()
                    .with_cldr(data_root.join("cldr"))
                    .unwrap()
                    .with_icuexport(data_root.join("icuexport"))
                    .unwrap()
                    .with_segmenter_lstm(data_root.join("lstm"))
                    .unwrap()
            })
            .clone()
    }

    /// Adds CLDR data to this `SourceData`. The root should point to a local
    /// `cldr-{tag}-json-full.zip` directory or ZIP file (see
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

    /// Adds ICU export data to this `SourceData`. The path should point to a local
    /// `icuexportdata_{tag}.zip` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            source: SourceData {
                icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
                ..self.source
            },
        })
    }

    /// Adds segmenter LSTM data to this `SourceData`. The path should point to a local
    /// `models.zip` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/lstm_word_segmentation/releases)).
    pub fn with_segmenter_lstm(self, root: PathBuf) -> Result<Self, DataError> {
        Ok(Self {
            source: SourceData {
                segmenter_lstm_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
                ..self.source
            },
        })
    }

    /// Adds CLDR data to this `SourceData`. The data will be downloaded from GitHub
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

    /// Adds ICU export data to this `SourceData`. The data will be downloaded from GitHub
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

    /// Adds segmenter LSTM data to this `SourceData`. The data will be downloaded from GitHub
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

    pub(crate) fn cldr(&self) -> Result<&CldrCache, DataError> {
        self.source
            .cldr_paths
            .as_deref()
            .ok_or(crate::error::MISSING_CLDR_ERROR)
    }

    pub(crate) fn icuexport(&self) -> Result<&SerdeCache, DataError> {
        self.source
            .icuexport_paths
            .as_deref()
            .ok_or(crate::error::MISSING_ICUEXPORT_ERROR)
    }

    pub(crate) fn segmenter_lstm(&self) -> Result<&SerdeCache, DataError> {
        self.source
            .segmenter_lstm_paths
            .as_deref()
            .ok_or(crate::error::MISSING_SEGMENTER_LSTM_ERROR)
    }

    pub(crate) fn trie_type(&self) -> TrieType {
        self.source.trie_type
    }

    pub(crate) fn collation_han_database(&self) -> CollationHanDatabase {
        self.source.collation_han_database
    }

    /// List the locales for the given CLDR coverage levels
    pub fn locales_for_coverage_levels(
        &self,
        levels: impl IntoIterator<Item = CoverageLevel>,
    ) -> Result<Vec<icu_locid::LanguageIdentifier>, DataError> {
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

// SEMVER GRAVEYARD

/// Requires `legacy_api` Cargo feature
///
/// Bag of options for datagen source data.
///
/// Warning: this includes hardcoded segmentation data for backwards compatibility.
/// It is strongly discouraged to keep using this API, instead use [`DatagenProvider`]
/// and set segmentation data explicitly.
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
    pub(crate) icuexport_dictionary_fallback: Option<Arc<SerdeCache>>,
    #[cfg(feature = "legacy_api")]
    pub(crate) collations: Vec<String>,
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
            ..DatagenProvider::default().source
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
    /// See [`DatagenProvider::latest_tested`]
    pub fn latest_tested() -> Self {
        DatagenProvider::latest_tested().source
    }

    /// See [`DatagenProvider::with_cldr`]
    pub fn with_cldr(
        self,
        root: PathBuf,
        _use_default_here: crate::CldrLocaleSubset,
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
        _use_default_here: crate::CldrLocaleSubset,
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
    pub fn with_cldr_latest(
        self,
        _use_default_here: crate::CldrLocaleSubset,
    ) -> Result<Self, DataError> {
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
    /// See [`DatagenDriver::with_collations`](crate::DatagenDriver::with_collations)
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
            .ok_or(crate::error::MISSING_CLDR_ERROR)?
            .locales(levels.iter().copied())
    }
}
