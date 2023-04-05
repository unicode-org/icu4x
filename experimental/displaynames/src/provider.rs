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
type UnvalidatedLanguage = TinyAsciiStr<3>;
type UnvalidatedScript = TinyAsciiStr<4>;
type UnvalidatedLocale = UnvalidatedStr;

#[icu_provider::data_struct(RegionDisplayNamesV1Marker = "displaynames/regions@1")]
#[derive(Debug, PartialEq, Clone, Default)]
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
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// LanguageDisplayNames provides mapping between languages and display names.
pub struct LanguageDisplayNamesV1<'data> {
    /// Mapping for language to display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to short display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to long display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long_names: ZeroMap<'data, UnvalidatedLanguage, str>,
    /// Mapping for language to menu variant display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub menu_names: ZeroMap<'data, UnvalidatedLanguage, str>,
}

#[icu_provider::data_struct(ScriptDisplayNamesV1Marker = "displaynames/scripts@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// ScriptDisplayNames provides mapping between a script code and it's display name.
pub struct ScriptDisplayNamesV1<'data> {
    /// Mapping for script to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedScript, str>,
    /// Mapping for script to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedScript, str>,
}

#[icu_provider::data_struct(LocaleDisplayNamesV1Marker = "displaynames/locales@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_displaynames::provider),
)]
#[yoke(prove_covariance_manually)]
/// LocaleDisplayNames provides mapping between locales and display names.
pub struct LocaleDisplayNamesV1<'data> {
    /// Mapping for locale to display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to short display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to long display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long_names: ZeroMap<'data, UnvalidatedLocale, str>,
    /// Mapping for locale to menu variant display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub menu_names: ZeroMap<'data, UnvalidatedLocale, str>,
}
