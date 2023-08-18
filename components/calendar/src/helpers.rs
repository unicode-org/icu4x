// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    astronomy::{Location, PI},
    rata_die::RataDie,
    types::Moment,
};

/// Calculate `(n / d, n % d)` such that the remainder is always positive.
///
/// Also see [`i32::div_euclid`], [`i32::rem_euclid`].
pub const fn div_rem_euclid(n: i32, d: i32) -> (i32, i32) {
    debug_assert!(d > 0);
    let (a, b) = (n / d, n % d);
    if n >= 0 || b == 0 {
        (a, b)
    } else {
        (a - 1, d + b)
    }
}

/// [`div_rem_euclid`] for 64-bit inputs
///
/// Also see [`i64::div_euclid`], [`i64::rem_euclid`].
pub fn div_rem_euclid64(n: i64, d: i64) -> (i64, i64) {
    debug_assert!(d > 0);
    let (a, b) = (n / d, n % d);
    if n >= 0 || b == 0 {
        (a, b)
    } else {
        (a - 1, d + b)
    }
}

/// [`div_rem_euclid`] for f64
pub fn div_rem_euclid_f64(n: f64, d: f64) -> (f64, f64) {
    debug_assert!(d > 0.0);
    let (a, b) = (n / d, n % d);
    if n >= 0.0 || b == 0.0 {
        (a, b)
    } else {
        (a - 1.0, d + b)
    }
}

/// Calculates `n / d` such that the remainder is always positive.
/// This is achieved by performing an integer division and then, if the numerator is positive and there is a non-zero remainder,
/// incrementing the quotient by 1.
/// This is equivalent to ceiling() in the Reingold/Dershowitz Lisp code
pub fn ceil_div(n: i64, d: i64) -> i64 {
    debug_assert!(d > 0);
    let (a, b) = (n / d, n % d);
    if n <= 0 || b == 0 {
        a
    } else {
        a + 1
    }
}

/// Calculate `n / d` such that the remainder is always positive.
/// This is equivalent to quotient() in the Reingold/Dershowitz Lisp code
///
/// Also see [`i32::div_euclid`]
pub const fn quotient(n: i32, d: i32) -> i32 {
    debug_assert!(d > 0);
    // Code can use int_roundings once stabilized
    // https://github.com/rust-lang/rust/issues/88581
    let (a, b) = (n / d, n % d);
    if n >= 0 || b == 0 {
        a
    } else {
        a - 1
    }
}

/// [`quotient`] for 64-bit inputs
///
/// Also see [`i64::div_euclid`]
pub const fn quotient64(n: i64, d: i64) -> i64 {
    debug_assert!(d > 0);
    // Code can use int_roundings once stabilized
    // https://github.com/rust-lang/rust/issues/88581
    let (a, b) = (n / d, n % d);
    if n >= 0 || b == 0 {
        a
    } else {
        a - 1
    }
}

// cosine of x in radians
pub fn cos_degrees(x: f64) -> f64 {
    let radians = x.to_radians();
    libm::cos(radians)
}

// sine of x in radians
pub fn sin_degrees(x: f64) -> f64 {
    let radians = x.to_radians();
    libm::sin(radians)
}

// tan of x in radians
pub fn tan_degrees(x: f64) -> f64 {
    let radians = x.to_radians();
    libm::tan(radians)
}

// Arccosine of x in degrees
pub fn arccos_degrees(x: f64) -> f64 {
    let radians = libm::acos(x);
    radians.to_degrees()
}

// Arcsine of x in degrees
pub fn arcsin_degrees(x: f64) -> f64 {
    let radians = libm::asin(x);
    let r = radians.to_degrees();

    div_rem_euclid_f64(r, 360.0).1
}

fn mod_degrees(x: f64) -> f64 {
    ((x % 360.0) + 360.0) % 360.0
}

pub fn mod3(x: f64, a: f64, b: f64) -> f64 {
    // The value of x shifted into the range [a..b).
    // Returns x if a=b.
    if libm::fabs(a - b) < f64::EPSILON {
        x
    } else {
        let (_, rem) = div_rem_euclid_f64(x - a, b - a);
        a + rem
    }
}

