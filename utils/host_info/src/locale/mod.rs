// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Host-specific Locale representations.
//!
//! Some popular host environments provide custom definition of a `Locale`.
//! This module contains APIs allowing for encoding of those variants and their conversion
//! to ICU4X Locale.
pub mod posix;
pub mod windows;

pub use posix::PosixLocale;
pub use windows::WindowsLocale;
