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
use zerovec::{VarZeroVec, ZeroMap};

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked2;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as unitsconversion;
    }
    icu_unitsconversion_data::make_provider!(Baked2);
    icu_unitsconversion_data::impl_units_info_v1!(Baked2);
};

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
    databake(path = icu_unitsconversion::units_provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsInfoV1<'data> {
    /// Maps from unit name (e.g. foot) to the index of the unit in the `unit_quantity` vector.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub units_info: ZeroMap<'data, str, UnitsInfoIndex>,

    /// Contains the quantity information for the units.
    /// For example, the quantity for the unit `foot` is `length`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub unit_quantity: VarZeroVec<'data, UnitQuantityULE>,

    /// Contains the conversion information, such as the conversion rate and the base unit.
    /// For example, the conversion information for the unit `foot` is `1 foot = 0.3048 meter`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub convert_units: VarZeroVec<'data, ConvertUnitsULE>,
}

#[zerovec::make_ule(UnitsInfoIndexULE)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_unitsconversion::units_provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct UnitsInfoIndex {
    /// Contains the index of the quantity in the `unit_quantity` vector.
    /// If the unit does not have a quantity, this field is `None`.
    pub quantity: Option<u16>,

    /// Contains the index of the convert unit in the `convert_units` vector.
    /// If the unit does not have a convert unit, this field is `None`.
    pub convert_unit: Option<u16>,
}

#[zerovec::make_ule(QuantitySimplicityULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::units_provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum QuantitySimplicity {
    #[default]
    Simple = 0,
    Complex = 1,
}

#[zerovec::make_varule(UnitQuantityULE)]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_unitsconversion::units_provider),
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
pub struct UnitQuantity<'data> {
    /// Contains the quantity name.
    // TODO(#4173): Consider using an enum for the quantity name.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantity: Cow<'data, str>,

    /// Represents the simplicity of the quantity.
    pub constant_exactness: QuantitySimplicity,
}

#[zerovec::make_varule(ConvertUnitsULE)]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_unitsconversion::units_provider),
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
pub struct ConvertUnits<'data> {
    /// Contains the base unit which the unit is converted to.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub base_unit: Cow<'data, str>,

    /// Contains the conversion factor.
    // TODO(#4172): Convert this field to a fraction form, for both the factor and offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub factor: Cow<'data, str>,
}
