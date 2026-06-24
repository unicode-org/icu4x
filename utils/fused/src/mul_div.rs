// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Fused multiply-divide algorithms.

use crate::fallback::{fallback_mul_div_f32, fallback_mul_div_f64};

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

/// Computes `(a * num) / den` with a single rounding, using FMA to track error.
///
/// If `den` or the intermediate product `a * num` is subnormal, or if the result
/// is not finite, falls back to a highly stable factored scaling.
#[inline]
pub fn f64_mul_div(a: f64, num: f64, den: f64) -> f64 {
    if a == 0.0 || num == 0.0 {
        return (a * num) / den;
    }
    let prod = a * num;
    // Fall back if den is subnormal or if the intermediate product is subnormal
    // to avoid numerical instability and precision loss in subnormal FMA arithmetic.
    if den.abs() < f64::MIN_POSITIVE || prod.abs() < f64::MIN_POSITIVE {
        return fallback_mul_div_f64(a, num, den);
    }
    let double_rounded = prod / den;
    let err_mul = a.mul_add(num, -prod);
    let err_div = double_rounded.mul_add(-den, prod);
    let corrected = double_rounded + (err_div + err_mul) / den;
    if corrected.is_finite() {
        corrected
    } else {
        fallback_mul_div_f64(a, num, den)
    }
}

/// Computes `(a * num) / den` with a single rounding, using FMA to track error.
///
/// If `den` or the intermediate product `a * num` is subnormal, or if the result
/// is not finite, falls back to a highly stable factored scaling.
#[inline]
pub fn f32_mul_div(a: f32, num: f32, den: f32) -> f32 {
    if a == 0.0 || num == 0.0 {
        return (a * num) / den;
    }
    let prod = a * num;
    // Fall back if den is subnormal or if the intermediate product is subnormal
    // to avoid numerical instability and precision loss in subnormal FMA arithmetic.
    if den.abs() < f32::MIN_POSITIVE || prod.abs() < f32::MIN_POSITIVE {
        return fallback_mul_div_f32(a, num, den);
    }
    let double_rounded = prod / den;
    let err_mul = a.mul_add(num, -prod);
    let err_div = double_rounded.mul_add(-den, prod);
    let corrected = double_rounded + (err_div + err_mul) / den;
    if corrected.is_finite() {
        corrected
    } else {
        fallback_mul_div_f32(a, num, den)
    }
}
