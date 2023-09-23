// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use crate::rules::runtime::ast::Rule;
use icu_provider::prelude::*;
use icu_provider::DataMarker;
use zerovec::ZeroMap2d;

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
        pub use crate as plurals;
        pub use icu_locid_transform as locid_transform;
    }
    icu_plurals_data::make_provider!(Baked);
    icu_plurals_data::impl_plurals_ordinal_v1!(Baked);
    icu_plurals_data::impl_plurals_cardinal_v1!(Baked);
    icu_plurals_data::impl_plurals_ranges_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of keys required by this component.
pub const KEYS: &[DataKey] = &[
    CardinalV1Marker::KEY,
    OrdinalV1Marker::KEY,
    PluralRangesV1Marker::KEY,
];

use crate::PluralCategory;

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: <https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    CardinalV1Marker = "plurals/cardinal@1",
    OrdinalV1Marker = "plurals/ordinal@1"
)]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_plurals::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PluralRulesV1<'data> {
    /// Rule that matches [`PluralCategory::Zero`], or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub zero: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::One`], or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub one: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::Two`], or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub two: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::Few`], or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub few: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::Many`], or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub many: Option<Rule<'data>>,
}

pub(crate) struct ErasedPluralRulesV1Marker;

impl DataMarker for ErasedPluralRulesV1Marker {
    type Yokeable = PluralRulesV1<'static>;
}

/// [`PluralCategory`] but serializable as provider data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_plurals::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[zerovec::make_ule(RawPluralCategoryULE)]
#[repr(u8)]
pub enum RawPluralCategory {
    /// CLDR "other" plural category.
    Other = 0,
    /// CLDR "zero" plural category.
    Zero = 1,
    /// CLDR "one" plural category.
    One = 2,
    /// CLDR "two" plural category.
    Two = 3,
    /// CLDR "few" plural category.
    Few = 4,
    /// CLDR "many" plural category.
    Many = 5,
}

impl RawPluralCategory {
    #[inline]
    /// Converts to an [`UnvalidatedPluralCategory`].
    pub fn to_unvalidated(self) -> UnvalidatedPluralCategory {
        UnvalidatedPluralCategory(self as u8)
    }
}

impl From<PluralCategory> for RawPluralCategory {
    fn from(value: PluralCategory) -> Self {
        match value {
            PluralCategory::Zero => RawPluralCategory::Zero,
            PluralCategory::One => RawPluralCategory::One,
            PluralCategory::Two => RawPluralCategory::Two,
            PluralCategory::Few => RawPluralCategory::Few,
            PluralCategory::Many => RawPluralCategory::Many,
            PluralCategory::Other => RawPluralCategory::Other,
        }
    }
}

/// An `u8` that is expected to be a `PluralCategory` tag but does not enforce this variant.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_plurals::provider),
)]
#[zerovec::make_ule(UnvalidatedPluralCategoryULE)]
pub struct UnvalidatedPluralCategory(pub u8);

#[cfg(feature = "datagen")]
impl serde::Serialize for UnvalidatedPluralCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        RawPluralCategory::new_from_u8(self.0)
            .ok_or_else(|| S::Error::custom("invalid tag in UnvalidatedPluralCategory"))?
            .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UnvalidatedPluralCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Ok(RawPluralCategory::deserialize(deserializer)?.to_unvalidated())
        } else {
            Ok(Self(<u8>::deserialize(deserializer)?))
        }
    }
}

/// Plural categories for ranges.
///
/// Obtains the plural category of a range from the categories of its endpoints. It is required that
/// the start value must be strictly less than the end value, and both values must be strictly positive.
///
/// More information: <https://unicode.org/reports/tr35/tr35-numbers.html#Plural_Ranges>
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(PluralRangesV1Marker = "plurals/ranges@1")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_plurals::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PluralRangesV1<'data> {
    /// Map between the categories of the endpoints of a range and its corresponding
    /// category.
    ///
    /// - `key0` corresponds to the start category of the range.
    /// - `key1` corresponds to the end category of the range.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ranges:
        ZeroMap2d<'data, UnvalidatedPluralCategory, UnvalidatedPluralCategory, RawPluralCategory>,
}
