// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fused::*;
use num_bigint::BigInt;
use num_rational::Ratio;
use proptest::prelude::*;

mod common;
use common::{round_ratio_to_f32, round_ratio_to_f64, ulp_distance_f32, ulp_distance_f64};

// --- Helpers to convert floats to exact rational representation ---

fn float_to_ratio(f: f64) -> Option<Ratio<BigInt>> {
    if !f.is_finite() {
        return None;
    }
    let bits = f.to_bits();
    let sign = if (bits >> 63) == 1 { -1 } else { 1 };
    let exp = ((bits >> 52) & 0x7FF) as i32;
    let frac = bits & 0xF_FFFF_FFFF_FFFF;

    let (m, e) = if exp == 0 {
        (frac, -1022 - 52)
    } else {
        (frac | (1 << 52), exp - 1023 - 52)
    };

    let big_m = BigInt::from(m) * sign;
    let ratio = if e >= 0 {
        Ratio::new(big_m << (e as usize), BigInt::from(1))
    } else {
        Ratio::new(big_m, BigInt::from(1) << ((-e) as usize))
    };
    Some(ratio)
}

fn float32_to_ratio(f: f32) -> Option<Ratio<BigInt>> {
    if !f.is_finite() {
        return None;
    }
    let bits = f.to_bits();
    let sign = if (bits >> 31) == 1 { -1 } else { 1 };
    let exp = ((bits >> 23) & 0xFF) as i32;
    let frac = bits & 0x7F_FFFF;

    let (m, e) = if exp == 0 {
        (frac, -126 - 23)
    } else {
        (frac | (1 << 23), exp - 127 - 23)
    };

    let big_m = BigInt::from(m) * sign;
    let ratio = if e >= 0 {
        Ratio::new(big_m << (e as usize), BigInt::from(1))
    } else {
        Ratio::new(big_m, BigInt::from(1) << ((-e) as usize))
    };
    Some(ratio)
}

// --- Uniform Exponent Strategy generators ---

fn any_finite_f64_uniform_exponent() -> impl Strategy<Value = f64> {
    (0..=1u64, 0..=2046u64, 0..(1u64 << 52)).prop_map(|(sign, exponent, fraction)| {
        f64::from_bits((sign << 63) | (exponent << 52) | fraction)
    })
}

fn any_finite_f32_uniform_exponent() -> impl Strategy<Value = f32> {
    (0..=1u32, 0..=254u32, 0..(1u32 << 23)).prop_map(|(sign, exponent, fraction)| {
        f32::from_bits((sign << 31) | (exponent << 23) | fraction)
    })
}

// --- Property test suites ---

