// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core_foundation_sys::{
    array::{CFArrayGetCount, CFArrayGetValueAtIndex},
    base::{CFIndex, CFRelease, CFRetain},
    calendar::{CFCalendarCopyCurrent, CFCalendarCopyLocale, CFCalendarGetIdentifier},
    locale::{CFLocaleCopyPreferredLanguages, CFLocaleGetIdentifier},
    string::{
        kCFStringEncodingUTF8, CFStringGetCString, CFStringGetCStringPtr, CFStringGetLength,
        CFStringRef,
    },
    timezone,
};
use libc::c_char;
use std::ffi::{CStr, CString};

use crate::RetrievalError;

/// Helps to get string, it tries to get the string directly from the pointer itself, in case it is unable to retrieve
/// the string (c_str_ptr is NULL) a buffer is created of size `length + 1` and we perform manual allocations to get
/// the string
fn get_string(ptr: CFStringRef) -> Result<String, RetrievalError> {
    // SAFETY: The call to `CFStringGetCStringPtr` because the reference of string we are accessing is not `NULL`
    // Returns pointer in O(1) without any memory allocation. This can return NULL so we are handling it by directly
    // copying it using `CFStringGetCString`
    let c_str_ptr: *const c_char = unsafe { CFStringGetCStringPtr(ptr, kCFStringEncodingUTF8) };

    if !c_str_ptr.is_null() {
        // SAFETY: A valid `NULL` terminator is present which is a requirement of `from_ptr`
        let lang_rust_str = unsafe { CStr::from_ptr(c_str_ptr) }.to_str()?;
        Ok(lang_rust_str.to_string())
    } else {
        // `c_str_ptr` is null, i.e. `CFStringGetCStringPtr` couldn't give desired output, trying with
        // manual allocations
        // SAFETY: It returns length of the string, from above conditional statement we ensure
        // that the `lang_ptr` is not NULL thus making it safe to call
        let length = unsafe { CFStringGetLength(ptr) as usize };

        let mut c_str_buf: Vec<u8> = vec![0; length + 1];

        // SAFETY: Safety is ensured by following points
        // 1. `lang_ptr` is not NULL, checked through conditional statement
        // 2. `c_str_buf` is large enough and in scope after this call
        unsafe {
            CFStringGetCString(
                ptr,
                c_str_buf.as_mut_ptr() as *mut c_char,
                c_str_buf.len() as CFIndex,
                kCFStringEncodingUTF8,
            );
        }

        let c_string = CString::from_vec_with_nul(c_str_buf)?;
        c_string
            .into_string()
            .map_err(|e| RetrievalError::ConversionError(e.utf8_error()))
    }
}

pub fn get_locales() -> Result<Vec<String>, RetrievalError> {
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
            let lang_ptr = unsafe { CFArrayGetValueAtIndex(locale_carr_ref, i) };

            if !lang_ptr.is_null() {
                let locale_str = get_string(lang_ptr as CFStringRef)?;
                languages.push(locale_str);
            } else {
                return Err(RetrievalError::NullLocale);
            }
        }
    } else {
        // No need to release memory for `locale_carr_ref` since it is NULL
        return Err(RetrievalError::NullLocale);
    }
    // Release for memory
    unsafe {
        CFRelease(locale_carr_ref as _);
    }

    Ok(languages)
}

pub fn get_system_calendars() -> Result<Vec<(String, String)>, RetrievalError> {
    let mut calendars = Vec::new();
    let calendar_locale_str: String;
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
            calendar_locale_str = get_string(locale_identifier as CFStringRef)?;

            // SAFETY: Releases the locale object which was retained
            unsafe { CFRelease(locale as _) };
        } else {
            return Err(RetrievalError::NullLocale);
        }

        if !identifier.is_null() {
            calendar_identifier_str = get_string(identifier as CFStringRef)?;
        }
        // SAFETY: Release the calendar when done to avoid memory leaks
        unsafe { CFRelease(calendar as _) };

        calendars.push((calendar_locale_str, calendar_identifier_str));
    } else {
        return Err(RetrievalError::NullCalendar);
    }

    Ok(calendars)
}

pub fn get_system_timezone() -> Result<String, RetrievalError> {
    // SAFETY: Returns the time zone currently used by the system
    // Returns an immutable reference to TimeZone object owned by us
    let timezone = unsafe { timezone::CFTimeZoneCopySystem() };

    if !timezone.is_null() {
        // SAFETY: Extracts name of time zone from the TimeZone object, reference to timezone
        // is guaranteed to be not NULL
        let cf_string = unsafe { timezone::CFTimeZoneGetName(timezone) };

        if !cf_string.is_null() {
            return get_string(cf_string);
        }
    }
    Err(RetrievalError::NullTimeZone)
}
