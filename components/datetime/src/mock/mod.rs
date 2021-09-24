// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of temporary structs and utilities to input data for tests, benchmarks,
//! and examples.

use icu_calendar::{DateTime, DateTimeError, Gregorian};

/// Temporary time zone input utilities.
pub mod time_zone;

/// Temporary zoned DateTime input utilities.
pub mod zoned_datetime;

/// Temporary function for parsing a `DateTime<Gregorian>`
///
/// This utility is for easily creating dates, not a complete robust solution. The
/// string must take a specific form of the ISO-8601 format: `YYYY-MM-DDThh:mm:ss`.
///
/// ```
/// use icu::datetime::mock::parse_gregorian_from_str;
/// use icu_calendar::{DateTime, Gregorian};
///
/// let date: DateTime<Gregorian> = parse_gregorian_from_str("2020-10-14T13:21:00")
///     .expect("Failed to parse a datetime.");
/// ```
pub fn parse_gregorian_from_str(input: &str) -> Result<DateTime<Gregorian>, DateTimeError> {
    let year: i32 = input[0..4].parse()?;
    let month: u8 = input[5..7].parse()?;
    let day: u8 = input[8..10].parse()?;
    let hour: u8 = input[11..13].parse()?;
    let minute: u8 = input[14..16].parse()?;
    let second: u8 = input[17..19].parse()?;
    DateTime::new_gregorian_datetime_from_integers(year, month, day, hour, minute, second)
}
