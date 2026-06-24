// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fused::*;

mod common;
use common::{ulp_distance_f32, ulp_distance_f64};

#[test]
fn test_ratio_invariants() {
    // Valid ratio
    assert!(RatioF64::new(1.0, 2.0).is_some());
    assert!(RatioF32::new(1.0, 2.0).is_some());

    // Zero denominator is invalid
    assert!(RatioF64::new(1.0, 0.0).is_none());
    assert!(RatioF64::new(1.0, -0.0).is_none());
    assert!(RatioF32::new(1.0, 0.0).is_none());

    // Infinities are invalid
    assert!(RatioF64::new(f64::INFINITY, 1.0).is_none());
    assert!(RatioF64::new(1.0, f64::INFINITY).is_none());
    assert!(RatioF64::new(f64::NEG_INFINITY, 1.0).is_none());
    assert!(RatioF64::new(1.0, f64::NEG_INFINITY).is_none());

    // NaNs are invalid
    assert!(RatioF64::new(f64::NAN, 1.0).is_none());
    assert!(RatioF64::new(1.0, f64::NAN).is_none());

    // Reciprocal
    let r = RatioF64::new(2.0, 3.0).unwrap();
    let rec = r.reciprocal().unwrap();
    assert_eq!(rec.numerator(), 3.0);
    assert_eq!(rec.denominator(), 2.0);

    // Reciprocal of zero numerator is invalid
    let r_zero = RatioF64::new(0.0, 1.0).unwrap();
    assert!(r_zero.reciprocal().is_none());
}

#[test]
fn test_double_rounding_prevention() {
    // Prevent compile-time constant folding
    let x = core::hint::black_box(0.1f64);

    // Naive double rounded: 0.1 * 0.1 / 0.1 -> 0.10000000000000002
    let naive = x * x / x;
    assert_eq!(naive, 0.10000000000000002);
    assert_eq!(naive.to_bits(), 0x3fb999999999999b);

    // Fused exact single rounded -> 0.1
    let exact = f64_mul_div(x, x, x);
    assert_eq!(exact, 0.1);
    assert_eq!(exact.to_bits(), 0x3fb999999999999a);
}

#[test]
fn test_counterintuitive_case() {
    let a = core::hint::black_box(0.3f64);
    let b = core::hint::black_box(3.0f64);
    let c = core::hint::black_box(1.0f64);

    // Naive double rounded: 0.3 * 3 / 1 -> 0.8999999999999999
    let naive = a * b / c;
    assert_eq!(naive, 0.8999999999999999);

    // Fused exact single rounded -> 0.8999999999999999
    let exact = f64_mul_div(a, b, c);
    assert_eq!(exact, 0.8999999999999999);
}

#[test]
fn test_intermediate_overflow() {
    let a = core::hint::black_box(1.5e200_f64);
    let num = core::hint::black_box(2.0e150_f64);
    let den = core::hint::black_box(3.0e100_f64);

    assert!((a * num / den).is_infinite()); // Naive overflows
    let res = f64_mul_div(a, num, den);
    assert!(ulp_distance_f64(res, 1.0e250) <= 1);

    // Also verify f32 intermediate overflow:
    let a_f32 = core::hint::black_box(2.0e30_f32);
    let num_f32 = core::hint::black_box(2.0e20_f32);
    let den_f32 = core::hint::black_box(4.0e25_f32);

    assert!((a_f32 * num_f32 / den_f32).is_infinite()); // Naive overflows
    let res_f32 = f32_mul_div(a_f32, num_f32, den_f32);
    assert!(ulp_distance_f32(res_f32, 1.0e25_f32) <= 1);
}

#[test]
fn test_intermediate_underflow() {
    let a = core::hint::black_box(1.0e-100_f64);
    let b = core::hint::black_box(1.0e-200_f64);
    let c = core::hint::black_box(1.0e-150_f64);

    assert!((a / (b * c)).is_infinite()); // Naive underflows to zero divisor
    let res = f64_div_mul(a, b, c);
    assert!(ulp_distance_f64(res, 1.0e250) <= 1);

    // Also verify f32:
    let a_f32 = core::hint::black_box(1.0e-20_f32);
    let b_f32 = core::hint::black_box(1.0e-25_f32);
    let c_f32 = core::hint::black_box(1.0e-25_f32);

    assert!((a_f32 / (b_f32 * c_f32)).is_infinite()); // Naive underflows
    let res_f32 = f32_div_mul(a_f32, b_f32, c_f32);
    assert!(ulp_distance_f32(res_f32, 1.0e30_f32) <= 1);
}

