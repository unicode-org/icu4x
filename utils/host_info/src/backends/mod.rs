// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Per-host implementations for `HostInfo`.
//!
//! This module contains traits implemented for per-host backends.
//!
//! When compiling for any given host architecture, the developer
//! may access the per-backend implementation via `icu_host_info::backends::{arch}::{Arch}HostInfoBackend`.
//!
//! # RawHostInfoBackend
//!
//! This trait provides low level implementation of per-host bindings to retrieve regional preferences in their
//! original form.
//!
//! # HostInfoBackend
//!
//! This trait provides high level implementation of per-host bindings to convert raw values into their ICU4X
//! types.
use std::str::FromStr;

use icu_locale_core::{
    extensions::unicode::{self, key, Unicode, Value},
    preferences::extensions::unicode::keywords::{
        CalendarAlgorithm, CollationType, FirstDay, HourCycle, MeasurementSystem,
        MeasurementUnitOverride,
    },
    subtags::{Language, Region},
    Locale,
};

use crate::error::HostInfoError;

mod shared;

#[cfg(target_os = "android")]
#[doc(hidden)]
pub mod android;

#[cfg(target_os = "ios")]
#[doc(hidden)]
pub mod macos;

#[cfg(target_os = "linux")]
#[doc(hidden)]
pub mod linux;

#[cfg(target_os = "macos")]
#[doc(hidden)]
pub mod macos;

#[cfg(target_os = "windows")]
#[doc(hidden)]
pub mod windows;

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "windows"
)))]
#[doc(hidden)]
mod unavailable;

/// High level implementation of per-host bindings to convert raw values into their ICU4X types.
pub trait HostInfoBackend: RawHostInfoBackend {
    /// The implementation should attempt to collect all relevant regional preferences available in the given
    /// host environment into a unicode extensions bag.
    fn unicode_extensions() -> Result<Unicode, HostInfoError> {
        let mut result = Unicode::new();
        if let Some(calendar) = Self::calendar()? {
            result.keywords.set(key!("ca"), calendar.into());
        }
        if let Some(hc) = Self::hour_cycle()? {
            result.keywords.set(key!("hc"), hc.into());
        }
        if let Some(ms) = Self::measurement_system()? {
            result.keywords.set(key!("ms"), ms.into());
        }
        if let Some(mu) = Self::measurement_unit_override()? {
            result.keywords.set(key!("mu"), mu.into());
        }
        if let Some(fw) = Self::first_day_of_week()? {
            result.keywords.set(key!("fw"), fw.into());
        }
        if let Some((_lang, co)) = Self::collation()? {
            result.keywords.set(key!("co"), co.into());
        }
        if let Some(rg) = Self::region()? {
            let mut rg_str = rg.to_string();
            rg_str.push_str("zzzz");
            if let Ok(value) = Value::try_from_str(&rg_str) {
                result.keywords.set(key!("rg"), value);
            }
        }
        Ok(result)
    }

    /// The implementation should attempt to retrieve date/time related regional preferences and collect
    /// them into `DateTimeFormatterPreferences` bag.
    #[cfg(feature = "datetime")]
    fn datetime_preferences() -> Result<icu_datetime::DateTimeFormatterPreferences, HostInfoError> {
        use icu_locale_core::Locale;

        let requested_locales = Self::requested_locales()?;
        let requested_locale = requested_locales
            .first()
            .cloned()
            .unwrap_or(Locale::UNKNOWN);
        let mut result = icu_datetime::DateTimeFormatterPreferences::from(requested_locale);
        result.numbering_system = None;
        result.hour_cycle = Self::hour_cycle()?;
        result.calendar_algorithm = Self::calendar()?;
        Ok(result)
    }

    /// The implementation should attempt to retrieve requested locales set by the user in the host system.
    fn requested_locales() -> Result<Vec<Locale>, HostInfoError> {
        Ok(Self::raw_requested_locales()?
            .into_iter()
            .filter_map(|s| Locale::try_from_str(&s).ok())
            .collect())
    }

    /// The implementation should attempt to retrieve calendar set by the user in the host system.
    fn calendar() -> Result<Option<CalendarAlgorithm>, HostInfoError> {
        Ok(Self::raw_calendar()?
            .and_then(|raw| unicode::Value::from_str(&raw).ok())
            .and_then(|value| CalendarAlgorithm::try_from(&value).ok()))
    }

    /// The implementation should attempt to retrieve region set by the user in the host system.
    fn region() -> Result<Option<Region>, HostInfoError> {
        Ok(Self::raw_region()?.and_then(|raw| Region::try_from_str(&raw).ok()))
    }

    /// The implementation should attempt to retrieve hour_cycle set by the user in the host system.
    fn hour_cycle() -> Result<Option<HourCycle>, HostInfoError> {
        Ok(Self::raw_hour_cycle()?
            .and_then(|raw| unicode::Value::from_str(&raw).ok())
            .and_then(|value| HourCycle::try_from(&value).ok()))
    }

