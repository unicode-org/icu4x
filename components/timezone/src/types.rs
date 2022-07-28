// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::TimeZoneError;
use core::str::FromStr;

/// The GMT offset in seconds for a timezone
#[derive(Copy, Clone, Debug, Default)]
pub struct GmtOffset(i32);

impl GmtOffset {
    /// Attempt to create a [`GmtOffset`] from a seconds input. It returns an error when the seconds
    /// overflows or underflows.
    pub fn try_new(seconds: i32) -> Result<Self, TimeZoneError> {
        // Valid range is from GMT-12 to GMT+14 in seconds.
        if seconds < -(12 * 60 * 60) {
            Err(TimeZoneError::OffsetOutOfBounds)
        } else if seconds > (14 * 60 * 60) {
            Err(TimeZoneError::OffsetOutOfBounds)
        } else {
            Ok(Self(seconds))
        }
    }

    /// Returns the raw offset value in seconds.
    pub fn raw_offset_seconds(&self) -> i32 {
        self.0
    }

    /// Returns `true` if the [`GmtOffset`] is positive, otherwise `false`.
    pub fn is_positive(&self) -> bool {
        self.0 >= 0
    }

    /// Returns `true` if the [`GmtOffset`] is zero, otherwise `false`.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the [`GmtOffset`] has non-zero minutes, otherwise `false`.
    pub fn has_minutes(&self) -> bool {
        self.0 % 3600 / 60 > 0
    }

    /// Returns `true` if the [`GmtOffset`] has non-zero seconds, otherwise `false`.
    pub fn has_seconds(&self) -> bool {
        self.0 % 3600 % 60 > 0
    }
}

impl FromStr for GmtOffset {
    type Err = TimeZoneError;

    /// Parse a [`GmtOffset`] from a string.
    ///
    /// The offset must range from GMT-12 to GMT+14.
    /// The string must be an ISO 8601 time zone designator:
    /// e.g. Z
    /// e.g. +05
    /// e.g. +0500
    /// e.g. +05:00
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_timezone::GmtOffset;
    ///
    /// let offset0: GmtOffset = "Z".parse().expect("Failed to parse a GMT offset.");
    /// let offset1: GmtOffset = "-09".parse().expect("Failed to parse a GMT offset.");
    /// let offset2: GmtOffset = "-0930".parse().expect("Failed to parse a GMT offset.");
    /// let offset3: GmtOffset = "-09:30".parse().expect("Failed to parse a GMT offset.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let offset_sign = match input.chars().next() {
            Some('+') => 1,
            /* ASCII */ Some('-') => -1,
            /* U+2212 */ Some('−') => -1,
            Some('Z') => return Ok(Self(0)),
            _ => return Err(TimeZoneError::InvalidOffset),
        };

        let seconds = match input.chars().count() {
            /* ±hh */
            3 => {
                #[allow(clippy::indexing_slicing)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                let hour: u8 = input[1..3]
                    .parse()
                    .map_err(|_| TimeZoneError::InvalidOffset)?;
                offset_sign * (hour as i32 * 60 * 60)
            }
            /* ±hhmm */
            5 => {
                #[allow(clippy::indexing_slicing)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                let hour: u8 = input[1..3]
                    .parse()
                    .map_err(|_| TimeZoneError::InvalidOffset)?;
                #[allow(clippy::indexing_slicing)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                let minute: u8 = input[3..5]
                    .parse()
                    .map_err(|_| TimeZoneError::InvalidOffset)?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            /* ±hh:mm */
            6 => {
                #[allow(clippy::indexing_slicing)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                let hour: u8 = input[1..3]
                    .parse()
                    .map_err(|_| TimeZoneError::InvalidOffset)?;
                #[allow(clippy::indexing_slicing)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                let minute: u8 = input[4..6]
                    .parse()
                    .map_err(|_| TimeZoneError::InvalidOffset)?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            #[allow(clippy::panic)] // TODO(#1668) Clippy exceptions need docs or fixing.
            _ => panic!("Invalid time-zone designator"),
        };

        Self::try_new(seconds)
    }
}
