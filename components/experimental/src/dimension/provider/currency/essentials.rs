// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerovec::VarZeroVec;

use icu_pattern::{DoublePlaceholderKey, DoublePlaceholderPattern, PatternItem};

icu_provider::data_marker!(
    /// Provides standard and accounting currency patterns for currency formatting.
    ///
    /// This data is keyed by locale (with an optional numbering system marker attribute), not by currency.
    CurrencyEssentialsV1,
    CurrencyEssentials<'static>
);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
// TODO: Pack these 8 distinct index fields into a single 32-bit integer bitfield (3 bits per index) to minimize stack size.
pub struct PatternIndices {
    pub standard: u8,
    pub standard_negative: Option<u8>,
    pub standard_alpha_next_to_number: u8,
    pub standard_alpha_next_to_number_negative: Option<u8>,
    pub accounting_positive: u8,
    pub accounting_negative: Option<u8>,
    pub accounting_alpha_next_to_number_positive: u8,
    pub accounting_alpha_next_to_number_negative: Option<u8>,
}

/// This type contains all of the essential data for currency formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, PartialEq, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentials<'data> {
    /// A packed list of distinct currency patterns referenced by [`PatternIndices`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, DoublePlaceholderPattern>,

    /// Indices into `patterns` for each formatting variant.
    pub indices: PatternIndices,
}

icu_provider::data_struct!(CurrencyEssentials<'_>, #[cfg(feature = "datagen")]);

/// Which ends of a currency symbol are letters.
///
/// The getters on [`CurrencyEssentials`] use this to choose the `alphaNextToNumber`
/// patterns, which add a space when a letter of the symbol would touch the number
/// (`USD 12.00` instead of `USD12.00`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SymbolLetters {
    /// Neither end is a letter, like `$` or `€`.
    Neither,
    /// Only the first character is a letter, like `US$`.
    First,
    /// Only the last character is a letter, like `$US` (USD in `fr`).
    Last,
    /// Both ends are letters, like an ISO code (`USD`).
    Both,
}

impl SymbolLetters {
    fn first_is_letter(self) -> bool {
        matches!(self, Self::First | Self::Both)
    }

    fn last_is_letter(self) -> bool {
        matches!(self, Self::Last | Self::Both)
    }
}