// Arctangent of y/x in degrees, handling zero cases (using atan2)
pub fn arctan_degrees(y: f64, x: f64) -> f64 {
    mod_degrees(libm::atan2(y, x) * 180.0 / PI)
}

// TODO: convert recursive into iterative
pub fn poly(x: f64, coeffs: &[f64]) -> f64 {
    match coeffs.split_first() {
        Some((first, rest)) => first + x * poly(x, rest),
        None => 0.0,
    }
}

// A generic function that finds a value within an interval
// where a certain condition is satisfied.
pub fn binary_search<F, G>(mut l: f64, mut h: f64, test: F, end: G) -> f64
where
    F: Fn(f64) -> bool, // function that checks a condition to decide which direction to go.
    G: Fn(f64, f64) -> bool, // function that checks if the interval is small enough to terminate the search.
{
    loop {
        // Calculate the midpoint between `l` and `h`.
        let x = (h + l) / 2.0;

        // Determine which direction to go. `test` returns true if we need to go left.
        let left = test(x);

        l = if left { l } else { x };
        h = if left { x } else { h };

        // If the `end` condition is met (typically when the interval becomes smaller than a certain threshold),
        // we break the loop and return the current midpoint `x`.
        if end(h, l) {
            return x;
        }
    }
}

// Returns a number that represents the sign of `self`.
// - `1.0` if the number is positive, `+0.0` or `INFINITY`
// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
// - NaN if the number is NaN
pub fn signum(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }

    if x.is_sign_positive() {
        1.0
    } else {
        -1.0
    }
}

#[allow(dead_code)] // TODO: Remove dead_code tag after use
pub fn invert_angular<F: Fn(f64) -> f64>(f: F, y: f64, r: (f64, f64)) -> f64 {
    let varepsilon = 1.0 / 100000.0; // Desired accuracy
    binary_search(
        r.0,
        r.1,
        |x| div_rem_euclid_f64(f(x) - y, 360.0).1 < 180.0,
        |u, l| (u - l) < varepsilon,
    )
}

pub(crate) fn next_moment<F>(mut index: Moment, location: Location, condition: F) -> RataDie
where
    F: Fn(Moment, Location) -> bool,
{
    loop {
        if condition(index, location) {
            return index.as_rata_die();
        }
        index += 1.0;
    }
}

pub(crate) fn next<F>(mut index: RataDie, condition: F) -> RataDie
where
    F: Fn(RataDie) -> bool,
{
    loop {
        if condition(index) {
            return index;
        }
        index += 1;
    }
}

pub(crate) fn next_u8<F>(mut index: u8, condition: F) -> u8
where
    F: Fn(u8) -> bool,
{
    loop {
        if condition(index) {
            return index;
        }
        index += 1;
    }
}

// "Final" is a reserved keyword in rust, which explains the naming convention here.
pub(crate) fn final_func<F>(mut index: i32, condition: F) -> i32
where
    F: Fn(i32) -> bool,
{
    while condition(index) {
        index += 1;
    }
    index - 1
}

#[test]
fn test_binary_search() {
    struct TestCase {
        test_fn: fn(f64) -> bool,
        range: (f64, f64),
        expected: f64,
    }

    let end = |h: f64, l: f64| (h - l).abs() < 0.0001;

    let test_cases = [
        TestCase {
            test_fn: |x: f64| x >= 4.0,
            range: (0.0, 10.0),
            expected: 4.0,
        },
        TestCase {
            test_fn: |x: f64| x * x >= 2.0,
            range: (0.0, 2.0),
            expected: 2.0f64.sqrt(),
        },
        TestCase {
            test_fn: |x: f64| x >= -4.0,
            range: (-10.0, 0.0),
            expected: -4.0,
        },
        TestCase {
            test_fn: |x: f64| x >= 0.0,
            range: (0.0, 10.0),
            expected: 0.0,
        },
        TestCase {
            test_fn: |x: f64| x > 10.0,
            range: (0.0, 10.0),
            expected: 10.0,
        },
    ];

    for case in test_cases {
        let result = binary_search(case.range.0, case.range.1, case.test_fn, end);
        assert!((result - case.expected).abs() < 0.0001);
    }
}

