// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

#[cfg(feature = "std")]
impl std::error::Error for DateTimeError {}

/// A list of possible error outcomes for working with various inputs to DateTime inputs
/// and operations.
#[derive(Display, Debug, Copy, Clone)]
pub enum DateTimeError {
    /// An input could not be parsed.
    #[displaydoc("Could not parse as integer")]
    Parse,
    /// An input overflowed its range.
    #[displaydoc("{field} must be between 0-{max}")]
    Overflow {
        /// The name of the field
        field: &'static str,
        /// The maximum value
        max: usize,
    },
    #[displaydoc("{field} must be between {min}-0")]
    /// An input underflowed its range.
    Underflow {
        /// The name of the field
        field: &'static str,
        /// The minimum value
        min: isize,
    },
    /// The time zone offset was invalid.
    #[displaydoc("Failed to parse time-zone offset")]
    InvalidTimeZoneOffset,
    /// Out of range
    // TODO(Manishearth) turn this into a proper variant
    OutOfRange,
    /// An input was missing.
    #[displaydoc("No value for {0}")]
    MissingInput(&'static str),
}

impl From<core::num::ParseIntError> for DateTimeError {
    fn from(_: core::num::ParseIntError) -> Self {
        DateTimeError::Parse
    }
}
