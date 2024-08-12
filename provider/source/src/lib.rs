// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_source` defines [`SourceDataProvider`], the authorative ICU4X [`DataProvider`] that produces data from
//! CLDR and ICU sources.
//!
//! [`SourceDataProvider`] is mainly intended as a source for the `icu_provider_export` crate,
//! which can be used to transform the data into a more efficient format.
//!
//! # Cargo features
//!
//! * `networking`
//!   * enables networking support to download CLDR and ICU source data from GitHub
//! * `use_wasm` / `use_icu4c`
//!   * see the documentation on [`icu_codepointtrie_builder`](icu_codepointtrie_builder#build-configuration)
//! * `experimental`
//!   * enables markers defined in the unstable `icu::experimental` module

use cldr_cache::CldrCache;
use elsa::sync::FrozenMap;
use icu_provider::prelude::*;
use source::{AbstractFs, SerdeCache};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::path::Path;
use std::sync::{Arc, OnceLock};

mod calendar;
mod characters;
mod cldr_serde;
mod collator;
#[cfg(feature = "experimental")]
mod currency;
mod datetime;
mod decimal;
#[cfg(feature = "experimental")]
mod displaynames;
#[cfg(feature = "experimental")]
mod duration;
mod list;
mod locale;
mod normalizer;
#[cfg(feature = "experimental")]
mod percent;
#[cfg(feature = "experimental")]
mod personnames;
mod plurals;
mod properties;
#[cfg(feature = "experimental")]
mod relativetime;
mod segmenter;
mod time_zones;
#[cfg(feature = "experimental")]
mod transforms;
mod ucase;
#[cfg(feature = "experimental")]
mod units;

mod cldr_cache;
mod source;

#[cfg(test)]
mod tests;

/// An [`ExportableProvider`](icu_provider::export::ExportableProvider) backed by raw CLDR and ICU data.
///
/// This provider covers all markers that are used by ICU4X. It is intended as the canonical
/// provider for `ExportDriver::export`.
///
/// If a required data source has not been set, `DataProvider::load` will
/// fail with the appropriate error:
/// * [`is_missing_cldr_error`](Self::is_missing_cldr_error)
/// * [`is_missing_icuexport_error`](Self::is_missing_icuexport_error)
/// * [`is_missing_segmenter_lstm_error`](Self::is_missing_segmenter_lstm_error)
#[allow(clippy::exhaustive_structs)] // any information will be added to SourceData
#[derive(Debug, Clone)]
pub struct SourceDataProvider {
    cldr_paths: Option<Arc<CldrCache>>,
    icuexport_paths: Option<Arc<SerdeCache>>,
    segmenter_lstm_paths: Option<Arc<SerdeCache>>,
    trie_type: TrieType,
    collation_han_database: CollationHanDatabase,
    #[allow(clippy::type_complexity)] // not as complex as it appears
    requests_cache: Arc<
        FrozenMap<
            DataMarkerInfo,
            Box<OnceLock<Result<HashSet<DataIdentifierCow<'static>>, DataError>>>,
        >,
    >,
}

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        icu_provider::export::make_exportable_provider!(SourceDataProvider, [
            $($marker,)+
            $(#[cfg(feature = "experimental")] $emarker,)+
        ]);

        #[cfg(test)]
        icu_provider::dynutil::impl_dynamic_data_provider!(SourceDataProvider, [
            $($marker,)+
            $(#[cfg(feature = "experimental")] $emarker,)+
        ], icu_provider::any::AnyMarker);
    }
}
icu_provider_registry::registry!(cb);

icu_provider::marker::impl_data_provider_never_marker!(SourceDataProvider);

impl SourceDataProvider {
    /// The latest CLDR JSON tag that has been verified to work with this version of `SourceDataProvider`.
    pub const LATEST_TESTED_CLDR_TAG: &'static str = "45.0.0";

    /// The latest ICU export tag that has been verified to work with this version of `SourceDataProvider`.
    pub const LATEST_TESTED_ICUEXPORT_TAG: &'static str = "icu4x/2024-05-16/75.x";

    /// The latest segmentation LSTM model tag that has been verified to work with this version of `SourceDataProvider`.
    pub const LATEST_TESTED_SEGMENTER_LSTM_TAG: &'static str = "v0.1.0";

