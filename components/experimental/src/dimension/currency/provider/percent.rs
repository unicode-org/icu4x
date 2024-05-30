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

#[icu_provider::data_struct(PercentEssentialsV1Marker = "percent/essentials@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::currency::provider::percent),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PercentEssentialsV1<'data> {
    /// The index of the number placeholder in the standard pattern.
    pub number_index: u8,

    /// Prefix and suffix to apply to a percent sign when needed.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub percent_sign_affixes: PercentAffixesV1<'data>,

    /// The percent symbol.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub percent_sign_symbol: Cow<'data, str>,

    /// The index of the percent symbol in the standard pattern.
    pub percent_symbol_index: u8,

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
    databake(path = icu_experimental::dimension::currency::provider::percent),
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