proptest! {
    #![proptest_config(ProptestConfig::with_cases(5000))]

    #[test]
    fn test_prop_mul_div_f64(
        a in any_finite_f64_uniform_exponent(),
        num in any_finite_f64_uniform_exponent(),
        den in any_finite_f64_uniform_exponent()
    ) {
        if den != 0.0 && a.is_finite() && num.is_finite() && den.is_finite() {
            let res = f64_mul_div(a, num, den);

            let a_ratio = float_to_ratio(a).unwrap();
            let num_ratio = float_to_ratio(num).unwrap();
            let den_ratio = float_to_ratio(den).unwrap();
            let exact_ratio = a_ratio * num_ratio / den_ratio;
            let expected = round_ratio_to_f64(&exact_ratio);

            if expected.is_finite() {
                let dist = ulp_distance_f64(res, expected);
                let prod = a * num;
                let is_fallback = (den < f64::MIN_POSITIVE && den > -f64::MIN_POSITIVE)
                    || (prod < f64::MIN_POSITIVE && prod > -f64::MIN_POSITIVE);
                let max_dist = if is_fallback { 2 } else { 1 };
                prop_assert!(dist <= max_dist, "f64_mul_div({}, {}, {}) = {}, expected = {} (ulp distance = {}, max allowed = {}, fallback = {})", a, num, den, res, expected, dist, max_dist, is_fallback);
            }
        }
    }

    #[test]
    fn test_prop_mul_div_f32(
        a in any_finite_f32_uniform_exponent(),
        num in any_finite_f32_uniform_exponent(),
        den in any_finite_f32_uniform_exponent()
    ) {
        if den != 0.0 && a.is_finite() && num.is_finite() && den.is_finite() {
            let res = f32_mul_div(a, num, den);

            let a_ratio = float32_to_ratio(a).unwrap();
            let num_ratio = float32_to_ratio(num).unwrap();
            let den_ratio = float32_to_ratio(den).unwrap();
            let exact_ratio = a_ratio * num_ratio / den_ratio;
            let expected = round_ratio_to_f32(&exact_ratio);

            if expected.is_finite() {
                let dist = ulp_distance_f32(res, expected);
                let prod = a * num;
                let is_fallback = (den < f32::MIN_POSITIVE && den > -f32::MIN_POSITIVE)
                    || (prod < f32::MIN_POSITIVE && prod > -f32::MIN_POSITIVE);
                let max_dist = if is_fallback { 2 } else { 1 };
                prop_assert!(dist <= max_dist, "f32_mul_div({}, {}, {}) = {}, expected = {} (ulp distance = {}, max allowed = {}, fallback = {})", a, num, den, res, expected, dist, max_dist, is_fallback);
            }
        }
    }

    #[test]
    fn test_prop_mul_div_add_f64(
        a in any_finite_f64_uniform_exponent(),
        num in any_finite_f64_uniform_exponent(),
        den in any_finite_f64_uniform_exponent(),
        offset in any_finite_f64_uniform_exponent()
    ) {
        if den != 0.0 && a.is_finite() && num.is_finite() && den.is_finite() && offset.is_finite() {
            let res = f64_mul_div_add(a, num, den, offset);

            let a_ratio = float_to_ratio(a).unwrap();
            let num_ratio = float_to_ratio(num).unwrap();
            let den_ratio = float_to_ratio(den).unwrap();
            let offset_ratio = float_to_ratio(offset).unwrap();
            let exact_ratio = a_ratio * num_ratio / den_ratio + offset_ratio;
            let expected = round_ratio_to_f64(&exact_ratio);

            if expected.is_finite() {
                let dist = ulp_distance_f64(res, expected);
                // Allow up to 32 ULPs to accommodate relative error magnification during catastrophic cancellation
                let max_dist = 32;
                prop_assert!(dist <= max_dist, "f64_mul_div_add({}, {}, {}, {}) = {}, expected = {} (ulp distance = {}, max allowed = {})", a, num, den, offset, res, expected, dist, max_dist);
            }
        }
    }

    #[test]
    fn test_prop_mul_div_add_f32(
        a in any_finite_f32_uniform_exponent(),
        num in any_finite_f32_uniform_exponent(),
        den in any_finite_f32_uniform_exponent(),
        offset in any_finite_f32_uniform_exponent()
    ) {
        if den != 0.0 && a.is_finite() && num.is_finite() && den.is_finite() && offset.is_finite() {
            let res = f32_mul_div_add(a, num, den, offset);

            let a_ratio = float32_to_ratio(a).unwrap();
            let num_ratio = float32_to_ratio(num).unwrap();
            let den_ratio = float32_to_ratio(den).unwrap();
            let offset_ratio = float32_to_ratio(offset).unwrap();
            let exact_ratio = a_ratio * num_ratio / den_ratio + offset_ratio;
            let expected = round_ratio_to_f32(&exact_ratio);

            if expected.is_finite() {
                let dist = ulp_distance_f32(res, expected);
                // Allow up to 32 ULPs to accommodate relative error magnification during catastrophic cancellation
                let max_dist = 32;
                prop_assert!(dist <= max_dist, "f32_mul_div_add({}, {}, {}, {}) = {}, expected = {} (ulp distance = {}, max allowed = {})", a, num, den, offset, res, expected, dist, max_dist);
            }
        }
    }

    #[test]
    fn test_prop_div_mul_f64(
        a in any_finite_f64_uniform_exponent(),
        b in any_finite_f64_uniform_exponent(),
        c in any_finite_f64_uniform_exponent()
    ) {
        if b != 0.0 && c != 0.0 && a.is_finite() && b.is_finite() && c.is_finite() {
            let res = f64_div_mul(a, b, c);

            let a_ratio = float_to_ratio(a).unwrap();
            let b_ratio = float_to_ratio(b).unwrap();
            let c_ratio = float_to_ratio(c).unwrap();
            let exact_ratio = a_ratio / (b_ratio * c_ratio);
            let expected = round_ratio_to_f64(&exact_ratio);

            if expected.is_finite() {
                let dist = ulp_distance_f64(res, expected);
                let hi = b * c;
                let is_fallback = hi == 0.0 || !hi.is_finite() || (hi < f64::MIN_POSITIVE && hi > -f64::MIN_POSITIVE);
                let max_dist = if is_fallback { 2 } else { 1 };
                prop_assert!(dist <= max_dist, "f64_div_mul({}, {}, {}) = {}, expected = {} (ulp distance = {}, max allowed = {}, fallback = {})", a, b, c, res, expected, dist, max_dist, is_fallback);
            }
        }
    }

    #[test]
    fn test_prop_div_mul_f32(
        a in any_finite_f32_uniform_exponent(),
        b in any_finite_f32_uniform_exponent(),
        c in any_finite_f32_uniform_exponent()
    ) {
        if b != 0.0 && c != 0.0 && a.is_finite() && b.is_finite() && c.is_finite() {
            let res = f32_div_mul(a, b, c);

            let a_ratio = float32_to_ratio(a).unwrap();
            let b_ratio = float32_to_ratio(b).unwrap();
            let c_ratio = float32_to_ratio(c).unwrap();
            let exact_ratio = a_ratio / (b_ratio * c_ratio);
            let expected = round_ratio_to_f32(&exact_ratio);

            if expected.is_finite() {
                let dist = ulp_distance_f32(res, expected);
                let hi = b * c;
                let is_fallback = hi == 0.0 || !hi.is_finite() || (hi < f32::MIN_POSITIVE && hi > -f32::MIN_POSITIVE);
                let max_dist = if is_fallback { 2 } else { 1 };
                prop_assert!(dist <= max_dist, "f32_div_mul({}, {}, {}) = {}, expected = {} (ulp distance = {}, max allowed = {}, fallback = {})", a, b, c, res, expected, dist, max_dist, is_fallback);
            }
        }
    }
}
