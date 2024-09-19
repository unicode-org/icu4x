// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Some useful parsing functions for tests.

use icu_calendar::{DateTime, Gregorian};
use icu_timezone::{FormattableZonedDateTime, ZonedDateTime};

/// Temporary function for parsing a `DateTime<Gregorian>`
///
/// This utility is for easily creating dates, not a complete robust solution. The
/// string must take a specific form of the ISO-8601 format: `YYYY-MM-DDThh:mm:ss`.
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::mock::parse_gregorian_from_str;
///
/// let date: DateTime<Gregorian> =
///     parse_gregorian_from_str("2020-10-14T13:21:00")
///         .expect("Failed to parse a datetime.");
/// ```
///
/// Optionally, fractional seconds can be specified: `YYYY-MM-DDThh:mm:ss.SSS`.
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::mock::parse_gregorian_from_str;
///
/// let date: DateTime<Gregorian> =
///     parse_gregorian_from_str("2020-10-14T13:21:00.101")
///         .expect("Failed to parse a datetime.");
/// assert_eq!(u32::from(date.time.nanosecond), 101_000_000);
/// ```
pub fn parse_gregorian_from_str(input: &str) -> DateTime<Gregorian> {
    let datetime_iso = DateTime::try_iso_from_str(input).unwrap();
    datetime_iso.to_calendar(Gregorian)
}

/// Parse a [`FormattableZonedDateTime`] from a string.
///
/// This utility is for easily creating dates, not a complete robust solution. The
/// string must take a specific form of the ISO 8601 format:
/// `YYYY-MM-DDThh:mm:ssZ`,
/// `YYYY-MM-DDThh:mm:ss±hh`,
/// `YYYY-MM-DDThh:mm:ss±hhmm`,
/// `YYYY-MM-DDThh:mm:ss±hh:mm`,
///
/// # Examples
///
/// ```
/// use icu::datetime::mock;
///
/// let (date, zone) =
///     mock::parse_zoned_gregorian_from_str("2020-10-14T13:21:00+05:30")
///         .expect("Failed to parse a zoned datetime.");
/// ```
pub fn parse_zoned_gregorian_from_str(input: &str) -> FormattableZonedDateTime<Gregorian> {
    let datetime_iso = ZonedDateTime::try_iso_from_str(input).unwrap();
    datetime_iso.to_calendar(Gregorian).into_formattable()
}
