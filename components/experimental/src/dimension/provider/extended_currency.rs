// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_plurals::PluralCategory;
use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap, ZeroMap2d, ZeroVec};

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

use super::currency::{CurrencyPatternConfig, PatternSelection, PlaceholderValue};

/// Currency Extended V1 data struct.
#[icu_provider::data_struct(marker(CurrencyExtendedDataV1Marker, "currency/extended@1"))]
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::extended_currency)
)]
#[yoke(prove_covariance_manually)]
pub struct CurrencyExtendedDataV1<'data> {
    /// A mapping from each currency's ISO code to its associated formatting patterns.
    // Using CurrencyPatternConfig until implementing AsULE for ExtendedCurrencyPatternConfig.
    // use short for long right now.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns_config: ZeroMap<'data, Count, CurrencyPatternConfig>,

    /// Contains the counted display names for the currency.
    /// For example, for "en" locale, and "USD" currency:
    ///        "US Dollars" for `zero` count,
    ///        "US Dollar"  for `one`  count,
    ///     ... etc.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub extended_placeholders: VarZeroVec<'data, str>,
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
    databake(path = icu_experimental::dimension::provider::extended_currency)
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
    // TODO(younies): revise this for currency
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

#[zerovec::make_ule(ExtendedCurrencyPatternConfigULE)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::extended_currency),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct ExtendedCurrencyPatternConfig {
    // TODO: remove it once all of them are the same.
    /// The pattern selection for the current placeholder.
    pub pattern_selection: PatternSelection,
    // /// Points to the index of the placeholder in the extended placeholders list.
    // pub placeholder_value: Option<PlaceholderValue>,
}
