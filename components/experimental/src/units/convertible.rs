// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::ToPrimitive;

/// A trait for types that can be converted between two units.
pub trait Convertible: Clone {
    type Factor: core::fmt::Debug + Clone;
    type Addend: core::fmt::Debug + Clone;
    type Result: core::fmt::Debug;

    /// Computes `self * factor + addend`.
    fn mul_add(self, factor: &Self::Factor, addend: &Self::Addend) -> Self::Result;

    /// Computes `self * factor`
    fn mul(self, factor: &Self::Factor) -> Self::Result;

    /// Computes `1/(self * factor)`
    fn reciprocal_mul(self, factor: &Self::Factor) -> Self::Result;

    /// Converts a [`Ratio<BigInt>`] to a [`Self::Factor`].
    fn factor_from_ratio_bigint(factor: Ratio<BigInt>) -> Self::Factor;

    /// Converts a [`Ratio<BigInt>`] to a [`Self::Addend`].
    fn addend_from_ratio_bigint(addend: Ratio<BigInt>) -> Self::Addend;
}

impl Convertible for &'_ Ratio<BigInt> {
    type Factor = Ratio<BigInt>;
    type Addend = Ratio<BigInt>;
    type Result = Ratio<BigInt>;

    // Exact
    #[inline]
    fn mul(self, factor: &Self::Factor) -> Self::Result {
        self * factor
    }

    // Exact
    #[inline]
    fn mul_add(self, factor: &Self::Factor, addend: &Self::Addend) -> Self::Result {
        self * factor + addend
    }

    // Exact
    #[inline]
    fn reciprocal_mul(self, factor: &Self::Factor) -> Self::Result {
        (self * factor).recip()
    }

    #[inline]
    fn factor_from_ratio_bigint(factor: Ratio<BigInt>) -> Self::Factor {
        factor
    }

    #[inline]
    fn addend_from_ratio_bigint(addend: Ratio<BigInt>) -> Self::Addend {
        addend
    }
}

/// A conversion ratio for `f64` that preserves the rational representation.
///
/// We store the `numerator` and `denominator` as `f64` (instead of integers)
/// so they can be fed directly into the precision-improving division algorithm
/// (`f64_mul_div`).
///
/// # Invariant
/// - Both `numerator` and `denominator` are finite integers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RatioF64 {
    pub(crate) numerator: f64,
    pub(crate) denominator: f64,
}

impl Convertible for f64 {
    type Factor = RatioF64;
    type Addend = f64;
    type Result = f64;

    #[inline]
    fn mul(self, factor: &Self::Factor) -> Self::Result {
        super::f64_mul_div::f64_mul_div(self, factor.numerator, factor.denominator)
    }

    // TODO: reduce error
    #[inline]
    fn mul_add(self, factor: &Self::Factor, addend: &Self::Addend) -> Self::Result {
        super::f64_mul_div::f64_mul_div(self, factor.numerator, factor.denominator) + addend
    }

    // TODO: reduce error
    #[inline]
    fn reciprocal_mul(self, factor: &Self::Factor) -> Self::Result {
        1.0 / super::f64_mul_div::f64_mul_div(self, factor.numerator, factor.denominator)
    }

    // Ratio::<BigInt>::to_f64 is infallible
    #[inline]
    fn factor_from_ratio_bigint(factor: Ratio<BigInt>) -> Self::Factor {
        let numerator = factor.numer().to_f64().unwrap_or(f64::NAN);
        let denominator = factor.denom().to_f64().unwrap_or(f64::NAN);
        RatioF64 {
            numerator,
            denominator,
        }
    }

    // Ratio::<BigInt>::to_f64 is infallible
    #[inline]
    fn addend_from_ratio_bigint(addend: Ratio<BigInt>) -> Self::Addend {
        addend.to_f64().unwrap_or(f64::NAN)
    }
}
