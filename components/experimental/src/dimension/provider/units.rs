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
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap2d};

// TODO: implement the units provider
// thoughts:
//   use the category (e.g. length, temperature, etc.) as secondary key
//   have four maps:
//     - one for the long width patterns (e.g. "{0} meter")
//     - one for the short width (e.g. "{0} m"?)
//     - one for the narrow width (e.g. "{0} m")
//     - one for the DisplayNames (e.g. "meter")

#[derive(Debug)]
pub struct UnitsProvider<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    long_width: ZeroMap2d<'data, str, Count, str>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    short_width: ZeroMap2d<'data, str, Count, str>,

    #[cfg_attr(feature = "serde", serde(borrow))]
    narrow_width: ZeroMap2d<'data, str, Count, str>,
}

// TODO: revise this.
/// A CLDR plural keyword, or the explicit value 1.
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