// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::CoreFloat;

/// Computes `a / (b * c)` with high precision using FMA, exact remainder, and Taylor expansion.
///
/// This algorithm computes the primary product of the denominator `hi = b * c` and its exact
/// rounding error `err = b * c - hi` using FMA. It then computes the primary quotient
/// `res = a / hi` and its exact division remainder `rem = a - res * hi`. The final compensated
/// result is assembled using a first-order Taylor expansion of the reciprocal division:
/// `corrected = res + (res * -err + rem) / hi`.
///
/// If the compensated result is not finite, it falls back to the standard, uncompensated
/// operation `a / (b * c)`.
#[inline]
pub fn f64_div_mul(a: f64, b: f64, c: f64) -> f64 {
    // Fast path: high-precision compensated algorithm.
    // 1. Primary denominator product: hi = b * c
    let hi = b * c;
    // 2. Exact product rounding error: err = b * c - hi
    let err = b.mul_add(c, -hi);

    // 3. Primary quotient: res = a / hi
    let res = a / hi;
    // 4. Exact division remainder: rem = a - res * hi
    let rem = res.mul_add(-hi, a);

    // 5. Final compensated result using first-order Taylor expansion:
    //    corrected = res + (rem - res * err) / hi
    let corrected = res + (res.mul_add(-err, rem)) / hi;

    // 6. Robustness check and zero-cost fallback
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
    fn test_f64_div_mul_basic() {
        assert_eq!(f64_div_mul(6.0, 3.0, 2.0), 1.0);
        assert_eq!(f64_div_mul(10.0, 4.0, 2.5), 1.0);
        assert_eq!(f64_div_mul(0.0, 5.0, 2.0), 0.0);
    }

    #[test]
    fn test_f64_div_mul_special() {
        assert!(f64_div_mul(1.0, 0.0, 1.0).is_infinite());
        assert!(f64_div_mul(0.0, 0.0, 1.0).is_nan());
        assert!(f64_div_mul(f64::INFINITY, 1.0, 1.0).is_infinite());
        assert!(f64_div_mul(f64::NAN, 1.0, 1.0).is_nan());
        assert!(f64_div_mul(1.0, 1.0, f64::NAN).is_nan());
    }

    #[test]
    fn test_f64_div_mul_precision() {
        let a = 1.0;
        let b = 3.0;
        let c = 1.0 / 3.0;
        assert_eq!(f64_div_mul(a, b, c), 1.0);
    }
}
