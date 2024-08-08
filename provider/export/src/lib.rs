// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::needless_doctest_main)]
//! `icu_provider_export` is a library to generate data files that can be used in ICU4X data providers.
//!
//! For command-line usage, see the [`icu4x-datagen` binary](https://crates.io/crate/icu4x-datagen).
//!
//! Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md).
//!
//! # Examples
//!
//! ```no_run
//! use icu_provider_export::blob_exporter::*;
//! use icu_provider_export::prelude::*;
//! use icu_provider_source::SourceDataProvider;
//! use std::fs::File;
//!
//! let provider = SourceDataProvider::new_latest_tested();
//!
//! ExportDriver::new([DataLocaleFamily::FULL], DeduplicationStrategy::None.into(), LocaleFallbacker::try_new_unstable(&provider).unwrap())
//!     .with_markers([icu::list::provider::AndListV2Marker::INFO])
//!     .export(
//!         &provider,
//!         BlobExporter::new_v2_with_sink(Box::new(
//!             File::create("data.postcard").unwrap(),
//!         )),
//!     )
//!     .unwrap();
//! ```
//!
//! # Cargo features
//!
//! This crate has a lot of dependencies, some of which are not required for all operating modes. These default Cargo features
//! can be disabled to reduce dependencies:
//! * `baked_exporter`
//!   * enables the [`baked_exporter`] module, a reexport of [`icu_provider_baked::export`]
//!   * enables the `--format mod` CLI argument
//! * `blob_exporter`
//!   * enables the [`blob_exporter`] module, a reexport of [`icu_provider_blob::export`]
//!   * enables the `--format blob` CLI argument
//! * `fs_exporter`
//!   * enables the [`fs_exporter`] module, a reexport of [`icu_provider_fs::export`]
//!   * enables the `--format dir` CLI argument
//! * `rayon`
//!   * enables parallelism during export
//! * `experimental`
//!   * enables data generation for markers defined in the unstable `icu_experimental` crate
//!   * note that this features affects the behaviour of `all_markers`

#![cfg_attr(
    not(test),
    deny(
        // This is a tool, and as such we don't care about panics too much
        // clippy::indexing_slicing,
        // clippy::unwrap_used,
        // clippy::expect_used,
        // clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

mod export_impl;
mod locale_family;
pub use locale_family::*;

#[cfg(feature = "baked_exporter")]
pub use icu_provider_baked::export as baked_exporter;
#[cfg(feature = "blob_exporter")]
pub use icu_provider_blob::export as blob_exporter;
#[cfg(feature = "fs_exporter")]
pub use icu_provider_fs::export as fs_exporter;

/// A prelude for using the datagen API
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        DataLocaleFamily, DeduplicationStrategy, ExportDriver, FallbackOptions, NoFallbackOptions,
    };
    #[doc(no_inline)]
    pub use icu_locale::{locale, LocaleFallbacker};
    #[doc(no_inline)]
    pub use icu_provider::{export::DataExporter, DataLocale, DataMarker, DataMarkerInfo};
}

use icu_locale::LocaleFallbacker;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;

/// Configuration for a data export operation.
///
/// Note that this only configures *which data* is exported. The input provider, usually
/// `SourceDataProvider`, might expose more options about the data itself.
///
/// # Examples
///
/// ```no_run
/// use icu_provider_export::blob_exporter::*;
/// use icu_provider_export::prelude::*;
/// use icu_provider_source::SourceDataProvider;
///
/// let provider = SourceDataProvider::new_latest_tested();
///
/// ExportDriver::new([DataLocaleFamily::FULL], DeduplicationStrategy::None.into(), LocaleFallbacker::try_new_unstable(&provider).unwrap())
///     .with_markers([icu::list::provider::AndListV2Marker::INFO])
///     .export(
///         &provider,
///         BlobExporter::new_with_sink(Box::new(&mut Vec::new())),
///     )
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct ExportDriver {
    markers: Option<HashSet<DataMarkerInfo>>,
    requested_families: HashMap<DataLocale, DataLocaleFamilyAnnotations>,
    #[allow(clippy::type_complexity)] // sigh
    attributes_filters:
        HashMap<String, Arc<Box<dyn Fn(&DataMarkerAttributes) -> bool + Send + Sync + 'static>>>,
    fallbacker: LocaleFallbacker,
    include_full: bool,
    deduplication_strategy: DeduplicationStrategy,
}