#[test]
fn test_invert_angular() {
    struct TestCase {
        f: fn(f64) -> f64,
        y: f64,
        r: (f64, f64),
        expected: f64,
    }

    fn f1(x: f64) -> f64 {
        div_rem_euclid_f64(2.0 * x, 360.0).1
    }

    fn f2(x: f64) -> f64 {
        div_rem_euclid_f64(3.0 * x, 360.0).1
    }

    fn f3(x: f64) -> f64 {
        div_rem_euclid_f64(x, 360.0).1
    }
    // tolerance for comparing floating points.
    let tolerance = 1e-5;

    let test_cases = [
        TestCase {
            f: f1,
            y: 4.0,
            r: (0.0, 10.0),
            expected: 4.0,
        },
        TestCase {
            f: f2,
            y: 6.0,
            r: (0.0, 20.0),
            expected: 6.0,
        },
        TestCase {
            f: f3,
            y: 400.0,
            r: (0.0, 10.0),
            expected: 10.0,
        },
        TestCase {
            f: f3,
            y: 0.0,
            r: (0.0, 10.0),
            expected: 0.0,
        },
        TestCase {
            f: f3,
            y: 10.0,
            r: (0.0, 10.0),
            expected: 10.0,
        },
    ];

    for case in test_cases {
        let x = invert_angular(case.f, case.y, case.r);
        assert!((div_rem_euclid_f64((case.f)(x), 360.0).1 - case.expected).abs() < tolerance);
    }
}

/// Return y if x.rem_euclid(y) == 0, x.rem_euclid(y) otherwise
pub fn adjusted_rem_euclid(x: i32, y: i32) -> i32 {
    let remainder = div_rem_euclid(x, y).1;
    if remainder == 0 {
        y
    } else {
        remainder
    }
}

#[test]
fn test_adjusted_rem_euclid() {
    #[derive(Debug)]
    struct TestCase {
        x: i32,
        y: i32,
        expected: i32,
    }

    let cases = [
        TestCase {
            x: 3,
            y: 7,
            expected: 3,
        },
        TestCase {
            x: 7,
            y: 3,
            expected: 1,
        },
        TestCase {
            x: -11,
            y: 9,
            expected: 7,
        },
        TestCase {
            x: 11,
            y: 9,
            expected: 2,
        },
        TestCase {
            x: 11,
            y: 11,
            expected: 11,
        },
        TestCase {
            x: -22,
            y: 11,
            expected: 11,
        },
    ];
    for case in cases {
        let result = adjusted_rem_euclid(case.x, case.y);
        assert_eq!(
            case.expected, result,
            "Adjusted rem euclid failed for case: {case:?}"
        );
    }
}

/// The value of x shifted into the range [a..b); returns x if a == b; for f64 types
pub fn interval_mod_f64(x: f64, a: f64, b: f64) -> f64 {
    if a == b {
        x
    } else {
        a + div_rem_euclid_f64(x - a, b - a).1
    }
}

#[test]
fn test_interval_mod() {
    #[derive(Debug)]
    struct TestCase {
        x: f64,
        a: f64,
        b: f64,
        expected: f64,
    }

    let cases = [
        TestCase {
            x: 5.0,
            a: 10.0,
            b: 20.0,
            expected: 15.0,
        },
        TestCase {
            x: -5.0,
            a: 10.0,
            b: 20.0,
            expected: 15.0,
        },
        TestCase {
            x: 2.0,
            a: 12.0,
            b: 17.0,
            expected: 12.0,
        },
        TestCase {
            x: 9.0,
            a: 9.0,
            b: 10.0,
            expected: 9.0,
        },
        TestCase {
            x: 16.5,
            a: 13.5,
            b: 20.0,
            expected: 16.5,
        },
        TestCase {
            x: 9.0,
            a: 3.0,
            b: 9.0,
            expected: 3.0,
        },
        TestCase {
            x: 17.0,
            a: 1.0,
            b: 5.5,
            expected: 3.5,
        },
    ];

    for case in cases {
        let result = interval_mod_f64(case.x, case.a, case.b);
        assert_eq!(
            case.expected, result,
            "Interval mod test failed for case: {case:?}"
        );
    }
}

