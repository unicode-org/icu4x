// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap};

#[cfg(feature = "datagen")]
/// The latest minimum set of keys required by this component.
pub const KEYS: &[DataKey] = &[
    CurrencyEssentialsV1Marker::KEY,
    PercentEssentialsV1Marker::KEY,
];

/// This type contains all of the essential data for currency formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(CurrencyEssentialsV1Marker = "currency/essentials@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_dimension::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentialsV1<'data> {
    /// Maps from currency iso code to currency patterns
    /// which points to which pattern to use and the place holder index.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub currency_patterns_map: ZeroMap<'data, UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>,

    /// Represents the standard pattern.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard: Cow<'data, str>,

    /// Represents the standard alpha_next_to_number pattern.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard_alpha_next_to_number: Cow<'data, str>,

    /// Contains all the place holders.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub place_holders: VarZeroVec<'data, str>,

    /// Represents the currency patten in case the currency patterns map does not contain the currency.
    pub default_pattern: CurrencyPatterns,
}

#[zerovec::make_ule(PatternSelectionULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_dimension::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PatternSelection {
    /// Use the standard pattern.
    #[default]
    Standard = 0,

    /// Use the standard_alpha_next_to_number pattern.
    StandardAlphaNextToNumber = 1,
}

#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_dimension::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u16)]
pub enum PlaceholderValue {
    /// The index of the place holder in the place holders list.
    /// NOTE: the maximum value is MAX_PLACE_HOLDER_INDEX which is 2045 (0b0111_1111_1101).
    Index(u16),

    /// The place holder is the iso code.
    ISO,
}
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_dimension::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct CurrencyPatterns {
    /// If it is true, then use the standard pattern.
    /// Otherwise, use the standard_alpha_next_to_number pattern.
    pub short_pattern_standard: PatternSelection,

    /// If it is true, then use the standard pattern.
    /// Otherwise, use the standard_alpha_next_to_number pattern.
    pub narrow_pattern_standard: PatternSelection,

    /// The index of the short pattern place holder in the place holders list.
    /// If the value is `None`, this means that the short pattern does not have a place holder.
    pub short_place_holder_index: Option<PlaceholderValue>,

    /// The index of the narrow pattern place holder in the place holders list.
    /// If the value is `None`, this means that the narrow pattern does not have a place holder.
    pub narrow_place_holder_index: Option<PlaceholderValue>,
}

#[icu_provider::data_struct(PercentEssentialsV1Marker = "percent/essentials@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_dimension::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PercentEssentialsV1<'data> {
    /// The index of the number placeholder in the standard pattern.
    pub number_index: u16,

    /// Prefix and suffix to apply to a percent sign when needed.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub percent_sign_affixes: PercentAffixesV1<'data>,

    /// The percent symbol.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub percent_sign_symbol: Cow<'data, str>,

    /// The index of the percent symbol in the standard pattern.
    pub percent_symbol_index: u16,

    /// Represents the standard pattern.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard: Cow<'data, str>,
}

/// A collection of strings to affix to a percent number.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Default, Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_dimension::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PercentAffixesV1<'data> {
    /// String to prepend before the percent sign.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub prefix: Cow<'data, str>,

    /// String to append after the percent sign.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub suffix: Cow<'data, str>,
}
