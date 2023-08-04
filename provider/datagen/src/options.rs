// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options bag for [`DatagenProvider`](crate::DatagenProvider).

pub use crate::transform::cldr::source::CoverageLevel;

use icu_locid::LanguageIdentifier;
use std::collections::HashSet;

/// Defines how fallback will apply to the generated data. If in doubt, use
/// [`FallbackMode::PreferredForExporter`], which selects the best mode for your
/// chosen data provider.
///
/// # Fallback Mode Comparison
///
/// The modes differ primarily in their approaches to runtime fallback and data size.
///
/// | Mode | Runtime Fallback | Data Size |
/// |---|---|---|
/// | [`Runtime`] | Yes, Automatic | Smallest |
/// | [`RuntimeManual`] | Yes, Manual | Smallest |
/// | [`Preresolved`] | No | Small |
/// | [`Hybrid`] | Optional | Medium |
///
/// If you are not 100% certain of the closed set of locales you need at runtime, you should
/// use a provider with runtime fallback enabled.
///
/// [`Runtime`]: FallbackMode::Runtime
/// [`RuntimeManual`]: FallbackMode::RuntimeManual
/// [`Preresolved`]: FallbackMode::Preresolved
/// [`Hybrid`]: FallbackMode::Hybrid
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum FallbackMode {
    /// Selects the fallback mode based on [`DataExporter::supports_built_in_fallback()`](
    /// icu_provider::datagen::DataExporter::supports_built_in_fallback()), resolving to either
    /// [`Runtime`] or [`Hybrid`].
    ///
    /// [`Runtime`]: Self::Runtime
    /// [`Hybrid`]: Self::Hybrid
    #[default]
    PreferredForExporter,
    /// This mode generates the minimal set of locales that cover the requested locales when
    /// fallback is used at runtime. For example, if "en" and "en-US" are both requested but
    /// they contain the same value, only "en" will be included, since "en-US" falls back to
    /// "en" at runtime.
    ///
    /// If [`LocaleInclude::Explicit`] is used, this mode includes all ancestors and descendants
    /// (usually regional variants) of the explicitly listed locales. For example, if "pt-PT" is
    /// requested, then "pt", "pt-PT", and children like "pt-MO" will be included. Note that the
    /// children of "pt-PT" usually inherit from it and therefore don't take up a significant
    /// amount of space in the data file.
    ///
    /// This mode is only supported with the baked data provider, and it builds fallback logic
    /// into the generated code. To use this mode with other providers that don't bundle fallback
    /// logic, use [`FallbackMode::RuntimeManual`] or [`FallbackMode::Hybrid`].
    ///
    /// This is the default fallback mode for the baked provider.
    Runtime,
    /// Same as [`FallbackMode::Runtime`] except that the fallback logic is not included in the
    /// generated code. It must be enabled manually with a [`LocaleFallbackProvider`].
    ///
    /// This mode is supported on all data provider implementations.
    ///
    /// [`LocaleFallbackProvider`]: icu_provider_adapters::fallback::LocaleFallbackProvider
    RuntimeManual,
    /// This mode generates data for exactly the supplied locales. If data doesn't exist for a
    /// locale, fallback will be performed and the fallback value will be exported.
    ///
    /// Requires using [`LocaleInclude::Explicit`].
    ///
    /// Note: in data exporters that deduplicate values (such as `BakedExporter` and
    /// `BlobDataExporter`), the impact on data size as compared to [`FallbackMode::Runtime`]
    /// is limited to the pointers in the explicitly listed locales.
    ///
    /// Data generated in this mode can be used without runtime fallback and guarantees that all
    /// locales are present. If you wish to also support locales that were not explicitly listed
    /// with runtime fallback, see [`FallbackMode::Hybrid`].
    Preresolved,
    /// This mode passes through CLDR data without performing locale deduplication.
    ///
    /// If [`LocaleInclude::Explicit`] is used, this mode includes all ancestors and descendants
    /// (usually regional variants) of the explicitly listed locales. For example, if "pt-PT" is
    /// requested, then "pt", "pt-PT", and children like "pt-MO" will be included.
    ///
    /// Note: in data exporters that deduplicate values (such as `BakedExporter` and
    /// `BlobDataExporter`), the impact on data size as compared to [`FallbackMode::Runtime`]
    /// is limited to the pointers in the explicitly listed locales.
    ///
    /// Data generated in this mode is suitable for use with or without runtime fallback. To
    /// enable runtime fallback, use a [`LocaleFallbackProvider`].
    ///
    /// This is the default fallback mode for the blob and filesystem providers.
    ///
    /// [`LocaleFallbackProvider`]: icu_provider_adapters::fallback::LocaleFallbackProvider
    Hybrid,
}

/// Options bag for [`DatagenProvider`](crate::DatagenProvider).
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct Options {
    /// The set of keys to generate. See [`icu_datagen::keys`],
    /// [`icu_datagen::all_keys`], [`icu_datagen::key`] and [`icu_datagen::keys_from_bin`].
    ///
    /// [`icu_datagen::keys`]: crate::keys
    /// [`icu_datagen::all_keys`]: crate::all_keys
    /// [`icu_datagen::key`]: crate::key
    /// [`icu_datagen::keys_from_bin`]: crate::keys_from_bin
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
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum LocaleInclude {
    /// All locales
    All,
    /// No locales
    None,
    /// An explicit set of locales. Note that ancestors and children (such as regional variants)
    /// may be included as well, depending on the [`FallbackMode`].
    Explicit(HashSet<LanguageIdentifier>),
    /// All locales with the given CLDR coverage levels
    CldrSet(HashSet<CoverageLevel>),
    /// A recommended set of locales.
    ///
    /// This currently resolves to `CldrSet({Modern, Moderate, Basic})` but
    /// might change in future releases.
    #[default]
    Recommended,
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

/// Specifies the trie type to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
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

/// The segmentation models to include
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum SegmenterModelInclude {
    /// Include the recommended set of models. This will cover all languages supported
    /// by ICU4X: Thai, Burmese, Khmer, Lao, Chinese, and Japanese. Both dictionary
    /// and LSTM models will be included, to the extent required by the chosen data keys.
    #[default]
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
