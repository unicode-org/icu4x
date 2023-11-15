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
use zerovec::{VarZeroVec, ZeroMap, ZeroVec};

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
    /// Maps from unit name (e.g. foot) to the index of the unit in the `unit_quantity` vector.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub units_info: ZeroMap<'data, str, UnitsInfoIndex>,

    /// Contains the dimensions information for the units.
    /// For example, the dimension for the unit `foot` is `length`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub unit_dimensions: VarZeroVec<'data, UnitQuantityULE>,

    /// Contains the conversion information, such as the conversion rate and the base unit.
    /// For example, the conversion information for the unit `foot` is `1 foot = 0.3048 meter`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub convert_infos: VarZeroVec<'data, ConvertUnitsULE>,
}

#[zerovec::make_ule(UnitsInfoIndexULE)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct UnitsInfoIndex {
    /// Contains the index of the dimension in the `unit_dimensions` vector.
    /// If the unit does not have a quantity, this field is `None`.
    pub dimension: Option<u16>,

    /// Contains the index of the conversion info in the `convert_infos` vector.
    /// If the unit does not have a convert unit, this field is `None`.
    pub convert_info: Option<u16>,
}

/// Specifies if the unit is a base unit or a derived unit.
/// If derived, this means each unit is derived from a base unit.
/// For example: "foot-per-second" is derived from "meter" and "second".
#[zerovec::make_ule(QuantitySimplicityULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum DerivationSpecifier {
    #[default]
    Base = 0,
    Derived = 1,
}

#[zerovec::make_varule(UnitQuantityULE)]
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
pub struct Dimension<'data> {
    /// Contains the quantity name.
    // TODO(#4173): Consider using an enum for the quantity name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantity: Cow<'data, str>,

    /// Represents the simplicity of the quantity.
    pub constant_exactness: DerivationSpecifier,
}

#[zerovec::make_varule(ConvertUnitsULE)]
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
    pub base_unit: Cow<'data, str>,

    /// Contains the conversion factor.
    // TODO(#4172): Convert this field to a fraction form, for both the factor and offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub factor: Cow<'data, str>,
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

/// This enum is used to represent the type of a constant value.
/// It can be either `ConstantType::Actual` or `ConstantType::Approximate`.
/// If the constant type is `ConstantType::Approximate`, it indicates that the value is not numerically accurate.
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
pub struct Factor<'data> {
    /// Represents the numerator of the conversion factor.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub numerator: ZeroVec<'data, u8>,

    /// Represents the denominator of the conversion factor.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub denominator: ZeroVec<'data, u8>,

    /// Represents the sign of the conversion factor.
    pub sign: Sign,

    /// Represents the numerator of the offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset_num: ZeroVec<'data, u8>,

    /// Represents the denominator of the offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset_den: ZeroVec<'data, u8>,

    /// Represents the sign of the offset.
    pub offset_sign: Sign,

    /// Represents the exactness of the conversion factor.
    pub exactness: Exactness,
}
