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

fn try_get_time_component([tens, ones]: [u8; 2]) -> Option<i32> {
    Some(((tens as char).to_digit(10)? * 10 + (ones as char).to_digit(10)?) as i32)
}

impl GmtOffset {
    /// Attempt to create a [`GmtOffset`] from a seconds input. It returns
    /// [`TimeZoneError::OffsetOutOfBounds`] when the seconds are out of bounds.
    pub fn try_from_offset_seconds(seconds: i32) -> Result<Self, TimeZoneError> {
        if seconds.unsigned_abs() > 18 * 60 * 60 {
            Err(TimeZoneError::OffsetOutOfBounds)
        } else {
            Ok(Self(seconds))
        }
    }

    /// Creates a [`GmtOffset`] at UTC.
    pub const fn utc() -> Self {
        Self(0)
    }

    /// Parse a [`GmtOffset`] from bytes.
    ///
    /// The offset must range from GMT-12 to GMT+14.
    /// The string must be an ISO-8601 time zone designator:
    /// e.g. Z
    /// e.g. +05
    /// e.g. +0500
    /// e.g. +05:00
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::GmtOffset;
    /// use icu::timezone::TimeZoneError;
    ///
    /// let offset0: GmtOffset =
    ///     GmtOffset::try_from_bytes(b"Z").expect("Failed to parse a time zone");
    /// let offset1: GmtOffset =
    ///     GmtOffset::try_from_bytes(b"+05").expect("Failed to parse a time zone");
    /// let offset2: GmtOffset = GmtOffset::try_from_bytes(b"+0500")
    ///     .expect("Failed to parse a time zone");
    /// let offset3: GmtOffset = GmtOffset::try_from_bytes(b"-05:00")
    ///     .expect("Failed to parse a time zone");
    /// let offset_err0: TimeZoneError =
    ///     GmtOffset::try_from_bytes(b"0500").expect_err("Invalid input");
    /// let offset_err1: TimeZoneError =
    ///     GmtOffset::try_from_bytes(b"+05000").expect_err("Invalid input");
    ///
    /// assert_eq!(offset0.offset_seconds(), 0);
    /// assert_eq!(offset1.offset_seconds(), 18000);
    /// assert_eq!(offset2.offset_seconds(), 18000);
    /// assert_eq!(offset3.offset_seconds(), -18000);
    /// assert_eq!(offset_err0, TimeZoneError::InvalidOffset);
    /// assert_eq!(offset_err1, TimeZoneError::InvalidOffset);
    /// ```
    pub fn try_from_bytes(mut chars: &[u8]) -> Result<Self, TimeZoneError> {
        let offset_sign = match chars {
            [b'+', rest @ ..] => {
                chars = rest;
                1
            }
            [b'-', rest @ ..] => {
                chars = rest;
                -1
            }
            // Unicode minus ("\u{2212}" == [226, 136, 146])
            [226, 136, 146, rest @ ..] => {
                chars = rest;
                -1
            }
            [b'Z'] => return Ok(Self(0)),
            _ => return Err(TimeZoneError::InvalidOffset),
        };

        let hours = match chars {
            &[h1, h2, ..] => try_get_time_component([h1, h2]),
            _ => None,
        }
        .ok_or(TimeZoneError::InvalidOffset)?;

        let minutes = match chars {
            /* ±hh */
            &[_, _] => Some(0),
            /* ±hhmm, ±hh:mm */
            &[_, _, m1, m2] | &[_, _, b':', m1, m2] => {
                try_get_time_component([m1, m2]).filter(|&m| m < 60)
            }
            _ => None,
        }
        .ok_or(TimeZoneError::InvalidOffset)?;

        Self::try_from_offset_seconds(offset_sign * (hours * 60 + minutes) * 60)
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
    /// let offset0: GmtOffset = "Z".parse().expect("Failed to parse a GMT offset");
    /// let offset1: GmtOffset =
    ///     "-09".parse().expect("Failed to parse a GMT offset");
    /// let offset2: GmtOffset =
    ///     "-0930".parse().expect("Failed to parse a GMT offset");
    /// let offset3: GmtOffset =
    ///     "-09:30".parse().expect("Failed to parse a GMT offset");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        GmtOffset::try_from_bytes(input.as_bytes())
    }
}

/// A time zone variant: currently either daylight time or standard time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, ULE)]
#[repr(transparent)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_timezone))]
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
