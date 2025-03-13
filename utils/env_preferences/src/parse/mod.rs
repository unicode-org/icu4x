// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing functionality for various popular operating systems

// Re-export all alias functions
mod aliases;
pub use aliases::*;

#[cfg(any(doc, feature = "parse_apple", target_os = "macos"))]
pub mod apple;
#[cfg(any(doc, feature = "parse_posix", target_os = "linux"))]
pub mod posix;
#[cfg(any(doc, feature = "parse_windows", target_os = "windows"))]
pub mod windows;
