// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{collections::HashMap, ffi::CStr, ptr};

use libc::{setlocale, LC_ALL, LC_TIME};
use std::str::FromStr;

use crate::RetrievalError;

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
}

impl FromStr for LocaleCategory {
    type Err = RetrievalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LC_CTYPE" => Ok(Self::Character),
            "LC_NUMERIC" => Ok(Self::Number),
            "LC_TIME" => Ok(Self::Time),
            "LC_COLLATE" => Ok(Self::Collate),
            "LC_MONETARY" => Ok(Self::Monetary),
            "LC_MESSAGES" => Ok(Self::Messages),
            "LC_PAPER" => Ok(Self::Paper),
            "LC_NAME" => Ok(Self::Name),
            "LC_ADDRESS" => Ok(Self::Address),
            "LC_TELEPHONE" => Ok(Self::Telephone),
            "LC_MEASUREMENT" => Ok(Self::Measurement),
            "LC_IDENTIFICATION" => Ok(Self::Identification),
            "LC_ALL" => Ok(Self::All),
            _ => Err(RetrievalError::UnknownCategory),
        }
    }
}

pub fn get_locales() -> Result<HashMap<LocaleCategory, String>, RetrievalError> {
    let mut locale_map = HashMap::new();

    // SAFETY: Safety is ensured because we pass a `NULL` pointer and retrieve the locale there is
    // no subsequent calls for `setlocale` which could change the locale of this particular thread
    let locales_ptr = unsafe { setlocale(LC_ALL, ptr::null()) };

    if locales_ptr.is_null() {
        return Err(RetrievalError::NullLocale);
    }

    // SAFETY: A valid `NULL` terminator is present which is a requirement of `from_ptr`
    let locales_str = unsafe { CStr::from_ptr(locales_ptr) }.to_str()?;
    let locale_pairs = locales_str.split(';');
    for locale_pair in locale_pairs {
        let mut parts = locale_pair.split('=');
        if let Some(value) = parts.next() {
            if let Some(key) = parts.next() {
                if let Ok(category) = LocaleCategory::from_str(value) {
                    locale_map.insert(category, key.to_string());
                }
            } else {
                // Handle case where only a single locale
                locale_map.insert(LocaleCategory::All, value.to_string());
            }
        }
    }
    Ok(locale_map)
}

/// This only returns the calendar locale,`gnome-calendar` is the default calendar in linux
/// The locale returned is for `Gregorian` calendar
/// Related issue: `<https://gitlab.gnome.org/GNOME/gnome-calendar/-/issues/998>`
pub fn get_system_calendars() -> Result<String, RetrievalError> {
    // SAFETY: Safety is ensured because we pass a `NULL` pointer and retrieve the locale there is
    // no subsequent calls for `setlocale` which could change the locale of this particular thread
    let locale_ptr = unsafe { setlocale(LC_TIME, ptr::null()) };

    if !locale_ptr.is_null() {
        // SAFETY: A valid `NULL` terminator is present which is a requirement of `from_ptr`
        let rust_str = unsafe { CStr::from_ptr(locale_ptr) }.to_str()?;
        let calendar_locale = rust_str.to_string();
        return Ok(calendar_locale);
    }
    Err(RetrievalError::NullLocale)
}
