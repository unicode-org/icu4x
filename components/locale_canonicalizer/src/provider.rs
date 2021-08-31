// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::vec::Vec;
use icu_locid::LanguageIdentifier;
use icu_provider::yoke::{self, *};
use tinystr::{TinyStr4, TinyStr8};

/// A collection of [`ResourceKey`] structs for LocaleCanonicalizer providers.
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
/// Alias data for locale canonicalization.
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
/// Likely subtags data for maximize and minimize.
pub struct LikelySubtagsV1 {
    /// Language and script.
    pub language_script: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)>,
    /// Language and region.
    pub language_region: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)>,
    /// Just language.
    pub language: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Script and region.
    pub script_region: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)>,
    /// Just script.
    pub script: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Just region.
    pub region: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Undefined.
    pub und: LanguageIdentifier,
}
