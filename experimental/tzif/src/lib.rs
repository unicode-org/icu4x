// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![warn(missing_docs)]

//! `icu_provider_tzif` contains implementations of the [`ICU4X`] data provider interface
//! based on the TZif files that can be created from the [IANA Time Zone Database]
//!
//! [`ICU4X`]: ../icu/index.html
//! [IANA Time Zone Database]: https://www.iana.org/time-zones

/// The core structs and traits that comprise the parsing infrastructure.
pub mod core;
