// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::CoreFloat;

/// Computes the reciprocal division `a / (b * c)` in double-precision floating-point
/// arithmetic with a single rounding error.
///
/// This function is highly useful when scaling by the reciprocal of a product of two factors.
/// It uses a first-order Taylor series expansion to compensate for the rounding error of
/// the product in the denominator, evaluating the correction term exactly using FMA.
///
/// # Mathematical Model
///
/// 1. The divisor product is represented exactly as a double-word \( D = hi + err \)
///    where \(hi = b \otimes c\) and \(err = \text{fma}(b, c, -hi)\).
/// 2. We expand the quotient using the first-order Taylor expansion:
///    \[ Q = \frac{a}{hi + err} \approx \frac{a}{hi} - \frac{a \cdot err}{hi^2} \]
/// 3. Let \(res = a \oplus hi\) be the primary quotient, and \(rem = a - res \cdot hi\) be its
///    exact division remainder (computed via \(rem = \text{fma}(res, -hi, a)\)).
/// 4. Substituting these, the corrected quotient is:
///    \[ Q \approx res + \frac{rem - res \cdot err}{hi} \]
///    where the numerator \(rem - res \cdot err\) is evaluated exactly in a single FMA operation:
///    \[ \text{corrected} = res + \mathbb{F}\left(\frac{\text{fma}(res, -err, rem)}{hi}\right) \]
///
/// # Robustness and Fallback
///
/// If any intermediate term overflows or results in a non-finite form (such as division by zero
/// or underflow to zero in the denominator), the function detects this using a zero-cost check
/// (`corrected.is_finite()`) and falls back to the naive calculation `a / (b * c)`.
///
/// # Examples
///
/// ```
/// use fused::f64_div_mul;
///
/// let val = 1.0;
/// let b = 10.0;
/// let c = 10.0;
/// // Naive and FMA both yield 0.01.
/// assert_eq!(f64_div_mul(val, b, c), 0.01);
/// ```
#[inline]
pub fn f64_div_mul(a: f64, b: f64, c: f64) -> f64 {
    // 1. Compute the product b * c exactly as (hi, err)
    let hi = b * c;
    let err = b.mul_add(c, -hi);

    // 2. Compute the primary quotient
    let res = a / hi;

    // 3. Compute the exact division remainder
    let rem = res.mul_add(-hi, a);

    // 4. Apply first-order Taylor correction
    let correction = res.mul_add(-err, rem) / hi;
    let corrected = res + correction;

    // 5. Fallback to naive calculation on intermediate overflow or non-finite cases.
    if corrected.is_finite() {
        corrected
    } else {
        a / (b * c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_div_mul_precision() {
        // A case where naive double rounding would occur:
        // a = 1.0, b = 3.0, c = 7.0
        // b * c = 21.0
        // Naive: 1.0 / (3.0 * 7.0) = 0.047619047619047616
        // Let's verify if FMA provides a different, more accurate result.
        // Actually, let's just make sure it behaves correctly.
        let val = 1.0;
        let b = 3.0;
        let c = 7.0;
        let corrected = f64_div_mul(val, b, c);
        let naive = val / (b * c);
        assert_eq!(corrected, naive);
        assert_eq!(corrected, 1.0 / 21.0);
        // Let's test with non-exact product terms where naive arithmetic suffers from double rounding:
        // b = 0.101, c = 0.101.
        // Naive: 98.02960494069207
        // FMA: 98.02960494069208 (exactly correctly rounded)
        let b_ne = 0.101;
        let c_ne = 0.101;
        let naive_ne = val / (b_ne * c_ne);
        let corrected_ne = f64_div_mul(val, b_ne, c_ne);
        assert_ne!(naive_ne, corrected_ne);
        assert_eq!(corrected_ne, 98.02960494069208);
        assert_eq!(naive_ne, 98.02960494069207);
    }

    #[test]
    fn test_f64_div_mul_extreme_cases() {
        // NaN propagation
        assert!(f64_div_mul(f64::NAN, 1.0, 2.0).is_nan());
        assert!(f64_div_mul(1.0, f64::NAN, 2.0).is_nan());

        // Infinity propagation
        assert_eq!(f64_div_mul(f64::INFINITY, 1.0, 2.0), f64::INFINITY);
        assert_eq!(f64_div_mul(1.0, 1.0, f64::INFINITY), 0.0);

        // Division by zero
        assert_eq!(f64_div_mul(1.0, 0.0, 2.0), f64::INFINITY);
        assert_eq!(f64_div_mul(1.0, 2.0, 0.0), f64::INFINITY);
        assert!(f64_div_mul(0.0, 0.0, 2.0).is_nan()); // 0/0 is NaN
    }
}
