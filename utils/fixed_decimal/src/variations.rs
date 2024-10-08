// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{FixedDecimal, Sign};

/// This module defines variations of numeric types, including signed values,
/// values with infinity, and values with NaN.

/// The `Signed` struct represents a numeric value with an associated sign.
pub struct Signed<T> {
    pub sign: Sign,
    pub value: T,
}

/// The `WithInfinity` enum represents a numeric value that may be either infinite or finite.
pub enum WithInfinity<T> {
    Infinity,
    Finite(T),
}

/// The `WithNaN` enum represents a numeric value that may be NaN.
pub enum WithNaN<T> {
    NaN,
    N(T),
}

/// The `WithCompactExponent` struct represents a numeric value with a compact exponent.
pub struct WithCompactExponent<T> {
    pub exponent: u8,
    pub significand: T,
}

/// The `WithScientificExponent` struct represents a numeric value with a scientific exponent.
pub struct WithScientificExponent<T> {
    pub exponent: i16,
    pub significand: T,
}

pub type SignedFixedDecimal = Signed<FixedDecimal>;

pub type FixedDecimalOrInfinity = WithInfinity<FixedDecimal>;

pub type SignedFixedDecimalOrInfinity = Signed<FixedDecimalOrInfinity>;

pub type SignedFixedDecimalOrInfinityOrNan = WithNaN<SignedFixedDecimalOrInfinity>;
