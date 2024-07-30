// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: Add meaningful tests

#[cfg(target_os = "macos")]
#[cfg(test)]
mod apple_tests {
    use env_preferences::apple_prefs::{get_locales, get_system_calendars};

    #[test]
    fn test_get_locale() {
        print!("{:?}",get_locales());
    }

    #[test]
    fn test_get_calendar() {
        get_system_calendars();
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod linux_tests {
    use env_preferences::linux_prefs::get_locales;
    #[test]
    fn test_get_locale() {
        get_locales();
    }
}
