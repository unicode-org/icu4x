// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::string::String;
use alloc::string::ToString;

/// Represents the base of an si prefix.
#[zerovec::make_ule(BaseULE)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::measure::provider::si_prefix))]
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

// TODO: Consider reducing the size of this struct while implementing the ULE.
/// Represents the SI prefix.
#[zerovec::make_ule(SiPrefixULE)]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::measure::provider::si_prefix))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SiPrefix {
    /// The absolute value of the power of the si prefix.
    pub power: i8,

    /// The base of the si prefix.
    pub base: Base,
}

/// Represents the si prefix as a string in a short representation.
///
/// # Examples
///
/// ```
/// use icu_experimental::measure::provider::si_prefix::SiPrefix;
/// use icu_experimental::measure::provider::si_prefix::Base;
///
/// let si_prefix = SiPrefix { power: 3, base: Base::Decimal };
/// let string_representation = si_prefix.to_string();
/// assert_eq!(string_representation, "D3");
///
/// let si_prefix = SiPrefix { power: -3, base: Base::Binary };
/// let string_representation = si_prefix.to_string();
/// assert_eq!(string_representation, "B-3");
/// ```
impl ToString for SiPrefix {
    fn to_string(&self) -> String {
        let base_char = if self.base == Base::Decimal { 'D' } else { 'B' };
        let power_str = self.power.to_string();
        let mut result = String::with_capacity(1 + power_str.len());
        result.push(base_char);
        result.push_str(&power_str);
        result
    }
}
