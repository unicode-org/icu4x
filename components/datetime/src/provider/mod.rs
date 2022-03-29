// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

#[cfg(doc)]
use icu_provider::prelude::ResourceKey;

/// Data providers for the Gregorian Calendar.
pub mod calendar;

/// Data providers for time zones.
pub mod time_zones;

/// Provider for week data.
pub mod week_data;

/// Traits for managing data needed by [`DateTimeFormat`](crate::DateTimeFormat).
pub(crate) mod date_time;

/// TODO(#1109): Use vertical fallback here
pub(crate) fn combine_langid_and_calendar(langid: icu_locid::LanguageIdentifier, cal: &str) -> icu_locid::Locale {
    use alloc::string::ToString;
    let mut locale_str = "und-u-ca-".to_string();
    locale_str.push_str(cal);
    // This is temporary code that will be removed as part of #1109
    #[allow(clippy::unwrap_used)]
    let mut loc_with_calendar: icu_locid::Locale = locale_str.parse().unwrap();
    loc_with_calendar.id = langid;
    loc_with_calendar
}

// TODO(#1109): Use vertical fallback here
pub(crate) fn region_to_locale(region: Option<icu_locid::subtags::Region>) -> icu_locid::Locale {
    let mut loc_with_region = icu_locid::Locale::default();
    loc_with_region.id.region = region;
    loc_with_region
}
