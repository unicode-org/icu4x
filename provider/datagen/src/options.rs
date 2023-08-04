// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options bag for [`DatagenProvider`](crate::DatagenProvider).

pub use crate::transform::cldr::source::CoverageLevel;

/// Defines how fallback will apply to the generated data.
#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum FallbackMode {
    /// This mode tries to generate data for the supplied locales. If data doesn't exist for a locale, it will be skipped.
    ///
    /// This is the pre-1.2 behavior, and requires manual runtime fallback.
    Legacy,
    /// This mode generates a minimum set of data that is sufficient under fallback at runtime. For example if en and en-US have
    /// the same values, en-US will not be included, as it is available through fallback.
    ///
    /// Data generated in this mode automatically uses runtime fallback, it is not possible to use such data without fallback.
    Runtime,
    /// This mode generates data for *exactly* the supplied locales. If data doesn't exist for a locale, fallback will be
    /// performed and the fallback value will be exported. Note that for data exporters that deduplicate values (such as
    /// `BakedExporter` and `BlobDataExporter`), the only impact on data size will be additional keys (i.e `en-US`).
    ///
    /// Requires using `LocaleInclude::Explicit`.
    ///
    /// Data generated in this mode can be used without runtime fallback and guarantees that all locales are present.
    Expand,
}

impl Default for FallbackMode {
    fn default() -> Self {
        Self::Legacy
    }
}

use icu_locid::LanguageIdentifier;
use std::collections::HashSet;

/// Options bag for [`DatagenProvider`](crate::DatagenProvider).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Options {
    /// The set of keys to generate. See [`icu_datagen::keys`], 
    /// [`icu_datagen::all_keys`], [`icu_datagen::key`] and [`icu_datagen::keys_for_bin`].
    pub keys: HashSet<icu_provider::DataKey>,
    /// Defines the locales to include
    pub locales: LocaleInclude,
    /// The collation types to include.
    ///
    /// The special string `"search*"` causes all search collation tables to be included.
    pub collations: HashSet<String>,
    /// The type of fallback that the data should be generated for. If locale fallback is
    /// used at runtime, smaller data can be generated.
    pub fallback: FallbackMode,
    /// The segmentation models to include
    pub segmenter_models: SegmenterModelInclude,
}

/// Defines the locales to include
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LocaleInclude {
    /// All locales
    All,
    /// No locales
    None,
    /// An explicit set of locales
    Explicit(HashSet<LanguageIdentifier>),
    /// All locales with the given CLDR coverage levels
    CldrSet(HashSet<CoverageLevel>),
    /// A recommended set of locales.
    ///
    /// This currently resolves to `CldrSet({Modern, Moderate, Basic})` but
    /// might change in future releases.
    Recommended,
}

impl Default for LocaleInclude {
    fn default() -> Self {
        Self::All
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
/// The segmentation models to include
pub enum SegmenterModelInclude {
    /// Include the recommended set of models. This will cover all languages supported
    /// by ICU4X: Thai, Burmese, Khmer, Lao, Chinese, and Japanese. Both dictionary
    /// and LSTM models will be included, to the extent required by the chosen data keys.
    Recommended,
    /// Include no dictionary or LSTM models. This will make line and word segmenters
    /// behave like simple rule-based segmenters, which will be incorrect when handling text
    /// that contains Thai, Burmese, Khmer, Lao, Chinese, or Japanese.
    None,
    /// Include an explicit list of LSTM or dictionary models, to the extent required by the
    /// chosen data keys.
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
    Explicit(Vec<String>),
}

impl Default for SegmenterModelInclude {
    fn default() -> Self {
        Self::Recommended
    }
}
