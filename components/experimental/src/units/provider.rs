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
use num_bigint::BigInt;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::{ule::AsULE, VarZeroVec, ZeroVec};

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

use super::ratio::IcuRatio;

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
    databake(path = icu_experimental::units::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct UnitsInfoV1<'data> {
    /// Maps from unit name (e.g. foot) to it is conversion information.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub units_conversion_trie: ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,

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
    databake(path = icu_experimental::units::provider),
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
    /// Contains the base unit (after parsing) which what the unit is converted to.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub basic_units: ZeroVec<'data, MeasureUnitItem>,

    /// Represents the numerator of the conversion factor.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub factor: Ratio<'data>,

    // TODO(#4311).
    /// Represents the numerator of the offset.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub offset: Ratio<'data>,

    /// Represents the exactness of the conversion factor.
    pub exactness: Exactness,
}

#[zerovec::make_varule(RatioULE)]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_experimental::units::provider),
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
pub struct Ratio<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub num: ZeroVec<'data, u8>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub den: ZeroVec<'data, u8>,

    pub sign: Sign,
}

/// This enum is used to represent the sign of a constant value.
#[zerovec::make_ule(SignULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::units::provider),
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
    databake(path = icu_experimental::units::provider),
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
    databake(path = icu_experimental::units::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Base {
    /// The base of the si prefix is 10.
    #[default]
    Decimal = 0,

    /// The base of the si prefix is 2.
    Binary = 1,
}

/// Represents an Item of a MeasureUnit.
/// For example, the MeasureUnit `kilometer-per-square-second` contains two items:
///    1. `kilometer` with power 1 and prefix 3 with base 10.
///    2. `second` with power -2 and prefix `NotExist`.
#[zerovec::make_ule(MeasureUnitItemULE)]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::units::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct MeasureUnitItem {
    /// The power of the unit.
    pub power: i8,

    /// The si base of the unit.
    pub si_prefix: SiPrefix,

    /// The id of the unit.
    pub unit_id: u16,
}

// TODO: Consider reducing the size of this struct while implementing the ULE.
/// Represents the SI prefix.
#[zerovec::make_ule(SiPrefixULE)]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::units::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SiPrefix {
    /// The absolute value of the power of the si prefix.
    pub power: i8,

    /// The base of the si prefix.
    pub base: Base,
}

impl ConversionInfoULE {
    /// Extracts the conversion factor as [`super::ratio::IcuRatio`].
    pub(crate) fn factor_as_ratio(&self) -> IcuRatio {
        let sign: num_bigint::Sign = Sign::from_unaligned(self.factor_sign).into();
        IcuRatio::from_big_ints(
            BigInt::from_bytes_le(sign, self.factor_num().as_ule_slice()),
            BigInt::from_bytes_le(num_bigint::Sign::Plus, self.factor_den().as_ule_slice()),
        )
    }

    /// Extracts the offset as [`super::ratio::IcuRatio`].
    pub(crate) fn offset_as_ratio(&self) -> IcuRatio {
        let sign: num_bigint::Sign = Sign::from_unaligned(self.offset_sign).into();
        IcuRatio::from_big_ints(
            BigInt::from_bytes_le(sign, self.offset_num().as_ule_slice()),
            BigInt::from_bytes_le(num_bigint::Sign::Plus, self.offset_den().as_ule_slice()),
        )
    }
}