impl core::fmt::Debug for ExportDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExportDriver")
            .field("markers", &self.markers)
            .field("requested_families", &self.requested_families)
            .field("attributes_filters", &self.attributes_filters.keys())
            .field("fallbacker", &self.fallbacker)
            .field("include_full", &self.include_full)
            .field("deduplication_strategy", &self.deduplication_strategy)
            .finish()
    }
}

impl ExportDriver {
    /// Creates a [`ExportDriver`].
    ///
    /// The fallbacker is used to resolve locale families, and to dedpulicate data if requested.
    /// Make sure to use the same fallback data when loading from the provider at runtime.
    /// Commonly, you will export the fallback markers, in which case you should construct
    /// your fallbacker with the source provider (i.e. [`LocaleFallbacker::try_new_unstable`]).
    pub fn new(
        locales: impl IntoIterator<Item = DataLocaleFamily>,
        options: FallbackOptions,
        fallbacker: LocaleFallbacker,
    ) -> Self {
        let mut include_full = false;
        Self {
            markers: Default::default(),
            requested_families: locales
                .into_iter()
                .filter_map(|family| {
                    Some((
                        family.locale.or_else(|| {
                            // Full locale family: set the bit instead of adding to the set
                            debug_assert_eq!(
                                family.annotations,
                                DataLocaleFamily::FULL.annotations
                            );
                            include_full = true;
                            None
                        })?,
                        family.annotations,
                    ))
                })
                .collect(),
            attributes_filters: Default::default(),
            include_full,
            fallbacker,
            deduplication_strategy: options.deduplication_strategy,
        }
        .with_recommended_segmenter_models()
        .with_additional_collations([])
    }

    /// TODO
    pub fn with_marker_attributes_filter(
        mut self,
        domain: &str,
        filter: impl Fn(&DataMarkerAttributes) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.attributes_filters
            .insert(String::from(domain), Arc::new(Box::new(filter)));
        self
    }

    /// Sets this driver to generate the given data markers.
    ///
    /// If this is not called, all markers supported by the provider will be exported.
    pub fn with_markers(self, markers: impl IntoIterator<Item = DataMarkerInfo>) -> Self {
        Self {
            markers: Some(markers.into_iter().collect()),
            ..self
        }
    }

    /// This option is only relevant if using `icu::collator`.
    ///
    /// By default, the collations `big5han`, `gb2312`, and those starting with `search`
    /// are excluded. This method can be used to reennable them.
    ///
    /// The special string `"search*"` causes all search collation tables to be included.
    pub fn with_additional_collations(
        self,
        additional_collations: impl IntoIterator<Item = String>,
    ) -> Self {
        let set = additional_collations.into_iter().collect::<HashSet<_>>();
        self.with_marker_attributes_filter("collator", move |attrs| {
            attrs.is_empty()
                || set.contains(attrs.as_str())
                || if attrs.as_str().starts_with("search") {
                    set.contains("search*")
                } else {
                    !["big5han", "gb2312"].contains(&attrs.as_str())
                }
        })
    }

    /// This option is only relevant if using `icu::segmenter`.
    ///
    /// Sets this driver to generate the recommended segmentation models, to the extent required by the
    /// chosen data markers.
    pub fn with_recommended_segmenter_models(self) -> Self {
        self.with_segmenter_models([
            "cjdict".into(),
            "burmesedict".into(),
            "khmerdict".into(),
            "laodict".into(),
            "thaidict".into(),
            "Burmese_codepoints_exclusive_model4_heavy".into(),
            "Khmer_codepoints_exclusive_model4_heavy".into(),
            "Lao_codepoints_exclusive_model4_heavy".into(),
            "Thai_codepoints_exclusive_model4_heavy".into(),
        ])
    }

