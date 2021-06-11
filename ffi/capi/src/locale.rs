// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::custom_writeable::ICU4XWriteable;
use icu_locid::extensions::unicode::Key;
use icu_locid::{subtags, LanguageIdentifier, Locale};
use std::slice;
use tinystr::tinystr4;

/// Opaque type for use behind a pointer, is [`Locale`]
///
/// Can be obtained via [`icu4x_locale_create()`] and destroyed via [`icu4x_locale_destroy()`]
pub type ICU4XLocale = Locale;

macro_rules! write_extension {
    ( $key:expr, $locale: ident, $write: ident ) => {{
        use writeable::Writeable;
        let key = Key::from_tinystr4_unchecked($key);
        if let Some(value) = $locale.extensions.unicode.keywords.get(key) {
            let result = value.write_to($write).is_ok();
            $write.flush();
            if result {
                ICU4XLocaleResult::Ok
            } else {
                ICU4XLocaleResult::Error
            }
        } else {
            ICU4XLocaleResult::Undefined
        }
    }};
}

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
/// Write a string representation of the basename to `write`
/// This is the LanguageIdentifier portion of the id, without variants.
pub extern "C" fn icu4x_locale_basename(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    use writeable::Writeable;

    let langid = LanguageIdentifier {
        language: locale.id.language,
        script: locale.id.script,
        region: locale.id.region,
        variants: subtags::Variants::default(),
    };

    let result = langid.write_to(write).is_ok();
    write.flush();
    if result {
        ICU4XLocaleResult::Ok
    } else {
        ICU4XLocaleResult::Error
    }
}

#[no_mangle]
/// Write a string representation of the calendar extension to `write`
pub extern "C" fn icu4x_locale_calendar(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    write_extension!(tinystr4!("ca"), locale, write)
}

#[no_mangle]
/// Write a string representation of the case first extension to `write`
pub extern "C" fn icu4x_locale_case_first(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    write_extension!(tinystr4!("kf"), locale, write)
}

#[no_mangle]
/// Write a string representation of the collation extension to `write`
pub extern "C" fn icu4x_locale_collation(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    write_extension!(tinystr4!("co"), locale, write)
}

#[no_mangle]
/// Write a string representation of the hour cycle extension to `write`
pub extern "C" fn icu4x_locale_hour_cycle(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    write_extension!(tinystr4!("hc"), locale, write)
}

#[no_mangle]
/// Write a string representation of the numeric extension to `write`
pub extern "C" fn icu4x_locale_numeric(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    write_extension!(tinystr4!("kn"), locale, write)
}

#[no_mangle]
/// Write a string representation of the number system extension to `write`
pub extern "C" fn icu4x_locale_number_system(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    write_extension!(tinystr4!("nu"), locale, write)
}

#[no_mangle]
/// Write a string representation of [`ICU4XLocale`] language to `write`
pub extern "C" fn icu4x_locale_language(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    use writeable::Writeable;

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
    use writeable::Writeable;

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
    use writeable::Writeable;

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
/// Write a string representation of the [`LanguageIdentifier`] part of
/// [`ICU4XLocale`] to `write`.
pub extern "C" fn icu4x_locale_langid_tostring(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    use writeable::Writeable;

    let result = locale.id.write_to(write).is_ok();
    write.flush();
    if result {
        ICU4XLocaleResult::Ok
    } else {
        ICU4XLocaleResult::Error
    }
}

#[no_mangle]
/// Write a string representation of [`ICU4XLocale`] to `write`
pub extern "C" fn icu4x_locale_tostring(
    locale: &ICU4XLocale,
    write: &mut ICU4XWriteable,
) -> ICU4XLocaleResult {
    use writeable::Writeable;

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
