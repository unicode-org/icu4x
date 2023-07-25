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

#[icu_provider::data_struct(CurrencyEssentialsV1Maker = "currency/essentials@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_singlenumberformatter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentialsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub currency_patterns_map: ZeroMap<'data, UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard: Cow<'data, str>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard_alpha_next_to_number: Cow<'data, str>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub place_holders: VarZeroVec<'data, str>,
}

// TODO(https://github.com/unicode-org/icu4x/issues/3737): Reduce the size of CurrencyPatternsULE.
#[zerovec::make_ule(CurrencyPatternsULE)]
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
    pub short_pattern_standard: bool,

    /// If it is true, then use the standard pattern.
    /// Otherwise, use the standard_alpha_next_to_number pattern.
    pub narrow_pattern_standard: bool,

    /// The index of the short pattern place holder in the place holders list.
    /// If the value is u16::MAX, this means that the short pattern does not have a place holder.
    pub short_place_holder_index: u16,

    /// The index of the narrow pattern place holder in the place holders list.
    /// If the value is u16::MAX, this means that the narrow pattern does not have a place holder.
    pub narrow_place_holder_index: u16,
}
