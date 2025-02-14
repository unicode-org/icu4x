// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::InvalidOffsetError;
use core::str::FromStr;
use icu_calendar::{AsCalendar, Date, RangeError};

/// This macro defines a struct for 0-based date fields: hours, minutes, seconds
/// and fractional seconds. Each unit is bounded by a range. The traits implemented
/// here will return a Result on whether or not the unit is in range from the given
/// input.
macro_rules! dt_unit {
    ($name:ident, $storage:ident, $value:expr, $(#[$docs:meta])+) => {
        $(#[$docs])+
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $name($storage);

        impl $name {
            /// Gets the numeric value for this component.
            pub const fn number(self) -> $storage {
                self.0
            }

            /// Creates a new value at 0.
            pub const fn zero() -> $name {
                Self(0)
            }

            /// Returns whether the value is zero.
            #[inline]
            pub fn is_zero(self) -> bool {
                self.0 == 0
            }
        }

        impl TryFrom<$storage> for $name {
            type Error = RangeError;

            fn try_from(input: $storage) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(RangeError {
                        field: stringify!($name),
                        min: 0,
                        max: $value,
                        value: input as i32,
                    })
                } else {
                    Ok(Self(input))
                }
            }
        }

        impl TryFrom<usize> for $name {
            type Error = RangeError;

            fn try_from(input: usize) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(RangeError {
                        field: "$name",
                        min: 0,
                        max: $value,
                        value: input as i32,
                    })
                } else {
                    Ok(Self(input as $storage))
                }
            }
        }

        impl From<$name> for $storage {
            fn from(input: $name) -> Self {
                input.0
            }
        }

        impl From<$name> for usize {
            fn from(input: $name) -> Self {
                input.0 as Self
            }
        }
    };
}

dt_unit!(
    Hour,
    u8,
    23,
    /// An ISO-8601 hour component, for use with ISO calendars.
    ///
    /// Must be within inclusive bounds `[0, 23]`.
);

dt_unit!(
    Minute,
    u8,
    59,
    /// An ISO-8601 minute component, for use with ISO calendars.
    ///
    /// Must be within inclusive bounds `[0, 59]`.
);

dt_unit!(
    Second,
    u8,
    60,
    /// An ISO-8601 second component, for use with ISO calendars.
    ///
    /// Must be within inclusive bounds `[0, 60]`. `60` accommodates for leap seconds.
);

dt_unit!(
    Nanosecond,
    u32,
    999_999_999,
    /// A fractional second component, stored as nanoseconds.
    ///
    /// Must be within inclusive bounds `[0, 999_999_999]`."
);

/// A representation of a time in hours, minutes, seconds, and nanoseconds
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Time {
    /// Hour
    pub hour: Hour,

    /// Minute
    pub minute: Minute,

    /// Second
    pub second: Second,

    /// Subsecond
    pub subsecond: Nanosecond,
}

impl Time {
    /// Construct a new [`Time`], without validating that all components are in range
    pub const fn new(hour: Hour, minute: Minute, second: Second, subsecond: Nanosecond) -> Self {
        Self {
            hour,
            minute,
            second,
            subsecond,
        }
    }

    /// Construct a new [`Time`] representing midnight (00:00.000)
    pub const fn midnight() -> Self {
        Self {
            hour: Hour::zero(),
            minute: Minute::zero(),
            second: Second::zero(),
            subsecond: Nanosecond::zero(),
        }
    }

    /// Construct a new [`Time`], whilst validating that all components are in range
    pub fn try_new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self, RangeError> {
        Ok(Self {
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
            subsecond: nanosecond.try_into()?,
        })
    }
}

/// A date + time for a given calendar.
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DateTime<A: AsCalendar> {
    /// The date
    pub date: Date<A>,
    /// The time
    pub time: Time,
}

/// A date and time local to a specified custom time zone.
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ZonedDateTime<A: AsCalendar, Z> {
    /// The date, local to the time zone
    pub date: Date<A>,
    /// The time, local to the time zone
    pub time: Time,
    /// The time zone
    pub zone: Z,
}

/// An offset from Coordinated Universal Time (UTC)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct UtcOffset(i32);

impl UtcOffset {
    /// Attempt to create a [`UtcOffset`] from a seconds input.
    ///
    /// Returns [`InvalidOffsetError`] if the seconds are out of bounds.
    pub fn try_from_seconds(seconds: i32) -> Result<Self, InvalidOffsetError> {
        if seconds.unsigned_abs() > 18 * 60 * 60 {
            Err(InvalidOffsetError)
        } else {
            Ok(Self(seconds))
        }
    }

    /// Creates a [`UtcOffset`] from eighths of an hour.
    ///
    /// This is chosen because eighths of an hour cover all current time zones
    /// and all values of i8 are within range of this type.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::UtcOffset;
    ///
    /// assert_eq!(
    ///     UtcOffset::try_from_str("-0600").unwrap(),
    ///     UtcOffset::from_eighths_of_hour(-6 * 8),
    /// );
    /// ```
    pub const fn from_eighths_of_hour(eighths_of_hour: i8) -> Self {
        Self(eighths_of_hour as i32 * 450)
    }

