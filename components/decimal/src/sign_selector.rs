// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Algorithms to determine how to render the sign.

use crate::options::SignDisplay;
use fixed_decimal::Signum;

pub enum SignSelection {
    /// Render the minus sign.
    Minus,
    /// Do not render a sign.
    Neither,
    /// Render the plus sign.
    Plus,
}

/// Map a signum and sign display option to what type of sign to render.
pub fn select(signum: Signum, sign_display: SignDisplay) -> SignSelection {
    use SignDisplay::*;
    use SignSelection::*;
    use Signum::*;
    match sign_display {
        Auto => match signum {
            BelowZero | NegativeZero => Minus,
            AboveZero | PositiveZero => Neither,
        },
        Always => match signum {
            BelowZero | NegativeZero => Minus,
            AboveZero | PositiveZero => Plus,
        },
        Never => Neither,
        ExceptZero => match signum {
            BelowZero => Minus,
            NegativeZero | PositiveZero => Neither,
            AboveZero => Plus,
        },
        Negative => match signum {
            BelowZero => Minus,
            AboveZero | NegativeZero | PositiveZero => Neither,
        },
    }
}
