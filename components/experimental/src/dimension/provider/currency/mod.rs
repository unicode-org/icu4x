// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod displayname;
pub mod essentials;
pub mod extended;
pub mod fractions;
mod fractions_ule;
pub mod no_currency;
pub mod patterns;
pub mod symbols;

use fixed_decimal::Sign;
use icu_pattern::DoublePlaceholderPattern;

/// A positive currency pattern and, if the locale has one, an explicit negative pattern.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PatternPair<'a> {
    /// The pattern for positive values.
    pub positive: &'a DoublePlaceholderPattern,
    /// The pattern for negative values. `None` means negative values use the positive
    /// pattern with the sign in front (UTS #35, Section 3.2.1).
    pub negative: Option<&'a DoublePlaceholderPattern>,
}

impl<'a> PatternPair<'a> {
    /// Returns the pattern for a value with `sign`, and the sign the number still has to show.
    ///
    /// An explicit negative pattern already contains the sign, so the number shows none.
    pub fn for_sign(self, sign: Sign) -> (&'a DoublePlaceholderPattern, Sign) {
        match (sign, self.negative) {
            (Sign::Negative, Some(negative)) => (negative, Sign::None),
            _ => (self.positive, sign),
        }
    }
}
