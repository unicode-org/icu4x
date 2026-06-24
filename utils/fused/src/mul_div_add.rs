// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)]
use core_maths::CoreFloat;

/// Knuth's 2Sum algorithm.
/// Computes the sum `s = x + y` and the exact rounding error `e` such that `x + y = s + e` exactly.
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

/// Computes `(a * num) / den + offset` with high precision.
///
/// This algorithm represents the quotient `(a * num) / den` as a double-word (two-float) value
/// `(q_high, q_low)`. It then performs an exact addition of `q_high` and `offset` using Knuth's
/// 2Sum algorithm, yielding a new head `s` and error `e`. Finally, the remaining tails
/// `e` and `q_low` are added to `s`.
///
/// If the compensated result is not finite, it falls back to the standard, uncompensated
/// operation `((a * num) / den) + offset`.
#[inline]
pub fn f64_mul_div_add(a: f64, num: f64, den: f64, offset: f64) -> f64 {
    // Fast path: high-precision compensated algorithm.
    // 1. Exact product decomposition: a * num = p + t
    let p = a * num;
    let t = a.mul_add(num, -p);

    // 2. Exact division remainder: p = q_high * den + r
    let q_high = p / den;
    let r = (-q_high).mul_add(den, p);

    // 3. Quotient low part: q_low = (r + t) / den
    let q_low = (r + t) / den;

    // 4. Exact addition of q_high and offset: q_high + offset = s + e
    let (s, e) = two_sum(q_high, offset);

    // 5. Final compensation
    let corrected = s + (e + q_low);

    // 6. Robustness check and zero-cost fallback
    if corrected.is_finite() {
        corrected
    } else {
        ((a * num) / den) + offset
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
