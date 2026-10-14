// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod common;

use common::{round_ratio_to_f64, ulp_distance_f64};
use fused::{f64_div_mul, f64_mul_div, f64_mul_div_add};
use num_rational::Ratio;
use num_traits::Zero;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Generates a random f64 across the entire representable range, including subnormals.
fn random_float<R: Rng>(rng: &mut R) -> f64 {
    let exp = rng.random_range(-1074..=1023);
    let sign = if rng.random::<bool>() { 1.0 } else { -1.0 };
    let mantissa: u64 = rng.random::<u64>() & 0x000fffffffffffff;
    let bits = if exp == -1074 {
        mantissa
    } else {
        let biased_exp = (exp + 1023) as u64;
        (biased_exp << 52) | mantissa
    };
    let f = f64::from_bits(bits);
    f * sign
}

/// Generates a random f64 with occasional special values.
fn random_float_with_specials<R: Rng>(rng: &mut R) -> f64 {
    match rng.random_range(0..100) {
        0 => f64::INFINITY,
        1 => f64::NEG_INFINITY,
        2 => f64::NAN,
        3 => 0.0,
        4 => -0.0,
        _ => random_float(rng),
    }
}

// Ground truth using num_rational::Ratio and our bit-accurate rounder
fn ground_truth_mul_div(a: f64, num: f64, den: f64) -> Option<f64> {
    let a_r = Ratio::from_float(a)?;
    let num_r = Ratio::from_float(num)?;
    let den_r = Ratio::from_float(den)?;
    if den_r.is_zero() {
        return None;
    }
    let res_r = (a_r * num_r) / den_r;
    Some(round_ratio_to_f64(&res_r))
}

fn ground_truth_mul_div_add(a: f64, num: f64, den: f64, offset: f64) -> Option<f64> {
    let a_r = Ratio::from_float(a)?;
    let num_r = Ratio::from_float(num)?;
    let den_r = Ratio::from_float(den)?;
    let offset_r = Ratio::from_float(offset)?;
    if den_r.is_zero() {
        return None;
    }
    let res_r = (a_r * num_r) / den_r + offset_r;
    Some(round_ratio_to_f64(&res_r))
}

fn ground_truth_div_mul(a: f64, b: f64, c: f64) -> Option<f64> {
    let a_r = Ratio::from_float(a)?;
    let b_r = Ratio::from_float(b)?;
    let c_r = Ratio::from_float(c)?;
    let denom = b_r * c_r;
    if denom.is_zero() {
        return None;
    }
    let res_r = a_r / denom;
    Some(round_ratio_to_f64(&res_r))
}

#[test]
fn test_differential_mul_div() {
    let mut rng = SmallRng::seed_from_u64(0xdeadbeef);
    let iterations = 200_000;

    for _ in 0..iterations {
        let a = random_float_with_specials(&mut rng);
        let num = random_float_with_specials(&mut rng);
        let den = random_float_with_specials(&mut rng);

        let actual = f64_mul_div(a, num, den);
        let expected_opt = ground_truth_mul_div(a, num, den);
        let standard = (a * num) / den;

        if let Some(expected) = expected_opt {
            if expected.is_finite() {
                let dist = ulp_distance_f64(actual, expected);
                assert!(
                    dist <= 1,
                    "f64_mul_div(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}): actual={}, expected={}, ULP dist={}",
                    a.to_bits(),
                    num.to_bits(),
                    den.to_bits(),
                    actual,
                    expected,
                    dist
                );
            } else {
                assert!(
                    !actual.is_finite(),
                    "f64_mul_div(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}): actual={}, expected={}",
                    a.to_bits(),
                    num.to_bits(),
                    den.to_bits(),
                    actual,
                    expected
                );
            }
        } else {
            assert!(
                ulp_distance_f64(actual, standard) == 0 || (actual.is_nan() && standard.is_nan()),
                "f64_mul_div(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}): actual={}, fallback={}",
                a.to_bits(),
                num.to_bits(),
                den.to_bits(),
                actual,
                standard
            );
        }
    }
}

#[test]
fn test_differential_mul_div_add() {
    let mut rng = SmallRng::seed_from_u64(0xbadc0ffe);
    let iterations = 200_000;

    for _ in 0..iterations {
        let a = random_float_with_specials(&mut rng);
        let num = random_float_with_specials(&mut rng);
        let den = random_float_with_specials(&mut rng);
        let offset = random_float_with_specials(&mut rng);

        let actual = f64_mul_div_add(a, num, den, offset);
        let expected_opt = ground_truth_mul_div_add(a, num, den, offset);
        let standard = ((a * num) / den) + offset;

        if let Some(expected) = expected_opt {
            if expected.is_finite() {
                let dist = ulp_distance_f64(actual, expected);
                assert!(
                    dist <= 1,
                    "f64_mul_div_add(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}, offset_bits: {:#x}): actual={}, expected={}, ULP dist={}",
                    a.to_bits(),
                    num.to_bits(),
                    den.to_bits(),
                    offset.to_bits(),
                    actual,
                    expected,
                    dist
                );
            } else {
                assert!(
                    !actual.is_finite(),
                    "f64_mul_div_add(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}, offset_bits: {:#x}): actual={}, expected={}",
                    a.to_bits(),
                    num.to_bits(),
                    den.to_bits(),
                    offset.to_bits(),
                    actual,
                    expected
                );
            }
        } else {
            assert!(
                ulp_distance_f64(actual, standard) == 0 || (actual.is_nan() && standard.is_nan()),
                "f64_mul_div_add(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}, offset_bits: {:#x}): actual={}, fallback={}",
                a.to_bits(),
                num.to_bits(),
                den.to_bits(),
                offset.to_bits(),
                actual,
                standard
            );
        }
    }
}

#[test]
fn test_differential_div_mul() {
    let mut rng = SmallRng::seed_from_u64(0xfeedface);
    let iterations = 200_000;

    for _ in 0..iterations {
        let a = random_float_with_specials(&mut rng);
        let b = random_float_with_specials(&mut rng);
        let c = random_float_with_specials(&mut rng);

        let actual = f64_div_mul(a, b, c);
        let expected_opt = ground_truth_div_mul(a, b, c);
        let standard = a / (b * c);

        if let Some(expected) = expected_opt {
            if expected.is_finite() {
                let dist = ulp_distance_f64(actual, expected);
                assert!(
                    dist <= 1,
                    "f64_div_mul(a_bits: {:#x}, b_bits: {:#x}, c_bits: {:#x}): actual={}, expected={}, ULP dist={}",
                    a.to_bits(),
                    b.to_bits(),
                    c.to_bits(),
                    actual,
                    expected,
                    dist
                );
            } else {
                assert!(
                    !actual.is_finite(),
                    "f64_div_mul(a_bits: {:#x}, b_bits: {:#x}, c_bits: {:#x}): actual={}, expected={}",
                    a.to_bits(),
                    b.to_bits(),
                    c.to_bits(),
                    actual,
                    expected
                );
            }
        } else {
            assert!(
                ulp_distance_f64(actual, standard) == 0 || (actual.is_nan() && standard.is_nan()),
                "f64_div_mul(a_bits: {:#x}, b_bits: {:#x}, c_bits: {:#x}): actual={}, fallback={}",
                a.to_bits(),
                b.to_bits(),
                c.to_bits(),
                actual,
                standard
            );
        }
    }
}
