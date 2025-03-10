// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # env_preferences
//!
//! `env_preferences` is a crate to retrieve system locale and preferences for
//! Apple, Linux & Windows systems
//!
//! It currently fetches locales for the operating system
//! currently in `String` format.
//!
//! In the current setup, it is not ensured that the locale retrieved will be
//! converted to [`ICU4X Locale`](https://crates.io/crates/icu_locale)
//!
//! It also retrieves preferences for [`Calendar`](https://crates.io/crates/icu_calendar)
//! & [`TimeZone`](https://crates.io/crates/icu_time)

mod error;

pub use error::{LocaleError, RetrievalError};

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
pub use apple::{self as system, AppleLocale as SystemLocale};
#[cfg(target_os = "linux")]
pub use posix::{self as system, PosixLocale as SystemLocale};
#[cfg(target_os = "windows")]
pub use windows::{self as system, WindowsLocale as SystemLocale};

#[cfg(not(target_os = "linux"))]
// There are no parsing errors on most platforms, so just alias to the broader [`LocaleError`] enum
use error::LocaleError as SystemLocaleError;
#[cfg(target_os = "linux")]
use posix::PosixParseError as SystemLocaleError;

pub fn get_raw_locales() -> Result<Vec<String>, RetrievalError> {
    system::get_raw_locales()
}

pub fn get_locales_lossy() -> Result<Vec<icu_locale::Locale>, LocaleError> {
    let raw_locales = get_raw_locales()?;
    let system_locales = raw_locales
        .iter()
        .map(String::as_str)
        .map(SystemLocale::try_from_str)
        .collect::<Result<Vec<SystemLocale>, SystemLocaleError>>()?;

    system_locales
        .iter()
        .map(SystemLocale::try_convert_lossy)
        .map(|result| result.map_err(LocaleError::from))
        .collect()
}
