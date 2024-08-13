// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerovec::ZeroMap2d;

/// Currency Compact V1 data struct.
#[icu_provider::data_struct(marker(CurrencyCompactV1Marker, "currency/compact@1"))]
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency_compact)
)]
#[yoke(prove_covariance_manually)]
pub struct CurrencyCompactV1<'data> {
    // TODO: this map should include a `DoublePattern` as a value.
    /// Contains the compact patterns for a compact currency format based on the plural rules.
    /// NOTE:
    ///       A map keyed on log10 of the CLDR `type` attribute.
    ///       For example:
    ///         `"1000-count-one": "¤0K"`
    ///                 -> key1 = 3, key2 = CompactCount::One, value = "¤0K"
    ///         `"1000-count-one-alt-alphaNextToNumber": "¤ 0K"`
    ///                 -> key1 = 3, key2 = CompactCount::OneAlt, value = "¤ 0K"
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub compact_patterns: ZeroMap2d<'data, i8, CompactCount, str>,
}

#[zerovec::make_ule(CompactCountULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency_compact)
)]
#[repr(u8)]
pub enum CompactCount {
    /// CompactPattern `zero`.
    Zero = 0,
    /// Compact Pattern `zero` alternative.
    ZeroAlt = 1,

    /// CompactPattern `one`.
    One = 2,
    /// Compact Pattern `one` alternative.
    OneAlt = 3,

    /// CompactPattern `two`.
    Two = 4,
    /// Compact Pattern `two` alternative.
    TwoAlt = 5,

    /// Compact Pattern `few`.
    Few = 6,
    /// Compact Pattern `few` alternative.
    FewAlt = 7,

    /// CompactPattern `many`.
    Many = 8,
    /// Compact Pattern `many` alternative.
    ManyAlt = 9,

    /// CompactPattern `other`.
    Other = 10,
    /// Compact Pattern `other` alternative.
    OtherAlt = 11,
}
