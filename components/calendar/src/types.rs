// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains various types used by `icu_calendar` and `icu_datetime`

use crate::error::DateTimeError;
use core::convert::TryFrom;
use core::convert::TryInto;
use core::ops::{Add, Sub};
use core::str::FromStr;
use tinystr::TinyStr8;

/// TODO(#486): Implement era codes.
#[derive(Clone, Debug, PartialEq)]
pub struct Era(pub TinyStr8);

/// Representation of a formattable year.
#[derive(Clone, Debug, PartialEq)]
pub struct Year {
    /// The era containing the year.
    pub era: Era,

    /// The year number in the current era (usually 1-based).
    pub number: i32,

    /// The related ISO year. This is normally the ISO (proleptic Gregorian) year having the greatest
    /// overlap with the calendar year. It is used in certain date formatting patterns.
    pub related_iso: i32,
}

/// TODO(#486): Implement month codes.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthCode(pub TinyStr8);

/// Representation of a formattable month.
#[derive(Clone, Debug, PartialEq)]
pub struct Month {
    /// A month number in a year. In normal years, this is usually the 1-based month index. In leap
    /// years, this is what the month number would have been in a non-leap year.
    ///
    /// For example:
    ///
    /// - January = 1
    /// - December = 12
    /// - Adar, Adar I, and Adar II = 6
    ///
    /// The `code` property is used to distinguish between unique months in leap years.
    pub number: u32,

    /// The month code, used to distinguish months during leap years.
    pub code: MonthCode,
}

/// A struct containing various details about the position of the day within a year. It is returned
// by the [`day_of_year_info()`](trait.DateInput.html#tymethod.day_of_year_info) method of the
// [`DateInput`] trait.
#[derive(Clone, Debug, PartialEq)]
pub struct DayOfYearInfo {
    /// The current day of the year, 1-based.
    pub day_of_year: u32,
    /// The number of days in a year.
    pub days_in_year: u32,
    /// The previous year.
    pub prev_year: Year,
    /// The next year.
    pub next_year: Year,
}

/// A day number in a month. Usually 1-based.
pub struct DayOfMonth(pub u32);

/// A week number in a month. Usually 1-based.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeekOfMonth(pub u32);

/// A week number in a year. Usually 1-based.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeekOfYear(pub u32);

/// This macro defines a struct for 0-based date fields: hours, minutes, and seconds. Each
/// unit is bounded by a range. The traits implemented here will return a Result on
/// whether or not the unit is in range from the given input.
macro_rules! dt_unit {
    ($name:ident, $value:expr, $docs:expr) => {
        #[doc=$docs]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
        pub struct $name(u8);

        impl $name {
            /// Do not validate the numeric input for this component.
            pub const fn new_unchecked(input: u8) -> Self {
                Self(input)
            }
        }

        impl FromStr for $name {
            type Err = DateTimeError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                let val: u8 = input.parse()?;
                if val > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(val))
                }
            }
        }

        impl TryFrom<u8> for $name {
            type Error = DateTimeError;

            fn try_from(input: u8) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(input))
                }
            }
        }

        impl TryFrom<usize> for $name {
            type Error = DateTimeError;

            fn try_from(input: usize) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(DateTimeError::Overflow {
                        field: "$name",
                        max: $value,
                    })
                } else {
                    Ok(Self(input as u8))
                }
            }
        }

        impl From<$name> for u8 {
            fn from(input: $name) -> Self {
                input.0
            }
        }

        impl From<$name> for usize {
            fn from(input: $name) -> Self {
                input.0 as Self
            }
        }

        impl Add<u8> for $name {
            type Output = Self;

            fn add(self, other: u8) -> Self {
                Self(self.0 + other)
            }
        }

        impl Sub<u8> for $name {
            type Output = Self;

            fn sub(self, other: u8) -> Self {
                Self(self.0 - other)
            }
        }
    };
}

dt_unit!(
    IsoHour,
    24,
    "An ISO-8601 hour component, for use with the [`IsoTimeInput`]."
);

dt_unit!(
    IsoMinute,
    60,
    "An ISO-8601 minute component, for use with the [`IsoTimeInput`]."
);

dt_unit!(
    IsoSecond,
    61,
    "An ISO-8601 second component, for use with the [`IsoTimeInput`]."
);

#[derive(Debug)]
pub struct Time {
    /// 0-based hour.
    pub hour: IsoHour,

    /// 0-based minute.
    pub minute: IsoMinute,

    /// 0-based second.
    pub second: IsoSecond,
}

