// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Retrieval of system locales and preferences.

mod error;
pub use error::RetrievalError;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
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
