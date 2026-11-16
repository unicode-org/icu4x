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
/// If the compensated result is not finite, it falls back to a mathematically robust,
/// magnitude-reordered fallback operation.
#[inline]
pub fn f64_div_mul(a: f64, b: f64, c: f64) -> f64 {
    // Proactively filter out non-finite inputs, NaNs, and division by zero.
    // In these cases, the standard uncompensated float operation naturally propagates
    // the correct IEEE 754 special values, avoiding FMA indeterminate forms.
    if !a.is_finite() || !b.is_finite() || !c.is_finite() || b == 0.0 || c == 0.0 {
        return a / (b * c);
    }

    let hi = b * c;

    // 1. Proactive Subnormal Guarding:
    // We must check if either the denominator product `hi` is zero, non-finite, or subnormal.
    // - Mathematical necessity: If `hi` is subnormal, the FMA error-tracking term
    //   `b.mul_add(c, -hi)` can underflow to zero. When divided by a subnormal denominator,
    //   this lost error term is magnified by up to 2^1074, causing a catastrophic precision
    //   collapse of up to 2^50 ULPs.
    // - Microarchitectural necessity: Checking this proactively avoids FPU microcode traps
    //   (subnormal assists) that take 100-300 cycles on x86_64, and ensures compatibility with DAZ/FTZ.
    // - Logical flow difference: Unlike `mul_div`, denominator product overflow/underflow behaves
    //   monotonically with the final quotient (i.e. if the denominator product overflows, the final
    //   quotient must underflow to zero, and vice versa). Thus, we do not need intermediate overflow
    //   FMA branching, and can use a clean binary split.
    let use_fallback = hi == 0.0 || !hi.is_finite() || hi.abs() < f64::MIN_POSITIVE;

    if !use_fallback {
        // Fast path: high-precision compensated algorithm.
        
        // 2. Exact product rounding error: err = b * c - hi (Dekker's Theorem)
        let err = b.mul_add(c, -hi);

        // 3. Primary quotient: res = a / hi
        let res = a / hi;
        // 4. Exact division remainder: rem = a - res * hi (Jeannerod's Theorem)
        let rem = res.mul_add(-hi, a);

        // 5. Final compensated result using first-order Taylor expansion:
        //    corrected = res + (rem - res * err) / hi
        // This evaluates the Taylor correction term exactly in a single FMA and division.
        let corrected = res + (res.mul_add(-err, rem)) / hi;

        // 6. Robustness check and zero-cost fallback
        if corrected.is_finite() {
            return corrected;
        }
    }

    // 2. Smart Simple Fallback (Magnitude-Reordered Reciprocal Division):
    // When the subnormal check triggers or when the FMA math fails (non-finite result),
    // we use a magnitude-reordered fallback.
    // By dynamically choosing between `(a / b) / c` and `(a / c) / b` based on the magnitude
    // of `a / b`, we scale the intermediate terms down, completely avoiding intermediate overflow
    // or underflow of the product `b * c` in extreme ranges.
    let a_div = a / b;
    if a != 0.0 && (a_div.abs() < f64::MIN_POSITIVE || !a_div.is_finite()) {
        (a / c) / b
    } else {
        a_div / c
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
