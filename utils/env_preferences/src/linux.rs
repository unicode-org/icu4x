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

        unsafe {
            // Thread safety is ensured by fallbacking to the default locale of `linux` which is `C`
            let locales_ptr = libc::setlocale(LC_ALL, ptr::null());
            if locales_ptr.is_null() {
                locale_map.insert(LocaleCategory::LcALL, "C".to_string());
                return locale_map;
            }

            let locales_cstr = CStr::from_ptr(locales_ptr);
            if let Ok(locales_str) = locales_cstr.to_str() {
                let locale_pairs = locales_str.split(';');
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
            } else {
                locale_map.insert(LocaleCategory::LcALL, "C".to_string());
            }
        }

        locale_map
    }

    pub fn get_system_calendars_linux(
    ) -> Box<dyn Iterator<Item = (Cow<'static, str>, Cow<'static, str>)>> {
        unsafe {
            let locale_ptr = setlocale(LC_TIME, ptr::null());
            if !locale_ptr.is_null() {
                let c_str = CStr::from_ptr(locale_ptr);
                if let Ok(str_slice) = c_str.to_str() {
                    // `gnome-calendar` is the default calendar and it only supports `Gregorian`.
                    // Related issue: https://gitlab.gnome.org/GNOME/gnome-calendar/-/issues/998
                    return Box::new(
                        Some((
                            Cow::Owned(str_slice.to_string()),
                            Cow::Borrowed("Gregorian"),
                        ))
                        .into_iter(),
                    );
                }
            }
            Box::new(
                None.into_iter()
                    .chain(Some((Cow::Borrowed("C"), Cow::Borrowed("Gregorian")))),
            )
        }
    }
}
