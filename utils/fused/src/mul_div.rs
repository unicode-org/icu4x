// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::CoreFloat;

/// Computes `(a * num) / den` with high precision using FMA and Dekker/Jeannerod compensation.
///
/// This algorithm extracts the exact error of the multiplication `a * num` and the exact
/// remainder of the division of the product by `den`, and uses them to compute a compensated
/// result.
///
/// If the compensated result is not finite (due to overflow, underflow, or special inputs like NaN/Inf),
/// it falls back to the standard, uncompensated operation `(a * num) / den` to ensure zero-cost
/// robustness.
#[inline]
pub fn f64_mul_div(a: f64, num: f64, den: f64) -> f64 {
    // Fast path: high-precision compensated algorithm.
    // 1. Exact product decomposition: a * num = p + t
    let p = a * num;
    let t = a.mul_add(num, -p);

    // 2. Exact division remainder: p = q * den + r
    let q = p / den;
    let r = (-q).mul_add(den, p);

    // 3. Compensation: corrected = q + (r + t) / den
    let corrected = q + (r + t) / den;

    // 4. Robustness check and zero-cost fallback
    if corrected.is_finite() {
        corrected
    } else {
        (a * num) / den
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mul_div_basic() {
        assert_eq!(f64_mul_div(2.0, 3.0, 4.0), 1.5);
        assert_eq!(f64_mul_div(10.0, 5.0, 2.0), 25.0);
        assert_eq!(f64_mul_div(0.0, 5.0, 2.0), 0.0);
    }

    #[test]
    fn test_f64_mul_div_special() {
        assert!(f64_mul_div(1.0, 1.0, 0.0).is_infinite());
        assert!(f64_mul_div(0.0, 0.0, 0.0).is_nan());
        assert!(f64_mul_div(f64::INFINITY, 1.0, 1.0).is_infinite());
        assert!(f64_mul_div(f64::NAN, 1.0, 1.0).is_nan());
        assert!(f64_mul_div(1.0, f64::NAN, 1.0).is_nan());
        assert!(f64_mul_div(1.0, 1.0, f64::NAN).is_nan());
    }

    #[test]
    fn test_f64_mul_div_precision() {
        // A case where standard (a * num) / den might round differently than compensated.
        // We will verify this more thoroughly in differential tests, but here is a simple check.
        let a = 1.2345678901234567e300;
        let num = 1.0000000000000002;
        let den = 1.2345678901234567e300;
        // Exact is 1.0000000000000002
        assert_eq!(f64_mul_div(a, num, den), 1.0000000000000002);
    }
}
