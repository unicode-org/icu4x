// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::ToPrimitive;

/// A trait for types that can be converted between two units.
pub trait Convertible: Clone {
    type Result: core::fmt::Debug;

    /// Computes `self * factor + addend`.
    fn add_mul_ratio_bigint(&self, factor: &Ratio<BigInt>, addend: &Ratio<BigInt>) -> Self::Result;

    /// Computes `self * factor`
    fn mul_ratio_bigint(&self, factor: &Ratio<BigInt>) -> Self::Result;

    /// Computes `1/(self * factor)`
    fn reciprocal_mul_ratio_bigint(&self, factor: &Ratio<BigInt>) -> Self::Result;
}

impl Convertible for &'_ Ratio<BigInt> {
    type Result = Ratio<BigInt>;

    // Exact
    fn mul_ratio_bigint(&self, factor: &Ratio<BigInt>) -> Self::Result {
        *self * factor
    }

    // Exact
    fn add_mul_ratio_bigint(&self, factor: &Ratio<BigInt>, addend: &Ratio<BigInt>) -> Self::Result {
        *self * factor + addend
    }

    // Exact
    fn reciprocal_mul_ratio_bigint(&self, factor: &Ratio<BigInt>) -> Self::Result {
        (*self * factor).recip()
    }
}

impl Convertible for f64 {
    type Result = Self;

    // TODO: reduce error
    fn mul_ratio_bigint(&self, factor: &Ratio<BigInt>) -> Self::Result {
        // Ratio::<BigInt>::to_f64 is infallible
        self * factor.to_f64().unwrap_or(f64::NAN)
    }

    // TODO: reduce error
    fn add_mul_ratio_bigint(&self, factor: &Ratio<BigInt>, addend: &Ratio<BigInt>) -> Self::Result {
        // Ratio::<BigInt>::to_f64 is infallible
        self * factor.to_f64().unwrap_or(f64::NAN) + addend.to_f64().unwrap_or(f64::NAN)
    }

    // TODO: reduce error
    fn reciprocal_mul_ratio_bigint(&self, factor: &Ratio<BigInt>) -> Self::Result {
        // Ratio::<BigInt>::to_f64 is infallible
        1.0 / (self * factor.to_f64().unwrap_or(f64::NAN))
    }
}
