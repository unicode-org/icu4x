// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[zerovec::make_ule(CountULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::currency_patterns)
)]
#[repr(u8)]
pub enum Count {
    /// UnitPattern `zero`.
    Zero = 0,
    /// UnitPattern `one`.
    One = 1,
    /// UnitPattern `two`.
    Two = 2,
    /// UnitPattern `few`.
    Few = 3,
    /// UnitPattern `many`.
    Many = 4,
    /// UnitPattern `other`.
    Other = 5,
}
