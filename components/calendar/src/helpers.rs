// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Calculate `(n / d, n % d)` such that the remainder is always positive.
///
/// Also see [`i32::div_euclid`], [`i32::rem_euclid`].
pub fn div_rem_euclid(n: i32, d: i32) -> (i32, i32) {
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

pub enum I32Result {
    BelowMin(i64),
    WithinRange(i32),
    AboveMax(i64),
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
    match i64_to_i32(input) {
        I32Result::BelowMin(_) => i32::MIN,
        I32Result::WithinRange(x) => x,
        I32Result::AboveMax(_) => i32::MAX,
    }
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
