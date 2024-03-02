// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//! Parse records for `ixdtf`'s parsers to target.

use alloc::string::String;

/// An `IsoParseRecord` is an intermediary record returned by ISO parsing functions.
///
/// `IsoParseRecord` is converted into the ISO AST Nodes.
#[non_exhaustive]
#[derive(Default, Debug, PartialEq)]
pub struct IsoParseRecord {
    /// Parsed Date Record
    pub date: DateRecord,
    /// Parsed Time
    pub time: Option<TimeRecord>,
    /// Parsed `TimeZone` data (UTCOffset | IANA name)
    pub tz: Option<TimeZone>,
    /// The parsed calendar value.
    pub calendar: Option<String>,
}

#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TimeRecord {
    /// An hour
    pub hour: u8,
    /// A minute value
    pub minute: u8,
    /// A second value.
    pub second: u8,
    /// A millisecond value
    pub millisecond: u16,
    /// A Microsecond value
    pub microsecond: u16,
    /// A Nanosecond value
    pub nanosecond: u16,
}

/// `TimeZone` data
#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TimeZone {
    /// TimeZoneIANAName
    pub name: Option<String>,
    /// TimeZoneOffset
    pub offset: Option<UTCOffset>,
}

/// A full precision `UtcOffset`
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UTCOffset {
    /// The `+`/`-` sign of this `UtcOffset`
    pub sign: i8,
    /// The hour value of the `UtcOffset`
    pub hour: u8,
    /// The minute value of the `UtcOffset`.
    pub minute: u8,
    /// The second value of the `UtcOffset`.
    pub second: u8,
    /// Any millisecond value of the `UTCOffset`
    pub millisecond: u16,
    /// Any microsecond value of the `UTCOffset`
    pub microsecond: u16,
    /// Any nanosecond value of the `UTCOffset`
    pub nanosecond: u16,
}

/// An ISO8601 `DurationRecord` Parse Node.
#[non_exhaustive]
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
    pub hours: f64,
    /// The `minutes` value.
    pub minutes: f64,
    /// The `seconds` value.
    pub seconds: f64,
    /// The `milliseconds` value.
    pub milliseconds: f64,
    /// The `microseconds` value.
    pub microseconds: f64,
    /// The `nanoseconds` value.
    pub nanoseconds: f64,
}

impl DurationParseRecord {
    pub(crate) fn from_records(
        sign: bool,
        date: DateDurationRecord,
        time: TimeDurationRecord,
    ) -> Self {
        let minutes = if time.fhours > 0.0 {
            time.fhours * 60.0
        } else {
            f64::from(time.minutes)
        };

        let seconds = if time.fminutes > 0.0 {
            time.fminutes * 60.0
        } else if time.seconds > 0 {
            f64::from(time.seconds)
        } else {
            (minutes % 1.0) * 60.0
        };

        let milliseconds = if time.fseconds > 0.0 {
            time.fseconds * 1000.0
        } else {
            (seconds % 1.0) * 1000.0
        };

        let microseconds = (milliseconds % 1.0) * 1000.0;
        let nanoseconds = (microseconds % 1.0) * 1000.0;

        let sign: i32 = if sign { 1 } else { -1 };

        Self {
            years: date.years * sign,
            months: date.months * sign,
            weeks: date.weeks * sign,
            days: date.days * sign,
            hours: f64::from(time.hours) * f64::from(sign),
            minutes: minutes * f64::from(sign),
            seconds: seconds * f64::from(sign),
            milliseconds: milliseconds * f64::from(sign),
            microseconds: microseconds * f64::from(sign),
            nanoseconds: nanoseconds * f64::from(sign),
        }
    }
}

/// A `DateDuration` Parse Node.
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
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct TimeDurationRecord {
    /// Hours value.
    pub(crate) hours: i32,
    /// Hours fraction value.
    pub(crate) fhours: f64,
    /// Minutes value with fraction.
    pub(crate) minutes: i32,
    /// Minutes fraction value.
    pub(crate) fminutes: f64,
    /// Seconds value with fraction.
    pub(crate) seconds: i32,
    /// Seconds fraction value,
    pub(crate) fseconds: f64,
}
