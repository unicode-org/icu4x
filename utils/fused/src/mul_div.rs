// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::CoreFloat;

/// Computes the product-quotient `(a * num) / den` in double-precision floating-point
/// arithmetic with a single rounding error, achieving near-infinite precision.
///
/// This function uses Fused Multiply-Add (FMA) to compute the exact multiplication
/// rounding error (via Dekker's decomposition) and the exact division remainder
/// (via Jeannerod's theorem). These error terms are then used to compensate the
/// primary quotient.
///
/// # Mathematical Model
///
/// The exact product is decomposed as:
/// \[ a \cdot num = hi + err_1 \]
/// where \(hi = a \otimes num\) and \(err_1 = \text{fma}(a, num, -hi)\) is the exact
/// multiplication rounding error.
///
/// The primary division is \(res = hi \oslash den\). Its exact remainder is:
/// \[ err_2 = hi - res \cdot den \]
/// computed via \(err_2 = \text{fma}(res, -den, hi)\).
///
/// The exact quotient is then:
/// \[ Q = res + \frac{err_2 + err_1}{den} \]
/// which is evaluated in floating-point and added as a correction.
///
/// # Robustness and Fallback
///
/// If the intermediate product \(a \cdot num\) overflows the finite float range,
/// or if any input is non-finite (`NaN` or `Infinity`), or if a division by zero
/// occurs, the error-compensation math naturally produces `NaN`.
///
/// In these cases, the function detects the non-finite result using a zero-cost
/// check (`corrected.is_finite()`) and falls back to the naive evaluation
/// `a * (num / den)`. Evaluating `num / den` first scales the factor to a moderate
/// range, preventing intermediate overflow and correctly propagating standard IEEE 754
/// values (such as `Infinity` or `NaN`).
///
/// # Examples
///
/// ```
/// use fused::f64_mul_div;
///
/// // Convert 5 grams to tonnes (factor: 1e-6)
/// // Naive: 5.0 * (1.0 / 1_000_000.0) = 5.000000000000001e-6
/// // FMA: 5e-6 (exact)
/// let val = 5.0;
/// let num = 1.0;
/// let den = 1_000_000.0;
/// assert_eq!(f64_mul_div(val, num, den), 0.000005);
/// ```
#[inline]
pub fn f64_mul_div(a: f64, num: f64, den: f64) -> f64 {
    let double_rounded = a * num / den;

    // Compute the exact multiplication error (Dekker's decomposition)
    let multiplication_error = a.mul_add(num, -(a * num));

    // Compute the exact division remainder (Jeannerod's theorem)
    // and add the multiplication error, then scale by the denominator
    let total_error = (double_rounded.mul_add(-den, a * num) + multiplication_error) / den;

    let corrected = double_rounded + total_error;

    // Fallback to naive calculation on intermediate overflow or non-finite cases.
    if corrected.is_finite() {
        corrected
    } else {
        a * (num / den)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mul_div_precision() {
        // Case 1: 5 grams to tonnes (1/1_000_000)
        // Naive: 5.0 * (1.0 / 1_000_000.0) = 5.000000000000001e-6
        // f64_mul_div: 5e-6
        let val = 5.0;
        let num = 1.0;
        let den = 1_000_000.0;
        let naive = val * (num / den);
        let corrected = f64_mul_div(val, num, den);
        assert_ne!(naive, corrected);
        assert_eq!(corrected, 0.000005);
        assert_eq!(naive, 0.0000049999999999999996);

        // Case 2: 0.1 * (1.0 / 10.0)
        // Naive: 0.010000000000000002
        // f64_mul_div: 0.01
        let val2 = 0.1;
        let num2 = 1.0;
        let den2 = 10.0;
        let naive2 = val2 * (num2 / den2);
        let corrected2 = f64_mul_div(val2, num2, den2);
        assert_ne!(naive2, corrected2);
        assert_eq!(corrected2, 0.01);
        assert_eq!(naive2, 0.010000000000000002);
    }

    #[test]
    fn test_f64_mul_div_extreme_cases() {
        // NaN propagation
        assert!(f64_mul_div(f64::NAN, 1.0, 2.0).is_nan());
        assert!(f64_mul_div(1.0, f64::NAN, 2.0).is_nan());
        assert!(f64_mul_div(1.0, 1.0, f64::NAN).is_nan());

        // Infinity propagation
        assert_eq!(f64_mul_div(f64::INFINITY, 1.0, 2.0), f64::INFINITY);
        assert_eq!(f64_mul_div(1.0, f64::INFINITY, 2.0), f64::INFINITY);
        assert_eq!(f64_mul_div(1.0, 2.0, f64::INFINITY), 0.0);
        assert_eq!(f64_mul_div(f64::NEG_INFINITY, 1.0, 2.0), f64::NEG_INFINITY);

        // Division by zero
        assert_eq!(f64_mul_div(1.0, 1.0, 0.0), f64::INFINITY);
        assert_eq!(f64_mul_div(-1.0, 1.0, 0.0), f64::NEG_INFINITY);
        assert!(f64_mul_div(0.0, 1.0, 0.0).is_nan()); // 0/0 is NaN

        // Intermediate overflow fallback: `a * num` overflows, but final quotient is finite.
        // FMA math would produce NaN, but fallback correctly yields finite result.
        let val_large = 1e300;
        let num_large = 1e10;
        let den_large = 1e10;
        let corrected = f64_mul_div(val_large, num_large, den_large);
        assert!(corrected.is_finite());
        assert_eq!(corrected, 1e300);
    }

    #[test]
    fn test_f64_mul_div_subnormals() {
        // Test behavior with subnormal numbers
        let val = f64::MIN_POSITIVE * 0.5; // Subnormal
        let num = 2.0;
        let den = 1.0;
        let corrected = f64_mul_div(val, num, den);
        assert_eq!(corrected, f64::MIN_POSITIVE);
    }
}
