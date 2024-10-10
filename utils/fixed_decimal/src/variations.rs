// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Sign;

/// This module defines variations of numeric types, including signed values,
/// values with infinity, and values with NaN.

/// The `Signed` struct represents a numeric value with an associated sign.
#[derive(Debug)]
pub struct Signed<T> {
    pub sign: Sign,
    pub value: T,
}

/// The `WithInfinity` enum represents a numeric value that may be either infinite or finite.
#[derive(Debug)]
pub enum WithInfinity<T> {
    Infinity,
    Finite(T),
}

/// The `WithNaN` enum represents a numeric value that may be NaN.
#[derive(Debug)]
pub enum WithNaN<T> {
    NaN,
    N(T),
}

/// The `WithCompactExponent` struct represents a numeric value with a compact exponent.
#[derive(Debug)]
pub struct WithCompactExponent<T> {
    pub exponent: u8,
    pub significand: T,
}

/// The `WithScientificExponent` struct represents a numeric value with a scientific exponent.
#[derive(Debug)]
pub struct WithScientificExponent<T> {
    pub exponent: i16,
    pub significand: T,
}
