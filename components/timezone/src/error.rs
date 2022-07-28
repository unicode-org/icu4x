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
    /// An input overflowed its range.
    #[displaydoc("GmtOffset must be between (-12 × 60 × 60) - (14 × 60 × 60)")]
    OffsetOutOfBounds,
    /// The time zone offset was invalid.
    #[displaydoc("Failed to parse time-zone offset")]
    InvalidOffset,
}