    /// A provider using the latest data that has been verified to work with this version of `SourceDataProvider`.
    ///
    /// See [`LATEST_TESTED_CLDR_TAG`](Self::LATEST_TESTED_CLDR_TAG),
    /// [`LATEST_TESTED_ICUEXPORT_TAG`](Self::LATEST_TESTED_ICUEXPORT_TAG),
    /// [`LATEST_TESTED_SEGMENTER_LSTM_TAG`](Self::LATEST_TESTED_SEGMENTER_LSTM_TAG).
    ///
    /// ✨ *Enabled with the `networking` Cargo feature.*
    #[cfg(feature = "networking")]
    pub fn new_latest_tested() -> Self {
        // Singleton so that all instantiations share the same cache.
        static SINGLETON: std::sync::OnceLock<SourceDataProvider> = std::sync::OnceLock::new();
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
            requests_cache: Default::default(),
        }
    }

    /// Adds CLDR source data to the provider. The root should point to a local
    /// `cldr-{tag}-json-full` directory or ZIP file (see
    /// [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    pub fn with_cldr(self, root: &Path) -> Result<Self, DataError> {
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
    pub fn with_icuexport(self, root: &Path) -> Result<Self, DataError> {
        Ok(Self {
            icuexport_paths: Some(Arc::new(SerdeCache::new(AbstractFs::new(root)?))),
            ..self
        })
    }

    /// Adds segmenter LSTM source data to the provider. The path should point to a local
    /// `models` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/lstm_word_segmentation/releases)).
    pub fn with_segmenter_lstm(self, root: &Path) -> Result<Self, DataError> {
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

    const MISSING_CLDR_ERROR: DataError =
        DataError::custom("Missing CLDR data. Use `.with_cldr[_for_tag]` to set CLDR data.");

    const MISSING_ICUEXPORT_ERROR: DataError =
        DataError::custom("Missing ICU data. Use `.with_icuexport[_for_tag]` to set ICU data.");

    const MISSING_SEGMENTER_LSTM_ERROR: DataError = DataError::custom(
        "Missing segmenter data. Use `.with_segmenter_lstm[_for_tag]` to set segmenter data.",
    );

    /// Identifies errors that are due to missing CLDR data.
    pub fn is_missing_cldr_error(mut e: DataError) -> bool {
        e.marker_path = None;
        e == Self::MISSING_CLDR_ERROR
    }

    /// Identifies errors that are due to missing ICU export data.
    pub fn is_missing_icuexport_error(mut e: DataError) -> bool {
        e.marker_path = None;
        e == Self::MISSING_ICUEXPORT_ERROR
    }

    /// Identifies errors that are due to missing segmenter LSTM data.
    pub fn is_missing_segmenter_lstm_error(mut e: DataError) -> bool {
        e.marker_path = None;
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
    ) -> Result<impl IntoIterator<Item = DataLocale>, DataError> {
        self.cldr()?.locales(levels)
    }
}

impl SourceDataProvider {
    fn check_req<M: DataMarker>(&self, req: DataRequest) -> Result<(), DataError>
    where
        SourceDataProvider: IterableDataProviderCached<M>,
    {
        if <M as DataMarker>::INFO.is_singleton {
            if !req.id.locale.is_und() {
                Err(DataErrorKind::InvalidRequest)
            } else {
                Ok(())
            }
        } else if !self.populate_requests_cache()?.contains(&req.id.as_cow()) {
            Err(DataErrorKind::IdentifierNotFound)
        } else {
            Ok(())
        }
        .map_err(|e| e.with_req(<M as DataMarker>::INFO, req))
    }
}

#[test]
fn test_check_req() {
    use icu::locale::langid;
    use icu_provider::hello_world::*;

    #[allow(non_local_definitions)] // test-scoped, only place that uses it
    impl DataProvider<HelloWorldV1Marker> for SourceDataProvider {
        fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
            HelloWorldProvider.load(req)
        }
    }

    #[allow(non_local_definitions)] // test-scoped, only place that uses it
    impl crate::IterableDataProviderCached<HelloWorldV1Marker> for SourceDataProvider {
        fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
            Ok(HelloWorldProvider.iter_ids()?.into_iter().collect())
        }
    }

    let provider = SourceDataProvider::new_testing();
    assert!(provider
        .check_req::<HelloWorldV1Marker>(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("fi").into()),
            ..Default::default()
        })
        .is_ok());
    assert!(provider
        .check_req::<HelloWorldV1Marker>(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("arc").into()),
            ..Default::default()
        })
        .is_err());
}

trait IterableDataProviderCached<M: DataMarker>: DataProvider<M> {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError>;
}

impl SourceDataProvider {
    #[allow(clippy::type_complexity)] // not as complex as it appears
    fn populate_requests_cache<M: DataMarker>(
        &self,
    ) -> Result<&HashSet<DataIdentifierCow>, DataError>
    where
        SourceDataProvider: IterableDataProviderCached<M>,
    {
        self.requests_cache
            .insert_with(M::INFO, || Box::new(OnceLock::new()))
            // write lock gets dropped here, `iter_ids_cached` might be expensive
            .get_or_init(|| self.iter_ids_cached())
            .as_ref()
            .map_err(|&e| e)
    }
}

impl<M: DataMarker> IterableDataProvider<M> for SourceDataProvider
where
    SourceDataProvider: IterableDataProviderCached<M>,
{
    fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow>, DataError> {
        Ok(if <M as DataMarker>::INFO.is_singleton {
            [Default::default()].into_iter().collect()
        } else {
            self.populate_requests_cache()?
                .iter()
                .map(|id| id.as_borrowed().as_cow())
                .collect()
        })
    }
}

/// Specifies the collation Han database to use.
///
/// Unihan is more precise but significantly increases data size. See
/// <https://github.com/unicode-org/icu/blob/main/docs/userguide/icu::data/buildtool.md#collation-ucadata>
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