#[test]
fn test_div_rem_euclid() {
    assert_eq!(div_rem_euclid(i32::MIN, 1), (-2147483648, 0));
    assert_eq!(div_rem_euclid(i32::MIN, 2), (-1073741824, 0));
    assert_eq!(div_rem_euclid(i32::MIN, 3), (-715827883, 1));

    assert_eq!(div_rem_euclid(-10, 1), (-10, 0));
    assert_eq!(div_rem_euclid(-10, 2), (-5, 0));
    assert_eq!(div_rem_euclid(-10, 3), (-4, 2));

    assert_eq!(div_rem_euclid(-9, 1), (-9, 0));
    assert_eq!(div_rem_euclid(-9, 2), (-5, 1));
    assert_eq!(div_rem_euclid(-9, 3), (-3, 0));

    assert_eq!(div_rem_euclid(-8, 1), (-8, 0));
    assert_eq!(div_rem_euclid(-8, 2), (-4, 0));
    assert_eq!(div_rem_euclid(-8, 3), (-3, 1));

    assert_eq!(div_rem_euclid(-2, 1), (-2, 0));
    assert_eq!(div_rem_euclid(-2, 2), (-1, 0));
    assert_eq!(div_rem_euclid(-2, 3), (-1, 1));

    assert_eq!(div_rem_euclid(-1, 1), (-1, 0));
    assert_eq!(div_rem_euclid(-1, 2), (-1, 1));
    assert_eq!(div_rem_euclid(-1, 3), (-1, 2));

    assert_eq!(div_rem_euclid(0, 1), (0, 0));
    assert_eq!(div_rem_euclid(0, 2), (0, 0));
    assert_eq!(div_rem_euclid(0, 3), (0, 0));

    assert_eq!(div_rem_euclid(1, 1), (1, 0));
    assert_eq!(div_rem_euclid(1, 2), (0, 1));
    assert_eq!(div_rem_euclid(1, 3), (0, 1));

    assert_eq!(div_rem_euclid(2, 1), (2, 0));
    assert_eq!(div_rem_euclid(2, 2), (1, 0));
    assert_eq!(div_rem_euclid(2, 3), (0, 2));

    assert_eq!(div_rem_euclid(8, 1), (8, 0));
    assert_eq!(div_rem_euclid(8, 2), (4, 0));
    assert_eq!(div_rem_euclid(8, 3), (2, 2));

    assert_eq!(div_rem_euclid(9, 1), (9, 0));
    assert_eq!(div_rem_euclid(9, 2), (4, 1));
    assert_eq!(div_rem_euclid(9, 3), (3, 0));

    assert_eq!(div_rem_euclid(10, 1), (10, 0));
    assert_eq!(div_rem_euclid(10, 2), (5, 0));
    assert_eq!(div_rem_euclid(10, 3), (3, 1));

    assert_eq!(div_rem_euclid(i32::MAX, 1), (2147483647, 0));
    assert_eq!(div_rem_euclid(i32::MAX, 2), (1073741823, 1));
    assert_eq!(div_rem_euclid(i32::MAX, 3), (715827882, 1));

    for n in -100..100 {
        for d in 1..5 {
            let (x1, y1) = div_rem_euclid(n, d);
            let (x2, y2) = div_rem_euclid64(n as i64, d as i64);
            let (x3, y3) = (n.div_euclid(d), n.rem_euclid(d));
            assert_eq!(x1, x2 as i32);
            assert_eq!(x1, x3);
            assert_eq!(y1, y2 as i32);
            assert_eq!(y1, y3);
        }
    }
}

