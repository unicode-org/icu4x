// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fused::{f64_div_mul, f64_mul_div, f64_mul_div_add};
use num_rational::Ratio;
use num_traits::{ToPrimitive, Zero};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Computes the ULP (Unit in the Last Place) distance between two f64 values.
/// Returns 0 if they are equal, or if both are NaN.
/// Returns u64::MAX if they have different signs (and are not both zero) or if one is infinite and the other is not.
fn ulp_distance(actual: f64, expected: f64) -> u64 {
    if actual.is_nan() && expected.is_nan() {
        return 0;
    }
    if actual == expected {
        return 0;
    }
    if actual.is_infinite() || expected.is_infinite() {
        if actual == expected {
            return 0;
        } else {
            return u64::MAX;
        }
    }

    // Treat -0.0 and 0.0 as equal
    if actual == 0.0 && expected == 0.0 {
        return 0;
    }

    if actual.signum() != expected.signum() {
        return u64::MAX;
    }

    let actual_bits = actual.to_bits();
    let expected_bits = expected.to_bits();

    let actual_int = (actual_bits & 0x7fffffffffffffff) as i64;
    let expected_int = (expected_bits & 0x7fffffffffffffff) as i64;

    (actual_int - expected_int).abs() as u64
}

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

// Ground truth using num_rational::Ratio
fn ground_truth_mul_div(a: f64, num: f64, den: f64) -> Option<f64> {
    let a_r = Ratio::from_float(a)?;
    let num_r = Ratio::from_float(num)?;
    let den_r = Ratio::from_float(den)?;
    if den_r.is_zero() {
        return None;
    }
    let res_r = (a_r * num_r) / den_r;
    res_r.to_f64()
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
    res_r.to_f64()
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
    res_r.to_f64()
}

#[inline]
fn is_subnormal_or_zero(f: f64) -> bool {
    f == 0.0 || f.abs() < f64::MIN_POSITIVE
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
                let dist = ulp_distance(actual, expected);

                let p = a * num;
                let intermediate_underflow = is_subnormal_or_zero(p);
                let intermediate_overflow = !standard.is_finite();

                // If the final result is subnormal, OR if the intermediate product underflowed to zero,
                // we have reduced precision (double rounding) and allow up to 4 ULPs.
                let is_subnormal = expected.abs() < f64::MIN_POSITIVE;
                let max_dist = if is_subnormal || intermediate_underflow {
                    4
                } else {
                    1
                };

                if dist > max_dist {
                    if intermediate_underflow || intermediate_overflow {
                        let fallback_dist = ulp_distance(actual, standard);
                        assert!(
                            fallback_dist <= 2 || (actual.is_nan() && standard.is_nan()),
                            "f64_mul_div(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}): actual={}, standard={} (expected={}, ULP dist={}, fallback ULP dist={})",
                            a.to_bits(),
                            num.to_bits(),
                            den.to_bits(),
                            actual,
                            standard,
                            expected,
                            dist,
                            fallback_dist
                        );
                    } else {
                        panic!(
                            "f64_mul_div(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}): actual={}, expected={}, ULP dist={}",
                            a.to_bits(),
                            num.to_bits(),
                            den.to_bits(),
                            actual,
                            expected,
                            dist
                        );
                    }
                }
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
                ulp_distance(actual, standard) == 0,
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
                let dist = ulp_distance(actual, expected);

                let p = a * num;
                let q_std = p / den;
                let intermediate_underflow = is_subnormal_or_zero(p) || is_subnormal_or_zero(q_std);
                let intermediate_overflow = !standard.is_finite();

                // If the final result is subnormal, OR if the intermediate steps underflowed to zero,
                // we allow up to 4 ULPs due to precision limits.
                let is_subnormal = expected.abs() < f64::MIN_POSITIVE;
                let max_dist = if is_subnormal || intermediate_underflow {
                    4
                } else {
                    1
                };

                if dist > max_dist {
                    if intermediate_underflow || intermediate_overflow {
                        let fallback_dist = ulp_distance(actual, standard);
                        assert!(
                            fallback_dist <= 2 || (actual.is_nan() && standard.is_nan()),
                            "f64_mul_div_add(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}, offset_bits: {:#x}): actual={}, standard={} (expected={}, ULP dist={}, fallback ULP dist={})",
                            a.to_bits(),
                            num.to_bits(),
                            den.to_bits(),
                            offset.to_bits(),
                            actual,
                            standard,
                            expected,
                            dist,
                            fallback_dist
                        );
                    } else {
                        panic!(
                            "f64_mul_div_add(a_bits: {:#x}, num_bits: {:#x}, den_bits: {:#x}, offset_bits: {:#x}): actual={}, expected={}, ULP dist={}",
                            a.to_bits(),
                            num.to_bits(),
                            den.to_bits(),
                            offset.to_bits(),
                            actual,
                            expected,
                            dist
                        );
                    }
                }
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
                ulp_distance(actual, standard) == 0,
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
                let dist = ulp_distance(actual, expected);

                let hi = b * c;
                let res = a / hi;
                let intermediate_underflow = is_subnormal_or_zero(hi) || is_subnormal_or_zero(res);
                let intermediate_overflow = !standard.is_finite() || !hi.is_finite();

                // We allow up to 4 ULPs if the final result is subnormal, or if the intermediate
                // product/quotient underflowed to zero (causing double rounding in the correction term).
                let is_subnormal = expected.abs() < f64::MIN_POSITIVE;
                let max_dist = if is_subnormal || intermediate_underflow {
                    4
                } else {
                    1
                };

                if dist > max_dist {
                    if intermediate_underflow || intermediate_overflow {
                        let fallback_dist = ulp_distance(actual, standard);
                        assert!(
                            fallback_dist <= 2 || (actual.is_nan() && standard.is_nan()),
                            "f64_div_mul(a_bits: {:#x}, b_bits: {:#x}, c_bits: {:#x}): actual={}, standard={} (expected={}, ULP dist={}, fallback ULP dist={})",
                            a.to_bits(),
                            b.to_bits(),
                            c.to_bits(),
                            actual,
                            standard,
                            expected,
                            dist,
                            fallback_dist
                        );
                    } else {
                        panic!(
                            "f64_div_mul(a_bits: {:#x}, b_bits: {:#x}, c_bits: {:#x}): actual={}, expected={}, ULP dist={}",
                            a.to_bits(),
                            b.to_bits(),
                            c.to_bits(),
                            actual,
                            expected,
                            dist
                        );
                    }
                }
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
                ulp_distance(actual, standard) == 0,
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
