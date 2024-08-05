// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerovec::ZeroMap;

/// Currency Extended V1 data struct.
#[icu_provider::data_struct(marker(CurrencyPatternsDataV1Marker, "currency/patterns@1"))]
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency_patterns)
)]
#[yoke(prove_covariance_manually)]
pub struct CurrencyPatternsDataV1<'data> {
    /// Contains the unit patterns for a currency based on plural rules.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub unit_patterns: ZeroMap<'data, PatternCount, str>,
}

#[zerovec::make_ule(UnitPatternULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency_patterns)
)]
#[repr(u8)]
pub enum PatternCount {
    /// UnitPattern `zero`.
    Zero = 0,
    /// UnitPattern `one`.
    One = 1,
    /// UnitPattern `two`.
    Two = 2,
    /// UnitPattern `few`.
    Few = 3,
    /// UnitPattern `many`.
    Many = 4,
    /// UnitPattern `other`.
    Other = 5,
}
