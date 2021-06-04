// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::locale::ICU4XLocale;
use crate::provider::ICU4XDataProvider;
use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
use std::ptr;

/// Opaque type for use behind a pointer, is [`LocaleCanonicalizer`]
///
/// Can be obtained via [`icu4x_localecanonicalizer_create()`] and
/// destroyed via [`icu4x_localecanonicalizer_destroy()`]
pub type ICU4XLocaleCanonicalizer<'d, 's> = LocaleCanonicalizer<'d, 'static>;

#[repr(C)]
c_enum! {
    /// FFI version of [`CanonicalizationResult`]. See its docs for more details.
    pub c_enum ICU4XCanonicalizationResult is CanonicalizationResult {
        Modified,
        Unmodified,
    }
}

#[no_mangle]
/// FFI version of [`LocaleCanonicalizer::new()`], see its docs for more details
///
/// # Safety
/// - `provider` should be constructed via one of the functions in [`crate::provider`](crate::provider)
pub unsafe extern "C" fn icu4x_localecanonicalizer_create(
    provider: &ICU4XDataProvider,
) -> *mut ICU4XLocaleCanonicalizer {
    let provider = provider.as_dyn_ref();
    if let Ok(lc) = ICU4XLocaleCanonicalizer::new(provider) {
        let boxed = Box::new(lc);
        Box::into_raw(boxed)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
/// FFI version of [`LocaleCanonicalizer::canonicalize()`]. See its docs for more details.
pub extern "C" fn icu4x_localecanonicalizer_canonicalize(
    lc: &ICU4XLocaleCanonicalizer,
    locale: &mut ICU4XLocale,
) -> ICU4XCanonicalizationResult {
    lc.canonicalize(locale).into()
}

#[no_mangle]
/// FFI version of [`LocaleCanonicalizer::maximize()`]. See its docs for more details.
pub extern "C" fn icu4x_localecanonicalizer_maximize(
    lc: &ICU4XLocaleCanonicalizer,
    locale: &mut ICU4XLocale,
) -> ICU4XCanonicalizationResult {
    lc.maximize(locale).into()
}

#[no_mangle]
/// FFI version of [`LocaleCanonicalizer::minimize()`]. See its docs for more details.
pub extern "C" fn icu4x_localecanonicalizer_minimize(
    lc: &ICU4XLocaleCanonicalizer,
    locale: &mut ICU4XLocale,
) -> ICU4XCanonicalizationResult {
    lc.minimize(locale).into()
}

#[no_mangle]
/// Destructor for [`ICU4XLocaleCanonicalizer`].
///
/// # Safety
///
/// `lc` must be a pointer to a locale allocated by `icu4x_localecanonicalizer_create`.
pub unsafe extern "C" fn icu4x_localecanonicalizer_destroy(lc: *mut ICU4XLocaleCanonicalizer) {
    let _ = Box::from_raw(lc);
}
