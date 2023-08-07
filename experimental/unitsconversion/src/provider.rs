// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//
// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::{yoke, zerofrom};
use zerovec::{ZeroMap, ZeroVec};
use Default;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ConstantType {
    /// The constant value is exact.
    #[default]
    EXACT = 0,

    /// The constant value is approximate.
    APPROXIMATE = 1,
}

type ConstantTypeMask = u8;

#[icu_provider::data_struct(UnitsConstantsV1Maker = "units/constants@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsConstantsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub constants_map: ZeroMap<'data, str, u16>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub constants_values: ZeroVec<'data, ConstantValue>,
}

#[zerovec::make_ule(ConstantValueULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct ConstantValue {
    pub index: u8,

    pub repetition: u8,

    pub numerator: u32,

    pub dominator: u32,

    pub const_type: ConstantTypeMask,
}
