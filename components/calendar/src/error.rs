// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locid::extensions::unicode::Value;
use tinystr::{tinystr, TinyStr16, TinyStr4};

#[cfg(feature = "std")]
impl std::error::Error for DateTimeError {}

/// A list of possible error outcomes for working with various inputs to DateTime inputs
/// and operations.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
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
    /// Out of range
    // TODO(Manishearth) turn this into a proper variant
    OutOfRange,
    /// Unknown era
    #[displaydoc("No era named {0} for calendar {1}")]
    UnknownEra(TinyStr16, &'static str),
    /// Unknown month code for a given calendar
    #[displaydoc("No month code named {0} for calendar {1}")]
    UnknownMonthCode(TinyStr4, &'static str),
    /// Missing required input field for formatting
    #[displaydoc("No value for {0}")]
    MissingInput(&'static str),
    /// No support for a given calendar in AnyCalendar
    #[displaydoc("AnyCalendar does not support calendar {0}")]
    UnknownAnyCalendarKind(TinyStr16),
    /// An operation required a calendar but a calendar was not provided.
    #[displaydoc("An operation required a calendar but a calendar was not provided")]
    MissingCalendar,
}

impl From<core::num::ParseIntError> for DateTimeError {
    fn from(_: core::num::ParseIntError) -> Self {
        DateTimeError::Parse
    }
}

impl DateTimeError {
    pub(crate) fn unknown_kind_from_bytes(bytes: &[u8]) -> Self {
        let tiny = bytes
            .get(0..16)
            .and_then(|x| TinyStr16::from_bytes(x).ok())
            .unwrap_or(tinystr!(16, "invalid"));
        Self::UnknownAnyCalendarKind(tiny)
    }

    pub(crate) fn unknown_kind_from_value(value: &Value) -> Self {
        let string = writeable::Writeable::write_to_string(value);
        Self::unknown_kind_from_bytes(string.as_bytes())
    }
}