impl Time {
    /// Do not validate the numeric input for this component.
    pub const fn new(hour: IsoHour, minute: IsoMinute, second: IsoSecond) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    pub fn try_new(hour: u8, minute: u8, second: u8) -> Result<Self, DateTimeError> {
        Ok(Self {
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
        })
    }
}

// TODO(#485): Improve FractionalSecond.
/// A placeholder for fractional seconds support. See [Issue #485](https://github.com/unicode-org/icu4x/issues/485)
/// for tracking the support of this feature.
#[derive(Clone, Debug, PartialEq)]
pub enum FractionalSecond {
    /// The millisecond component of the fractional second.
    Millisecond(u16),
    /// The microsecond component of the fractional second.
    Microsecond(u32),
    /// The nanosecond component of the fractional second.
    Nanosecond(u32),
}

/// The GMT offset in seconds for a [`MockTimeZone`](crate::mock::time_zone::MockTimeZone).
#[derive(Copy, Clone, Debug, Default)]
pub struct GmtOffset(i32);

impl GmtOffset {
    /// Attempt to create a [`GmtOffset`] from a seconds input. It returns an error when the seconds
    /// overflows or underflows.
    pub fn try_new(seconds: i32) -> Result<Self, DateTimeError> {
        // Valid range is from GMT-12 to GMT+14 in seconds.
        if seconds < -(12 * 60 * 60) {
            Err(DateTimeError::Underflow {
                field: "GmtOffset",
                min: -(12 * 60 * 60),
            })
        } else if seconds > (14 * 60 * 60) {
            Err(DateTimeError::Overflow {
                field: "GmtOffset",
                max: (14 * 60 * 60),
            })
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
    type Err = DateTimeError;

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
    /// use icu::datetime::date::GmtOffset;
    ///
    /// let offset0: GmtOffset = "Z".parse().expect("Failed to parse a GMT offset.");
    /// let offset1: GmtOffset = "-09".parse().expect("Failed to parse a GMT offset.");
    /// let offset2: GmtOffset = "-0930".parse().expect("Failed to parse a GMT offset.");
    /// let offset3: GmtOffset = "-09:30".parse().expect("Failed to parse a GMT offset.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let offset_sign;
        match input.chars().next() {
            Some('+') => offset_sign = 1,
            /* ASCII  */ Some('-') => offset_sign = -1,
            /* U+2212 */ Some('−') => offset_sign = -1,
            Some('Z') => return Ok(Self(0)),
            _ => return Err(DateTimeError::InvalidTimeZoneOffset),
        };

        let seconds = match input.chars().count() {
            /* ±hh */
            3 => {
                let hour: u8 = input[1..3].parse()?;
                offset_sign * (hour as i32 * 60 * 60)
            }
            /* ±hhmm */
            5 => {
                let hour: u8 = input[1..3].parse()?;
                let minute: u8 = input[3..5].parse()?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            /* ±hh:mm */
            6 => {
                let hour: u8 = input[1..3].parse()?;
                let minute: u8 = input[4..6].parse()?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            _ => panic!("Invalid time-zone designator"),
        };

        Self::try_new(seconds)
    }
}

/// A weekday in a 7-day week, according to ISO-8601.
///
/// The discriminant values correspond to ISO-8601 weekday numbers (Monday = 1, Sunday = 7).
///
/// # Examples
///
/// ```
/// use icu::datetime::date::IsoWeekday;
///
/// assert_eq!(1, IsoWeekday::Monday as usize);
/// assert_eq!(7, IsoWeekday::Sunday as usize);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)] // The weekday variants should be self-obvious.
#[repr(i8)]
pub enum IsoWeekday {
    Monday = 1,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<usize> for IsoWeekday {
    /// Convert from an ISO-8601 weekday number to an [`IsoWeekday`] enum. 0 is automatically converted
    /// to 7 (Sunday). If the number is out of range, it is interpreted modulo 7.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::date::IsoWeekday;
    ///
    /// assert_eq!(IsoWeekday::Sunday, IsoWeekday::from(0));
    /// assert_eq!(IsoWeekday::Monday, IsoWeekday::from(1));
    /// assert_eq!(IsoWeekday::Sunday, IsoWeekday::from(7));
    /// assert_eq!(IsoWeekday::Monday, IsoWeekday::from(8));
    /// ```
    fn from(input: usize) -> Self {
        let mut ordinal = (input % 7) as i8;
        if ordinal == 0 {
            ordinal = 7;
        }
        unsafe { core::mem::transmute(ordinal) }
    }
}
