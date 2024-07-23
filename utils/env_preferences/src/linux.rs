// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod linux_prefs {
    use std::{borrow::Cow, collections::HashMap, ffi::CStr, ptr};

    use libc::{setlocale, LC_ALL, LC_TIME};
    use std::str::FromStr;

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub enum LocaleCategory {
        LcCTYPE,
        LcNUMERIC,
        LcTIME,
        LcCOLLATE,
        LcMONETARY,
        LcMESSAGES,
        LcPAPER,
        LcNAME,
        LcADDRESS,
        LcTELEPHONE,
        LcMEASUREMENT,
        LcIDENTIFICATION,
        LcALL,
        Other(String),
    }

    impl FromStr for LocaleCategory {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "LC_CTYPE" => Ok(LocaleCategory::LcCTYPE),
                "LC_NUMERIC" => Ok(LocaleCategory::LcNUMERIC),
                "LC_TIME" => Ok(LocaleCategory::LcTIME),
                "LC_COLLATE" => Ok(LocaleCategory::LcCOLLATE),
                "LC_MONETARY" => Ok(LocaleCategory::LcMONETARY),
                "LC_MESSAGES" => Ok(LocaleCategory::LcMESSAGES),
                "LC_PAPER" => Ok(LocaleCategory::LcPAPER),
                "LC_NAME" => Ok(LocaleCategory::LcNAME),
                "LC_ADDRESS" => Ok(LocaleCategory::LcADDRESS),
                "LC_TELEPHONE" => Ok(LocaleCategory::LcTELEPHONE),
                "LC_MEASUREMENT" => Ok(LocaleCategory::LcMEASUREMENT),
                "LC_IDENTIFICATION" => Ok(LocaleCategory::LcIDENTIFICATION),
                "LC_ALL" => Ok(LocaleCategory::LcALL),
                _ => Ok(LocaleCategory::Other(s.to_string())),
            }
        }
    }

    pub fn get_locales() -> HashMap<LocaleCategory, String> {
        let mut locale_map = HashMap::new();

        // SAFETY: In case `libc::setlocale` returns a NULL pointer it fallbacks to the default locale "C"

        unsafe {
            let locales_ptr = setlocale(LC_ALL, ptr::null());
            if locales_ptr.is_null() {
                locale_map.insert(LocaleCategory::LcALL, "C".to_string());
                return locale_map;
            }

            // SAFETY: Creating a `CStr` from a non-null pointer and no mutation is being performed.

            let locales_cstr = CStr::from_ptr(locales_ptr);

            // SAFETY: Returns `&[str]` slice

            if let Ok(locales_str) = locales_cstr.to_str() {
                let locale_pairs = locales_str.split(';');

                // To handle cases in case a single locale is returned or a list of locale
                if locale_pairs.clone().count() == 1 {
                    locale_map.insert(LocaleCategory::LcALL, "C".to_string());
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
        }

        locale_map
    }

    pub fn get_system_calendars() -> impl Iterator<Item = (Cow<'static, str>, Cow<'static, str>)> {
        unsafe {
            let locale_ptr = setlocale(LC_TIME, ptr::null());

            // SAFETY: In case we get a `NULL` pointer for `LC_TIME` from `setlocale`, fallbacks
            // to default locale "C" and default calendar "Gregorian"

            if !locale_ptr.is_null() {
                // SAFETY: Creating a `CStr` from a non-null pointer and no mutation is being performed.

                let c_str = CStr::from_ptr(locale_ptr);
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
}
