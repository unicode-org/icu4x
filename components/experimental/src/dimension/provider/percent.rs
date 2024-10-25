// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_pattern::{DoublePlaceholderPattern, SinglePlaceholderPattern};
use icu_provider::prelude::*;

#[cfg(feature = "serde")]
use icu_pattern::{DoublePlaceholder, SinglePlaceholder};

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

#[icu_provider::data_struct(PercentEssentialsV1Marker = "percent/essentials@1")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::percent))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
/// A struct including the essentials to create a Percent.
///
/// If an `approximate` or `explicit plus` are required, use the negative pattern as explained below:
/// <https://www.unicode.org/reports/tr35/tr35-numbers.html#approximate-number-formatting>
/// <https://www.unicode.org/reports/tr35/tr35-numbers.html#explicit-plus-signs>
pub struct PercentEssentialsV1<'data> {
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    /// Represents the standard pattern for signed percents.
    /// NOTE: place holder 0 is the place of the percent value.
    ///       place holder 1 is the place of the plus, minus, or approximate signs.
    pub signed_pattern: Cow<'data, DoublePlaceholderPattern>,

    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<SinglePlaceholder, _>"
        )
    )]
    /// Represents the standard pattern for unsigned percents.
    pub unsigned_pattern: Cow<'data, SinglePlaceholderPattern>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The localize approximate sign.
    pub approximately_sign: Cow<'data, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The localize minus sign.
    pub minus_sign: Cow<'data, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The localize plus sign.
    pub plus_sign: Cow<'data, str>,
}
