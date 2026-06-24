// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fused::{f64_div_mul, f64_mul_div, f64_mul_div_add};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::ToPrimitive;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Converts an f64 to an exact rational representation.
fn to_ratio(val: f64) -> Option<Ratio<BigInt>> {
    Ratio::from_float(val)
}

/// Helper to assert that two floats are either exactly equal or at most 1 ULP apart.
///
/// 1 ULP difference is acceptable in extremely rare boundary cases where the exact
/// real value is a midpoint and minor double rounding vs single rounding differences
/// occur.
fn assert_close_or_equal(actual: f64, expected: f64, msg: &str) {
    if actual.is_nan() && expected.is_nan() {
        return;
    }
    if actual == expected {
        return;
    }
    if actual.is_finite() && expected.is_finite() {
        let next_up = expected.next_up();
        let next_down = expected.next_down();
        if actual == next_up || actual == next_down {
            return;
        }
    }
    panic!(
        "{}: actual ({:?}) is not equal or 1 ULP close to expected ({:?}) [bounds: {:?}, {:?}]",
        msg,
        actual,
        expected,
        expected.next_down(),
        expected.next_up()
    );
}

#[test]
fn test_curated_differential_cases() {
    // 1. Multiply-Divide Proportional Conversions
    // Curated case 1: 5 grams to tonnes (factor: 1 / 1_000_000)
    {
        let a = 5.0;
        let num = 1.0;
        let den = 1_000_000.0;
        let actual = f64_mul_div(a, num, den);

        let a_rat = to_ratio(a).unwrap();
        let num_rat = to_ratio(num).unwrap();
        let den_rat = to_ratio(den).unwrap();
        let expected_rat = (&a_rat * &num_rat) / &den_rat;
        let expected = expected_rat.to_f64().unwrap();

        assert_close_or_equal(actual, expected, "5g to tonnes");
    }

    // Curated case 2: 0.1 * (1.0 / 10.0)
    {
        let a = 0.1;
        let num = 1.0;
        let den = 10.0;
        let actual = f64_mul_div(a, num, den);

        let expected_rat = (to_ratio(a).unwrap() * to_ratio(num).unwrap()) / to_ratio(den).unwrap();
        let expected = expected_rat.to_f64().unwrap();

        assert_close_or_equal(actual, expected, "0.1 * (1/10)");
    }

    // 2. Multiply-Divide-Add Offset Conversions
    // Curated case 3: Celsius to Fahrenheit (C * 9/5 + 32)
    {
        let temperatures = [100.0, 0.0, -40.0, 37.0, -273.15];
        for &c in &temperatures {
            let actual = f64_mul_div_add(c, 9.0, 5.0, 32.0);

            let c_rat = to_ratio(c).unwrap();
            let expected_rat =
                (c_rat * to_ratio(9.0).unwrap()) / to_ratio(5.0).unwrap() + to_ratio(32.0).unwrap();
            let expected = expected_rat.to_f64().unwrap();

            assert_close_or_equal(actual, expected, &format!("Celsius to Fahrenheit: {}", c));
        }
    }

    // 3. Reciprocal Division
    // Curated case 4: 1.0 / (0.1 * 0.1)
    {
        let a = 1.0;
        let b = 0.1;
        let c = 0.1;
        let actual = f64_div_mul(a, b, c);

        let expected_rat = to_ratio(a).unwrap() / (to_ratio(b).unwrap() * to_ratio(c).unwrap());
        let expected = expected_rat.to_f64().unwrap();

        assert_close_or_equal(actual, expected, "1 / (0.1 * 0.1)");
    }
}

#[test]
fn test_randomized_differential_fuzzing() {
    let mut rng = SmallRng::seed_from_u64(0x1234_5678_9ABC_DEF0);

    // We will generate 100,000 random test cases across different scales
    let scales = [1e-15, 1e-10, 1e-5, 1.0, 1e5, 1e10, 1e15];

    for _ in 0..30_000 {
        // Choose a random scale for each parameter to cover mixed magnitude arithmetic
        let scale_a = scales[rng.random_range(0..scales.len())];
        let scale_num = scales[rng.random_range(0..scales.len())];
        let scale_den = scales[rng.random_range(0..scales.len())];
        let scale_offset = scales[rng.random_range(0..scales.len())];

        let a: f64 = rng.random::<f64>() * scale_a;
        let num: f64 = rng.random::<f64>() * scale_num;
        let den: f64 = rng.random::<f64>() * scale_den;
        let offset: f64 = rng.random::<f64>() * scale_offset;

        // Skip if denominator is zero or near-zero to avoid trivial divisions by zero
        if den.abs() < 1e-30 {
            continue;
        }

        // --- 1. Test f64_mul_div ---
        {
            let actual = f64_mul_div(a, num, den);

            let a_rat = to_ratio(a);
            let num_rat = to_ratio(num);
            let den_rat = to_ratio(den);

            if let (Some(a_r), Some(num_r), Some(den_r)) = (a_rat, num_rat, den_rat) {
                if den_r.numer() != &0.into() {
                    let expected_rat = (a_r * num_r) / den_r;
                    if let Some(expected) = expected_rat.to_f64() {
                        // If the expected value overflows to infinity, the actual FMA code might
                        // also yield infinity or fallback. We check bounds.
                        assert_close_or_equal(
                            actual,
                            expected,
                            &format!("f64_mul_div({}, {}, {})", a, num, den),
                        );
                    }
                }
            }
        }

        // --- 2. Test f64_mul_div_add ---
        {
            let actual = f64_mul_div_add(a, num, den, offset);

            let a_rat = to_ratio(a);
            let num_rat = to_ratio(num);
            let den_rat = to_ratio(den);
            let offset_rat = to_ratio(offset);

            if let (Some(a_r), Some(num_r), Some(den_r), Some(offset_r)) =
                (a_rat, num_rat, den_rat, offset_rat)
            {
                if den_r.numer() != &0.into() {
                    let expected_rat = (a_r * num_r) / den_r + offset_r;
                    if let Some(expected) = expected_rat.to_f64() {
                        assert_close_or_equal(
                            actual,
                            expected,
                            &format!("f64_mul_div_add({}, {}, {}, {})", a, num, den, offset),
                        );
                    }
                }
            }
        }

        // --- 3. Test f64_div_mul ---
        {
            let actual = f64_div_mul(a, num, den); // computes a / (num * den)

            let a_rat = to_ratio(a);
            let num_rat = to_ratio(num);
            let den_rat = to_ratio(den);

            if let (Some(a_r), Some(num_r), Some(den_r)) = (a_rat, num_rat, den_rat) {
                let divisor_rat = num_r * den_r;
                if divisor_rat.numer() != &0.into() {
                    let expected_rat = a_r / divisor_rat;
                    if let Some(expected) = expected_rat.to_f64() {
                        assert_close_or_equal(
                            actual,
                            expected,
                            &format!("f64_div_mul({}, {}, {})", a, num, den),
                        );
                    }
                }
            }
        }
    }
}
