// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::ToPrimitive;

// TODO: add Mul & Add for references to avoid cloning.
/// A trait for types that can be converted between two units.
pub trait Convertible: Clone {
    /// The type representing the conversion ratio.
    type Ratio: Clone + core::fmt::Debug;

    /// Adds two values by reference, avoiding data cloning.
    fn add_refs(&self, other: &Self) -> Self;

    /// Multiplies the value by the conversion ratio.
    fn mul_ratio(&self, ratio: &Self::Ratio) -> Self;

    /// Converts a [`Ratio<BigInt>`] to the implementing type.
    fn from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self>;

    /// Converts a [`Ratio<BigInt>`] to the associated [`Self::Ratio`] type.
    fn ratio_from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self::Ratio>;

    /// Returns the reciprocal of the implementing type.
    fn reciprocal(&self) -> Self;
}

impl Convertible for Ratio<BigInt> {
    type Ratio = Ratio<BigInt>;

    #[inline]
    fn mul_ratio(&self, ratio: &Self::Ratio) -> Self {
        self * ratio
    }

    #[inline]
    fn add_refs(&self, other: &Self) -> Self {
        self + other
    }

    #[inline]
    fn from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self> {
        Some(ratio)
    }

    #[inline]
    fn ratio_from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self::Ratio> {
        Some(ratio)
    }

    #[inline]
    fn reciprocal(&self) -> Self {
        self.recip()
    }
}

/// A conversion ratio for `f64` that preserves the rational representation.
///
/// We store the `numerator` and `denominator` as `f64` (instead of integers)
/// so they can be fed directly into the precision-improving division algorithm
/// (`f64_mul_div`).
///
/// The numerator and denominator are always integers, so this is an exact
/// representation of a `Ratio<BigInt>`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RatioF64 {
    pub(crate) numerator: f64,
    pub(crate) denominator: f64,
}

impl Convertible for f64 {
    type Ratio = RatioF64;

    #[inline]
    fn mul_ratio(&self, ratio: &Self::Ratio) -> Self {
        super::f64_mul_div::f64_mul_div(*self, ratio.numerator, ratio.denominator)
    }

    #[inline]
    fn add_refs(&self, other: &Self) -> Self {
        self + other
    }

    #[inline]
    fn from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self> {
        ratio.to_f64()
    }

    #[inline]
    fn ratio_from_ratio_bigint(ratio: Ratio<BigInt>) -> Option<Self::Ratio> {
        let numerator = ratio.numer().to_f64()?;
        let denominator = ratio.denom().to_f64()?;
        if numerator.is_finite() && denominator.is_finite() {
            Some(RatioF64 {
                numerator,
                denominator,
            })
        } else {
            None
        }
    }

    #[inline]
    fn reciprocal(&self) -> Self {
        self.recip()
    }
}
