// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_provider::{yoke, zerofrom};
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap};

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
    databake(path = icu_singlenumberformatter::provider),
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
}

#[zerovec::make_ule(PatternSelectionULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_singlenumberformatter::provider),
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

// TODO(#3836): replace this with Option<PlaceHolder>, enum PlaceHolder { Index(usize), ISO }
// and encapsulate the encoding in the ULE implementation.
pub const NO_PLACE_HOLDER: u16 = 0b0111_1111_1111;
pub const USE_ISO_CODE: u16 = 0b0111_1111_1110;
pub const MAX_PLACE_HOLDER_INDEX: u16 = 0b0111_1111_1101;

#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_singlenumberformatter::provider),
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
    /// If the value is `NO_PLACE_HOLDER`, this means that the short pattern does not have a place holder.
    /// If the value is `USE_ISO_CODE`, this means that the short pattern equals to the iso code.
    pub short_place_holder_index: u16,

    /// The index of the narrow pattern place holder in the place holders list.
    /// If the value is `NO_PLACE_HOLDER`, this means that the narrow pattern does not have a place holder.
    /// If the value is `USE_ISO_CODE`, this means that the narrow pattern equals to the iso code.
    pub narrow_place_holder_index: u16,
}
