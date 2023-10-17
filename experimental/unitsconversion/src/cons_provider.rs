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

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as unitsconversion;
    }
    icu_unitsconversion_data::make_provider!(Baked);
    icu_unitsconversion_data::imple_units_constants_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of keys required by this component.
pub const KEYS: &[DataKey] = &[UnitsConstantsV1Marker::KEY];

/// This type encapsulates all the constant data required for unit conversions.
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

/// This enum is used to represent the type of a constant value.
/// It can be either `ConstantType::Actual` or `ConstantType::Approximate`.
/// If the constant type is `ConstantType::Approximate`, it indicates that the value is not numerically accurate.
#[zerovec::make_ule(ConstantExactnessULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ConstantExactness {
    #[default]
    Exact = 0,
    Approximate = 1,
}

/// This enum is used to represent the sign of a constant value.
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

// TODO(#4098): Improve the ULE representation. Consider using a single byte for sign and type representation.
/// This struct encapsulates a constant value, comprising a numerator, denominator, sign, and type.
#[zerovec::make_varule(ConstantValueULE)]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[zerovec::derive(Debug)]
pub struct ConstantValue<'data> {
    // TODO(https://github.com/unicode-org/icu4x/issues/4092).
    /// The numerator of the constant value in bytes starting with the least significant byte.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub numerator: ZeroVec<'data, u8>,

    // TODO(https://github.com/unicode-org/icu4x/issues/4092).
    /// The denominator of the constant value in bytes starting with the least significant byte.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub denominator: ZeroVec<'data, u8>,

    /// Determines whether the constant value is positive or negative.
    pub sign: Sign,

    /// Determines whether the constant value is actual or approximate.
    pub constant_exactness: ConstantExactness,
}