    /// This option is only relevant if using `icu::segmenter`.
    ///
    /// Sets this driver to generate the given segmentation models, to the extent required by the
    /// chosen data markers.
    ///
    /// The currently supported dictionary models are
    /// * `cjdict`
    /// * `burmesedict`
    /// * `khmerdict`
    /// * `laodict`
    /// * `thaidict`
    ///
    /// The currently supported LSTM models are
    /// * `Burmese_codepoints_exclusive_model4_heavy`
    /// * `Khmer_codepoints_exclusive_model4_heavy`
    /// * `Lao_codepoints_exclusive_model4_heavy`
    /// * `Thai_codepoints_exclusive_model4_heavy`
    ///
    /// If a model is not included, the resulting line or word segmenter will apply rule-based
    /// segmentation when encountering text in a script that requires the model, which will be
    /// incorrect.
    ///
    /// If multiple models for the same language and segmentation type (dictionary/LSTM) are
    /// listed, the first one will be used.
    pub fn with_segmenter_models(self, models: impl IntoIterator<Item = String>) -> Self {
        let set = models.into_iter().collect::<HashSet<_>>();
        self.with_marker_attributes_filter("segmenter", move |attrs| set.contains(attrs.as_str()))
    }
}

/// Options bag configuring locale inclusion and behavior when runtime fallback is disabled.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct NoFallbackOptions {}

/// Choices for determining the deduplication of locales for exported data payloads.
///
/// Deduplication affects the lookup table from locales to data payloads. If a child locale
/// points to the same payload as its parent locale, then the child locale can be removed from
/// the lookup table. Therefore, all deduplication strategies guarantee that data requests for
/// selected locales will succeed so long as fallback is enabled at runtime (either internally
/// or externally). They also do not impact which _payloads_ are included: only the lookup table.
///
/// Comparison of the deduplication strategies:
///
/// | Name | Data file size | Supported locale queries? | Needs runtime fallback? |
/// |---|---|---|---|
/// | [`Maximal`] | Smallest | No | Yes |
/// | [`RetainBaseLanguages`] | Small | Yes | Yes |
/// | [`None`] | Medium/Small | Yes | No |
///
/// [`Maximal`]: DeduplicationStrategy::Maximal
/// [`RetainBaseLanguages`]: DeduplicationStrategy::RetainBaseLanguages
/// [`None`]: DeduplicationStrategy::None
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeduplicationStrategy {
    /// Removes from the lookup table any locale whose parent maps to the same data.
    Maximal,
    /// Removes from the lookup table any locale whose parent maps to the same data, except if
    /// the parent is `und`.
    RetainBaseLanguages,
    /// Keeps all selected locales in the lookup table.
    None,
}

/// Options bag configuring locale inclusion and behavior when runtime fallback is enabled.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct FallbackOptions {
    /// The aggressiveness of deduplication of data payloads.
    pub deduplication_strategy: DeduplicationStrategy,
}

impl From<DeduplicationStrategy> for FallbackOptions {
    fn from(deduplication_strategy: DeduplicationStrategy) -> Self {
        Self {
            deduplication_strategy,
        }
    }
}

/// Test that the last option with multiple conflicting families wins.
#[test]
fn test_family_precedence() {
    let driver = ExportDriver::new(
        [
            "en".parse().unwrap(),
            "%en".parse().unwrap(),
            "@en".parse().unwrap(),
            "%zh-TW".parse().unwrap(),
            "^zh-TW".parse().unwrap(),
        ],
        DeduplicationStrategy::None.into(),
        LocaleFallbacker::new_without_data(),
    );

    assert_eq!(
        driver.requested_families,
        [
            (
                icu::locale::langid!("en").into(),
                DataLocaleFamilyAnnotations::single()
            ),
            (
                icu::locale::langid!("zh-TW").into(),
                DataLocaleFamilyAnnotations::without_descendants()
            ),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
    );
}
