// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
#![warn(unused_imports)]
//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use zerovec::ule::UnvalidatedStr;
use zerovec::ZeroMap;

// We use raw TinyAsciiStrs for map keys, as we then don't have to
// validate them as subtags on deserialization. Map lookup can be
// done even if they are not valid tags (an invalid key will just
// become inaccessible).
type UnvalidatedRegion = TinyAsciiStr<3>;

#[icu_provider::data_struct(RegionDisplayNamesV1Marker = "displaynames/regions@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// RegionDisplayNames provides mapping between a region code and locale display name.
pub struct RegionDisplayNamesV1<'data> {
    /// Mapping for region to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedRegion, str>,
    /// Mapping for region to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedRegion, str>,
}

#[icu_provider::data_struct(LanguageDisplayNamesV1Marker = "displaynames/languages@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// LanguageDisplayNames provides mapping between a language code and it's display name.
pub struct LanguageDisplayNamesV1<'data> {
    /// Mapping for language to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedStr, str>,
    /// Mapping for language to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedStr, str>,
    /// Mapping for language to locale display long name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long_names: ZeroMap<'data, UnvalidatedStr, str>,
    /// Mapping for language to locale display menu variant name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub menu_names: ZeroMap<'data, UnvalidatedStr, str>,
}
