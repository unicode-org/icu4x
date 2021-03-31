// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Algorithms to generate decimal digit characters.

use std::convert::TryFrom;

/// Returns the decimal digit with the given offset from the zero digit for a numbering system.
pub fn get(zero_digit: char, value: u8) -> char {
    // This should almost always succeed, but in case it fails (e.g. bad data), fall back
    // gracefully to the Unicode replacement char U+FFFD.
    char::try_from((zero_digit as u32) + (value as u32)).unwrap_or('\u{FFFD}')
}
