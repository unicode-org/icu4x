// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
#![warn(unused_imports)]
//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_locale_core::subtags::{Language, Region, Script, Variant};
use icu_pattern::DoublePlaceholderPattern;
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroCow, ZeroMap};

// We use raw TinyAsciiStrs for map keys, as we then don't have to
// validate them as subtags on deserialization. Map lookup can be
// done even if they are not valid tags (an invalid key will just
// become inaccessible).
type UnvalidatedRegion = UnvalidatedTinyAsciiStr<3>;
type UnvalidatedLanguage = UnvalidatedTinyAsciiStr<3>;
type UnvalidatedScript = UnvalidatedTinyAsciiStr<4>;
type UnvalidatedLocale = PotentialUtf8;
type UnvalidatedVariant = UnvalidatedTinyAsciiStr<8>;

icu_provider::data_marker!(
    /// `LocaleDisplayNamesV1`
    LocaleDisplayNamesV1,
    LocaleDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `VariantDisplayNamesV1`
    VariantDisplayNamesV1,
    VariantDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `ScriptDisplayNamesV1`
    ScriptDisplayNamesV1,
    ScriptDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `LanguageDisplayNamesV1`
    LanguageDisplayNamesV1,
    LanguageDisplayNames<'static>
);
icu_provider::data_marker!(
    /// `RegionDisplayNamesV1`
    RegionDisplayNamesV1,
    RegionDisplayNames<'static>
);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// [`RegionDisplayNames`] provides mapping between a region code and locale display name.
pub struct RegionDisplayNames<'data> {
    /// Mapping for region to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedRegion, str>,
    /// Mapping for region to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedRegion, str>,
}

