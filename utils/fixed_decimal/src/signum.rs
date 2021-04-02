// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Sign function for FixedDecimal.

/// Return value of the [sign function][wiki] for FixedDecimal, indicating the sign of the number
/// as well as whether the number is nonzero.
///
/// Unlike the mathematical sign function *sgn*, the FixedDecimal sign function distinguishes
/// between positive and negative zero, since in computing, the sign bit is orthogonal to the
/// magnitude of the numerical value.
///
/// [wiki]: https://en.wikipedia.org/wiki/Sign_function
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Signum {
    /// A negative, nonzero value.
    BelowZero,
    /// A zero value with the sign bit set.
    NegativeZero,
    /// A zero value without the sign bit.
    PositiveZero,
    /// A positive, nonzero value.
    AboveZero,
}
