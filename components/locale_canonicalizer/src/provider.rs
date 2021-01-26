// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locid::subtags::{Language, Region, Script};

pub mod key {
    use icu_provider::{resource::ResourceKey, resource_key};
    pub const LIKELY_SUBTAGS_V1: ResourceKey = resource_key!(likelysubtags, "likelysubtags", 1);
}

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LikelySubtagsResultV1 {
    pub language: Option<Language>,
    pub script: Option<Script>,
    pub region: Option<Region>,
}

#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LikelySubtagsV1 {
    pub language_script_region: Vec<((Language, Script, Region), LikelySubtagsResultV1)>,
    pub language_script: Vec<((Language, Script), LikelySubtagsResultV1)>,
    pub language_region: Vec<((Language, Region), LikelySubtagsResultV1)>,
    pub language: Vec<(Language, LikelySubtagsResultV1)>,
}
