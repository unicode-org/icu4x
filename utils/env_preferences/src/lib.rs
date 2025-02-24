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
pub use error::RetrievalError;

#[cfg(target_os = "linux")]
mod posix;
#[cfg(target_os = "linux")]
pub use posix::fetch::*;
#[cfg(target_os = "macos")]
mod apple;
#[cfg(target_os = "macos")]
pub use apple::*;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!(
    "Unsupported target OS. Supported operating systems are Apple, Linux & Windows as of now"
);
