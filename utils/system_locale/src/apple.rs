// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod apple {
    use core_foundation_sys::{
        array::{CFArrayGetCount, CFArrayGetValueAtIndex},
        calendar::{CFCalendarCopyCurrent, CFCalendarCopyLocale, CFCalendarGetIdentifier},
        locale::{CFLocaleCopyPreferredLanguages, CFLocaleGetIdentifier},
        string::CFStringGetCStringPtr,
    };
    use std::ffi::CStr;

    pub fn get_locales_mac() -> Vec<String> {
        let mut languages: Vec<String> = Vec::new();

        unsafe {
            let locale_carr_ref = CFLocaleCopyPreferredLanguages();
            if !locale_carr_ref.is_null() {
                let count = CFArrayGetCount(locale_carr_ref as _);

                for i in 0..count {
                    let lang_ptr = CFArrayGetValueAtIndex(locale_carr_ref as _, i);
                    if !lang_ptr.is_null() {
                        let lang_str = CFStringGetCStringPtr(lang_ptr as _, 0);
                        if !lang_str.is_null() {
                            let lang_rust_str = CStr::from_ptr(lang_str)
                                .to_str()
                                .unwrap_or("Unknown")
                                .to_string();
                            languages.push(lang_rust_str);
                        }
                    }
                }
            }
        }

        // Defaulting to `und`
        if languages.is_empty() {
            languages.push(String::from("und"));
        }
        languages
    }

    pub fn get_system_calendars_macos() -> Vec<(String, String)> {
        let mut calendars = Vec::new();
        let mut calendar_locale_str = String::new();
        let mut calendar_identifier_str = String::new();

        unsafe {
            let calendar = CFCalendarCopyCurrent();
            if !calendar.is_null() {
                let locale = CFCalendarCopyLocale(calendar as _);
                let identifier = CFCalendarGetIdentifier(calendar as _);

                if !locale.is_null() {
                    let locale_identifier = CFLocaleGetIdentifier(locale);
                    let locale_cstr = CFStringGetCStringPtr(locale_identifier, 0);

                    if !locale_cstr.is_null() {
                        calendar_locale_str = CStr::from_ptr(locale_cstr)
                            .to_str()
                            .unwrap_or("")
                            .to_string();
                    }
                }

                if !identifier.is_null() {
                    let identifier_cstr = CFStringGetCStringPtr(identifier, 0);

                    if !identifier_cstr.is_null() {
                        calendar_identifier_str = CStr::from_ptr(identifier_cstr)
                            .to_str()
                            .unwrap_or("")
                            .to_string();
                    }
                }

                calendars.push((calendar_locale_str, calendar_identifier_str.clone()));
            }
        }

        if calendars.is_empty() {
            calendars.push((String::from("und"), String::from("Gregorian")));
        }

        calendars
    }
}
