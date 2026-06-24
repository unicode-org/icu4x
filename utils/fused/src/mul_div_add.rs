// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::CoreFloat;

/// Computes the product-quotient with an offset `(a * num) / den + offset` in double-precision
/// floating-point arithmetic with a single rounding error.
///
/// This function represents the intermediate product-quotient as a high-precision double-word
/// `q_hi + q_lo` (computed using the same error compensation as `f64_mul_div`), and then
/// performs an exact addition of `q_hi` and `offset` using the Knuth 2Sum algorithm.
/// The small error terms are accumulated and applied as a final correction, ensuring that
/// the result is rounded exactly once to the nearest representable float.
///
/// # Mathematical Model
///
/// 1. The product-quotient is evaluated as a double-word \(q_{\text{hi}} + q_{\text{lo}}\).
/// 2. The exact sum of the high part and the offset is computed via the **2Sum algorithm**:
///    \[ q_{\text{hi}} + offset = s + e \]
///    where \(s = q_{\text{hi}} \oplus offset\) is the rounded sum, and \(e\) is the exact
///    mathematical rounding error (with no precision lost in its calculation).
/// 3. The exact total sum is:
///    \[ S_{\text{exact}} = s + e + q_{\text{lo}} \]
///    which is evaluated in floating-point by summing the small error terms first:
///    \[ \text{corrected} = s + (e + q_{\text{lo}}) \]
///
/// # Robustness and Fallback
///
/// If any intermediate term overflows or results in a non-finite form, the function
/// automatically catches this using a zero-cost check (`corrected.is_finite()`) and falls
/// back to the naive calculation `(a * (num / den)) + offset`.
///
/// # Examples
///
/// ```
/// use fused::f64_mul_div_add;
///
/// // Convert 100 Celsius to Fahrenheit: F = C * 9 / 5 + 32
/// // Both naive and FMA yield 212.0 (exact).
/// assert_eq!(f64_mul_div_add(100.0, 9.0, 5.0, 32.0), 212.0);
/// ```
#[inline]
pub fn f64_mul_div_add(a: f64, num: f64, den: f64, offset: f64) -> f64 {
    // 1. Compute division as a double-word (q_hi, q_lo)
    let q_hi = a * num / den;
    let err_mul = a.mul_add(num, -(a * num));
    let q_lo = (q_hi.mul_add(-den, a * num) + err_mul) / den;

    // 2. Exact addition of q_hi and offset via Knuth's 2Sum algorithm
    let s = q_hi + offset;
    let q_hi_prime = s - offset;
    let offset_prime = s - q_hi_prime;
    let e = (q_hi - q_hi_prime) + (offset - offset_prime);

    // 3. Apply the accumulated corrections
    let corrected = s + (e + q_lo);

    // 4. Fallback to naive calculation on intermediate overflow or non-finite cases.
    if corrected.is_finite() {
        corrected
    } else {
        (a * (num / den)) + offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mul_div_add_precision() {
        // Celsius to Fahrenheit: F = C * 9/5 + 32
        // 100 C = 212 F
        assert_eq!(f64_mul_div_add(100.0, 9.0, 5.0, 32.0), 212.0);
        // 0 C = 32 F
        assert_eq!(f64_mul_div_add(0.0, 9.0, 5.0, 32.0), 32.0);
        // -40 C = -40 F
        assert_eq!(f64_mul_div_add(-40.0, 9.0, 5.0, 32.0), -40.0);

        // Fahrenheit to Celsius: C = (F - 32) * 5/9
        // This is a linear conversion with an offset, which can be rearranged.
        // We can test other scaling + offset operations.

        // Let's test a case where double rounding would occur in naive arithmetic.
        // A verified case: a = 1.1, num = 1.1, den = 1.2, offset = 0.35.
        // Naive: 1.3583333333333334 (double rounded)
        // FMA: 1.3583333333333336 (exactly correctly rounded)
        let val = 1.1;
        let num = 1.1;
        let den = 1.2;
        let offset = 0.35;
        let naive = (val * (num / den)) + offset;
        let corrected = f64_mul_div_add(val, num, den, offset);
        assert_ne!(naive, corrected);
        assert_eq!(corrected, 1.3583333333333336);
        assert_eq!(naive, 1.3583333333333334);
    }

    #[test]
    fn test_f64_mul_div_add_extreme_cases() {
        // NaN propagation
        assert!(f64_mul_div_add(f64::NAN, 1.0, 2.0, 3.0).is_nan());
        assert!(f64_mul_div_add(1.0, 1.0, 2.0, f64::NAN).is_nan());

        // Infinity propagation
        assert_eq!(f64_mul_div_add(f64::INFINITY, 1.0, 2.0, 3.0), f64::INFINITY);
        assert_eq!(f64_mul_div_add(1.0, 1.0, 2.0, f64::INFINITY), f64::INFINITY);

        // Division by zero
        assert_eq!(f64_mul_div_add(1.0, 1.0, 0.0, 3.0), f64::INFINITY);

        // Intermediate overflow fallback
        let val_large = 1e300;
        let num_large = 1e10;
        let den_large = 1e10;
        let offset = 100.0;
        let corrected = f64_mul_div_add(val_large, num_large, den_large, offset);
        assert!(corrected.is_finite());
        assert_eq!(corrected, 1e300 + 100.0);
    }
}
