// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::ToPrimitive;

#[allow(unused_imports)]
use core_maths::*;

// TODO: add Mul & Add for references to avoid cloning.
/// A trait for types that can be converted between two units.
pub trait Convertible: Clone {
    /// The type representing the conversion rate.
    type Rate: Clone + core::fmt::Debug;

    /// Adds two values by reference, avoiding data cloning.
    fn add_refs(&self, other: &Self) -> Self;

    /// Multiplies the value by the conversion rate.
    fn mul_rate(&self, rate: &Self::Rate) -> Self;

    /// Converts a [`Ratio<BigInt>`] to the implementing type.
    fn from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self>;

    /// Converts a [`Ratio<BigInt>`] to the associated [`Self::Rate`] type.
    fn rate_from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self::Rate>;

    /// Returns the reciprocal of the implementing type.
    fn reciprocal(&self) -> Self;
}

impl Convertible for Ratio<BigInt> {
    type Rate = Ratio<BigInt>;

    fn mul_rate(&self, rate: &Self::Rate) -> Self {
        self * rate
    }

    fn add_refs(&self, other: &Self) -> Self {
        self + other
    }

    fn from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self> {
        Some(ratio)
    }

    fn rate_from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self::Rate> {
        Some(ratio)
    }

    fn reciprocal(&self) -> Self {
        self.recip()
    }
}

/// A conversion rate for `f64` that preserves the rational representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F64Rate {
    pub(crate) num: f64,
    pub(crate) den: f64,
}

#[inline]
fn f64_mul_div(a: f64, num: f64, den: f64) -> f64 {
    let hi = a * num;
    let err = a.mul_add(num, -hi);
    let res = hi / den;
    let rem = res.mul_add(-den, hi) + err;
    let b = res + (rem / den);

    if b.is_infinite() || b.is_nan() {
        // Fallback to simple multiplication if FMA fails due to overflow/NaN
        a * (num / den)
    } else {
        b
    }
}

impl Convertible for f64 {
    type Rate = F64Rate;

    fn mul_rate(&self, rate: &Self::Rate) -> Self {
        f64_mul_div(*self, rate.num, rate.den)
    }

    fn add_refs(&self, other: &Self) -> Self {
        self + other
    }

    fn from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self> {
        ratio.to_f64()
    }

    fn rate_from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self::Rate> {
        let num = ratio.numer().to_f64()?;
        let den = ratio.denom().to_f64()?;
        Some(F64Rate { num, den })
    }

    fn reciprocal(&self) -> Self {
        self.recip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mul_div_precision() {
        // Case 1: 5 grams to tonnes (1/1_000_000)
        // Naive: 5.0 * (1.0 / 1_000_000.0) = 5.000000000000001e-6
        // FMA: 5e-6
        let val = 5.0;
        let num = 1.0;
        let den = 1_000_000.0;
        let naive = val * (num / den);
        let fma = f64_mul_div(val, num, den);
        assert_ne!(naive, fma);
        assert_eq!(fma, 0.000005);
        assert_eq!(naive, 0.0000049999999999999996);

        // Case 2: 0.1 * (1.0 / 10.0)
        // Naive: 0.010000000000000002
        // FMA: 0.01
        let val = 0.1;
        let num = 1.0;
        let den = 10.0;
        let naive = val * (num / den);
        let fma = f64_mul_div(val, num, den);
        assert_ne!(naive, fma);
        assert_eq!(fma, 0.01);
        assert_eq!(naive, 0.010000000000000002);

        // Case 3: 0.1 * (1.0 / 5.0)
        // Naive: 0.020000000000000004
        // FMA: 0.02
        let val = 0.1;
        let num = 1.0;
        let den = 5.0;
        let naive = val * (num / den);
        let fma = f64_mul_div(val, num, den);
        assert_ne!(naive, fma);
        assert_eq!(fma, 0.02);
        assert_eq!(naive, 0.020000000000000004);
    }
}
