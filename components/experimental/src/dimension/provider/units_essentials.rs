// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use zerovec::{ZeroMap, ZeroMap2d};


use icu_provider::prelude::*;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;


/// This type contains all of the essential data for units formatting such as `per`, `power`, `times`, etc.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(UnitsEssentialsV1Marker = "units/essentials@1")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::units_essentials),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsEssentialsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub powers: ZeroMap2d<'data, str, CompoundCount, str>,

    // TODO: use an id instead of str as key.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub prefixes: ZeroMap<'data, str, str>,

    // TODO: store the pattern in a SinglePattern.
    // TODO: use MeasureUnit for the units key instead of strings.
    /// Contains the narrow width patterns for the units.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub times: Cow<'data, str>,
}

/// A CLDR plural keyword, or the explicit value 1.
/// See <https://www.unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>. // TODO??
#[zerovec::make_ule(CompoundCountULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::extended_currency)
)]
#[repr(u8)]
pub enum CompoundCount {
    /// The CLDR keyword `zero`.
    Zero = 0,
    /// The CLDR keyword `one`.
    One = 1,
    /// The CLDR keyword `two`.
    Two = 2,
    /// The CLDR keyword `few`.
    Few = 3,
    /// The CLDR keyword `many`.
    Many = 4,
    /// The CLDR keyword `other`.
    Other = 5,
    /// The explicit 1 case, see <https://www.unicode.org/reports/tr35/tr35-numbers.html#Explicit_0_1_rules>.
    Explicit1 = 6,
    // TODO(younies): revise this for currency
    // NOTE(egg): No explicit 0, because the compact decimal pattern selection
    // algorithm does not allow such a thing to arise.
}
