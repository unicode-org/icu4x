// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::{
    extensions::unicode::Unicode,
    preferences::extensions::unicode::keywords::{
        CalendarAlgorithm, CollationType, FirstDay, HourCycle, MeasurementSystem,
        MeasurementUnitOverride,
    },
    subtags::{Language, Region},
    Locale,
};

use crate::{
    backends::{self, HostInfoBackend},
    error::HostInfoError,
};

use super::HostKind;

pub(crate) const RESOLVED_BACKEND: Option<HostKind> = {
    #[cfg(target_os = "android")]
    {
        Some(HostKind::Android)
    }
    #[cfg(target_os = "ios")]
    {
        Some(HostKind::Ios)
    }
    #[cfg(target_os = "linux")]
    {
        Some(HostKind::Linux)
    }
    #[cfg(target_os = "macos")]
    {
        Some(HostKind::MacOS)
    }
    #[cfg(target_os = "windows")]
    {
        Some(HostKind::Windows)
    }
    #[cfg(not(any(
        target_os = "android",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos",
        target_os = "windows"
    )))]
    {
        None
    }
};

/// Retrieves `Unicode` extensions struct populated from host regional preferences.
///
/// # Example
///
/// ```
/// let ue = icu_host_info::unicode_extensions()
///     .expect("Failed to retrieve host info");
/// ```
pub fn unicode_extensions() -> Result<Unicode, HostInfoError> {
    backends::Impl::unicode_extensions()
}

/// Retrieves `Preferences` object for `DateTimeFormatter`.
///
/// # Example
///
/// ```
/// let ue = icu_host_info::datetime_preferences()
///     .expect("Failed to retrieve datetime preferences");
/// ```
#[cfg(feature = "datetime")]
pub fn datetime_preferences() -> Result<icu_datetime::DateTimeFormatterPreferences, HostInfoError> {
    backends::Impl::datetime_preferences()
}

/// Retrieves an ordered list of locales set as requested by the user in the host
/// environment regional preferences.
///
/// # Example
///
/// ```
/// let locales = icu_host_info::requested_locales()
///     .expect("Failed to retrieve requested locales");
/// ```
pub fn requested_locales() -> Result<Vec<Locale>, HostInfoError> {
    backends::Impl::requested_locales()
}

/// Retrieves a calendar preference.
///
/// In `::unicode_extensions()` this field is being encoded as `ca`.
///
/// # Example
///
/// ```
/// let calendar = icu_host_info::calendar()
///     .expect("Failed to retrieve calendar");
/// ```
pub fn calendar() -> Result<Option<CalendarAlgorithm>, HostInfoError> {
    backends::Impl::calendar()
}

/// Retrieves a region set in the host environment regional preferences.
///
/// That region may be already populated into `requested_locales` or not, depending
/// on the host.
/// In `::unicode_extensions()` this field is being encoded as `rg`.
///
/// # Example
///
/// ```
/// let region = icu_host_info::region()
///     .expect("Failed to retrieve region");
/// ```
pub fn region() -> Result<Option<Region>, HostInfoError> {
    backends::Impl::region()
}

/// Retrieves an hour_cycle preference.
///
/// In `::unicode_extensions()` this field is being encoded as `hc`.
///
/// # Example
///
/// ```
/// let region = icu_host_info::hour_cycle()
///     .expect("Failed to retrieve hour cycle");
/// ```
pub fn hour_cycle() -> Result<Option<HourCycle>, HostInfoError> {
    backends::Impl::hour_cycle()
}

/// Retrieves a measurement system preference.
///
/// In `::unicode_extensions()` this field is being encoded as `ms`.
///
/// # Example
///
/// ```
/// let ms = icu_host_info::measurement_system()
///     .expect("Failed to retrieve measurement system");
/// ```
pub fn measurement_system() -> Result<Option<MeasurementSystem>, HostInfoError> {
    backends::Impl::measurement_system()
}

/// Retrieves a first day of week preference.
///
/// In `::unicode_extensions()` this field is being encoded as `fd`.
///
/// # Example
///
/// ```
/// let fd = icu_host_info::first_day_of_week()
///     .expect("Failed to retrieve first day of week");
/// ```
pub fn first_day_of_week() -> Result<Option<FirstDay>, HostInfoError> {
    backends::Impl::first_day_of_week()
}

/// Retrieves a collation preference.
///
/// In `::unicode_extensions()` this field is being encoded as `co`.
///
/// # Example
///
/// ```
/// let collation = icu_host_info::collation()
///     .expect("Failed to retrieve collation");
/// ```
pub fn collation() -> Result<Option<(Language, CollationType)>, HostInfoError> {
    backends::Impl::collation()
}

/// Retrieves measurement unit override preference.
///
/// In `::unicode_extensions()` this field is being encoded as `mu`.
///
/// # Example
///
/// ```
/// let mu = icu_host_info::measurement_unit_override()
///     .expect("Failed to retrieve measurement unit override");
/// ```
pub fn measurement_unit_override() -> Result<Option<MeasurementUnitOverride>, HostInfoError> {
    backends::Impl::measurement_unit_override()
}

pub fn resolved_backend() -> Option<HostKind> {
    RESOLVED_BACKEND
}
