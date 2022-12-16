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
use zerovec::ZeroMap;

#[icu_provider::data_struct(CurrencyEssentialUsdV1Marker = "currency/usd/essential@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_singlenumberformatter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentialV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub symbol: Cow<'data, str>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: CurrencyPattern<'data>,
}

#[icu_provider::data_struct(CurrencyLongUsdV1Marker = "currency/usd/long@1")]
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_singlenumberformatter::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct CurrencyLong<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: ZeroMap<'data, Count, CurrencyPatternULE>,
}

#[derive(
    Debug, Clone, Default, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom, Ord, PartialOrd, Eq,
)]
#[zerovec::make_varule(CurrencyPatternULE)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_singlenumberformatter::provider),
    zerovec::derive(Serialize),
)]
#[zerovec::derive(Debug)]
#[cfg_attr(feature = "serde", zerovec::derive(Deserialize))]
pub struct CurrencyPattern<'data> {
    pub index: u8,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: Cow<'data, str>,
}

/// A CLDR plural keyword, or the explicit value 0 or 1.
/// See <https://www.unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>.
#[zerovec::make_ule(CountULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_singlenumberformatter::provider)
)]
#[repr(u8)]
pub enum Count {
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
    // The explicit 0 - 1 case, see <https://www.unicode.org/reports/tr35/tr35-numbers.html#Explicit_0_1_rules>.
    Explicit01 = 6,
}
