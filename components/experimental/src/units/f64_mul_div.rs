// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::*;

/// Computes `a * num / den` with higher precision using Fused Multiply-Add (FMA).
///
/// This function implements an algorithm that reduces rounding errors in floating-point
/// division by utilizing `f64::mul_add` to compute the errors of the multiplication
/// and division.
///
/// Used to compute unit conversions where the conversion factor is represented
/// as a rational fraction `num / den`, allowing us to compute `a * (num / den)`
/// more accurately than the naive calculation.
///
/// This uses the native FMA operation when available, falling back to the `libm` crate
/// on platforms that don't support it.
///
/// For more details and the mathematical justification, see
/// [this post by Waldemar Horwat](https://github.com/tc39/proposal-amount/issues/115).
///
/// # Limitations
/// This algorithm does not handle non-finite inputs (NaN, Infinity) or intermediate
/// overflows (e.g., `a * num > f64::MAX`) gracefully. In these cases, the error-tracking
/// math will naturally encounter indeterminate forms (like `Inf - Inf` or `0 * Inf`) and
/// return `NaN`, even if the naive calculation would have returned a sensible value
/// (like `Infinity` or `0.0`). We accept this limitation to keep the function extremely
/// simple, fast, and branchless, relying on callers to enforce finiteness invariants
/// (which we do in `RatioF64` and `Convertible::mul`) (tracked in
/// [unicode-org/icu4x#8092](https://github.com/unicode-org/icu4x/issues/8092)).
pub(super) fn f64_mul_div(a: f64, num: f64, den: f64) -> f64 {
    let double_rounded = a * num / den;

    // The multiplication error is the difference of the rounded multiplication
    // and the multiplication evaluated in full precision (FMA)
    let multiplication_error = a.mul_add(num, -(a * num));

    // The total error is the difference between the rounded multiplication+division
    // and the multiplication evaluated in full precision, plus the multiplication
    // error divided by the denominator, all evaluated in times-den-space.
    //
    // Note: Even if `a`, `num`, and `den` are finite, `total_error` (and the final
    // result) can still be non-finite (NaN) due to intermediate overflow in `a * num`
    // (see [unicode-org/icu4x#8092](https://github.com/unicode-org/icu4x/issues/8092)).
    let total_error = (double_rounded.mul_add(-den, a * num) + multiplication_error) / den;

    double_rounded + total_error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mul_div_precision() {
        // Case 1: 5 grams to tonnes (1/1_000_000)
        // Naive: 5.0 * (1.0 / 1_000_000.0) = 5.000000000000001e-6
        // f64_mul_div: 5e-6
        {
            let val = 5.0;
            let num = 1.0;
            let den = 1_000_000.0;
            let naive = val * (num / den);
            let f64_mul_div = f64_mul_div(val, num, den);
            assert_ne!(naive, f64_mul_div);
            assert_eq!(f64_mul_div, 0.000005);
            assert_eq!(naive, 0.0000049999999999999996);
        }

        // Case 2: 0.1 * (1.0 / 10.0)
        // Naive: 0.010000000000000002
        // f64_mul_div: 0.01
        {
            let val = 0.1;
            let num = 1.0;
            let den = 10.0;
            let naive = val * (num / den);
            let f64_mul_div = f64_mul_div(val, num, den);
            assert_ne!(naive, f64_mul_div);
            assert_eq!(f64_mul_div, 0.01);
            assert_eq!(naive, 0.010000000000000002);
        }

        // Case 3: 0.1 * (1.0 / 5.0)
        // Naive: 0.020000000000000004
        // f64_mul_div: 0.02
        {
            let val = 0.1;
            let num = 1.0;
            let den = 5.0;
            let naive = val * (num / den);
            let f64_mul_div = f64_mul_div(val, num, den);
            assert_ne!(naive, f64_mul_div);
            assert_eq!(f64_mul_div, 0.02);
            assert_eq!(naive, 0.020000000000000004);
        }
    }

    #[test]
    fn test_f64_mul_div_extreme_cases() {
        // These extreme cases verify that the function safely propagates to NaN
        // for non-finite inputs, division by zero, and intermediate overflows.
        // As documented in the function's limitations, many of these return NaN
        // even when naive math would return Infinity or 0.0, due to inherent
        // indeterminate forms in the branchless error-tracking math.

        // If we pass NaN, it should return NaN
        assert!(f64_mul_div(f64::NAN, 1.0, 2.0).is_nan());
        assert!(f64_mul_div(1.0, f64::NAN, 2.0).is_nan());
        assert!(f64_mul_div(1.0, 1.0, f64::NAN).is_nan());

        // If we pass Infinity, it might return NaN due to intermediate Inf - Inf
        assert!(f64_mul_div(f64::INFINITY, 1.0, 2.0).is_nan());
        assert!(f64_mul_div(1.0, f64::INFINITY, 2.0).is_nan());
        assert!(f64_mul_div(1.0, 1.0, f64::INFINITY).is_nan());

        assert!(f64_mul_div(f64::NEG_INFINITY, 1.0, 2.0).is_nan());
        assert!(f64_mul_div(1.0, f64::NEG_INFINITY, 2.0).is_nan());
        assert!(f64_mul_div(1.0, 1.0, f64::NEG_INFINITY).is_nan());

        // Division by zero
        assert!(f64_mul_div(1.0, 1.0, 0.0).is_nan());
        assert!(f64_mul_div(0.0, 1.0, 0.0).is_nan());

        // 0/0 (indeterminate forms)
        assert!(f64_mul_div(1.0, 0.0, 0.0).is_nan());
        assert!(f64_mul_div(0.0, 0.0, 0.0).is_nan());

        // Intermediate overflow: Even though all inputs (1e300) are finite, the
        // intermediate product `a * num` overflows, causing `total_error` to be
        // non-finite, which ultimately results in NaN (see
        // [unicode-org/icu4x#8092](https://github.com/unicode-org/icu4x/issues/8092)).
        assert!(f64_mul_div(1e300, 1e300, 1e300).is_nan());
    }
}
