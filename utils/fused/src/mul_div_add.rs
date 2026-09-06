// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::CoreFloat;

/// Knuth's 2Sum algorithm.
/// Computes the sum `s = x + y` and the exact rounding error `e` such that `x + y = s + e` exactly.
///
/// This is a fundamental building block of double-word arithmetic, allowing us to add two floats
/// exactly with zero precision loss using 6 standard floating-point operations.
#[inline]
fn two_sum(x: f64, y: f64) -> (f64, f64) {
    let s = x + y;
    let x_prime = s - y;
    let y_prime = s - x_prime;
    let delta_x = x - x_prime;
    let delta_y = y - y_prime;
    let e = delta_x + delta_y;
    (s, e)
}

/// Computes `(a * num) / den + offset` with high precision using FMA, 2Sum, and double-word compensation.
///
/// This algorithm represents the quotient `(a * num) / den` as an exact double-word (two-float) value
/// `(q_high, q_low)`. It then performs an exact addition of `q_high` and `offset` using Knuth's
/// 2Sum algorithm, yielding a new head `s` and error `e`. Finally, the remaining tails
/// `e` and `q_low` are added to `s` in a single rounded step.
///
/// If the compensated result is not finite (due to overflow, underflow, or special inputs like NaN/Inf),
/// it falls back to a mathematically robust, FMA-fused fallback operation.
#[inline]
pub fn f64_mul_div_add(a: f64, num: f64, den: f64, offset: f64) -> f64 {
    // Proactively filter out non-finite inputs, NaNs, and division by zero.
    // In these cases, the standard uncompensated float operation naturally propagates
    // the correct IEEE 754 special values, avoiding FMA indeterminate forms.
    if !a.is_finite() || !num.is_finite() || !den.is_finite() || !offset.is_finite() || den == 0.0 {
        return ((a * num) / den) + offset;
    }

    let prod = a * num;

    // 1. Proactive Subnormal Guarding:
    // If den or prod is subnormal, we must immediately route to the fallback.
    // - Mathematical necessity: FMA error-tracking and division remainder compensation suffer from
    //   underflow when inputs are subnormal, leading to precision collapse.
    // - Microarchitectural necessity: Checking this proactively avoids FPU microcode traps (subnormal assists)
    //   that take 100-300 cycles on x86_64, and ensures compatibility with DAZ/FTZ environments.
    // - We use an FMA-fused fallback to prevent catastrophic cancellation from amplifying errors.
    if den.abs() < f64::MIN_POSITIVE || prod.abs() < f64::MIN_POSITIVE {
        let a_div = a / den;
        return if a != 0.0 && (a_div.abs() < f64::MIN_POSITIVE || !a_div.is_finite()) {
            let num_div = num / den;
            num_div.mul_add(a, offset)
        } else {
            a_div.mul_add(num, offset)
        };
    }

    // 2. High-Precision FMA Paths:
    if !prod.is_finite() {
        // --- DIVISION-FIRST FMA PATH ---
        // Triggered when the intermediate product overflows the double-precision range,
        // but the final scaled quotient + offset is a representable, finite normal float.
        // We divide first to scale the terms down, extracting the exact division remainder.
        
        // Compute primary quotient: q = a / den
        let q = a / den;
        // Extract exact division remainder via FMA: r = a - q * den (Jeannerod's Theorem)
        let r = (-q).mul_add(den, a);
        let q_tail = r / den;

        // Decompose the scaled product: q * num = p_head + p_tail (Dekker's Theorem)
        let p_head = q * num;
        let p_tail = q.mul_add(num, -p_head);

        // Perform exact addition of the head and the offset via Knuth's 2Sum (Theorem 3)
        let (s, e) = two_sum(p_head, offset);
        // Assemble the compensated result, accumulating all tail terms: corrected = s + (e + p_tail + q_tail * num)
        let corrected = s + (e + p_tail + q_tail * num);

        if corrected.is_finite() {
            return corrected;
        }
    } else {
        // --- PRODUCT-FIRST FMA PATH ---
        // The standard, ultra-fast path for normal-range operations.
        
        // Extract exact product rounding error via FMA: t = a * num - prod (Dekker's Theorem)
        let t = a.mul_add(num, -prod);
        // Compute primary quotient head: q_high = prod / den
        let q_high = prod / den;
        // Extract exact division remainder via FMA: r = prod - q_high * den (Jeannerod's Theorem)
        let r = (-q_high).mul_add(den, prod);
        // Compute the division tail: q_low = (r + t) / den
        let q_low = (r + t) / den;
        // Perform exact addition of the quotient head and the offset via Knuth's 2Sum (Theorem 3)
        let (s, e) = two_sum(q_high, offset);
        // Assemble the compensated result: corrected = s + (e + q_low)
        let corrected = s + (e + q_low);

        if corrected.is_finite() {
            return corrected;
        }
    }

    // 3. Ultimate Fallback (FMA-fused Smart Simple Fallback):
    // In extreme boundary ranges, we use a magnitude-reordered fallback. We use FMA (`mul_add`)
    // to combine the scaling and offset addition into a single rounded operation, preventing
    // intermediate overflow/underflow and catastrophic cancellation errors.
    let a_div = a / den;
    if a != 0.0 && (a_div.abs() < f64::MIN_POSITIVE || !a_div.is_finite()) {
        let num_div = num / den;
        num_div.mul_add(a, offset)
    } else {
        a_div.mul_add(num, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_mul_div_add_basic() {
        assert_eq!(f64_mul_div_add(2.0, 3.0, 4.0, 0.5), 2.0);
        assert_eq!(f64_mul_div_add(10.0, 5.0, 2.0, -5.0), 20.0);
        assert_eq!(f64_mul_div_add(0.0, 5.0, 2.0, 7.0), 7.0);
    }

    #[test]
    fn test_f64_mul_div_add_special() {
        assert!(f64_mul_div_add(1.0, 1.0, 0.0, 1.0).is_infinite());
        assert!(f64_mul_div_add(0.0, 0.0, 0.0, 1.0).is_nan());
        assert!(f64_mul_div_add(f64::INFINITY, 1.0, 1.0, 1.0).is_infinite());
        assert!(f64_mul_div_add(f64::NAN, 1.0, 1.0, 1.0).is_nan());
        assert!(f64_mul_div_add(1.0, 1.0, 1.0, f64::NAN).is_nan());
    }

    #[test]
    fn test_f64_mul_div_add_precision() {
        // A case where the offset addition requires high precision to avoid rounding error.
        let a = 1.0;
        let num = 1.0;
        let den = 3.0; // 1/3
        let offset = 2.0 / 3.0; // 2/3
        // 1/3 + 2/3 should be exactly 1.0
        assert_eq!(f64_mul_div_add(a, num, den, offset), 1.0);
    }
}
