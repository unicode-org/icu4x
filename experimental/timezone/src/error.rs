// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

#[cfg(feature = "std")]
impl std::error::Error for TimeZoneError {}

/// A list of possible error outcomes for working with types in this crate
/// and operations.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum TimeZoneError {
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
}

impl From<core::num::ParseIntError> for TimeZoneError {
    fn from(_: core::num::ParseIntError) -> Self {
        TimeZoneError::Parse
    }
}
