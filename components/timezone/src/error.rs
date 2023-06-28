// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_provider::DataError;

#[cfg(feature = "std")]
impl std::error::Error for TimeZoneError {}

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum TimeZoneError {
    /// An input overflowed its range.
    #[displaydoc("GmtOffset must be within ±18:00:00")]
    OffsetOutOfBounds,
    /// The time zone offset was invalid.
    #[displaydoc("Failed to parse time-zone offset")]
    InvalidOffset,
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("{0}")]
    Data(DataError),
}

impl From<DataError> for TimeZoneError {
    fn from(e: DataError) -> Self {
        TimeZoneError::Data(e)
    }
}
