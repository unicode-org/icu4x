// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;
use std::ptr;

/// Opaque type for use behind a pointer, is [`FixedDecimal`]
///
/// Can be obtained via [`icu4x_fixed_decimal_create()`] and destroyed via [`icu4x_fixed_decimal_destroy()`]
pub type ICU4XFixedDecimal = FixedDecimal;

#[repr(C)]
/// This is the result returned by [`icu4x_plural_rules_create()`]
pub struct ICU4XCreateFixedDecimalResult {
    /// Will be null if `success` is [`false`]
    pub decimal: *mut ICU4XFixedDecimal,
    /// Currently just a boolean, but we might add a proper error enum as necessary
    pub success: bool,
}

#[no_mangle]
/// FFI version of [`FixedDecimal`]'s constructors. This constructs a [`FixedDecimal`] of the provided
/// `magnitude` and then multiplies it by `10 ^ pow10`.
///
/// # Safety
/// - Only access `decimal` in the result if `success` is [`true`].
//
// We can add additional constructors from strings, floats, etc as the need arises
pub extern "C" fn icu4x_fixed_decimal_create(
    magnitude: i64,
    pow10: i16,
) -> ICU4XCreateFixedDecimalResult {
    let fd = FixedDecimal::from(magnitude);
    if let Ok(multiplied) = fd.multiplied_pow10(pow10) {
        ICU4XCreateFixedDecimalResult {
            decimal: Box::into_raw(Box::new(multiplied)),
            success: true,
        }
    } else {
        ICU4XCreateFixedDecimalResult {
            decimal: ptr::null_mut(),
            success: false,
        }
    }
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
