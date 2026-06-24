// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Fused division-multiplication algorithms (reciprocal division).

use crate::fallback::{fallback_div_mul_f32, fallback_div_mul_f64};

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

/// Computes `a / (b * c)` with a single rounding, using FMA to track error.
///
/// If `b * c` underflows to `0.0` or a subnormal, or overflows, or if the result
/// is not finite, falls back to `(a / b) / c` to avoid precision loss or division by zero.
#[inline]
pub fn f64_div_mul(a: f64, b: f64, c: f64) -> f64 {
    let hi = b * c;
    // Fall back if hi is zero, infinite, NaN, or subnormal (to avoid FMA precision loss)
    if hi == 0.0 || !hi.is_finite() || hi.abs() < f64::MIN_POSITIVE {
        return fallback_div_mul_f64(a, b, c);
    }
    let err = b.mul_add(c, -hi);
    let res = a / hi;
    let rem = res.mul_add(-hi, a);
    let corrected = res + res.mul_add(-err, rem) / hi;
    if corrected.is_finite() {
        corrected
    } else {
        fallback_div_mul_f64(a, b, c)
    }
}

/// Computes `a / (b * c)` with a single rounding, using FMA to track error.
///
/// If `b * c` underflows to `0.0` or a subnormal, or overflows, or if the result
/// is not finite, falls back to `(a / b) / c` to avoid precision loss or division by zero.
#[inline]
pub fn f32_div_mul(a: f32, b: f32, c: f32) -> f32 {
    let hi = b * c;
    // Fall back if hi is zero, infinite, NaN, or subnormal (to avoid FMA precision loss)
    if hi == 0.0 || !hi.is_finite() || hi.abs() < f32::MIN_POSITIVE {
        return fallback_div_mul_f32(a, b, c);
    }
    let err = b.mul_add(c, -hi);
    let res = a / hi;
    let rem = res.mul_add(-hi, a);
    let corrected = res + res.mul_add(-err, rem) / hi;
    if corrected.is_finite() {
        corrected
    } else {
        fallback_div_mul_f32(a, b, c)
    }
}