icu_provider::data_struct!(RegionDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// [`LanguageDisplayNames`] provides mapping between languages and display names.
pub struct LanguageDisplayNames<'data> {
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

icu_provider::data_struct!(LanguageDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// [`ScriptDisplayNames`] provides mapping between a script code and it's display name.
pub struct ScriptDisplayNames<'data> {
    /// Mapping for script to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedScript, str>,
    /// Mapping for script to locale display short name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_names: ZeroMap<'data, UnvalidatedScript, str>,
}

icu_provider::data_struct!(ScriptDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// [`LocaleDisplayNames`] provides mapping between locales and display names.
pub struct LocaleDisplayNames<'data> {
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

icu_provider::data_struct!(LocaleDisplayNames<'_>, #[cfg(feature = "datagen")]);

#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[yoke(prove_covariance_manually)]
/// [`VariantDisplayNames`] provides the user-translated names for the variant-code values.
pub struct VariantDisplayNames<'data> {
    /// Mapping for Variant to locale display name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: ZeroMap<'data, UnvalidatedVariant, str>,
}

/// Display name parts for use in menus.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
#[zerovec::make_varule(MenuNamePartsULE)]
#[zerovec::derive(Debug)]
#[zerovec::skip_derive(Ord)]
#[cfg_attr(feature = "serde", zerovec::derive(Deserialize))]
#[cfg_attr(feature = "datagen", zerovec::derive(Serialize))]
pub struct MenuNameParts<'data> {
    /// The "core" part of a language menu display name.
    ///
    /// For example, "Kurdish" in "Kurdish (Kurmanji)".
    pub core: VarZeroCow<'data, str>,
    /// The "extension" part of a language menu display name.
    ///
    /// For example, "Kurmanji" in "Kurdish (Kurmanji)".
    ///
    /// Note: this is the empty string for language menu names that do not have an extension.
    /// For example, in CLDR 48, "Chinese, Mandarin" is the core and there is no extension.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub extension: VarZeroCow<'data, str>,
}

icu_provider::data_struct!(VariantDisplayNames<'_>, #[cfg(feature = "datagen")]);

/// [`LocaleNamesEssentials`] provides the formatting patterns used to combine subtags.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::displaynames::provider))]
pub struct LocaleNamesEssentials<'data> {
    /// The pattern used to combine the base language name with qualifiers (e.g., `"{0} ({1})"`).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub locale_pattern: VarZeroCow<'data, DoublePlaceholderPattern>,
    /// The separator used to join multiple qualifiers (e.g., `"{0}, {1}"`).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub locale_separator: VarZeroCow<'data, DoublePlaceholderPattern>,
}

icu_provider::data_struct!(LocaleNamesEssentials<'_>, #[cfg(feature = "datagen")]);

icu_provider::data_marker!(
    /// Data marker for region display names.
    LocaleNamesRegionMediumV1,
    "locale/names/region/medium/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_region",
);

icu_provider::data_marker!(
    /// Data marker for short region display names.
    LocaleNamesRegionShortV1,
    "locale/names/region/short/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_region",
);

icu_provider::data_marker!(
    /// Data marker for language display names.
    LocaleNamesLanguageMediumV1,
    "locale/names/language/medium/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_language",
);

icu_provider::data_marker!(
    /// Data marker for short language display names.
    LocaleNamesLanguageShortV1,
    "locale/names/language/short/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_language",
);

icu_provider::data_marker!(
    /// Data marker for long language display names.
    LocaleNamesLanguageLongV1,
    "locale/names/language/long/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_language",
);

icu_provider::data_marker!(
    /// Data marker for menu-medium language display names.
    LocaleNamesLanguageMenuMediumV1,
    "locale/names/language/menu/medium/v1",
    VarZeroCow<'static, MenuNamePartsULE>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_language",
);

icu_provider::data_marker!(
    /// Data marker for script display names.
    LocaleNamesScriptMediumV1,
    "locale/names/script/medium/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_script",
);

icu_provider::data_marker!(
    /// Data marker for short script display names.
    LocaleNamesScriptShortV1,
    "locale/names/script/short/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_script",
);

icu_provider::data_marker!(
    /// Data marker for variant display names.
    LocaleNamesVariantMediumV1,
    "locale/names/variant/medium/v1",
    VarZeroCow<'static, str>,
    #[cfg(feature = "datagen")]
    attributes_domain = "locale_names_variant",
);

icu_provider::data_marker!(
    /// Data marker for locale names essentials (patterns).
    LocaleNamesEssentialsV1,
    "locale/names/essentials/v1",
    LocaleNamesEssentials<'static>
);

impl LocaleNamesLanguageMediumV1 {
    /// Helper to construct infallible attributes from subtags.
    #[inline]
    pub(crate) fn make_attributes(
        language: Language,
        script: Option<Script>,
        region: Option<Region>,
        buffer: &mut tinystr::TinyAsciiStr<16>,
    ) -> &DataMarkerAttributes {
        const HYPHEN: tinystr::TinyAsciiStr<1> = tinystr::tinystr!(1, "-");
        let lang_str = language.to_tinystr();
        *buffer = match (script, region) {
            (Some(script), Some(region)) => {
                let script_str = script.to_tinystr();
                let region_str = region.to_tinystr();
                lang_str
                    .concat::<1, 16>(HYPHEN)
                    .concat::<4, 16>(script_str)
                    .concat::<1, 16>(HYPHEN)
                    .concat::<3, 16>(region_str)
            }
            (Some(script), None) => {
                let script_str = script.to_tinystr();
                lang_str.concat::<1, 16>(HYPHEN).concat::<4, 16>(script_str)
            }
            (None, Some(region)) => {
                let region_str = region.to_tinystr();
                lang_str.concat::<1, 16>(HYPHEN).concat::<3, 16>(region_str)
            }
            (None, None) => lang_str.resize::<16>(),
        };
        // This is infallible (will not panic) because validated `Language`, `Script`,
        // `Region`, and hyphens are guaranteed to conform to `DataMarkerAttributes` syntax.
        DataMarkerAttributes::from_str_or_panic(buffer)
    }
}

impl LocaleNamesRegionMediumV1 {
    /// Helper to create data marker attributes from a region.
    #[inline]
    pub(crate) fn make_attributes(region: &Region) -> &DataMarkerAttributes {
        // This is infallible (will not panic) because a validated `Region` is guaranteed to
        // conform to `DataMarkerAttributes` syntax.
        DataMarkerAttributes::from_str_or_panic(region.as_str())
    }
}

impl LocaleNamesRegionShortV1 {
    /// Helper to create data marker attributes from a region.
    #[inline]
    pub(crate) fn make_attributes(region: &Region) -> &DataMarkerAttributes {
        // This is infallible (will not panic) because a validated `Region` is guaranteed to
        // conform to `DataMarkerAttributes` syntax.
        DataMarkerAttributes::from_str_or_panic(region.as_str())
    }
}

impl LocaleNamesScriptMediumV1 {
    /// Helper to create data marker attributes from a script.
    #[inline]
    pub(crate) fn make_attributes(script: &Script) -> &DataMarkerAttributes {
        // This is infallible (will not panic) because a validated `Script` is guaranteed to
        // conform to `DataMarkerAttributes` syntax.
        DataMarkerAttributes::from_str_or_panic(script.as_str())
    }
}

impl LocaleNamesScriptShortV1 {
    /// Helper to create data marker attributes from a script.
    #[inline]
    pub(crate) fn make_attributes(script: &Script) -> &DataMarkerAttributes {
        // This is infallible (will not panic) because a validated `Script` is guaranteed to
        // conform to `DataMarkerAttributes` syntax.
        DataMarkerAttributes::from_str_or_panic(script.as_str())
    }
}

impl LocaleNamesVariantMediumV1 {
    /// Helper to create data marker attributes from a variant.
    #[inline]
    pub(crate) fn make_attributes(variant: &Variant) -> &DataMarkerAttributes {
        // This is infallible (will not panic) because a validated `Variant` is guaranteed to
        // conform to `DataMarkerAttributes` syntax.
        DataMarkerAttributes::from_str_or_panic(variant.as_str())
    }
}
