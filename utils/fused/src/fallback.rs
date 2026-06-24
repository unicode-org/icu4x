// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Consolidated mathematically stable fallback scaling algorithms.
//!
//! Provides smart proactive conditioning and reactive dynamic retry strategies
//! to prevent both intermediate overflow and denormalization precision loss.

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

/// Fallback for `(a * num) / den` in 64-bit float.
///
/// Proactively selects the division path that keeps intermediate values in the
/// normal range, and reactively retries the alternative if the result degenerates.
#[inline]
pub fn fallback_mul_div_f64(a: f64, num: f64, den: f64) -> f64 {
    if a == 0.0 || num == 0.0 {
        return (a * num) / den;
    }
    let abs_a = a.abs();
    let abs_num = num.abs();

    let a_div_den = a / den;
    let num_div_den = num / den;

    // Check if intermediate divisions are normal (not subnormal, not infinite, not zero)
    let a_div_ok =
        a_div_den.is_finite() && a_div_den != 0.0 && a_div_den.abs() >= f64::MIN_POSITIVE;
    let num_div_ok =
        num_div_den.is_finite() && num_div_den != 0.0 && num_div_den.abs() >= f64::MIN_POSITIVE;

    let res = if a_div_ok && !num_div_ok {
        a_div_den * num
    } else if num_div_ok && !a_div_ok {
        num_div_den * a
    } else {
        // If both are normal or both are bad, use magnitude heuristic to prevent overflow
        if abs_a < abs_num {
            a_div_den * num
        } else {
            num_div_den * a
        }
    };

    if res.is_finite() && res != 0.0 {
        res
    } else {
        // Reactive safety net: retry with the alternative association
        if abs_a < abs_num {
            (num / den) * a
        } else {
            (a / den) * num
        }
    }
}

/// Fallback for `(a * num) / den` in 32-bit float.
#[inline]
pub fn fallback_mul_div_f32(a: f32, num: f32, den: f32) -> f32 {
    if a == 0.0 || num == 0.0 {
        return (a * num) / den;
    }
    let abs_a = a.abs();
    let abs_num = num.abs();

    let a_div_den = a / den;
    let num_div_den = num / den;

    let a_div_ok =
        a_div_den.is_finite() && a_div_den != 0.0 && a_div_den.abs() >= f32::MIN_POSITIVE;
    let num_div_ok =
        num_div_den.is_finite() && num_div_den != 0.0 && num_div_den.abs() >= f32::MIN_POSITIVE;

    let res = if a_div_ok && !num_div_ok {
        a_div_den * num
    } else if num_div_ok && !a_div_ok {
        num_div_den * a
    } else {
        if abs_a < abs_num {
            a_div_den * num
        } else {
            num_div_den * a
        }
    };

    if res.is_finite() && res != 0.0 {
        res
    } else {
        if abs_a < abs_num {
            (num / den) * a
        } else {
            (a / den) * num
        }
    }
}

/// Fallback for `a / (b * c)` in 64-bit float.
///
/// Proactively selects the divisor that keeps intermediate division normal,
/// and reactively retries the alternative if the result degenerates.
#[inline]
pub fn fallback_div_mul_f64(a: f64, b: f64, c: f64) -> f64 {
    if a == 0.0 {
        return 0.0;
    }
    let abs_b = b.abs();
    let abs_c = c.abs();

    let a_div_b = a / b;
    let a_div_c = a / c;

    let b_ok = a_div_b.is_finite() && a_div_b != 0.0 && a_div_b.abs() >= f64::MIN_POSITIVE;
    let c_ok = a_div_c.is_finite() && a_div_c != 0.0 && a_div_c.abs() >= f64::MIN_POSITIVE;

    let res = if b_ok && !c_ok {
        a_div_b / c
    } else if c_ok && !b_ok {
        a_div_c / b
    } else {
        // If both are normal or both are bad, divide by the larger divisor first to prevent overflow
        if abs_b > abs_c {
            a_div_b / c
        } else {
            a_div_c / b
        }
    };

    if res.is_finite() && res != 0.0 {
        res
    } else {
        if abs_b > abs_c {
            (a / c) / b
        } else {
            (a / b) / c
        }
    }
}

/// Fallback for `a / (b * c)` in 32-bit float.
#[inline]
pub fn fallback_div_mul_f32(a: f32, b: f32, c: f32) -> f32 {
    if a == 0.0 {
        return 0.0;
    }
    let abs_b = b.abs();
    let abs_c = c.abs();

    let a_div_b = a / b;
    let a_div_c = a / c;

    let b_ok = a_div_b.is_finite() && a_div_b != 0.0 && a_div_b.abs() >= f32::MIN_POSITIVE;
    let c_ok = a_div_c.is_finite() && a_div_c != 0.0 && a_div_c.abs() >= f32::MIN_POSITIVE;

    let res = if b_ok && !c_ok {
        a_div_b / c
    } else if c_ok && !b_ok {
        a_div_c / b
    } else {
        if abs_b > abs_c {
            a_div_b / c
        } else {
            a_div_c / b
        }
    };

    if res.is_finite() && res != 0.0 {
        res
    } else {
        if abs_b > abs_c {
            (a / c) / b
        } else {
            (a / b) / c
        }
    }
}
