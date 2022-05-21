// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Int operations that are not yet in the standard library.

/// Computes `a - b` where `a` is signed and `b` is unsigned.
///
/// If overflow occurs, panics in debug mode and wraps in release mode.
#[inline]
pub fn i16_sub_unsigned(a: i16, b: u16) -> i16 {
    let c = a.wrapping_sub(b as i16);
    debug_assert_eq!(a as i32 - b as i32, c as i32);
    c
}

#[test]
fn test_i16_sub_unsigned() {
    assert_eq!(i16_sub_unsigned(5, 0), 5);
    assert_eq!(i16_sub_unsigned(5, 4), 1);
    assert_eq!(i16_sub_unsigned(5, 5), 0);
    assert_eq!(i16_sub_unsigned(5, 6), -1);
    assert_eq!(i16_sub_unsigned(0, 0), 0);
    assert_eq!(i16_sub_unsigned(0, 1), -1);
    assert_eq!(i16_sub_unsigned(-5, 0), -5);
    assert_eq!(i16_sub_unsigned(-5, 1), -6);

    assert_eq!(i16_sub_unsigned(i16::MAX, 0), i16::MAX);
    assert_eq!(i16_sub_unsigned(i16::MAX, 1), i16::MAX - 1);
    assert_eq!(i16_sub_unsigned(i16::MAX, i16::MAX as u16 - 1), 1);
    assert_eq!(i16_sub_unsigned(i16::MAX, i16::MAX as u16), 0);
    assert_eq!(i16_sub_unsigned(i16::MAX, i16::MAX as u16 + 1), -1);
    assert_eq!(i16_sub_unsigned(i16::MAX, u16::MAX - 1), i16::MIN + 1);
    assert_eq!(i16_sub_unsigned(i16::MAX, u16::MAX), i16::MIN);
}
