// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::custom_writeable::ICU4XWriteable;
use icu_locid::extensions::unicode::Key;
use icu_locid::Locale;
use std::slice;
use writeable::Writeable;

/// Opaque type for use behind a pointer, is [`Locale`]
///
/// Can be obtained via [`icu4x_locale_create()`] and destroyed via [`icu4x_locale_destroy()`]
pub type ICU4XLocale = Locale;

#[repr(C)]
pub enum ICU4XLocaleResult {
    Ok,
    Undefined,
    Error,
}

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
    if let Ok(loc) = ICU4XLocale::from_bytes(bytes) {
        Box::into_raw(Box::new(loc))
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`Locale::clone()`]
pub extern "C" fn icu4x_locale_clone(locale: &ICU4XLocale) -> *mut ICU4XLocale {
    let clone = locale.clone();
    Box::into_raw(Box::new(clone))
}

#[no_mangle]
/// Write a string representation of the [`LanguageIdentifier`] part of
/// [`ICU4XLocale`] to `write`.
pub extern "C" fn icu4x_locale_basename(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    let result = locale.id.write_to(write).is_ok();
    write.flush();
    if result {
        ICU4XLocaleResult::Ok
    } else {
        ICU4XLocaleResult::Error
    }
}

#[no_mangle]
/// Write a string representation of the unicode extension to `write`
///
/// # Safety
/// `value` and `len` should point to a valid ASCII string of length `len`.
///
/// It does not need to be be null terminated, and `len` should not include a null
/// terminator (this will just cause the function to panic, and is not a safety requirement).
pub unsafe extern "C" fn icu4x_locale_get_unicode_extension(
    locale: &ICU4XLocale,
    value: *const u8,
    len: usize,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    let bytes = slice::from_raw_parts(value, len);
    if let Ok(key) = Key::from_bytes(bytes) {
        if let Some(value) = locale.get_unicode_extension(&key) {
            let result = value.write_to(write).is_ok();
            write.flush();
            if result {
                ICU4XLocaleResult::Ok
            } else {
                ICU4XLocaleResult::Error
            }
        } else {
            ICU4XLocaleResult::Undefined
        }
    } else {
        ICU4XLocaleResult::Error
    }
}

#[no_mangle]
/// Write a string representation of [`ICU4XLocale`] language to `write`
pub extern "C" fn icu4x_locale_language(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    let result = locale.id.language.write_to(write).is_ok();
    write.flush();
    if result {
        ICU4XLocaleResult::Ok
    } else {
        ICU4XLocaleResult::Error
    }
}

#[no_mangle]
/// Write a string representation of [`ICU4XLocale`] region to `write`
pub extern "C" fn icu4x_locale_region(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    match locale.id.region {
        Some(region) => {
            let result = region.write_to(write).is_ok();
            write.flush();
            if result {
                ICU4XLocaleResult::Ok
            } else {
                ICU4XLocaleResult::Error
            }
        }
        None => ICU4XLocaleResult::Undefined,
    }
}

#[no_mangle]
/// Write a string representation of [`ICU4XLocale`] script to `write`
pub extern "C" fn icu4x_locale_script(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    match locale.id.script {
        Some(script) => {
            let result = script.write_to(write).is_ok();
            write.flush();
            if result {
                ICU4XLocaleResult::Ok
            } else {
                ICU4XLocaleResult::Error
            }
        }
        None => ICU4XLocaleResult::Undefined,
    }
}

#[no_mangle]
/// Write a string representation of [`ICU4XLocale`] to `write`
pub extern "C" fn icu4x_locale_tostring(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    let result = locale.write_to(write).is_ok();
    write.flush();
    if result {
        ICU4XLocaleResult::Ok
    } else {
        ICU4XLocaleResult::Error
    }
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
