// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable.
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
// Suppress a warning on zerovec::makevarule.
#![allow(missing_docs)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_provider::{yoke, zerofrom};
use zerovec::ZeroMap2d;

/// Relative time format V1 data struct.

#[icu_provider::data_struct(
    LongCompactDecimalFormatDataV1Marker = "compactdecimal/long@1",
    ShortCompactDecimalFormatDataV1Marker = "compactdecimal/short@1"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_compactdecimal::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct CompactDecimalPatternDataV1<'data> {
    /// A map keyed on log10 of the CLDR `type` attribute and the `cldr` count attribute.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: ZeroMap2d<'data, i8, Count, PatternULE>,
}

/// A CLDR plural keyword, or the explicit value 1.
/// See <https://www.unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>.
#[zerovec::make_ule(CountULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_compactdecimal::provider)
)]
#[repr(u8)]
pub enum Count {
    /// The CLDR keyword `other`.
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
    // NOTE(egg): No explicit 0, because the compact decimal pattern selection
    // algorithm does not allow such a thing to arise.
}

/// A compact decimal pattern, representing some literal text with an optional
/// placeholder, and the power of 10 expressed by the text.
#[derive(
    Debug, Clone, Default, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom, Ord, PartialOrd, Eq,
)]
#[zerovec::make_varule(PatternULE)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_compactdecimal::provider),
    zerovec::derive(Serialize),
)]
#[zerovec::derive(Debug)]
#[cfg_attr(feature = "serde", zerovec::derive(Deserialize))]
pub struct Pattern<'data> {
    /// The compact decimal exponent, e.g., 6 for "million".
    /// The value 0 indicates that compact notation is not used; in that case,
    /// literal text must be empty; this corresponds to the CLDR pattern "0".
    /// This is derived from the numbers of 0s in the pattern and the associated
    /// `type` attribute; it is a more convenient representation than the number
    /// of 0s, because it is often common to multiple types; for instance, the
    /// following correspond to the same [`Pattern`]:
    ///   <pattern type="1000000" count="other">0 M</pattern>
    ///   <pattern type="10000000" count="other">00 M</pattern>
    pub exponent: i8,
    /// The index in literal_text before which the placeholder is inserted;
    /// this is 0 for insertion at the beginning, which is most common.
    /// The value 255 indicates that the pattern does not have a placeholder,
    /// as in French "mille" for 1000.
    pub index: u8,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The underlying CLDR pattern with the placeholder removed, e.g.,
    /// " M" for the pattern "000 M"
    pub literal_text: Cow<'data, str>,
}
