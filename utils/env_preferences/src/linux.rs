// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod linux_prefs {
    use std::{borrow::Cow, collections::HashMap, ffi::CStr, ptr};

    use libc::{setlocale, LC_ALL, LC_TIME};
    use std::str::FromStr;

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub enum LocaleCategory {
        Character,
        Number,
        Time,
        Collate,
        Monetary,
        Messages,
        Paper,
        Name,
        Address,
        Telephone,
        Measurement,
        Identification,
        All,
        Other(String),
    }

    impl FromStr for LocaleCategory {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "LC_CTYPE" => Ok(LocaleCategory::Character),
                "LC_NUMERIC" => Ok(LocaleCategory::Number),
                "LC_TIME" => Ok(LocaleCategory::Time),
                "LC_COLLATE" => Ok(LocaleCategory::Collate),
                "LC_MONETARY" => Ok(LocaleCategory::Monetary),
                "LC_MESSAGES" => Ok(LocaleCategory::Messages),
                "LC_PAPER" => Ok(LocaleCategory::Paper),
                "LC_NAME" => Ok(LocaleCategory::Name),
                "LC_ADDRESS" => Ok(LocaleCategory::Address),
                "LC_TELEPHONE" => Ok(LocaleCategory::Telephone),
                "LC_MEASUREMENT" => Ok(LocaleCategory::Measurement),
                "LC_IDENTIFICATION" => Ok(LocaleCategory::Identification),
                "LC_ALL" => Ok(LocaleCategory::All),
                _ => Ok(LocaleCategory::Other(s.to_string())),
            }
        }
    }

    pub fn get_locales() -> HashMap<LocaleCategory, String> {
        let mut locale_map = HashMap::new();

        let locales_ptr = unsafe { setlocale(LC_ALL, ptr::null()) };

        if locales_ptr.is_null() {
            locale_map.insert(LocaleCategory::All, "C".to_string());
            return locale_map;
        }

        let locales_cstr = unsafe { CStr::from_ptr(locales_ptr) };

        if let Ok(locales_str) = locales_cstr.to_str() {
            let locale_pairs = locales_str.split(';');

            // To handle cases in case a single locale is returned or a list of locale
            if locale_pairs.clone().count() == 1 {
                locale_map.insert(LocaleCategory::All, "C".to_string());
            } else {
                for locale_pair in locale_pairs {
                    let mut parts = locale_pair.split('=');
                    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                        if let Ok(category) = LocaleCategory::from_str(key) {
                            locale_map.insert(category, value.to_string());
                        }
                    }
                }
            }
        }

        locale_map
    }

    pub fn get_system_calendars() -> impl Iterator<Item = (Cow<'static, str>, Cow<'static, str>)> {
        let locale_ptr = unsafe { setlocale(LC_TIME, ptr::null()) };

        if !locale_ptr.is_null() {
            let c_str = unsafe { CStr::from_ptr(locale_ptr) };

            if let Ok(str_slice) = c_str.to_str() {
                // `gnome-calendar` is the default calendar and it only supports `Gregorian`.
                // Related issue: https://gitlab.gnome.org/GNOME/gnome-calendar/-/issues/998
                return Some((
                    Cow::Owned(str_slice.to_string()),
                    Cow::Borrowed("Gregorian"),
                ))
                .into_iter()
                .chain(None);
            }
        }
        Some((Cow::Borrowed("C"), Cow::Borrowed("Gregorian")))
            .into_iter()
            .chain(None)
    }
}
