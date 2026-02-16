// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core_foundation::{
    array::CFArray,
    base::{kCFAllocatorDefault, CFGetTypeID, CFTypeRef, TCFType},
    dictionary::{
        CFDictionaryGetCount, CFDictionaryGetKeysAndValues, CFDictionaryGetTypeID, CFDictionaryRef,
    },
    number::{kCFNumberSInt32Type, CFNumberGetTypeID, CFNumberGetValue},
    string::{CFString, CFStringGetMaximumSizeForEncoding},
};
use core_foundation_sys::{
    base::CFRelease,
    calendar::{CFCalendarCopyCurrent, CFCalendarGetIdentifier},
    date_formatter::CFDateFormatterCreateDateFormatFromTemplate,
    locale::{
        kCFLocaleCountryCode, kCFLocaleMeasurementSystem, CFLocaleCopyCurrent,
        CFLocaleCopyPreferredLanguages, CFLocaleGetValue, CFLocaleRef,
    },
    preferences::{kCFPreferencesAnyHost, kCFPreferencesCurrentUser, CFPreferencesCopyValue},
    string::{kCFStringEncodingUTF8, CFStringGetCString, CFStringGetCStringPtr, CFStringRef},
};
use icu_locale_core::{
    extensions::unicode,
    preferences::extensions::unicode::keywords::{
        CalendarAlgorithm, CollationType, HourCycle, MeasurementSystem,
    },
    subtags::Language,
};
use std::os::raw::c_char;
use std::{ffi::CStr, str::FromStr};

use crate::{
    backends::{HostInfoBackend, RawHostInfoBackend},
    error::HostInfoError,
};

pub struct MacOSHostInfoBackend;

impl HostInfoBackend for MacOSHostInfoBackend {
    fn calendar() -> Result<Option<CalendarAlgorithm>, HostInfoError> {
        Ok(Self::raw_calendar()?
            .and_then(|raw| {
                let canonical = match raw.as_str() {
                    "gregorian" => "gregory",
                    r => r,
                };
                unicode::Value::from_str(canonical).ok()
            })
            .and_then(|value| CalendarAlgorithm::try_from(&value).ok()))
    }

    fn hour_cycle() -> Result<Option<HourCycle>, HostInfoError> {
        with_current_locale(|locale| {
            let template = CFString::new("j"); // hour-cycle probe

            // SAFETY: All parameters are valid - kCFAllocatorDefault is a system constant,
            // template is a valid CFStringRef, locale is non-null, and 0 is a valid options value.
            let format = unsafe {
                CFDateFormatterCreateDateFormatFromTemplate(
                    kCFAllocatorDefault,
                    template.as_concrete_TypeRef(),
                    0,
                    locale,
                )
            };

            if format.is_null() {
                return None;
            }

            // SAFETY: format is non-null and owned by us, so we use wrap_under_create_rule.
            // This properly handles ownership and will release the format when dropped.
            let format_string = unsafe { CFString::wrap_under_create_rule(format) };

            // Detect hour cycle from the first character of the format pattern
            match format_string.to_string().chars().next() {
                Some('K') => Some(HourCycle::H11),
                Some('h') => Some(HourCycle::H12),
                Some('H') => Some(HourCycle::H23),
                _ => None,
            }
        })
    }

    fn measurement_system() -> Result<Option<MeasurementSystem>, HostInfoError> {
        Ok(
            Self::raw_measurement_system()?.and_then(|raw| match raw.as_str() {
                "U.S." => Some(MeasurementSystem::USSystem),
                "U.K." => Some(MeasurementSystem::UKSystem),
                "Metric" => Some(MeasurementSystem::Metric),
                _ => None,
            }),
        )
    }

    fn collation() -> Result<Option<(Language, CollationType)>, HostInfoError> {
        Ok(Self::raw_collation()?.and_then(|(lang, col)| {
            if let Ok(val) = unicode::Value::from_str(&col) {
                if let Ok(col) = CollationType::try_from(&val) {
                    let lang = Language::try_from_str(lang.as_str()).unwrap_or(Language::UNKNOWN);
                    Some((lang, col))
                } else {
                    None
                }
            } else {
                None
            }
        }))
    }
}

