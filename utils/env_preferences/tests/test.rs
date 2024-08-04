// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(target_os = "linux")]
#[cfg(test)]
mod linux_tests {
    use std::collections::HashMap;

    use env_preferences::{get_locales, LocaleCategory};
    use icu_locale::Locale;

    #[test]
    // Testing fetching of locale, as `get_locales` fetches the locales for category
    // `LC_ALL`. For this category this should return non empty
    fn test_get_locales() {
        let locale_res = get_locales();

        match locale_res {
            Ok(locale_map) => {
                assert_eq!(locale_map.is_empty(), false);
                assert_eq!(locale_map.contains_key(&LocaleCategory::All), true);
            }
            Err(err) => {
                panic!("{:?}", err)
            }
        }
    }

    #[test]
    fn test_converting() {
        let mut locale_res: std::collections::HashMap<LocaleCategory, String> = HashMap::new();
        for locale in locale_res.into_values() {
            let parts: Vec<&str> = locale.split('.').collect();

            // Skipping "C" and those ending with "UTF-8", as they cannot be converted
            // into the locale
            if !parts.iter().any(|&part| part == "C")
                && (parts.len() > 1 && parts[parts.len() - 1] != "UTF-8")
            {
                let mut locale_converted: Locale = locale.parse().unwrap();
                locale_converted.extensions.unicode.clear();
                assert_eq!(locale_converted, locale.parse().unwrap());
            }
        }
    }

    #[test]
    fn test_calendar() {}
}
