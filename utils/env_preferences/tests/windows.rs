// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use env_preferences::parse::windows::WindowsLocale;
use std::sync::{LazyLock, Mutex};
use windows::Win32::{
    Foundation::LPARAM,
    Globalization::{EnumSystemLocalesEx, LOCALE_ALL},
};
use windows_core::{BOOL, PCWSTR};

// Since [`EnumSystemLocalesEx`] iterates using a callback with no obvious (safe) way to return data,
// store them in this static instead. Since this is only a single test with roughly 1,000 items,
// it shouldn't be much of a concern.
static LOCALES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Callback provided to the [`EnumSystemLocalesEx`] to enumerate over locales.
unsafe extern "system" fn callback(
    locale_name: PCWSTR,
    _locale_flags: u32,
    _callback_parameter: LPARAM,
) -> BOOL {
    // SAFETY: caller is the [`EnumSystemLocalesEx`] function, which guarantees a valid null-terminated string
    let locale_name = unsafe { locale_name.to_string() }.unwrap();

    // Skip empty locale 0x007F, marked as "Reserved for invariant locale behavior"
    // Source: MS-LCID version 16.0, page 13 (section 2.2 under "Language ID" table)
    if !locale_name.is_empty() {
        LOCALES.lock().unwrap().push(locale_name);
    }

    // Tell [`EnumSystemLocalesEx`] to continue enumeration
    BOOL::from(true)
}

/// Enumerate over all Windows locales, and make sure [`WindowsLocale`] can parse it without any (direct) errors.
#[test]
fn system_locales() -> windows_core::Result<()> {
    // Find the list of supported locales, using the [`EnumSystemLocalesEx`] API:
    // https://learn.microsoft.com/en-us/windows/win32/api/winnls/nf-winnls-enumsystemlocalesex
    // SAFETY: a valid function pointer is provided and lpReserved is set to NULL/None as required
    unsafe {
        EnumSystemLocalesEx(Some(callback), LOCALE_ALL, LPARAM::default(), None)?;
    }

    // Get the list of locales which the callback has been modifying
    let locales = LOCALES.lock().unwrap();

    // Make sure [`WindowsLocale`] can parse without any obvious issues
    for locale in locales.iter() {
        let windows_locale = WindowsLocale::try_from_str(locale).expect(locale);
        windows_locale.try_convert_lossy().expect(locale);
    }

    Ok(())
}
