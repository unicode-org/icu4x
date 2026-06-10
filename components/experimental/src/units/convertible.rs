// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::ToPrimitive;

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

    /// Converts a [`Ratio<BigInt>`] to the associated [`Rate`] type.
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

impl Convertible for f64 {
    type Rate = F64Rate;

    fn mul_rate(&self, rate: &Self::Rate) -> Self {
        let a = *self;
        let num = rate.num;
        let den = rate.den;

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
