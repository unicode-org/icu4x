// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The records that `ixdtf`'s contain the resulting values of parsing.

#[cfg(feature = "duration")]
use core::ops::Mul;

use alloc::vec::Vec;

/// An `IxdtfParseRecord` is an intermediary record returned by `IxdtfParser`.
#[non_exhaustive]
#[derive(Default, Debug, PartialEq)]
pub struct IsoParseRecord<'a> {
    /// Parsed Date Record
    pub date: Option<DateRecord>,
    /// Parsed Time
    pub time: Option<TimeRecord>,
    /// Parsed UtcOffset
    pub offset: Option<UTCOffsetRecord>,
    /// Parsed `TimeZone` data (UTCOffset | IANA name)
    pub tz: Option<TimeZoneRecord<'a>>,
    /// The parsed calendar value.
    pub calendar: Option<&'a str>,
    /// Annotations
    pub annotations: Vec<Annotation<'a>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
/// A record of an annotation.
pub struct Annotation<'a> {
    pub critical: bool,
    pub key: &'a str,
    pub value: &'a str,
}

#[allow(clippy::exhaustive_structs)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
/// The record of a parsed date.
pub struct DateRecord {
    /// Date Year
    pub year: i32,
    /// Date Month
    pub month: u8,
    /// Date Day
    pub day: u8,
}

/// Parsed Time info
#[allow(clippy::exhaustive_structs)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TimeRecord {
    /// An hour
    pub hour: u8,
    /// A minute value
    pub minute: u8,
    /// A second value.
    pub second: u8,
    /// A nanosecond value representing all sub-second components.
    pub nanosecond: u32,
}

/// `TimeZone` data
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum TimeZoneRecord<'a> {
    /// TimeZoneIANAName
    Name(&'a str),
    /// TimeZoneOffset
    Offset(UTCOffsetRecord),
}

/// A full precision `UtcOffset`
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UTCOffsetRecord {
    /// The `+`/`-` sign of this `UtcOffset`
    pub sign: i8,
    /// The hour value of the `UtcOffset`
    pub hour: u8,
    /// The minute value of the `UtcOffset`.
    pub minute: u8,
    /// The second value of the `UtcOffset`.
    pub second: u8,
    /// Any nanosecond value of the `UTCOffset`
    pub nanosecond: u32,
}

impl Default for UTCOffsetRecord {
    fn default() -> Self {
        Self {
            sign: 1,
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
        }
    }
}

/// The resulting record of a `Duration` parse.
#[non_exhaustive]
#[cfg(feature = "duration")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DurationParseRecord {
    /// Duration Sign
    pub years: i32,
    /// A `DateDuration` record.
    pub months: i32,
    /// The `weeks` value.
    pub weeks: i32,
    /// The `days` value.
    pub days: i32,
    /// The `hours` value.
    pub hours: i32,
    /// The `minutes` value.
    pub minutes: i32,
    /// The `seconds` value.
    pub seconds: i32,
    /// Any fraction part of a duration.
    pub fraction: Option<DurationFraction>,
}

/// An enum representing the fraction part of a duration.
#[non_exhaustive]
#[cfg(feature = "duration")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DurationFraction {
    /// The fraction value applied to an hour.
    Hours(i64),
    /// The fraction value applied to the minutes field.
    Minutes(i64),
    /// The fraction value applied to the seconds field.
    Seconds(i32),
}

#[cfg(feature = "duration")]
impl Mul<i32> for DurationFraction {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        match self {
            Self::Hours(i) => Self::Hours(i * i64::from(rhs)),
            Self::Minutes(i) => Self::Minutes(i * i64::from(rhs)),
            Self::Seconds(i) => Self::Seconds(i * rhs),
        }
    }
}

/// A `DateDuration` Parse Node.
#[cfg(feature = "duration")]
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct DateDurationRecord {
    /// Years value.
    pub(crate) years: i32,
    /// Months value.
    pub(crate) months: i32,
    /// Weeks value.
    pub(crate) weeks: i32,
    /// Days value.
    pub(crate) days: i32,
}

/// A `TimeDuration` Parse Node
#[cfg(feature = "duration")]
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct TimeDurationRecord {
    /// Hours value.
    pub(crate) hours: i32,
    /// Minutes value.
    pub(crate) minutes: i32,
    /// Seconds value.
    pub(crate) seconds: i32,
    /// Any parsed fraction value.
    pub(crate) fraction: Option<DurationFraction>,
}
