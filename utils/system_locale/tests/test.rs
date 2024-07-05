// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// The test is just here to silence the compiler warning, meaningful test will be added later

#[cfg(target_os = "macos")]
#[cfg(test)]
mod apple_tests {
    use system_locale::apple::apple_locales_prefs::{get_locales_mac, get_system_calendars_macos};

    #[test]
    fn test_get_locale() {
        get_locales_mac();
    }

    #[test]
    fn test_get_calendar() {
        get_system_calendars_macos();
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod linux_tests {
    use system_locale::linux::linux_locales_prefs::get_locales_linux;
    #[test]
    fn test_get_locale() {
        get_locales_linux();
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod linux_tests {

    #[test]
    fn test_get_locale() {}

    #[test]
    fn test_get_calendar() {}
}
