// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerovec::VarZeroVec;

use icu_pattern::{DoublePlaceholderKey, DoublePlaceholderPattern, PatternItem};

use super::PatternPair;
use crate::dimension::currency::options::CurrencyUsage;

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
/// [`CurrencyEssentials::patterns`] uses this to choose the `alphaNextToNumber`
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
    /// Returns the patterns for `usage` and a symbol with the given `letters`.
    pub fn patterns(&'a self, usage: CurrencyUsage, letters: SymbolLetters) -> PatternPair<'a> {
        let i = self.indices;
        let (plain, alpha_next_to_number) = match usage {
            CurrencyUsage::Standard => (
                self.pair(i.standard, i.standard_negative),
                self.pair(
                    i.standard_alpha_next_to_number,
                    i.standard_alpha_next_to_number_negative,
                ),
            ),
            CurrencyUsage::Accounting => (
                self.pair(i.accounting_positive, i.accounting_negative),
                self.pair(
                    i.accounting_alpha_next_to_number_positive,
                    i.accounting_alpha_next_to_number_negative,
                ),
            ),
        };

        // CLDR's `alphaNextToNumber` patterns are for symbols that touch the number with a
        // letter: `USD 12.00`, but `$12.00`. The positive pattern decides for both signs.
        let letter_touches_number = match SymbolPlacement::of(plain.positive) {
            SymbolPlacement::JustBefore => letters.last_is_letter(),
            SymbolPlacement::JustAfter => letters.first_is_letter(),
            SymbolPlacement::Apart => false,
        };
        if letter_touches_number {
            alpha_next_to_number
        } else {
            plain
        }
    }

    /// Looks up a positive pattern and an optional negative pattern by their indices.
    fn pair(&'a self, positive: u8, negative: Option<u8>) -> PatternPair<'a> {
        let get = |index: u8| {
            let pattern = self.patterns.get(index as usize);
            debug_assert!(pattern.is_some(), "Pattern index is out of bounds");
            pattern
        };
        PatternPair {
            positive: get(positive).unwrap_or_else(|| <&DoublePlaceholderPattern>::default()),
            negative: negative.and_then(get),
        }
    }
}

/// Where the currency symbol is in a pattern, relative to the number.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SymbolPlacement {
    /// Just before the number, like `¤#,##0.00`: the symbol's last character touches it.
    JustBefore,
    /// Just after the number, like `#,##0.00¤`: the symbol's first character touches it.
    JustAfter,
    /// Not next to the number, like `¤ #,##0.00`, or the pattern lacks one of them.
    Apart,
}

impl SymbolPlacement {
    fn of(pattern: &DoublePlaceholderPattern) -> Self {
        let position = |key| {
            pattern
                .iter()
                .position(|x| x == PatternItem::Placeholder(key))
        };
        match (
            position(DoublePlaceholderKey::Place0),
            position(DoublePlaceholderKey::Place1),
        ) {
            (Some(number), Some(symbol)) if symbol + 1 == number => Self::JustBefore,
            (Some(number), Some(symbol)) if number + 1 == symbol => Self::JustAfter,
            _ => Self::Apart,
        }
    }
}

#[test]
fn test_symbol_placement() {
    #[track_caller]
    fn check(pattern: &str, expected: SymbolPlacement) {
        let pattern = DoublePlaceholderPattern::try_from_str(pattern, Default::default()).unwrap();
        assert_eq!(SymbolPlacement::of(&pattern), expected);
    }

    check("{1}{0}", SymbolPlacement::JustBefore);
    check("{0}{1}", SymbolPlacement::JustAfter);
    check("{1} {0}", SymbolPlacement::Apart);
    // A pattern without one of the placeholders.
    check("{1}", SymbolPlacement::Apart);
    check("{0}", SymbolPlacement::Apart);
}

#[test]
fn test_patterns() {
    use CurrencyUsage::{Accounting, Standard};
    use SymbolLetters::{Both, First, Last, Neither};

    let patterns: Vec<_> = [
        // Standard: the symbol is just before the number.
        "{1}{0}",
        "-{1}{0}",
        "{1} {0}",
        "-{1} {0}",
        // Accounting: the symbol is just after the number.
        "{0}{1}",
        "({0}{1})",
        "{0} {1}",
        "({0} {1})",
    ]
    .into_iter()
    .map(|p| DoublePlaceholderPattern::try_from_str(p, Default::default()).unwrap())
    .collect();
    let essentials = CurrencyEssentials {
        patterns: VarZeroVec::from(&patterns),
        indices: PatternIndices {
            standard: 0,
            standard_negative: Some(1),
            standard_alpha_next_to_number: 2,
            standard_alpha_next_to_number_negative: Some(3),
            accounting_positive: 4,
            accounting_negative: Some(5),
            accounting_alpha_next_to_number_positive: 6,
            accounting_alpha_next_to_number_negative: Some(7),
        },
    };
    let pair = |positive: usize, negative: usize| PatternPair {
        positive: &patterns[positive],
        negative: Some(&*patterns[negative]),
    };

    // The symbol is just before the number, so its last character decides.
    assert_eq!(essentials.patterns(Standard, Neither), pair(0, 1));
    assert_eq!(essentials.patterns(Standard, First), pair(0, 1));
    assert_eq!(essentials.patterns(Standard, Last), pair(2, 3));
    assert_eq!(essentials.patterns(Standard, Both), pair(2, 3));
    // The symbol is just after the number, so its first character decides.
    assert_eq!(essentials.patterns(Accounting, Neither), pair(4, 5));
    assert_eq!(essentials.patterns(Accounting, First), pair(6, 7));
    assert_eq!(essentials.patterns(Accounting, Last), pair(4, 5));
    assert_eq!(essentials.patterns(Accounting, Both), pair(6, 7));
}
