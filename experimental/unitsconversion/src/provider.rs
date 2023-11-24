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
use zerotrie::ZeroTrie;
use zerovec::{VarZeroVec, ZeroVec};

#[cfg(feature = "datagen")]
/// The latest minimum set of keys required by this component.
pub const KEYS: &[DataKey] = &[UnitsInfoV1Marker::KEY];

/// This type encapsulates all the constant data required for unit conversions.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(UnitsInfoV1Marker, "units/info@1", singleton))]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsInfoV1<'data> {
    /// Maps from unit name (e.g. foot) to it is conversion information.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub units_conversion_trie: ZeroTrie<ZeroVec<'data, u8>>,

    /// Contains the conversion information, such as the conversion rate and the base unit.
    /// For example, the conversion information for the unit `foot` is `1 foot = 0.3048 meter`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub convert_infos: VarZeroVec<'data, ConversionInfoULE>,
}

/// Represents the conversion information for a unit.
/// Which includes the base unit (the unit which the unit is converted to), the conversion factor, and the offset.
#[zerovec::make_varule(ConversionInfoULE)]
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
pub struct ConversionInfo<'data> {
    /// Contains the base unit which the unit is converted to.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub base_unit: ZeroVec<'data, MeasureUnitItem>,

    /// Represents the numerator of the conversion factor.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub factor_num: ZeroVec<'data, u8>,

    /// Represents the denominator of the conversion factor.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub factor_den: ZeroVec<'data, u8>,

    /// Represents the sign of the conversion factor.
    pub factor_sign: Sign,

    // TODO(#4311).
    /// Represents the numerator of the offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset_num: ZeroVec<'data, u8>,

    // TODO(#4311).
    /// Represents the denominator of the offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset_den: ZeroVec<'data, u8>,

    /// Represents the sign of the offset.
    pub offset_sign: Sign,

    /// Represents the exactness of the conversion factor.
    pub exactness: Exactness,
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

/// This enum is used to represent the exactness of a factor
#[zerovec::make_ule(ExactnessULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Exactness {
    #[default]
    Exact = 0,
    Approximate = 1,
}

#[zerovec::make_ule(BaseULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Base {
    #[default]
    NotExist = 0,
    Binary = 1,
    Decimal = 2,
}

#[zerovec::make_ule(MeasureUnitItemULE)]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct MeasureUnitItem {
    pub power: i8,
    pub si_base: Base,
    pub si_prefix: i8,
    pub unit_id: u16,
}
