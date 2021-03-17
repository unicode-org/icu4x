// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::slice;

use icu_locid::Locale;

/// Opaque type for use behind a pointer, is [`Locale`]
///
/// Can be obtained via [`icu4x_locale_create()`] and destroyed via [`icu4x_locale_destroy()`]
pub type ICU4XLocale = Locale;

#[no_mangle]
/// FFI version of [`Locale::from_bytes()`], see its docs for more details
///
/// # Safety
/// `value` and `len` should point to a valid ASCII string of length `len`.
/// 
/// It does not need to be be null terminated, and `len` should not include a null
/// terminator (this will just cause the function to panic, and is not a safety requirement).
pub unsafe extern "C" fn icu4x_locale_create(value: *const u8, len: usize) -> *mut ICU4XLocale {
    let bytes = slice::from_raw_parts(value, len);
    // todo: return errors
    let loc = ICU4XLocale::from_bytes(bytes).unwrap();
    Box::into_raw(Box::new(loc))
}

#[no_mangle]
/// Destructor for [`ICU4XLocale`].
///
/// # Safety
///
/// `loc` must be a pointer to a locale allocated by `icu4x_locale_destroy`.
pub unsafe extern "C" fn icu4x_locale_destroy(loc: *mut ICU4XLocale) {
    let _ = Box::from_raw(loc);
}
