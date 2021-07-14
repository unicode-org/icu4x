// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_locid::LanguageIdentifier;
use icu_provider::yoke::{self, *};
use tinystr::{TinyStr4, TinyStr8};

pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    pub const ALIASES_V1: ResourceKey = resource_key!(aliases, "aliases", 1);
    pub const LIKELY_SUBTAGS_V1: ResourceKey = resource_key!(likelysubtags, "likelysubtags", 1);
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct AliasesV1 {
    pub language: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    pub language_variants: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    pub sgn_region: Vec<(TinyStr4, LanguageIdentifier)>,
    pub language_len2: Vec<(TinyStr4, LanguageIdentifier)>,
    pub language_len3: Vec<(TinyStr4, LanguageIdentifier)>,
    pub script: Vec<(TinyStr4, TinyStr4)>,
    pub region_alpha: Vec<(TinyStr4, TinyStr4)>,
    pub region_num: Vec<(TinyStr4, TinyStr4)>,
    pub complex_region: Vec<(TinyStr4, Vec<TinyStr4>)>,
    pub variant: Vec<(TinyStr8, TinyStr8)>,
    pub subdivision: Vec<(TinyStr8, TinyStr8)>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct LikelySubtagsV1 {
    pub language_script: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)>,
    pub language_region: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)>,
    pub language: Vec<(TinyStr4, LanguageIdentifier)>,
    pub script_region: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)>,
    pub script: Vec<(TinyStr4, LanguageIdentifier)>,
    pub region: Vec<(TinyStr4, LanguageIdentifier)>,
    pub und: LanguageIdentifier,
}
