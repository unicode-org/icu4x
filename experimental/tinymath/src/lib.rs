// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_std]

//! Routines for performing arithmetic on small data types.
//!
//! This focus of this crate is currently on functions to perform operations
//! of the form `a * b / 2^c`. Normally this operation requires using the
//! integer one size larger than the size of `a` and `b`, but this crate
//! performs these operations without resorting to a larger integer type.
//!
//! The included Criterion benchmarks indicate that, individually, these
//! operations are slightly faster than the equivalent operations utilizing
//! the larger integer type.
//!
//! The motivation to write this crate was to experiment with vectorization;
//! in theory, 8 ops in i8 could take as long as 4 ops in i16. This has not
//! been definitively measured.

/// Computes `a * b / 128` without exceeding the bounds of an `i8`.
///
/// `a` and `b` must not be `i8::MIN` since `i8::MIN * i8::MIN / 128` is not
/// within the bounds of an i8. The caller is responsible for enforcing this
/// invariant. If the invariant is violated, an unexpected-but-deterministic
/// value will be returned.
///
/// # Examples
///
/// ```
/// use tinymath::i8_mul_div_128;
///
/// assert_eq!(29, 50 * 75 / 128);
/// assert_eq!(29, i8_mul_div_128(50, 75));
/// ```
pub fn i8_mul_div_128(a: i8, b: i8) -> i8 {
    debug_assert_ne!(a, i8::MIN);
    debug_assert_ne!(b, i8::MIN);
    let au8 = a.unsigned_abs();
    let bu8 = b.unsigned_abs();
    let mut result = 0u8;
    result += if (au8 & 0b00000001) != 0 { bu8 } else { 0 };
    result >>= 1;
    result += if (au8 & 0b00000010) != 0 { bu8 } else { 0 };
    result >>= 1;
    result += if (au8 & 0b00000100) != 0 { bu8 } else { 0 };
    result >>= 1;
    result += if (au8 & 0b00001000) != 0 { bu8 } else { 0 };
    result >>= 1;
    result += if (au8 & 0b00010000) != 0 { bu8 } else { 0 };
    result >>= 1;
    result += if (au8 & 0b00100000) != 0 { bu8 } else { 0 };
    result >>= 1;
    result += if (au8 & 0b01000000) != 0 { bu8 } else { 0 };
    result >>= 1;
    let result = result as i8;
    if (a as u8 & 0x80) ^ (b as u8 & 0x80) == 0 {
        result
    } else {
        -result
    }
}

#[doc(hidden)]
pub fn i8_mul_div_128_reference(a: i8, b: i8) -> i8 {
    (a as i16 * b as i16 / 128) as i8
}

/// Computes `a * b / 1024` without exceeding the bounds of an `i16`.
///
/// Saturates the `i16` if overflow would occur. Never returns `i16::MIN`;
/// negative values are saturated to `-i16::MAX`.
///
/// # Examples
///
/// ```
/// use tinymath::saturating_i16_mul_div_1024;
///
/// assert_eq!(146, 200 * 750 / 1024);
/// assert_eq!(146, saturating_i16_mul_div_1024(200, 750));
/// ```
///
/// Saturates the `i16` if the result does not fit:
///
/// ```
/// use tinymath::saturating_i16_mul_div_1024;
///
/// assert_eq!(62500, 8000 * 8000 / 1024);
/// assert_eq!(32767, saturating_i16_mul_div_1024(8000, 8000));
/// ```
pub fn saturating_i16_mul_div_1024(a: i16, b: i16) -> i16 {
    let au16 = a.unsigned_abs();
    let bu16 = b.unsigned_abs();
    let mut result = 0u16;
    result += if (au16 & 0b0000000000000001) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000000000010) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000000000100) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000000001000) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000000010000) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000000100000) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000001000000) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000010000000) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000000100000000) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000001000000000) != 0 {
        bu16
    } else {
        0
    };
    result >>= 1;
    result += if (au16 & 0b0000010000000000) != 0 {
        bu16
    } else {
        0
    };
    // Have done enough right-shifts; now need to do some left-shifts
    result = result.saturating_add(if (au16 & 0b0000100000000000) != 0 {
        if bu16 >= 0b1000000000000000 {
            u16::MAX
        } else {
            bu16 << 1
        }
    } else {
        0
    });
    result = result.saturating_add(if (au16 & 0b0001000000000000) != 0 {
        if bu16 >= 0b0100000000000000 {
            u16::MAX
        } else {
            bu16 << 2
        }
    } else {
        0
    });
    result = result.saturating_add(if (au16 & 0b0010000000000000) != 0 {
        if bu16 >= 0b0010000000000000 {
            u16::MAX
        } else {
            bu16 << 3
        }
    } else {
        0
    });
    result = result.saturating_add(if (au16 & 0b0100000000000000) != 0 {
        if bu16 >= 0b0001000000000000 {
            u16::MAX
        } else {
            bu16 << 4
        }
    } else {
        0
    });
    result = result.saturating_add(if (au16 & 0b1000000000000000) != 0 {
        if bu16 >= 0b0000100000000000 {
            u16::MAX
        } else {
            bu16 << 5
        }
    } else {
        0
    });
    result = core::cmp::min(result, i16::MAX as u16);
    let result = result as i16;
    if (a as u16 & 0x8000) ^ (b as u16 & 0x8000) == 0 {
        result
    } else {
        -result
    }
}

#[doc(hidden)]
pub fn saturating_i16_mul_div_1024_reference(a: i16, b: i16) -> i16 {
    let result = a as i32 * b as i32 / 1024;
    if result > i16::MAX as i32 {
        i16::MAX
    } else if result < -i16::MAX as i32 {
        -i16::MAX
    } else {
        result as i16
    }
}
