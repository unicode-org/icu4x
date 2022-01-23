// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::vec::Vec;
use icu_locid::LanguageIdentifier;
use icu_provider::yoke::{self, *};
use litemap::LiteMap;
use tinystr::{TinyStr4, TinyStr8};

/// A collection of [`ResourceKey`](icu_provider::ResourceKey) structs for
/// LocaleCanonicalizer providers.
pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    /// Key for aliases data.
    pub const ALIASES_V1: ResourceKey = resource_key!(LocaleCanonicalizer, "aliases", 1);
    /// Key for likely subtags data.
    pub const LIKELY_SUBTAGS_V1: ResourceKey =
        resource_key!(LocaleCanonicalizer, "likelysubtags", 1);
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
/// This alias data is used for locale canonicalization. Each field defines a
/// mapping from an old identifier to a new identifier, based upon the rules in
/// from <http://unicode.org/reports/tr35/#LocaleId_Canonicalization>. The data
/// is stored in sorted order, allowing for binary search to identify rules to
/// apply. It is broken down into smaller vectors based upon some characteristic
/// of the data, to help avoid unnecessary searches. For example, the `sgn_region`
/// field contains aliases for sign language and region, so that it is not
/// necessary to search the data unless the input is a sign language.
///
/// The algorithm in tr35 is not guaranteed to terminate on data other than what
/// is currently in CLDR. For this reason, it is not a good idea to attempt to add
/// or modify aliases for use in this structure.
pub struct AliasesV1 {
    /// Language data not covered by other rules, normally this will be empty.
    pub language: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    /// Language and variant.
    pub language_variants: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    /// Sign language and region data.
    pub sgn_region: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Two character language codes.
    pub language_len2: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Three character language codes.
    pub language_len3: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Scripts.
    pub script: Vec<(TinyStr4, TinyStr4)>,
    /// Alphabetical region codes.
    pub region_alpha: Vec<(TinyStr4, TinyStr4)>,
    /// Numeric region codes.
    pub region_num: Vec<(TinyStr4, TinyStr4)>,
    /// Old regions which map to more than one new region.
    pub complex_region: Vec<(TinyStr4, Vec<TinyStr4>)>,
    /// Variants.
    pub variant: Vec<(TinyStr8, TinyStr8)>,
    /// Subdivisions.
    pub subdivision: Vec<(TinyStr8, TinyStr8)>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
/// This likely subtags data is used for the minimize and maximize operations.
/// Each field defines a mapping from an old identifier to a new identifier,
/// based upon the rules in
/// <https://www.unicode.org/reports/tr35/#Likely_Subtags>.
///
/// The data is stored in sorted order, allowing for binary search to identify
/// rules to apply. It is broken down into smaller vectors based upon the rules
/// defined for the likely subtags maximize algorithm.
///
/// For efficiency, only the relevant part of the LanguageIdentifier is stored
/// for searching. E.g., the `language_script` field is used to store rules for
/// `LanguageIdentifier` that contain a language and a script, but not a region.
/// This also allows for space savings by using a `TinyStr4` rather than a full
/// `LanguageIdentifier`.
pub struct LikelySubtagsV1 {
    /// Language and script.
    pub language_script: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier>,
    /// Language and region.
    pub language_region: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier>,
    /// Just language.
    pub language: LiteMap<TinyStr4, LanguageIdentifier>,
    /// Script and region.
    pub script_region: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier>,
    /// Just script.
    pub script: LiteMap<TinyStr4, LanguageIdentifier>,
    /// Just region.
    pub region: LiteMap<TinyStr4, LanguageIdentifier>,
    /// Undefined.
    pub und: LanguageIdentifier,
}