#[test]
fn test_mul_div_fallback_underflow_prevention() {
    // Underflow bug case identified by Numerical Reviewer:
    // a is subnormal, num is normal, den is normal.
    // Exact: 1.0e-322 (representable subnormal).
    // Old fallback (a/den)*num underflows completely to 0.0.
    // New dynamic retry fallback should succeed!
    let a = core::hint::black_box(1.0e-320_f64);
    let num = core::hint::black_box(1.0e10_f64);
    let den = core::hint::black_box(1.0e12_f64);

    let res = f64_mul_div(a, num, den);
    // The exact result is 1e-322.
    assert!(res > 0.0, "Result should not underflow to 0.0");
    assert!(ulp_distance_f64(res, 1.0e-322) <= 1);

    // Verify f32 version:
    // MIN_POSITIVE f32 is ~1.17e-38, MIN_SUBNORMAL is ~1.4e-45.
    let a_f32 = core::hint::black_box(1.0e-40_f32);
    let num_f32 = core::hint::black_box(1.0e2_f32);
    let den_f32 = core::hint::black_box(1.0e4_f32);

    let res_f32 = f32_mul_div(a_f32, num_f32, den_f32);
    assert!(res_f32 > 0.0, "f32 result should not underflow to 0.0");
    assert!(ulp_distance_f32(res_f32, 1.0e-42_f32) <= 1);
}

#[test]
fn test_div_mul_fallback_overflow_underflow_prevention() {
    // Overflow bug case identified by Numerical Reviewer:
    // a is normal, b is subnormal, c is large.
    // Exact mathematical result for the actual representable float inputs is ~1.0000111e10
    // because b = 1.0e-320 is subnormal and stored imprecisely as ~1.0000111e-320.
    // Old fallback (a/b)/c overflows intermediates to inf, returning inf.
    // New dynamic retry fallback should succeed by dividing by c first!
    let a = core::hint::black_box(1.0e-10_f64);
    let b = core::hint::black_box(1.0e-320_f64);
    let c = core::hint::black_box(1.0e300_f64);

    let res = f64_div_mul(a, b, c);
    assert!(res.is_finite(), "Result should not overflow to infinity");
    assert!(ulp_distance_f64(res, 10000111329.41258) <= 1);

    // Conversely, test the underflow case for div_mul fallback:
    // a is subnormal, b is normal, c is small.
    // Exact: 1.0e-322 (representable).
    // If we statically chose (a/c)/b, then a/c might overflow, but if we chose (a/b)/c,
    // a/b (1.0e-320 / 10^10) would underflow to 0.0.
    // New fallback should dynamically handle both!
    let a2 = core::hint::black_box(1.0e-320_f64);
    let b2 = core::hint::black_box(1.0e10_f64);
    let c2 = core::hint::black_box(1.0e-12_f64);

    let res2 = f64_div_mul(a2, b2, c2);
    assert!(res2 > 0.0, "Result should not underflow to 0.0");
    assert!(ulp_distance_f64(res2, 1.0e-318) <= 3, "Result should be within 3 ULPs of 1.0e-318 (allowing for subnormal precision limits of a2)");
}

#[test]
fn test_ieee754_special_states_mul_div() {
    let nan = f64::NAN;
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;

    assert!(f64_mul_div(nan, 1.0, 1.0).is_nan());
    assert!(f64_mul_div(1.0, nan, 1.0).is_nan());
    assert!(f64_mul_div(1.0, 1.0, nan).is_nan());

    assert_eq!(f64_mul_div(inf, 2.0, 1.0), inf);
    assert_eq!(f64_mul_div(1.0, inf, 2.0), inf);
    assert_eq!(f64_mul_div(1.0, 2.0, inf), 0.0);
    assert_eq!(f64_mul_div(neg_inf, 2.0, 1.0), neg_inf);

    assert!(f64_mul_div(inf, 1.0, inf).is_nan());
    assert!(f64_mul_div(0.0, inf, 1.0).is_nan());

    // Zero denominator fallback (division by zero)
    assert_eq!(f64_mul_div(1.0, 1.0, 0.0), inf);
    assert_eq!(f64_mul_div(-1.0, 1.0, 0.0), neg_inf);
    assert!(f64_mul_div(0.0, 1.0, 0.0).is_nan()); // 0 / 0 is NaN
}

#[test]
fn test_ieee754_special_states_mul_div_add() {
    let nan = f64::NAN;
    let inf = f64::INFINITY;

    assert!(f64_mul_div_add(nan, 1.0, 1.0, 0.0).is_nan());
    assert!(f64_mul_div_add(1.0, 1.0, 1.0, nan).is_nan());
    assert_eq!(f64_mul_div_add(inf, 1.0, 1.0, 5.0), inf);
    assert_eq!(f64_mul_div_add(1.0, 1.0, 1.0, inf), inf);
    assert!(f64_mul_div_add(inf, 1.0, 1.0, f64::NEG_INFINITY).is_nan());
}

#[test]
fn test_ieee754_special_states_div_mul() {
    let nan = f64::NAN;
    let inf = f64::INFINITY;

    assert!(f64_div_mul(nan, 1.0, 1.0).is_nan());
    assert!(f64_div_mul(1.0, nan, 1.0).is_nan());
    assert_eq!(f64_div_mul(inf, 1.0, 1.0), inf);
    assert_eq!(f64_div_mul(1.0, inf, 1.0), 0.0);
}
