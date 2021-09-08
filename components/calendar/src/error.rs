// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

#[cfg(feature = "std")]
impl std::error::Error for DateTimeError {}

/// A list of possible error outcomes for working with various inputs to DateTime inputs
/// and operations.
#[derive(Display, Debug)]
pub enum DateTimeError {
    /// An input could not be parsed.
    #[displaydoc("{0}")]
    Parse(core::num::ParseIntError),
    /// An input overflowed its range.
    #[allow(missing_docs)] // TODO(#686) - Add missing docs.
    #[displaydoc("{field} must be between 0-{max}")]
    Overflow { field: &'static str, max: usize },
    #[allow(missing_docs)] // TODO(#686) - Add missing docs.
    #[displaydoc("{field} must be between {min}-0")]
    /// An input underflowed its range.
    Underflow { field: &'static str, min: isize },
    /// The time zone offset was invalid.
    #[displaydoc("Failed to parse time-zone offset")]
    InvalidTimeZoneOffset,
    /// Out of range
    // TODO(Manishearth) turn this into a proper variant
    OutOfRange,
}

impl From<core::num::ParseIntError> for DateTimeError {
    fn from(e: core::num::ParseIntError) -> Self {
        DateTimeError::Parse(e)
    }
}
