// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # env_preferences
//!
//! `env_preferences` is a crate to retrieve system locale and preferences for
//! Apple, Linux & Windows systems.
//!
//! It provides functionality to fetch preferred locales from the user's operating
//! system and parse them lossily to an ICU4X [`Locale`](icu_locale_core::Locale).
//!
//! It also retrieves preferences for [`Calendar`](https://crates.io/crates/icu_calendar)
//! & [`TimeZone`](https://crates.io/crates/icu_time)

mod error;
pub mod parse;

pub use error::{LocaleError, ParseError, RetrievalError};

// doc
use core_foundation_sys as _;
#[cfg(target_os = "windows")]
use libc as _;

#[cfg(any(doc, target_os = "macos"))]
pub mod apple;
#[cfg(any(doc, target_os = "linux"))]
pub mod posix;
#[cfg(any(doc, target_os = "windows"))]
pub mod windows;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!(
    "Unsupported target OS. Supported operating systems are Apple, Linux & Windows as of now"
);

#[cfg(target_os = "macos")]
use apple as system;
#[cfg(target_os = "linux")]
use posix as system;
#[cfg(target_os = "windows")]
use windows as system;

#[cfg(target_os = "macos")]
use parse::apple::AppleLocale as SystemLocale;
#[cfg(target_os = "linux")]
use parse::posix::PosixLocale as SystemLocale;
#[cfg(target_os = "windows")]
use parse::windows::WindowsLocale as SystemLocale;

/// List the user's available locales as the platform-provided [`String`]s, ordered by preference.
///
/// <div class="warning">
///
/// The output of this function is platform-dependent and **is not guaranteed** to be a valid
/// BCP-47 identifier. To get a list of parsed locales, see [`get_locales_lossy()`].
///
/// </div>
///
/// Specific information can be found at the platform's implementation:
/// - [`apple::get_raw_locales()`]
/// - [`posix::get_raw_locales()`]
/// - [`windows::get_raw_locales()`]
pub fn get_raw_locales() -> Result<Vec<String>, RetrievalError> {
    system::get_raw_locales()
}

/// List the user's available locales as ICU4X [`Locale`](icu_locale_core::Locale)s, ordered by preference.
///
/// This performs a best-effort conversion that may lose some (or all!) data in certain cases.
/// For getting a list of raw system locales, see [`get_raw_locales()`].
///  
/// Specific information can be found at the platform's implementation:
/// - [`parse::apple::AppleLocale`]
/// - [`parse::posix::PosixLocale`]
/// - [`parse::windows::WindowsLocale`]
pub fn get_locales_lossy() -> Result<Vec<icu_locale_core::Locale>, LocaleError> {
    let raw_locales = get_raw_locales()?;
    let system_locales = raw_locales
        .iter()
        .map(String::as_str)
        .map(SystemLocale::try_from_str)
        .collect::<Result<Vec<SystemLocale>, ParseError>>()?;

    system_locales
        .iter()
        .map(SystemLocale::try_convert_lossy)
        .map(|result| result.map_err(LocaleError::from))
        .collect::<Result<Vec<icu_locale_core::Locale>, LocaleError>>()
}