    /// Creates a [`UtcOffset`] of zero.
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Parse a [`UtcOffset`] from bytes.
    ///
    /// The offset must range from UTC-12 to UTC+14.
    ///
    /// The string must be an ISO-8601 time zone designator:
    /// e.g. Z
    /// e.g. +05
    /// e.g. +0500
    /// e.g. +05:00
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::UtcOffset;
    ///
    /// let offset0: UtcOffset = UtcOffset::try_from_str("Z").unwrap();
    /// let offset1: UtcOffset = UtcOffset::try_from_str("+05").unwrap();
    /// let offset2: UtcOffset = UtcOffset::try_from_str("+0500").unwrap();
    /// let offset3: UtcOffset = UtcOffset::try_from_str("-05:00").unwrap();
    ///
    /// let offset_err0 =
    ///     UtcOffset::try_from_str("0500").expect_err("Invalid input");
    /// let offset_err1 =
    ///     UtcOffset::try_from_str("+05000").expect_err("Invalid input");
    ///
    /// assert_eq!(offset0.to_seconds(), 0);
    /// assert_eq!(offset1.to_seconds(), 18000);
    /// assert_eq!(offset2.to_seconds(), 18000);
    /// assert_eq!(offset3.to_seconds(), -18000);
    /// ```
    #[inline]
    pub fn try_from_str(s: &str) -> Result<Self, InvalidOffsetError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    pub fn try_from_utf8(mut code_units: &[u8]) -> Result<Self, InvalidOffsetError> {
        fn try_get_time_component([tens, ones]: [u8; 2]) -> Option<i32> {
            Some(((tens as char).to_digit(10)? * 10 + (ones as char).to_digit(10)?) as i32)
        }

        let offset_sign = match code_units {
            [b'+', rest @ ..] => {
                code_units = rest;
                1
            }
            [b'-', rest @ ..] => {
                code_units = rest;
                -1
            }
            // Unicode minus ("\u{2212}" == [226, 136, 146])
            [226, 136, 146, rest @ ..] => {
                code_units = rest;
                -1
            }
            [b'Z'] => return Ok(Self(0)),
            _ => return Err(InvalidOffsetError),
        };

        let hours = match code_units {
            &[h1, h2, ..] => try_get_time_component([h1, h2]),
            _ => None,
        }
        .ok_or(InvalidOffsetError)?;

        let minutes = match code_units {
            /* ±hh */
            &[_, _] => Some(0),
            /* ±hhmm, ±hh:mm */
            &[_, _, m1, m2] | &[_, _, b':', m1, m2] => {
                try_get_time_component([m1, m2]).filter(|&m| m < 60)
            }
            _ => None,
        }
        .ok_or(InvalidOffsetError)?;

        Self::try_from_seconds(offset_sign * (hours * 60 + minutes) * 60)
    }

    /// Create a [`UtcOffset`] from a seconds input without checking bounds.
    #[inline]
    pub fn from_seconds_unchecked(seconds: i32) -> Self {
        Self(seconds)
    }

    /// Returns the raw offset value in seconds.
    pub fn to_seconds(self) -> i32 {
        self.0
    }

    /// Returns the raw offset value in eights of an hour (7.5 minute units).
    pub fn to_eighths_of_hour(self) -> i8 {
        (self.0 / 450) as i8
    }

    /// Whether the [`UtcOffset`] is non-negative.
    pub fn is_non_negative(self) -> bool {
        self.0 >= 0
    }

    /// Whether the [`UtcOffset`] is zero.
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the hours part of if the [`UtcOffset`]
    pub fn hours_part(self) -> i32 {
        self.0 / 3600
    }

    /// Returns the minutes part of if the [`UtcOffset`].
    pub fn minutes_part(self) -> u32 {
        (self.0 % 3600 / 60).unsigned_abs()
    }

    /// Returns the seconds part of if the [`UtcOffset`].
    pub fn seconds_part(self) -> u32 {
        (self.0 % 60).unsigned_abs()
    }
}

impl FromStr for UtcOffset {
    type Err = InvalidOffsetError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

#[cfg(doc)]
use crate::TimeZoneInfo;

/// A time zone variant, such as Standard Time, or Daylight/Summer Time.
///
/// This should not generally be constructed by client code. Instead, use
/// * [`TimeZoneInfo::with_rearguard_isdst`]
/// * [`TimeZoneInfo::infer_zone_variant`]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[zerovec::make_ule(TimeZoneVariantULE)]
#[repr(u8)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum TimeZoneVariant {
    /// The variant corresponding to `"standard"` in CLDR.
    ///
    /// The semantics vary from time zone to time zone. The time zone display
    /// name of this variant may or may not be called "Standard Time".
    ///
    /// This is the variant with the lower UTC offset.
    Standard = 0,
    /// The variant corresponding to `"daylight"` in CLDR.
    ///
    /// The semantics vary from time zone to time zone. The time zone display
    /// name of this variant may or may not be called "Daylight Time".
    ///
    /// This is the variant with the higher UTC offset.
    Daylight = 1,
}
