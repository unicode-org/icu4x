// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;

use crate::custom_writeable::ICU4XWriteable;
use std::slice;
use writeable::Writeable;

/// Opaque type for use behind a pointer, is [`FixedDecimal`]
///
/// Can be obtained via [`icu4x_fixed_decimal_create()`] and destroyed via [`icu4x_fixed_decimal_destroy()`]
pub type ICU4XFixedDecimal = FixedDecimal;

#[repr(C)]
/// This is the result returned by [`icu4x_fixed_decimal_create_fromstr()`]
pub struct ICU4XCreateFixedDecimalResult {
    /// Will be null if `success` is [`false`]
    pub fd: *mut ICU4XFixedDecimal,
    /// Currently just a boolean, but we might add a proper error enum as necessary
    pub success: bool,
}

#[no_mangle]
/// FFI version of [`FixedDecimal`]'s constructors. This constructs a [`FixedDecimal`] of the provided
/// `number`.
pub extern "C" fn icu4x_fixed_decimal_create(number: i64) -> *mut ICU4XFixedDecimal {
    let fd = FixedDecimal::from(number);
    Box::into_raw(Box::new(fd))
}

#[no_mangle]
/// FFI version of [`FixedDecimal::FromStr()`], see its docs for more details
///
/// # Safety
/// `value` and `len` should point to a valid ASCII string of length `len`.
///
/// It does not need to be be null terminated, and `len` should not include a null
/// terminator (this will just cause the function to panic, and is not a safety requirement).
///
/// Returns nullptr if passed an invalid string.
pub unsafe extern "C" fn icu4x_fixed_decimal_create_fromstr(
    value: *const u8,
    len: usize,
) -> ICU4XCreateFixedDecimalResult {
    let bytes = slice::from_raw_parts(value, len);
    if let Ok(as_str) = std::str::from_utf8(bytes) {
        if let Ok(fd) = as_str.parse::<FixedDecimal>() {
            return ICU4XCreateFixedDecimalResult {
                fd: Box::into_raw(Box::new(fd)),
                success: true,
            };
        }
    }
    ICU4XCreateFixedDecimalResult {
        fd: std::ptr::null_mut(),
        success: false,
    }
}

#[no_mangle]
/// FFI version of [`FixedDecimal::multiply_pow10()`]. See its docs for more details.
///
/// Returns a boolean indicating whether the operation was successful.
pub extern "C" fn icu4x_fixed_decimal_multiply_pow10(
    fd: &mut ICU4XFixedDecimal,
    power: i16,
) -> bool {
    fd.multiply_pow10(power).is_ok()
}

#[no_mangle]
/// FFI version of [`FixedDecimal::negate()`]. See its docs for more details.
pub extern "C" fn icu4x_fixed_decimal_negate(fd: &mut ICU4XFixedDecimal) {
    fd.negate()
}

#[no_mangle]
/// FFI version of [`FixedDecimal::write_to()`]. See its docs for more details.
pub extern "C" fn icu4x_fixed_decimal_write_to(fd: &ICU4XFixedDecimal, to: &mut ICU4XWriteable) {
    fd.write_to(to).unwrap();
}

#[no_mangle]
/// Destructor for [`ICU4XFixedDecimal`]
///
/// # Safety
/// `fd` must be a pointer to a valid [`ICU4XFixedDecimal`] constructed by
/// [`icu4x_fixed_decimal_create()`].
pub unsafe extern "C" fn icu4x_fixed_decimal_destroy(fd: *mut ICU4XFixedDecimal) {
    let _ = Box::from_raw(fd);
}
