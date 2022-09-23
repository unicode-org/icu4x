// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::PluralOperands;

#[cfg(doc)]
use crate::PluralRules;

/// A struct for low-level users who want to construct a [`PluralOperands`]
/// directly based on the LDML Plural Operand definitions. This may be useful
/// for people with experimental rules parsing.
///
/// This struct is not intended for supported API use, and it is subject to breaking
/// changes (ex: a new Plural Operand needs to be supported).
///
/// Most users with numerical data inputs for places where [`PluralOperands`] is
/// accepted, like [`PluralRules::category_for`], should convert to [`PluralOperands`].
/// See [`PluralOperands`] for details.
#[cfg(feature = "experimental")]
#[allow(clippy::exhaustive_structs)] // experimental
pub struct RawPluralOperands {
    /// Integer value of input
    pub i: u64,
    /// Number of visible fraction digits with trailing zeros
    pub v: usize,
    /// Number of visible fraction digits without trailing zeros
    pub w: usize,
    /// Visible fraction digits with trailing zeros
    pub f: u64,
    /// Visible fraction digits without trailing zeros
    pub t: u64,
    /// Exponent of the power of 10 used in compact decimal formatting
    pub c: usize,
}

impl From<RawPluralOperands> for PluralOperands {
    fn from(rpo: RawPluralOperands) -> PluralOperands {
        Self {
            i: rpo.i,
            v: rpo.v,
            w: rpo.w,
            f: rpo.f,
            t: rpo.t,
            c: rpo.c,
        }
    }
}
