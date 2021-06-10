// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;

use crate::custom_writeable::ICU4XWriteable;
use writeable::Writeable;

/// Opaque type for use behind a pointer, is [`FixedDecimal`]
///
/// Can be obtained via [`icu4x_fixed_decimal_create()`] and destroyed via [`icu4x_fixed_decimal_destroy()`]
pub type ICU4XFixedDecimal = FixedDecimal;

#[no_mangle]
/// FFI version of [`FixedDecimal`]'s constructors. This constructs a [`FixedDecimal`] of the provided
/// `number`.
//
// We can add additional constructors from strings, floats, etc as the need arises
pub extern "C" fn icu4x_fixed_decimal_create(number: i64) -> *mut ICU4XFixedDecimal {
    let fd = FixedDecimal::from(number);
    Box::into_raw(Box::new(fd))
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
