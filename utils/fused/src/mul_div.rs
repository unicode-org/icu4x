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
///
/// # Mathematical Limits and Counterintuitive Rounding
///
/// High-precision compensation cannot bypass the fundamental representation limits of the IEEE 754
/// format, nor does it alter standard midpoint tie-breaking rules.
///
/// A classic, counterintuitive example is `f64_mul_div(0.3, 3.0, 1.0)`. Mathematically, this is
/// exactly `0.9`. However, both standard float arithmetic and this compensated algorithm return
/// `0.8999999999999999`.
///
/// This occurs because:
/// 1. **Representation Limit:** `0.3` is not exactly representable in binary. The nearest representable
///    float value is slightly less than `0.3` (specifically, `0.29999999999999998889...`).
/// 2. **Midpoint Tie-Breaking:** The exact mathematical product of this represented float and `3.0` is:
///    `0.8999999999999999666933092612453037872910501956939697265625`.
///    This value lands *exactly* halfway between the two adjacent representable floats `0.8999999999999999`
///    and `0.9`. Under the IEEE 754 round-to-nearest-even rule, the tie is broken by rounding to the
///    even float (significand ending in `0` in binary), which is `0.8999999999999999`.
///
/// Thus, even with infinite intermediate precision, the result must round to `0.8999999999999999`, which
/// is mathematically correct relative to the represented input value `0.3f64`.
#[inline]
pub fn f64_mul_div(a: f64, num: f64, den: f64) -> f64 {
    if !a.is_finite() || !num.is_finite() || !den.is_finite() || den == 0.0 {
        return (a * num) / den;
    }

    let prod = a * num;

    // 1. Proactive Subnormal Guarding:
    // If den or prod is subnormal, we must use the fallback to prevent precision collapse.
    if den.abs() < f64::MIN_POSITIVE || prod.abs() < f64::MIN_POSITIVE {
        let a_div = a / den;
        return if a != 0.0 && (a_div.abs() < f64::MIN_POSITIVE || !a_div.is_finite()) {
            (num / den) * a
        } else {
            a_div * num
        };
    }

    // 2. High-Precision FMA Paths:
    if !prod.is_finite() {
        // Division-first FMA path (when product overflows but final result is finite)
        let q = a / den;
        let r = (-q).mul_add(den, a);
        let corrected = q.mul_add(num, (r * num) / den);
        if corrected.is_finite() {
            return corrected;
        }
    } else {
        // Product-first FMA path (normal range)
        let t = a.mul_add(num, -prod);
        let q = prod / den;
        let r = (-q).mul_add(den, prod);
        let corrected = q + (r + t) / den;
        if corrected.is_finite() {
            return corrected;
        }
    }

    // 3. Ultimate Fallback (magnitude-reordered to prevent overflow/underflow)
    let a_div = a / den;
    if a != 0.0 && (a_div.abs() < f64::MIN_POSITIVE || !a_div.is_finite()) {
        (num / den) * a
    } else {
        a_div * num
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

    #[test]
    fn test_f64_mul_div_counterintuitive_midpoint() {
        // Mathematically, 0.3 * 3.0 / 1.0 = 0.9.
        // However, standard arithmetic yields 0.8999999999999999.
        // Even with high-precision compensation, the result remains 0.8999999999999999
        // due to binary representation limits and IEEE 754 round-to-nearest-even tie-breaking
        // at the exact midpoint.
        let a = 0.3;
        let num = 3.0;
        let den = 1.0;
        let result = f64_mul_div(a, num, den);
        assert_eq!(result, 0.8999999999999999);
    }
}