impl RawHostInfoBackend for MacOSHostInfoBackend {
    /// Retrieves system locales for Apple operating systems, in the order preferred by the
    /// user, using [`CFLocaleCopyPreferredLanguages`](https://developer.apple.com/documentation/corefoundation/1542887-cflocalecopypreferredlanguages).
    fn raw_requested_locales() -> Result<Vec<String>, HostInfoError> {
        // SAFETY: CFLocaleCopyPreferredLanguages returns an owned CFArrayRef that we must release.
        // The function is documented to return NULL only in exceptional circumstances.
        let arr_ref = unsafe { CFLocaleCopyPreferredLanguages() };
        if arr_ref.is_null() {
            return Ok(vec![]);
        }
        let arr = unsafe { CFArray::<CFString>::wrap_under_create_rule(arr_ref as _) };

        // Use iterator combinators for more idiomatic Rust
        let out = (0..arr.len())
            .filter_map(|i| arr.get(i))
            .map(|v| v.to_string())
            .collect();
        Ok(out)
    }

    /// Gets the current system calendar identifier.
    fn raw_calendar() -> Result<Option<String>, HostInfoError> {
        /// RAII wrapper for CFCalendarRef
        struct CFCalendarWrapper(core_foundation_sys::calendar::CFCalendarRef);

        impl CFCalendarWrapper {
            fn new() -> Option<Self> {
                // SAFETY: CFCalendarCopyCurrent returns an owned CFCalendarRef that we must release.
                let cal = unsafe { CFCalendarCopyCurrent() };
                if cal.is_null() {
                    None
                } else {
                    Some(CFCalendarWrapper(cal))
                }
            }

            fn get_identifier(&self) -> Option<String> {
                // SAFETY: self.0 is non-null. CFCalendarGetIdentifier expects a CFCalendarRef
                // cast to the appropriate type, and returns a borrowed CFStringRef that doesn't
                // need to be released.
                let identifier = unsafe { CFCalendarGetIdentifier(self.0 as _) };
                cfstring_to_string(identifier as CFStringRef)
            }
        }

        impl Drop for CFCalendarWrapper {
            fn drop(&mut self) {
                // SAFETY: We own the calendar reference and must release it.
                unsafe { CFRelease(self.0 as _) };
            }
        }

        let calendar = CFCalendarWrapper::new();
        Ok(calendar.and_then(|cal| cal.get_identifier()))
    }

    fn raw_region() -> Result<Option<String>, HostInfoError> {
        with_current_locale(|locale| {
            // SAFETY: locale is non-null and kCFLocaleCountryCode is a valid key.
            // CFLocaleGetValue returns a borrowed reference.
            let value = unsafe { CFLocaleGetValue(locale, kCFLocaleCountryCode) };

            if value.is_null() {
                return None;
            }

            // SAFETY: We use wrap_under_get_rule because the value is borrowed, not owned.
            let cf_string = unsafe { CFString::wrap_under_get_rule(value as CFStringRef) };
            Some(cf_string.to_string())
        })
    }

    fn raw_measurement_system() -> Result<Option<String>, HostInfoError> {
        with_current_locale(|locale| {
            // SAFETY: locale is non-null and kCFLocaleMeasurementSystem is a valid key.
            // CFLocaleGetValue returns a borrowed reference.
            let value = unsafe { CFLocaleGetValue(locale, kCFLocaleMeasurementSystem) };

            if value.is_null() {
                return None;
            }

            // SAFETY: We use wrap_under_get_rule because the value is borrowed, not owned.
            let cf_string = unsafe { CFString::wrap_under_get_rule(value as CFStringRef) };
            Some(cf_string.to_string())
        })
    }

    fn raw_measurement_unit_override() -> Result<Option<String>, HostInfoError> {
        unsafe {
            let key = CFString::new("AppleTemperatureUnit");
            let domain = CFString::new(".GlobalPreferences");
            let v = CFPreferencesCopyValue(
                key.as_concrete_TypeRef(),
                domain.as_concrete_TypeRef(),
                kCFPreferencesCurrentUser,
                kCFPreferencesAnyHost,
            );
            if v.is_null() {
                return Ok(None);
            }
            let s = core_foundation::string::CFString::wrap_under_get_rule(v as CFStringRef)
                .to_string();
            Ok(Some(s))
        }
    }

