// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::TimeZoneError;
use core::str::FromStr;
use tinystr::{tinystr, TinyAsciiStr};
use zerovec::ule::{AsULE, ULE};
use zerovec::{ZeroSlice, ZeroVec};

/// The GMT offset in seconds for a timezone
#[derive(Copy, Clone, Debug)]
pub struct GmtOffset(i32);

impl Default for GmtOffset {
    fn default() -> Self {
        Self::utc()
    }
}

impl GmtOffset {
    /// Attempt to create a [`GmtOffset`] from a seconds input. It returns an error when the seconds
    /// overflows or underflows.
    pub fn try_from_offset_seconds(seconds: i32) -> Result<Self, TimeZoneError> {
        // Valid range is from GMT-12 to GMT+14 in seconds.
        if seconds < -(12 * 60 * 60) || seconds > (14 * 60 * 60) {
            Err(TimeZoneError::OffsetOutOfBounds)
        } else {
            Ok(Self(seconds))
        }
    }

    /// Creates a [`GmtOffset`] at UTC.
    pub const fn utc() -> Self {
        Self(0)
    }

    /// Create a [`GmtOffset`] from a seconds input without checking bounds.
    ///
    /// # Safety
    ///
    /// The seconds must be a valid value as returned by [`Self::offset_seconds`].
    #[inline]
    pub unsafe fn from_offset_seconds_unchecked(seconds: i32) -> Self {
        Self(seconds)
    }

    /// Returns the raw offset value in seconds.
    pub fn offset_seconds(self) -> i32 {
        self.0
    }

    /// Returns `true` if the [`GmtOffset`] is positive, otherwise `false`.
    pub fn is_positive(self) -> bool {
        self.0 >= 0
    }

    /// Returns `true` if the [`GmtOffset`] is zero, otherwise `false`.
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the [`GmtOffset`] has non-zero minutes, otherwise `false`.
    pub fn has_minutes(self) -> bool {
        self.0 % 3600 / 60 > 0
    }

    /// Returns `true` if the [`GmtOffset`] has non-zero seconds, otherwise `false`.
    pub fn has_seconds(self) -> bool {
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
    /// let offset0: GmtOffset =
    ///     "Z".parse().expect("Failed to parse a GMT offset.");
    /// let offset1: GmtOffset =
    ///     "-09".parse().expect("Failed to parse a GMT offset.");
    /// let offset2: GmtOffset =
    ///     "-0930".parse().expect("Failed to parse a GMT offset.");
    /// let offset3: GmtOffset =
    ///     "-09:30".parse().expect("Failed to parse a GMT offset.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut chars = input.chars();
        let offset_sign = match chars.next() {
            Some('+') => 1,
            Some('-' | '\u{2212}') => -1,
            Some('Z') => return Ok(Self(0)),
            _ => return Err(TimeZoneError::InvalidOffset),
        };

        let input = chars.as_str();

        let seconds = match input.len() {
            /* ±hh */
            2 => {
                let hour = input
                    .parse::<u8>()
                    .map_err(|_| TimeZoneError::InvalidOffset)?;
                if hour > 24 {
                    return Err(TimeZoneError::InvalidOffset);
                }
                offset_sign * (hour as i32 * 60 * 60)
            }
            /* ±hhmm */
            4 if input.is_char_boundary(2) => {
                #[allow(clippy::indexing_slicing)] // validated
                {
                    let hour = input[0..2]
                        .parse::<u8>()
                        .map_err(|_| TimeZoneError::InvalidOffset)?;
                    let minute = input[2..4]
                        .parse::<u8>()
                        .map_err(|_| TimeZoneError::InvalidOffset)?;
                    offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
                }
            }
            /* ±hh:mm */
            5 => {
                if let Some((hour, minute)) = input.split_once(':') {
                    let hour = hour
                        .parse::<u8>()
                        .map_err(|_| TimeZoneError::InvalidOffset)?;
                    if hour > 24 {
                        return Err(TimeZoneError::InvalidOffset);
                    }
                    let minute = minute
                        .parse::<u8>()
                        .map_err(|_| TimeZoneError::InvalidOffset)?;
                    if minute > 60 {
                        return Err(TimeZoneError::InvalidOffset);
                    }
                    offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
                } else {
                    return Err(TimeZoneError::InvalidOffset);
                }
            }
            _ => return Err(TimeZoneError::InvalidOffset),
        };

        Self::try_from_offset_seconds(seconds)
    }
}

/// A time zone variant: currently either daylight time or standard time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, ULE)]
#[repr(transparent)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct ZoneVariant(pub TinyAsciiStr<2>);

impl FromStr for ZoneVariant {
    type Err = <TinyAsciiStr<2> as FromStr>::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse().map(ZoneVariant)
    }
}

impl ZoneVariant {
    /// Return the standard time `ZoneVariant`.
    ///
    /// Corresponds to the `"standard"` variant string in CLDR.
    pub const fn standard() -> Self {
        Self(tinystr!(2, "st"))
    }
    /// Return the daylight time `ZoneVariant`
    ///
    /// Corresponds to the `"daylight"` variant string in CLDR.
    pub const fn daylight() -> Self {
        Self(tinystr!(2, "dt"))
    }
}

impl From<TinyAsciiStr<2>> for ZoneVariant {
    fn from(other: TinyAsciiStr<2>) -> Self {
        Self(other)
    }
}

impl From<ZoneVariant> for TinyAsciiStr<2> {
    fn from(other: ZoneVariant) -> Self {
        other.0
    }
}

impl AsULE for ZoneVariant {
    type ULE = Self;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for ZoneVariant {
    type Container = ZeroVec<'a, ZoneVariant>;
    type Slice = ZeroSlice<ZoneVariant>;
    type GetType = ZoneVariant;
    type OwnedType = ZoneVariant;
}
