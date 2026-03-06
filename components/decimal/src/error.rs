// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Error types for functions in the decimal crate

use displaydoc::Display;

/// An error due to a [`CompactDecimal`](fixed_decimal::CompactDecimal) with an
/// exponent inconsistent with the compact decimal data for the locale, e.g.,
/// when formatting 1c5 in English (US).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not use this type unless you are prepared for things to occasionally break.
///
/// Graduation tracking issue: [issue #7161](https://github.com/unicode-org/icu4x/issues/7161).
/// </div>
///
/// ✨ *Enabled with the `unstable` Cargo feature.*
#[derive(Display, Copy, Clone, Debug)]
#[displaydoc("Expected compact exponent {expected} for 10^{log10_type}, got {actual}")]
pub struct ExponentError {
    /// The compact decimal exponent passed to the formatter.
    pub(crate) actual: u8,
    /// The appropriate compact decimal exponent for a number of the given magnitude.
    pub(crate) expected: u8,
    /// The magnitude of the number being formatted.
    pub(crate) log10_type: i16,
}

impl core::error::Error for ExponentError {}
