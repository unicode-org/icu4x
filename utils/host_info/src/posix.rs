// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)]

use libc::{setlocale, LC_ALL};
use std::{collections::HashMap, ffi::CStr, ptr, str::FromStr};

use crate::error::HostInfoError;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum LocaleCategory {
    Character, // LC_CTYPE
    Number,    // LC_NUMERIC
    Time,      // LC_TIME
    Collate,   // LC_COLLATE
    Monetary,  // LC_MONETARY
    Messages,  // LC_MESSAGES
    // GNU extensions (may not exist on non-gnu targets)
    Paper,          // LC_PAPER
    Name,           // LC_NAME
    Address,        // LC_ADDRESS
    Telephone,      // LC_TELEPHONE
    Measurement,    // LC_MEASUREMENT
    Identification, // LC_IDENTIFICATION
    All,            // LC_ALL
}

impl LocaleCategory {
    #[inline]
    fn to_env_var_name(self) -> &'static str {
        match self {
            LocaleCategory::Character => "LC_CTYPE",
            LocaleCategory::Number => "LC_NUMERIC",
            LocaleCategory::Time => "LC_TIME",
            LocaleCategory::Collate => "LC_COLLATE",
            LocaleCategory::Monetary => "LC_MONETARY",
            LocaleCategory::Messages => "LC_MESSAGES",
            LocaleCategory::Paper => "LC_PAPER",
            LocaleCategory::Name => "LC_NAME",
            LocaleCategory::Address => "LC_ADDRESS",
            LocaleCategory::Telephone => "LC_TELEPHONE",
            LocaleCategory::Measurement => "LC_MEASUREMENT",
            LocaleCategory::Identification => "LC_IDENTIFICATION",
            LocaleCategory::All => "LC_ALL",
        }
    }
}

impl FromStr for LocaleCategory {
    type Err = HostInfoError;

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
            _ => Err(HostInfoError::UnknownCategory),
        }
    }
}

// --- helpers ---

#[inline]
fn is_c_like(raw: &str) -> bool {
    let s = raw.trim();
    if s.is_empty() {
        return true;
    }
    let up = s.to_ascii_uppercase();
    // Strip charset and modifier suffixes like ".UTF-8" or "@euro"
    let base = up.split('.').next().unwrap_or(&up);
    let base = base.split('@').next().unwrap_or(base);
    base == "C" || base == "POSIX"
}

#[inline]
fn non_c_like_env(name: &str) -> Option<String> {
    std::env::var_os(name).and_then(|v| {
        let s = v.to_string_lossy();
        if s.is_empty() || is_c_like(&s) {
            None
        } else {
            Some(s.into_owned())
        }
    })
}

/// POSIX precedence: LC_ALL > LC_<CAT> > LANG.
/// Returns Some(non-C/POSIX) or None if unset/C-like.
fn resolve_env_for_category(cat: LocaleCategory) -> Option<String> {
    if let Some(v) = non_c_like_env("LC_ALL") {
        return Some(v);
    }
    if cat != LocaleCategory::All {
        if let Some(v) = non_c_like_env(cat.to_env_var_name()) {
            return Some(v);
        }
    }
    non_c_like_env("LANG")
}

/// Attempt to parse `setlocale(LC_ALL, NULL)` into a map.
/// Returns None if NULL or C/POSIX-like (uninformative), to trigger env fallback.
/// Note: We only check LC_ALL because if libc is uninitialized, all categories return "C".
/// If initialized, LC_ALL contains the composite snapshot of all category values.
fn parse_setlocale_snapshot() -> Option<HashMap<LocaleCategory, String>> {
    // SAFETY: read-only query of current thread's locale snapshot
    let ptr = unsafe { setlocale(LC_ALL, ptr::null()) };
    if ptr.is_null() {
        return None;
    }
    let s = unsafe { CStr::from_ptr(ptr) }.to_str().ok()?;
    if s.is_empty() || is_c_like(s) {
        return None;
    }

    let mut map = HashMap::new();
    if !s.contains('=') {
        // Single composite locale -> LC_ALL
        if !is_c_like(s) {
            map.insert(LocaleCategory::All, s.to_string());
        }
        return if map.is_empty() { None } else { Some(map) };
    }

    for pair in s.split(';') {
        let mut it = pair.splitn(2, '=');
        let k = it.next().unwrap_or_default().trim();
        let v = it.next().unwrap_or_default().trim();
        if v.is_empty() || is_c_like(v) {
            continue;
        }
        if let Ok(cat) = LocaleCategory::from_str(k) {
            map.insert(cat, v.to_string());
        }
    }

    if map.is_empty() {
        None
    } else {
        Some(map)
    }
}

// --- public ---

/// Retrieves locales for LC_ALL and any explicitly-set categories in this thread.
/// If libc is uninitialized (NULL/C/POSIX), falls back to env precedence.
/// If nothing resolves, returns `{ LC_ALL: "en-US-posix" }`.
pub(crate) fn raw_locale_categories() -> Result<HashMap<LocaleCategory, String>, HostInfoError> {
    if let Some(map) = parse_setlocale_snapshot() {
        return Ok(map);
    }

    // Env fallback: collect only categories that resolve to non-C/POSIX.
    const CATS: &[LocaleCategory] = &[
        LocaleCategory::Character,
        LocaleCategory::Number,
        LocaleCategory::Time,
        LocaleCategory::Collate,
        LocaleCategory::Monetary,
        LocaleCategory::Messages,
        LocaleCategory::Paper,
        LocaleCategory::Name,
        LocaleCategory::Address,
        LocaleCategory::Telephone,
        LocaleCategory::Measurement,
        LocaleCategory::Identification,
        LocaleCategory::All,
    ];

    let mut out = HashMap::new();

    for &cat in CATS {
        if let Some(v) = resolve_env_for_category(cat) {
            out.insert(cat, v);
        }
    }

    if out.is_empty() {
        out.insert(LocaleCategory::All, "en-US-posix".to_string());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::Locale;

    // Testing fetching of locale, as `get_locales` fetches the locales for category
    // `LC_ALL`. For this category this should return non empty
    #[test]
    fn test_get_raw_locale_categories() {
        let locale_res = raw_locale_categories().unwrap();
        assert!(
            !locale_res.is_empty(),
            "Empty hashmap for locales retrieved"
        );
        for locale in locale_res.into_values() {
            assert!(locale.is_ascii(), "Invalid form of locale retrieved")
        }
    }

    #[test]
    fn test_converting_locales() {
        let locale_res: std::collections::HashMap<LocaleCategory, String> =
            raw_locale_categories().unwrap();
        for locale in locale_res.into_values() {
            let parts: Vec<&str> = locale.split('.').collect();

            // Skipping "C" and those ending with "UTF-8", as they cannot be converted
            // into the locale
            if !parts.contains(&"C") && (parts.len() > 1 && parts[parts.len() - 1] != "UTF-8") {
                let mut locale_converted: Locale = locale.parse().unwrap();
                locale_converted.extensions.unicode.clear();
                assert_eq!(locale_converted, locale.parse().unwrap());
            }
        }
    }
}