#[test]
fn test_ceil_div() {
    assert_eq!(ceil_div(i64::MIN, 1), i64::MIN);
    assert_eq!(ceil_div(i64::MIN, 2), -4611686018427387904);
    assert_eq!(ceil_div(i64::MIN, 3), -3074457345618258602);

    assert_eq!(ceil_div(-10, 1), -10);
    assert_eq!(ceil_div(-10, 2), -5);
    assert_eq!(ceil_div(-10, 3), -3);

    assert_eq!(ceil_div(-9, 1), -9);
    assert_eq!(ceil_div(-9, 2), -4);
    assert_eq!(ceil_div(-9, 3), -3);

    assert_eq!(ceil_div(-8, 1), -8);
    assert_eq!(ceil_div(-8, 2), -4);
    assert_eq!(ceil_div(-8, 3), -2);

    assert_eq!(ceil_div(-2, 1), -2);
    assert_eq!(ceil_div(-2, 2), -1);
    assert_eq!(ceil_div(-2, 3), 0);

    assert_eq!(ceil_div(-1, 1), -1);
    assert_eq!(ceil_div(-1, 2), 0);
    assert_eq!(ceil_div(-1, 3), 0);

    assert_eq!(ceil_div(0, 1), 0);
    assert_eq!(ceil_div(0, 2), 0);
    assert_eq!(ceil_div(0, 3), 0);

    assert_eq!(ceil_div(1, 1), 1);
    assert_eq!(ceil_div(1, 2), 1);
    assert_eq!(ceil_div(1, 3), 1);

    assert_eq!(ceil_div(2, 1), 2);
    assert_eq!(ceil_div(2, 2), 1);
    assert_eq!(ceil_div(2, 3), 1);

    assert_eq!(ceil_div(8, 1), 8);
    assert_eq!(ceil_div(8, 2), 4);
    assert_eq!(ceil_div(8, 3), 3);

    assert_eq!(ceil_div(9, 1), 9);
    assert_eq!(ceil_div(9, 2), 5);
    assert_eq!(ceil_div(9, 3), 3);

    assert_eq!(ceil_div(10, 1), 10);
    assert_eq!(ceil_div(10, 2), 5);
    assert_eq!(ceil_div(10, 3), 4);

    assert_eq!(ceil_div(i64::MAX, 1), 9223372036854775807);
    assert_eq!(ceil_div(i64::MAX, 2), 4611686018427387904);
    assert_eq!(ceil_div(i64::MAX, 3), 3074457345618258603);

    for n in -100..100 {
        for d in 1..5 {
            let x1 = ceil_div(n, d);
            let x2 = (n as f64 / d as f64).ceil();
            assert_eq!(x1, x2 as i64);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum I32Result {
    BelowMin(i64),
    WithinRange(i32),
    AboveMax(i64),
}

impl I32Result {
    pub const fn saturate(self) -> i32 {
        match self {
            I32Result::BelowMin(_) => i32::MIN,
            I32Result::WithinRange(x) => x,
            I32Result::AboveMax(_) => i32::MAX,
        }
    }
}

#[inline]
pub const fn i64_to_i32(input: i64) -> I32Result {
    if input < i32::MIN as i64 {
        I32Result::BelowMin(input)
    } else if input > i32::MAX as i64 {
        I32Result::AboveMax(input)
    } else {
        I32Result::WithinRange(input as i32)
    }
}

#[inline]
pub const fn i64_to_saturated_i32(input: i64) -> i32 {
    i64_to_i32(input).saturate()
}

#[test]
fn test_signum_i64() {
    assert_eq!(signum(5.0), 1.0);
    assert_eq!(signum(-5.0), -1.0);
    assert_eq!(signum(0.0), 1.0);
    assert_eq!(signum(-0.0), -1.0);
    assert_eq!(signum(f64::INFINITY), 1.0);
    assert_eq!(signum(f64::NEG_INFINITY), -1.0);
    assert!(signum(f64::NAN).is_nan());
}

#[test]
fn test_i64_to_saturated_i32() {
    assert_eq!(i64_to_saturated_i32(i64::MIN), i32::MIN);
    assert_eq!(i64_to_saturated_i32(-2147483649), -2147483648);
    assert_eq!(i64_to_saturated_i32(-2147483648), -2147483648);
    assert_eq!(i64_to_saturated_i32(-2147483647), -2147483647);
    assert_eq!(i64_to_saturated_i32(-2147483646), -2147483646);
    assert_eq!(i64_to_saturated_i32(-100), -100);
    assert_eq!(i64_to_saturated_i32(0), 0);
    assert_eq!(i64_to_saturated_i32(100), 100);
    assert_eq!(i64_to_saturated_i32(2147483646), 2147483646);
    assert_eq!(i64_to_saturated_i32(2147483647), 2147483647);
    assert_eq!(i64_to_saturated_i32(2147483648), 2147483647);
    assert_eq!(i64_to_saturated_i32(2147483649), 2147483647);
    assert_eq!(i64_to_saturated_i32(i64::MAX), i32::MAX);
}
