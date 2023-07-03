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
    /// Defines the locales to include
    pub locales: LocaleInclude,
    /// Whether to optimize tries for speed or size
    pub trie_type: TrieType,
    /// Which Han collation to use
    pub collation_han_database: CollationHanDatabase,
    /// The collation types to include.
    ///
    /// The special string `"search*"` causes all search collation tables to be included.
    pub collations: HashSet<String>,
    /// The type of fallback that the data should be generated for. If locale fallback is
    /// used at runtime, smaller data can be generated.
    pub fallback: FallbackMode,
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

/// Specifies the collation Han database to use.
///
/// Unihan is more precise but significantly increases data size. See
/// <https://github.com/unicode-org/icu/blob/main/docs/userguide/icu_data/buildtool.md#collation-ucadata>
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CollationHanDatabase {
    /// Implicit
    #[serde(rename = "implicit")]
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

impl Default for CollationHanDatabase {
    fn default() -> Self {
        Self::Implicit
    }
}

/// Specifies the trie type to use.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum TrieType {
    /// Fast tries are optimized for speed
    #[serde(rename = "fast")]
    Fast,
    /// Small tries are optimized for size
    #[serde(rename = "small")]
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

impl Default for TrieType {
    fn default() -> Self {
        Self::Small
    }
}