    /// The implementation should attempt to retrieve measurement system set by the user in the host system.
    fn measurement_system() -> Result<Option<MeasurementSystem>, HostInfoError> {
        Ok(Self::raw_measurement_system()?
            .and_then(|raw| unicode::Value::from_str(&raw).ok())
            .and_then(|value| MeasurementSystem::try_from(&value).ok()))
    }

    /// The implementation should attempt to retrieve measurement unit override set by the user in the host system.
    fn measurement_unit_override() -> Result<Option<MeasurementUnitOverride>, HostInfoError> {
        Ok(Self::raw_measurement_unit_override()?
            .and_then(|raw| unicode::Value::from_str(&raw).ok())
            .and_then(|value| MeasurementUnitOverride::try_from(&value).ok()))
    }

    /// The implementation should attempt to retrieve first day of week set by the user in the host system.
    fn first_day_of_week() -> Result<Option<FirstDay>, HostInfoError> {
        Ok(Self::raw_first_day_of_week()?
            .and_then(|raw| unicode::Value::from_str(&raw).ok())
            .and_then(|value| FirstDay::try_from(&value).ok()))
    }

    /// The implementation should attempt to retrieve collation set by the user in the host system.
    fn collation() -> Result<Option<(Language, CollationType)>, HostInfoError> {
        Ok(Self::raw_collation()?.and_then(|(raw_lang, raw_col)| {
            unicode::Value::from_str(&raw_col)
                .ok()
                .and_then(|col| CollationType::try_from(&col).ok())
                .and_then(|col| Language::from_str(&raw_lang).ok().map(|lang| (lang, col)))
        }))
    }
}

/// Low level implementation of per-host bindings to retrieve regional preferences in their original form.
///
/// As per library design, the implementations should attempt to return `None` in scenarios where user
/// did not explicitly set a value for any of the preferences.
/// For example, if the user set `en-US` as their preferred locale, and did not manually set `HourCycle`
/// to any value, the host API may return hour cycle default value for en-US.
/// If possible, the implementation should attempt to distinguish between explicitly set value that matches
/// default for a given locale, from lack of explicit value set.
///
/// If that is not possible, the API should return the value retrieved from the system for each field getter.
///
/// The goal is to avoid constructing a `en-US-hc-h12` locale in a scenario where the user set their locale to `en-US`
/// but did not explicitly define hour cycle preference, and the `h12` value is just a default for `en-US`.
/// This becomes impactful when locale negotiation results in the system picking one of the fallback locales, and
/// needs to determine if it should follow its regional preferences, or take some from the host system.
/// For example, if the user set `["en-US", "de-DE"]` as their requested locales, and the host API returns `h12` for
/// the hour cycle getter, it may be problematic to not know if this is explicit preference of the user, or default
/// for `en-US`. As a result, it may become challenging to decide if `h12` should be used even if `de-DE` is being negotiated
/// as the locale for the given application.
pub trait RawHostInfoBackend {
    /// Attempt to retrieve a list of locales set in the host regional preferences as requested by the user.
    ///
    /// The list is ordered and should contain locales explicitly requested by the user, with an empty
    /// list being a valid response in case no locale has been set by the user, or the backend cannot retrieve any.
    fn raw_requested_locales() -> Result<Vec<String>, HostInfoError> {
        Ok(vec![])
    }

    /// Attempt to retrieve calendar system set in the host regional preferences by the user.
    fn raw_calendar() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve region set in the host regional preferences by the user.
    fn raw_region() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve hour cycle set in the host regional preferences by the user.
    fn raw_hour_cycle() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve measurement system set in the host regional preferences by the user.
    fn raw_measurement_system() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve measurement unit override set in the host regional preferences by the user.
    ///
    /// This should retrieve `temperature` unit.
    fn raw_measurement_unit_override() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve first day of week option set in the host regional preferences by the user.
    fn raw_first_day_of_week() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve collation set in the host regional preferences by the user.
    fn raw_collation() -> Result<Option<(String, String)>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve customized date format set in the host regional preferences by the user.
    fn raw_date_format() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }

    /// Attempt to retrieve customized number format set in the host regional preferences by the user.
    fn raw_number_format() -> Result<Option<String>, HostInfoError> {
        Ok(None)
    }
}

#[cfg(target_os = "android")]
#[doc(hidden)]
pub(crate) type Impl = android::AndroidHostInfoBackend;

#[cfg(target_os = "ios")]
#[doc(hidden)]
pub(crate) type Impl = macos::MacOSHostInfoBackend;

#[cfg(target_os = "linux")]
#[doc(hidden)]
pub(crate) type Impl = linux::LinuxHostInfoBackend;

#[cfg(target_os = "macos")]
#[doc(hidden)]
pub(crate) type Impl = macos::MacOSHostInfoBackend;

#[cfg(target_os = "windows")]
#[doc(hidden)]
pub(crate) type Impl = windows::WindowsHostInfoBackend;

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "windows"
)))]
#[doc(hidden)]
pub(crate) type Impl = unavailable::UnavailableHostInfoBackend;