    fn raw_first_day_of_week() -> Result<Option<String>, HostInfoError> {
        unsafe {
            let key = CFString::new("AppleFirstWeekday");
            let domain = CFString::new(".GlobalPreferences");
            let val: CFTypeRef = CFPreferencesCopyValue(
                key.as_concrete_TypeRef(),
                domain.as_concrete_TypeRef(),
                kCFPreferencesCurrentUser,
                kCFPreferencesAnyHost,
            );
            if val.is_null() {
                return Ok(None);
            }

            if CFGetTypeID(val) != CFDictionaryGetTypeID() {
                return Ok(None);
            }

            // take the first value in the dictionary
            let dict = val as CFDictionaryRef;
            let count = CFDictionaryGetCount(dict);
            if count == 0 {
                CFRelease(val);
                return Ok(None);
            }
            let mut keys: Vec<CFTypeRef> = vec![std::ptr::null_mut(); count as usize];
            let mut vals: Vec<CFTypeRef> = vec![std::ptr::null_mut(); count as usize];
            CFDictionaryGetKeysAndValues(dict, keys.as_mut_ptr() as _, vals.as_mut_ptr() as _);

            unsafe fn cfnum_i32(n: CFTypeRef) -> Option<i32> {
                if CFGetTypeID(n) != CFNumberGetTypeID() {
                    return None;
                }
                let mut out = 0i32;
                if CFNumberGetValue(n as _, kCFNumberSInt32Type, &mut out as *mut _ as _) {
                    Some(out)
                } else {
                    None
                }
            }
            Ok(vals.first().and_then(|&n| match cfnum_i32(n)? {
                1 => Some("sun".to_string()),
                2 => Some("mon".to_string()),
                3 => Some("tue".to_string()),
                4 => Some("wed".to_string()),
                5 => Some("thu".to_string()),
                6 => Some("fri".to_string()),
                7 => Some("sat".to_string()),
                _ => None,
            }))
        }
    }

    fn raw_collation() -> Result<Option<(String, String)>, HostInfoError> {
        /// Parse macOS "AppleCollationOrder" style locale strings into (language, collation).
        /// Accepts:
        /// - "zh@collation=stroke"
        /// - "zh-Hant@collation=zhuyin;foo=bar"
        /// - "zh-u-co-pinyin"
        ///
        /// Returns None if no collation is present or language is invalid.
        pub fn parse_mac_collation_locale(input: &str) -> Option<(String, String)> {
            if input.is_empty() {
                return None;
            }

            // 1) Split off any "@..." suffix first (Apple legacy syntax uses @collation=...)
            let (before_at, after_at_opt) = match input.split_once('@') {
                Some((head, tail)) => (head, Some(tail)),
                None => (input, None),
            };

            // 2) Extract language subtag from the head (before '@'): first token before '-' or '_'
            let lang = before_at
                .split(['-', '_'])
                .next()
                .map(|s| s.to_ascii_lowercase())
                .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic()))
                .filter(|s| (2..=8).contains(&s.len()))?; // permit 2–8 alpha per BCP47

            // Helper to validate and normalize collation value
            fn norm_co(s: &str) -> Option<String> {
                let t = s.to_ascii_lowercase();
                if t.is_empty() {
                    return None;
                }
                // Allow a–z and hyphen (e.g., "radical-stroke")
                if t.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
                    Some(t)
                } else {
                    None
                }
            }

            // 3a) Try legacy "@collation=" form
            if let Some(after_at) = after_at_opt {
                if let Some(rest) = after_at.strip_prefix("collation=") {
                    let co = rest
                        .split([';', '@']) // stop at next key or stray '@'
                        .next()
                        .and_then(norm_co);
                    if let Some(co) = co {
                        return Some((lang, co));
                    }
                }
                // If "@..." present but not collation, fall through to try -u- parsing below
            }

            // 3b) Try BCP47 U-extension with "co"
            // Look for "-u-" then scan for "-co-<value>"
            let lower = input.to_ascii_lowercase();
            if let Some((_, tail)) = lower.split_once("-u-") {
                let mut it = tail.split('-');
                while let Some(k) = it.next() {
                    if k == "co" {
                        if let Some(v) = it.next() {
                            if let Some(co) = norm_co(v) {
                                return Some((lang, co));
                            }
                        }
                        break;
                    }
                    // Skip possible multi-part values for other keys; for co it's single
                }
            }

            None
        }

        unsafe {
            let key = CFString::new("AppleCollationOrder");
            let domain = CFString::new(".GlobalPreferences");
            let val = CFPreferencesCopyValue(
                key.as_concrete_TypeRef(),
                domain.as_concrete_TypeRef(),
                kCFPreferencesCurrentUser,
                kCFPreferencesAnyHost,
            );
            if !val.is_null() {
                let cf = CFString::wrap_under_get_rule(val as CFStringRef);
                let s = cf.to_string();
                if let Some((lang, co)) = parse_mac_collation_locale(&s) {
                    return Ok(Some((lang, co)));
                }
                // Some locales like "pl" carry no @collation; ignore and fall through
            }
        }

        Ok(None)
    }
}

/// RAII wrapper for CFLocaleRef to ensure proper cleanup
struct CFLocaleWrapper(CFLocaleRef);

