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

use icu_pattern::DoublePlaceholderPattern;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

/// This type contains all of the essential data for currency formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(CurrencyEssentialsV1Marker = "currency/essentials@1")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentialsV1<'data> {
    /// A mapping from each currency's ISO code to its associated formatting patterns.
    /// This includes information on which specific pattern to apply as well as the index
    /// of placeholders within the `placeholders` vector.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern_config_map: ZeroMap<'data, UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>,

    // TODO(#4677): Implement the pattern to accept the signed negative and signed positive patterns.
    /// Represents the standard pattern.
    /// NOTE: place holder 0 is the place of the currency value.
    ///       place holder 1 is the place of the currency sign `¤`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard_pattern: Option<DoublePlaceholderPattern<Cow<'data, str>>>,

    // TODO(#4677): Implement the pattern to accept the signed negative and signed positive patterns.
    /// Represents the standard alpha_next_to_number pattern.
    /// NOTE: place holder 0 is the place of the currency value.
    ///       place holder 1 is the place of the currency sign `¤`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard_alpha_next_to_number_pattern: Option<DoublePlaceholderPattern<Cow<'data, str>>>,

    /// Contains all the place holders.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub placeholders: VarZeroVec<'data, str>,

    /// The fallback currency pattern configuration used
    /// when a specific currency's pattern is not found in the currency patterns map.
    pub default_pattern_config: CurrencyPatternConfig,
}

#[zerovec::make_ule(PatternSelectionULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency),
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
    databake(path = icu_experimental::dimension::provider::currency),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u16)]
pub enum PlaceholderValue {
    /// The index of the place holder in the place holders list.
    /// NOTE: the maximum value is MAX_PLACEHOLDER_INDEX which is 2045 (0b0111_1111_1101).
    Index(u16),

    /// The place holder is the iso code.
    ISO,
}

#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct CurrencyPatternConfig {
    /// Indicates which pattern to use for short currency formatting.
    pub short_pattern_selection: PatternSelection,

    /// Indicates which pattern to use for narrow currency formatting.
    pub narrow_pattern_selection: PatternSelection,

    /// The index of the short pattern place holder in the place holders list.
    /// If the value is `None`, this means that the short pattern does not have a place holder.
    pub short_placeholder_value: Option<PlaceholderValue>,

    /// The index of the narrow pattern place holder in the place holders list.
    /// If the value is `None`, this means that the narrow pattern does not have a place holder.
    pub narrow_placeholder_value: Option<PlaceholderValue>,
}
