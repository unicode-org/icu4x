// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_plurals::PluralCategory;
use icu_provider::prelude::*;
use zerovec::ZeroMap;

#[icu_provider::data_struct(marker(
    UnitsDisplayNameV1Marker,
    "units/displaynames@1",
    attributes_domain = "units"
))]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::units),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsDisplayNameV1<'data> {
    // TODO: store the pattern in a SinglePattern.
    // TODO: use `MeasureUnit` for the units key instead of strings.
    /// Contains the long width patterns for the units.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: ZeroMap<'data, Count, str>,
}

// TODO: revise this.
// TODO(younies): add a field to store the most commonly used unit pattern as the default.
/// Represents either a CLDR plural keyword or the explicit value 1.
/// See <https://www.unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>.
#[zerovec::make_ule(CountULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::compactdecimal::provider)
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
    /// The explicit 1 case, see <https://www.unicode.org/reports/tr35/tr35-numbers.html#Explicit_0_1_rules>.
    Explicit1 = 6,
    // TODO: revise this
    // NOTE(egg): No explicit 0, because the compact decimal pattern selection
    // algorithm does not allow such a thing to arise.
}

impl From<PluralCategory> for Count {
    fn from(other: PluralCategory) -> Self {
        use PluralCategory::*;
        match other {
            Zero => Count::Zero,
            One => Count::One,
            Two => Count::Two,
            Few => Count::Few,
            Many => Count::Many,
            Other => Count::Other,
        }
    }
}