impl CFLocaleWrapper {
    fn new() -> Option<Self> {
        // SAFETY: CFLocaleCopyCurrent returns an owned CFLocaleRef that we must release.
        let locale = unsafe { CFLocaleCopyCurrent() };
        if locale.is_null() {
            None
        } else {
            Some(CFLocaleWrapper(locale))
        }
    }

    fn as_ref(&self) -> CFLocaleRef {
        self.0
    }
}

impl Drop for CFLocaleWrapper {
    fn drop(&mut self) {
        // SAFETY: We own the locale reference and must release it.
        unsafe { CFRelease(self.0 as _) };
    }
}

/// Helper function to reduce duplication when working with current locale.
/// Handles the common pattern of getting current locale, using it, and releasing it.
fn with_current_locale<T, F>(f: F) -> Result<Option<T>, HostInfoError>
where
    F: FnOnce(CFLocaleRef) -> Option<T>,
{
    let locale = CFLocaleWrapper::new();
    Ok(locale.and_then(|loc| f(loc.as_ref())))
}

/// Converts a CFStringRef to a Rust String.
/// Returns None if the CFStringRef is null or conversion fails.
fn cfstring_to_string(cf_string: CFStringRef) -> Option<String> {
    if cf_string.is_null() {
        return None;
    }

    // SAFETY: cf_string is non-null as verified above.
    unsafe {
        // Try the fast path first - get direct pointer to UTF-8 data
        let direct_ptr = CFStringGetCStringPtr(cf_string, kCFStringEncodingUTF8);
        if !direct_ptr.is_null() {
            // SAFETY: CFStringGetCStringPtr returned non-null, so it points to valid UTF-8 data.
            return CStr::from_ptr(direct_ptr as *const c_char)
                .to_str()
                .ok()
                .map(str::to_owned);
        }

        // Fall back to copying the string data
        let length = core_foundation_sys::string::CFStringGetLength(cf_string);
        let max_size = CFStringGetMaximumSizeForEncoding(length, kCFStringEncodingUTF8) + 1;

        // Use stack buffer for small strings to avoid heap allocation
        const STACK_BUFFER_SIZE: usize = 256;
        let mut stack_buffer = [0u8; STACK_BUFFER_SIZE];

        if max_size <= STACK_BUFFER_SIZE as isize {
            // SAFETY: stack_buffer has sufficient size, cf_string is non-null,
            // and kCFStringEncodingUTF8 is a valid encoding.
            let success = CFStringGetCString(
                cf_string,
                stack_buffer.as_mut_ptr() as *mut i8,
                STACK_BUFFER_SIZE as isize,
                kCFStringEncodingUTF8,
            );

            if success != 0 {
                // SAFETY: CFStringGetCString succeeded, so buffer contains valid UTF-8 C string.
                return CStr::from_ptr(stack_buffer.as_ptr() as *const c_char)
                    .to_str()
                    .ok()
                    .map(str::to_owned);
            }
        } else {
            // Use heap allocation for larger strings
            let mut heap_buffer = vec![0u8; max_size as usize];

            // SAFETY: heap_buffer has the required size as calculated by CFStringGetMaximumSizeForEncoding,
            // cf_string is non-null, and kCFStringEncodingUTF8 is a valid encoding.
            let success = CFStringGetCString(
                cf_string,
                heap_buffer.as_mut_ptr() as *mut i8,
                max_size,
                kCFStringEncodingUTF8,
            );

            if success != 0 {
                // SAFETY: CFStringGetCString succeeded, so buffer contains valid UTF-8 C string.
                return CStr::from_ptr(heap_buffer.as_ptr() as *const c_char)
                    .to_str()
                    .ok()
                    .map(str::to_owned);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::backends::{macos::MacOSHostInfoBackend, RawHostInfoBackend};
    use icu_locale_core::Locale;

    #[test]
    fn test_get_raw_locales() {
        let locales_res = MacOSHostInfoBackend::raw_requested_locales();
        match locales_res {
            Ok(locales) => {
                for locale in locales {
                    assert!(!locale.is_empty(), "Empty locale retrieved");
                    assert!(locale.is_ascii(), "Invalid form of locale retrieved");
                }
            }
            Err(e) => {
                panic!("{e:?}")
            }
        }
    }

    #[test]
    fn test_converting_locales() {
        let locales = MacOSHostInfoBackend::raw_requested_locales().unwrap();
        for locale in locales {
            let _loc: Locale = locale.parse().unwrap();
        }
    }

    #[test]
    fn test_calendar() {
        let calendar = MacOSHostInfoBackend::raw_calendar().unwrap();
        assert!(calendar.is_some(), "Couldn't retrieve calendar");
        assert!(
            calendar.unwrap().is_ascii(),
            "Calendar identifier form is not valid"
        );
    }
}
