// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::slice;

// opaque type
pub use icu_locid::Locale as ICU4XLocale;

#[no_mangle]
pub unsafe extern "C" fn icu4x_locale_create(value: *const u8, len: usize) -> *mut ICU4XLocale {
    let bytes = slice::from_raw_parts(value, len);
    // todo: return errors
    let loc = ICU4XLocale::from_bytes(bytes).unwrap();
    Box::into_raw(Box::new(loc))
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_locale_destroy(loc: *mut ICU4XLocale) {
    let _ = Box::from_raw(loc);
}
