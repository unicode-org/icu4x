// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerovec::{ZeroMap, ZeroVec};

/// This type contains all of the constants data for units conversion.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(UnitsConstantsV1Marker, "units/constants@1", singleton))]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsConstantsV1<'data> {
    // TODO(#3882): Use a more efficient representation for the values with numerators and denominators.
    // Also, the constant types.
    /// Maps from constant name (e.g. ft_to_m) to the value of the constant (e.g. 0.3048).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub constants_map: ZeroMap<'data, str, ConstantValueULE>,
}

#[zerovec::make_ule(ConstantTypeULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ConstantType {
    #[default]
    Actual = 0,
    Approximate = 1,
}

#[zerovec::make_ule(SignULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Sign {
    #[default]
    Positive = 0,
    Negative = 1,
}

#[zerovec::make_varule(ConstantValueULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[zerovec::derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct ConstantValue<'data> {
    #[serde(borrow)]
    pub numerator: ZeroVec<'data, u8>,
    #[serde(borrow)]
    pub denominator: ZeroVec<'data, u8>,

    pub sign: Sign,

    pub constant_type: ConstantType,
}
