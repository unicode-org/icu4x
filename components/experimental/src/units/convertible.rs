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
    fn mul(self, factor: &Self::Factor) -> Self::Result {
        self * factor
    }

    // Exact
    fn mul_add(self, factor: &Self::Factor, addend: &Self::Addend) -> Self::Result {
        self * factor + addend
    }

    // Exact
    fn reciprocal_mul(self, factor: &Self::Factor) -> Self::Result {
        (self * factor).recip()
    }

    fn factor_from_ratio_bigint(factor: Ratio<BigInt>) -> Self::Factor {
        factor
    }

    fn addend_from_ratio_bigint(addend: Ratio<BigInt>) -> Self::Addend {
        addend
    }
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct RatioF64 {
    numerator: f64,
    denominator: f64,
}

impl Convertible for f64 {
    type Factor = RatioF64;
    type Addend = f64;
    type Result = f64;

    // This code performs up to 4 rounding operations:
    // * factor numerator and denominator from BigInt to f64 (factor_from_ratio_bigint)
    //   * this will be exact in most cases, but may be inexact for conversions involving
    //     large SI prefixes
    // * a single rounding for the multiplication and division
    //   * if the intermediate error is not representable, it falls back to double-rounding
    //     the multiplication and division
    fn mul(self, factor: &Self::Factor) -> Self::Result {
        let double_rounded = self * factor.numerator / factor.denominator;
        // The multiplication error is the difference of the rounded multiplication
        // and the multiplication evaluated in full precision (FMA)
        let multiplication_error = self.mul_add(factor.numerator, -(self * factor.numerator));
        // The total error is the difference between the rounded multiplication+division
        // and the multiplication evaluated in full precision, plus the multiplication
        // error divided by the denominator, all evaluated in times-den-space.
        let total_error = (double_rounded.mul_add(-factor.denominator, self * factor.numerator)
            + multiplication_error)
            / factor.denominator;

        if total_error.is_finite() {
            double_rounded + total_error
        } else {
            self * (factor.numerator / factor.denominator)
        }
    }

    // This code performs up to 5 rounding operations:
    // * factor numerator and denominator from BigInt to f64 (factor_from_ratio_bigint)
    //   * this will be exact in most cases, but may be inexact for conversions involving
    //     large SI prefixes
    // * addend from Ratio<BigInt> to f64 (addend_from_ratio_bigint)
    //   * this is exact for the known cases (Celsius/Fahrenheit/Kelvin)
    // * a single rounding for the multiplication and division
    //   * if the intermediate error is not representable, it falls back to double-rounding
    //     the multiplication and division
    // * a single rounding for the addition
    fn mul_add(self, factor: &Self::Factor, addend: &Self::Addend) -> Self::Result {
        Convertible::mul(self, factor) + addend
    }

    // This code performs up to 4 rounding operations:
    // * factor numerator and denominator from BigInt to f64 (factor_from_ratio_bigint)
    //   * this will be exact in most cases, but may be inexact for conversions involving
    //     large SI prefixes
    // * two roundings for the two divisions
    fn reciprocal_mul(self, factor: &Self::Factor) -> Self::Result {
        factor.denominator / self / factor.numerator
    }

    fn factor_from_ratio_bigint(factor: Ratio<BigInt>) -> Self::Factor {
        RatioF64 {
            // BigInt::to_f64 is infallible
            numerator: factor.numer().to_f64().unwrap_or(f64::NAN),
            // BigInt::to_f64 is infallible
            denominator: factor.denom().to_f64().unwrap_or(f64::NAN),
        }
    }

    fn addend_from_ratio_bigint(addend: Ratio<BigInt>) -> Self::Addend {
        // Ratio::<BigInt>::to_f64 is infallible
        addend.to_f64().unwrap_or(f64::NAN)
    }
}

#[test]
#[rustfmt::skip]
fn test_convertible_mul_precision() {
    let val = 5.0;
    let factor = RatioF64 {
        numerator: 1.0,
        denominator: 1_000_000.0,
    };
    assert_eq!(Convertible::mul(val, &factor), 0.000005);
    assert_eq!(val * (factor.numerator / factor.denominator), 0.0000049999999999999996);

    let val = 0.1;
    let factor = RatioF64 {
        numerator: 1.0,
        denominator: 10.0,
    };
    assert_eq!(Convertible::mul(val, &factor), 0.01);
    assert_eq!(val * (factor.numerator / factor.denominator), 0.010000000000000002);

    let val = 0.1;
    let factor = RatioF64 {
        numerator: 1.0,
        denominator: 5.0,
    };
    assert_eq!(Convertible::mul(val, &factor), 0.02);
    assert_eq!(val * (factor.numerator / factor.denominator), 0.020000000000000004);
}

#[test]
#[rustfmt::skip]
fn test_convertible_mul_extreme_cases() {
    // These extreme cases verify that the function safely propagates to NaN
    // for non-finite inputs, division by zero, and intermediate overflows.

    // If we pass NaN, it should return NaN
    assert!(Convertible::mul(f64::NAN, &RatioF64 { numerator: 1.0, denominator: 2.0 }).is_nan());
    assert!(Convertible::mul(1.0, &RatioF64 { numerator: f64::NAN, denominator: 2.0 }).is_nan());
    assert!(Convertible::mul(1.0, &RatioF64 { numerator: 1.0, denominator: f64::NAN }).is_nan());

    // If we pass Infinity, it does the correct thing
    assert_eq!(Convertible::mul(f64::INFINITY, &RatioF64 { numerator: 1.0, denominator: 2.0 }), f64::INFINITY);
    assert_eq!(Convertible::mul(1.0, &RatioF64 { numerator: f64::INFINITY, denominator: 2.0 }), f64::INFINITY);
    assert_eq!(Convertible::mul(1.0, &RatioF64 { numerator: 1.0, denominator: f64::INFINITY }), 0.0);

    assert_eq!(Convertible::mul(f64::NEG_INFINITY, &RatioF64 { numerator: 1.0, denominator: 2.0 }), f64::NEG_INFINITY);
    assert_eq!(Convertible::mul(1.0, &RatioF64 { numerator: f64::NEG_INFINITY, denominator: 2.0 }), f64::NEG_INFINITY);
    assert_eq!(Convertible::mul(1.0, &RatioF64 { numerator: 1.0, denominator: f64::NEG_INFINITY }), -0.0);

    // Division by zero
    assert_eq!(Convertible::mul(1.0, &RatioF64 { numerator: 1.0, denominator: 0.0 }), f64::INFINITY);
    assert!(Convertible::mul(0.0, &RatioF64 { numerator: 1.0, denominator: 0.0 }).is_nan());
    assert!(Convertible::mul(1.0, &RatioF64 { numerator: 0.0, denominator: 0.0 }).is_nan());
    assert!(Convertible::mul(0.0, &RatioF64 { numerator: 0.0, denominator: 0.0 }).is_nan());

    // Intermediate overflow, which is handled by the fallback path
    assert_eq!(Convertible::mul(1e300, &RatioF64 { numerator: 1e300, denominator: 1e300 }), 1e300);
}
