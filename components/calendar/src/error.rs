// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use tinystr::{tinystr, TinyStr16, TinyStr4};
use writeable::Writeable;

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
    /// Create an error when an [`AnyCalendarKind`] is expected but not available.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_calendar::AnyCalendarKind;
    /// use icu_calendar::DateTimeError;
    ///
    /// let cal_str = "maori";
    ///
    /// AnyCalendarKind::get_for_bcp47_string(cal_str)
    ///     .ok_or_else(|| DateTimeError::unknown_any_calendar_kind(cal_str))
    ///     .expect_err("Māori calendar is not yet supported");
    /// ```
    ///
    /// [`AnyCalendarKind`]: crate::AnyCalendarKind
    pub fn unknown_any_calendar_kind(description: impl Writeable) -> Self {
        let tiny = description
            .write_to_string()
            .get(0..16)
            .and_then(|x| TinyStr16::from_str(x).ok())
            .unwrap_or(tinystr!(16, "invalid"));
        Self::UnknownAnyCalendarKind(tiny)
    }
}
