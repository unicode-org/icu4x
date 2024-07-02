#[cfg(target_os = "macos")]
mod macos_locale {
    use core_foundation_sys::{
        array::{CFArrayGetCount, CFArrayGetValueAtIndex},
        calendar::{CFCalendarCopyCurrent, CFCalendarCopyLocale, CFCalendarGetIdentifier},
        locale::{CFLocaleCopyPreferredLanguages, CFLocaleGetIdentifier},
        string::CFStringGetCStringPtr,
    };
    use std::ffi::CStr;

    pub fn get_locale() -> Vec<String> {
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
        languages
    }

    pub fn get_system_calendar() -> Vec<(String, String)> {
        unsafe {
            let calendar = CFCalendarCopyCurrent();
            if !calendar.is_null() {
                let locale = CFCalendarCopyLocale(calendar as _);
                let identifier = CFCalendarGetIdentifier(calendar as _);

                let mut locale_str = String::new();
                let mut identifier_str = String::new();

                if !locale.is_null() {
                    let locale_identifier = CFLocaleGetIdentifier(locale);
                    let locale_cstr = CFStringGetCStringPtr(locale_identifier, 0);

                    if !locale_cstr.is_null() {
                        locale_str = CStr::from_ptr(locale_cstr)
                            .to_str()
                            .unwrap_or("")
                            .to_string();
                    }
                }

                if !identifier.is_null() {
                    let identifier_cstr = CFStringGetCStringPtr(identifier, 0);

                    if !identifier_cstr.is_null() {
                        identifier_str = CStr::from_ptr(identifier_cstr)
                            .to_str()
                            .unwrap_or("")
                            .to_string();
                    }
                }

                if !locale_str.is_empty() && !identifier_str.is_empty() {
                    return vec![(locale_str, identifier_str)];
                }
            }
        }

        vec![]
    }
}

#[cfg(target_os = "macos")]
pub use macos_locale::get_locale;
pub use macos_locale::get_system_calendar;
