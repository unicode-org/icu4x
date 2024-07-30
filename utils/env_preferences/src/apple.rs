// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod apple_prefs {
    use core_foundation_sys::{
        array::{CFArrayGetCount, CFArrayGetValueAtIndex},
        base::{CFRelease, CFRetain},
        calendar::{CFCalendarCopyCurrent, CFCalendarCopyLocale, CFCalendarGetIdentifier},
        locale::{CFLocaleCopyPreferredLanguages, CFLocaleGetIdentifier},
        string::CFStringGetCStringPtr,
    };
    use std::ffi::CStr;

    pub fn get_locales() -> Vec<String> {
        let mut languages: Vec<String> = Vec::new();

        // SAFETY: The call to `CFLocaleCopyPreferredLanguages` returns an immutable reference to `CFArray` which is owned by us
        // https://developer.apple.com/documentation/corefoundation/cfarrayref. It is ensured that `locale_carr_ref` is not mutated
        // Immutablility ensures that nothing is overriden during it's scope
        let locale_carr_ref = unsafe { CFLocaleCopyPreferredLanguages() };

        if !locale_carr_ref.is_null() {
            // SAFETY: The call to `CFArrayGetCount` is only made when is `locale_carr_ref` is not `NULL`
            let count = unsafe { CFArrayGetCount(locale_carr_ref as _) };
            for i in 0..count {
                // SAFETY: The call to `CFArrayGetValueAtIndex` is safe because we are iterating from 0 to count (`CFArrayGetCount`) which
                // in itself will always be greater than 0 and less than count ensuring we will not get "out of bounds" error
                let lang_ptr = unsafe { CFArrayGetValueAtIndex(locale_carr_ref as _, i) };

                if !lang_ptr.is_null() {
                    // SAFETY: The call to `CFStringGetCStringPtr` because the reference of string we are accessing is not `NULL`
                    // Returns pointer in O(1) without any memory allocation. This can return NULL so to handle it
                    let lang_str = unsafe { CFStringGetCStringPtr(lang_ptr as _, 0) };

                    if !lang_str.is_null() {
                        // SAFETY: A valid `NULL` terminator is present which is a requirement of `from_ptr`
                        let lang_rust_str = unsafe { CStr::from_ptr(lang_str) }
                            .to_str()
                            .unwrap_or("Unknown")
                            .to_string();
                        languages.push(lang_rust_str);
                    }

                    // TODO: In case `lang_str` is `NULL` try retrieving the string using `CFStringGetCString`
                    // Ref: https://developer.apple.com/documentation/corefoundation/1542721-cfstringgetcstring?language=objc
                    // Note: Not optimal and may give inconsistent results if buffer is not large enough, must add sanity checks
                    // whenever implemented
                }
            }
        }
        // Release for memory
        unsafe {CFRelease(locale_carr_ref as _ );}

        // Defaulting to `und`
        if languages.is_empty() {
            languages.push(String::from("und"));
        }
        languages
    }

    pub fn get_system_calendars() -> Vec<(String, String)> {
        let mut calendars = Vec::new();
        let mut calendar_locale_str = String::new();
        let mut calendar_identifier_str = String::new();

        // SAFETY: The call to `CFCalendarCopyCurrent` returns a calendar object owned by us
        // This calendar object is used extract locale and type of calendar (identifier)
        let calendar = unsafe { CFCalendarCopyCurrent() };

        if !calendar.is_null() {
            // SAFETY: Retaining the calendar object when not `NULL`
            // It is released when all actions are completed
            unsafe { CFRetain(calendar as _) };

            // SAFETY: Retrieves `CFLocale` object for the calendar, the `if` statement ensures we don't
            // pass in a `NULL` references
            let locale = unsafe { CFCalendarCopyLocale(calendar as _) };

            // SAFETY: Retrieves `CFString` (identifier) for the calendar, the `if` statement ensures
            // we don't pass in a `NULL` references
            let identifier = unsafe { CFCalendarGetIdentifier(calendar as _) };

            if !locale.is_null() {
                // SAFETY: Retain the locale object, released when we extracted the string
                unsafe { CFRetain(locale as _) };

                // SAFETY: Retrieves `CFString` (identifier) for the calendar, the `if` statement ensures
                // we don't pass in a `NULL` reference
                let locale_identifier = unsafe { CFLocaleGetIdentifier(locale) };

                // SAFETY: The call to `CFStringGetCStringPtr` because the reference of string we are accessing is not `NULL`
                // Returns pointer in O(1) without any memory allocation. This can return NULL so to handle it
                let locale_cstr = unsafe { CFStringGetCStringPtr(locale_identifier, 0) };

                if !locale_cstr.is_null() {
                    // SAFETY: A valid `NULL` terminator is present which is a requirement of `from_ptr`
                    calendar_locale_str = unsafe { CStr::from_ptr(locale_cstr) }
                        .to_str()
                        .unwrap_or("Unknown")
                        .to_string();
                }

                // SAFETY: Releases the locale object which was retained
                unsafe { CFRelease(locale as _) };
            }

            if !identifier.is_null() {
                // SAFETY: The call to `CFStringGetCStringPtr` because the reference of string we are accessing is not `NULL`
                // Returns pointer in O(1) without any memory allocation. This can return NULL so to handle it
                let identifier_cstr = unsafe { CFStringGetCStringPtr(identifier, 0) };

                if !identifier_cstr.is_null() {
                    // SAFETY: A valid `NULL` terminator is present which is a requirement of `from_ptr`
                    calendar_identifier_str = unsafe { CStr::from_ptr(identifier_cstr) }
                        .to_str()
                        .unwrap_or("Unknown")
                        .to_string();
                }
            }

            // SAFETY: Release the calendar when done to avoid memory leaks
            unsafe { CFRelease(calendar as _) };

            calendars.push((calendar_locale_str, calendar_identifier_str));
        }

        if calendars.is_empty() {
            calendars.push((String::from("und"), String::from("Gregorian")));
        }

        calendars
    }
}