impl<'a> CurrencyEssentials<'a> {
    pub fn get_positive(&'a self, letters: SymbolLetters) -> &'a DoublePlaceholderPattern {
        let standard = self
            .patterns
            .get(self.indices.standard as usize)
            .unwrap_or_else(|| {
                debug_assert!(false, "Standard pattern index is out of bounds");
                <&DoublePlaceholderPattern>::default()
            });

        if self.indices.standard_alpha_next_to_number != self.indices.standard
            && is_alpha_next_to_number(standard, letters)
        {
            self.patterns
                .get(self.indices.standard_alpha_next_to_number as usize)
                .unwrap_or_else(|| {
                    debug_assert!(false, "Standard alpha pattern index is out of bounds");
                    standard
                })
        } else {
            standard
        }
    }

    /// Returns the explicit standard negative pattern, if it exists.
    ///
    /// If the category has no explicit negative pattern, `None` is returned and the caller
    /// falls back to the positive pattern of the same category, applying the sign
    /// itself per UTS #35 (LDML Part 3: Numbers, Section 3.2.1).
    pub fn get_negative(&'a self, letters: SymbolLetters) -> Option<&'a DoublePlaceholderPattern> {
        let Some(standard) = self.patterns.get(self.indices.standard as usize) else {
            debug_assert!(false, "Standard pattern index is out of bounds");
            return None;
        };

        let negative_idx = if self.indices.standard_alpha_next_to_number_negative
            != self.indices.standard_negative
            && is_alpha_next_to_number(standard, letters)
        {
            self.indices.standard_alpha_next_to_number_negative
        } else {
            self.indices.standard_negative
        }?;

        let Some(pattern) = self.patterns.get(negative_idx as usize) else {
            debug_assert!(false, "Standard negative pattern index is out of bounds");
            return None;
        };
        Some(pattern)
    }

    pub fn get_positive_accounting(
        &'a self,
        letters: SymbolLetters,
    ) -> &'a DoublePlaceholderPattern {
        let standard = self
            .patterns
            .get(self.indices.accounting_positive as usize)
            .unwrap_or_else(|| {
                debug_assert!(false, "Accounting standard pattern index is out of bounds");
                <&DoublePlaceholderPattern>::default()
            });

        if self.indices.accounting_alpha_next_to_number_positive != self.indices.accounting_positive
            && is_alpha_next_to_number(standard, letters)
        {
            self.patterns
                .get(self.indices.accounting_alpha_next_to_number_positive as usize)
                .unwrap_or_else(|| {
                    debug_assert!(false, "Accounting alpha pattern index is out of bounds");
                    standard
                })
        } else {
            standard
        }
    }

    /// Returns the explicit accounting negative pattern, if it exists.
    ///
    /// If the selected category has no explicit negative pattern, `None` is returned and the
    /// caller falls back to the positive pattern of the same category, applying the
    /// sign itself per UTS #35 (LDML Part 3: Numbers, Section 3.2.1).
    pub fn get_negative_accounting(
        &'a self,
        letters: SymbolLetters,
    ) -> Option<&'a DoublePlaceholderPattern> {
        let Some(accounting) = self.patterns.get(self.indices.accounting_positive as usize) else {
            debug_assert!(false, "Accounting pattern index is out of bounds");
            return None;
        };

        let negative_idx = if self.indices.accounting_alpha_next_to_number_negative
            != self.indices.accounting_negative
            && is_alpha_next_to_number(accounting, letters)
        {
            self.indices.accounting_alpha_next_to_number_negative
        } else {
            self.indices.accounting_negative
        }?;

        let Some(pattern) = self.patterns.get(negative_idx as usize) else {
            debug_assert!(false, "Accounting negative pattern index is out of bounds");
            return None;
        };
        Some(pattern)
    }
}

fn is_alpha_next_to_number(pattern: &DoublePlaceholderPattern, letters: SymbolLetters) -> bool {
    let position = |key| {
        pattern
            .iter()
            .position(|x| x == PatternItem::Placeholder(key))
    };

    match (
        position(DoublePlaceholderKey::Place0),
        position(DoublePlaceholderKey::Place1),
    ) {
        // The symbol comes right after the number: its first character is next to the number.
        (Some(number), Some(currency)) if number + 1 == currency => letters.first_is_letter(),
        // The symbol comes right before the number: its last character is next to the number.
        (Some(number), Some(currency)) if currency + 1 == number => letters.last_is_letter(),
        // They aren't next to each other, or the pattern lacks one of them.
        _ => false,
    }
}

#[test]
fn test_is_alpha_next_to_number() {
    #[track_caller]
    fn check(pattern: &str, letters: SymbolLetters, expected: bool) {
        let pattern = DoublePlaceholderPattern::try_from_str(pattern, Default::default()).unwrap();
        assert_eq!(is_alpha_next_to_number(&pattern, letters), expected);
    }

    // The symbol comes right after the number: its first character is next to the number.
    check("{0}{1}", SymbolLetters::First, true);
    check("{0}{1}", SymbolLetters::Last, false);
    // The symbol comes right before the number: its last character is next to the number.
    check("{1}{0}", SymbolLetters::Last, true);
    check("{1}{0}", SymbolLetters::First, false);
    // No letters at either end.
    check("{1}{0}", SymbolLetters::Neither, false);
    // A space separates them.
    check("{1} {0}", SymbolLetters::Both, false);
    // A pattern without one of the placeholders.
    check("{1}", SymbolLetters::Both, false);
    check("{0}", SymbolLetters::Both, false);
}
