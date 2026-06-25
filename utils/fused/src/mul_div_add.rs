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
    if !a.is_finite() || !num.is_finite() || !den.is_finite() || !offset.is_finite() || den == 0.0 {
        return ((a * num) / den) + offset;
    }

    let prod = a * num;

    // 1. Proactive Subnormal Guarding:
    // If den or prod is subnormal, we must use the fallback to prevent precision collapse.
    // We use FMA-fused fallback to prevent catastrophic cancellation from amplifying errors.
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
        // Division-first FMA path (when product overflows but final result is finite)
        let q = a / den;
        let r = (-q).mul_add(den, a);
        let q_tail = r / den;

        let p_head = q * num;
        let p_tail = q.mul_add(num, -p_head);

        let (s, e) = two_sum(p_head, offset);
        let corrected = s + (e + p_tail + q_tail * num);

        if corrected.is_finite() {
            return corrected;
        }
    } else {
        // Product-first FMA path (normal range)
        let t = a.mul_add(num, -prod);
        let q_high = prod / den;
        let r = (-q_high).mul_add(den, prod);
        let q_low = (r + t) / den;
        let (s, e) = two_sum(q_high, offset);
        let corrected = s + (e + q_low);

        if corrected.is_finite() {
            return corrected;
        }
    }

    // 3. Ultimate Fallback (FMA-fused to prevent cancellation error)
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
