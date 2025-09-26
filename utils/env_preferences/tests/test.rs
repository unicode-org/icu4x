// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod datasets;
mod parse;

#[cfg(target_os = "linux")]
#[cfg(test)]
mod linux_tests {
    use env_preferences::posix::{get_raw_locale_categories, get_system_calendars, LocaleCategory};
    use env_preferences::RetrievalError;
    use icu_locale_core::Locale;
    use libc::setlocale;

    // Testing fetching of locale, as `get_locales` fetches the locales for category
    // `LC_ALL`. For this category this should return non empty
    #[test]
    fn test_get_raw_locale_categories() {
        let locale_res = get_raw_locale_categories().unwrap();
        assert!(
            !locale_res.is_empty(),
            "Empty hashmap for locales retrieved"
        );
        for locale in locale_res.into_values() {
            assert!(locale.is_ascii(), "Invalid form of locale retrieved")
        }
    }

    #[test]
    fn test_converting_locales() {
        let locale_res: std::collections::HashMap<LocaleCategory, String> =
            get_raw_locale_categories().unwrap();
        for locale in locale_res.into_values() {
            let parts: Vec<&str> = locale.split('.').collect();

            // Skipping "C" and those ending with "UTF-8", as they cannot be converted
            // into the locale
            if !parts.contains(&"C") && (parts.len() > 1 && parts[parts.len() - 1] != "UTF-8") {
                let mut locale_converted: Locale = locale.parse().unwrap();
                locale_converted.extensions.unicode.clear();
                assert_eq!(locale_converted, locale.parse().unwrap());
            }
        }
    }

    // This test contains unsafe code, the idea is to manually set a locale for `LC_TIME`,
    // compare the result from `get_locales` and `get_system_calendar` they must be equal
    #[test]
    fn test_calendar() {
        // Using "C" locale since it is the default, using any other locale like `en_IN` or `en_US`
        // may work on some system and may not others depending on the availability
        let test_calendar_locale = "C";
        let locale_cstr =
            std::ffi::CString::new(test_calendar_locale).expect("CString::new failed");

        // SAFETY: This call is safe because any subsequent call to `setlocale` we pass a `NULL` locale
        // to retrieve locale which does not sets the locale. The test locale `locale_cstr` is a CString
        // nul terminated string for which we have the ownership
        let tr = unsafe { setlocale(libc::LC_TIME, locale_cstr.as_ptr()) };

        if tr.is_null() {
            panic!("{:?}", RetrievalError::NullLocale);
        }

        let calendar_locale = get_system_calendars().unwrap();
        assert_eq!(test_calendar_locale.to_string(), calendar_locale);
    }
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod macos_test {
    use env_preferences::apple::{get_raw_locales, get_system_calendars, get_system_time_zone};
    use icu_locale_core::Locale;

    #[test]
    fn test_get_raw_locales() {
        let locales_res = get_raw_locales();
        match locales_res {
            Ok(locales) => {
                for locale in locales {
                    assert!(!locale.is_empty(), "Empty locale retrieved");
                    assert!(locale.is_ascii(), "Invalid form of locale retrieved");
                }
            }
            Err(e) => {
                panic!("{e:?}")
            }
        }
    }

    #[test]
    fn test_converting_locales() {
        let locales = get_raw_locales().unwrap();
        for locale in locales {
            let _loc: Locale = locale.parse().unwrap();
        }
    }

    #[test]
    fn test_calendar() {
        let calendar_res = get_system_calendars().unwrap();
        for calendar in calendar_res {
            assert!(!calendar.0.is_empty(), "Couldn't retreive calendar locale");
            assert!(calendar.0.is_ascii(), "Calendar locale form is not valid");
            assert!(!calendar.1.is_empty(), "Couldn't retreive calendar");
            assert!(
                calendar.1.is_ascii(),
                "Calendar identifier form is not valid"
            );
        }
    }

    #[test]
    fn test_time_zone() {
        let time_zone = get_system_time_zone().unwrap();
        assert!(!time_zone.is_empty(), "Couldn't retreive time_zone");
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod windows_test {
    use env_preferences::parse::windows::WindowsLocale;
    use env_preferences::windows::{get_raw_locales, get_system_calendars, get_system_time_zone};
    use icu_locale_core::Locale;
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

    #[test]
    fn test_get_raw_locales() {
        let locales = get_raw_locales().unwrap();
        for locale in locales {
            assert!(!locale.is_empty(), "Empty locale retrieved");
            assert!(locale.is_ascii(), "Invalid form of locale retrieved");
        }
    }

    #[test]
    fn test_converting_locales() {
        let locales = get_raw_locales().unwrap();
        for locale in locales {
            let _converted_locale: Locale = locale.parse().unwrap();
        }
    }

    #[test]
    fn test_calendar() {
        let calendars = get_system_calendars().unwrap();
        for calendar in calendars {
            assert!(!calendar.0.is_empty(), "Calendar locale is empty");
            assert!(calendar.0.is_ascii(), "Calendar locale form is not valid");
            assert!(!calendar.1.is_empty(), "Calendar identifier is empty");
            assert!(
                calendar.1.is_ascii(),
                "Calendar identifier form is not valid"
            );
        }
    }

    #[test]
    fn test_time_zone() {
        let time_zone = get_system_time_zone().unwrap();
        assert!(!time_zone.is_empty(), "Couldn't retreive time_zone");
        assert!(time_zone.is_ascii(), "Invalid TimeZone format");
    }
}
