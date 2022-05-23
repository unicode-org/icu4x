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
/// let date: DateTime<Gregorian> =
///     parse_gregorian_from_str("2020-10-14T13:21:00").expect("Failed to parse a datetime.");
/// ```
///
/// Optionally, fractional seconds can be specified: `YYYY-MM-DDThh:mm:ss.SSS`.
///
/// ```
/// use icu::datetime::mock::parse_gregorian_from_str;
/// use icu_calendar::{DateTime, Gregorian};
///
/// let date: DateTime<Gregorian> =
///     parse_gregorian_from_str("2020-10-14T13:21:00.101").expect("Failed to parse a datetime.");
/// assert_eq!(u32::from(date.time.nanosecond), 101_000_000);
/// ```
pub fn parse_gregorian_from_str(input: &str) -> Result<DateTime<Gregorian>, DateTimeError> {
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let year: i32 = input[0..4].parse()?;
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let month: u8 = input[5..7].parse()?;
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let day: u8 = input[8..10].parse()?;
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let hour: u8 = input[11..13].parse()?;
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let minute: u8 = input[14..16].parse()?;
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let second: u8 = input[17..19].parse()?;
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    let fraction: u32 = if input.len() > 20 {
        // Extract fractional input and trim any trailing zeros
        let mut fraction = input[20..].trim_end_matches('0');
        // Truncate to at most 9 digits
        if fraction.len() > 9 {
            fraction = &fraction[0..9];
        }
        let as_int: u32 = fraction.parse().unwrap_or(0);
        // Convert fraction to nanoseconds
        as_int * (10u32.pow(9 - fraction.len() as u32))
    } else {
        0
    };
    DateTime::new_gregorian_datetime_from_integers(year, month, day, hour, minute, second, fraction)
}

#[test]
fn test_parsing_fractional_seconds() {
    use icu::datetime::mock::parse_gregorian_from_str;
    use icu_calendar::{DateTime, Gregorian};

    // Milliseconds
    let date: DateTime<Gregorian> =
        parse_gregorian_from_str("2020-10-14T13:21:00.123").expect("Failed to parse a datetime.");
    assert_eq!(u32::from(date.time.nanosecond), 123_000_000);

    // All zeros
    let date: DateTime<Gregorian> =
        parse_gregorian_from_str("2020-10-14T13:21:00.000").expect("Failed to parse a datetime.");
    assert_eq!(u32::from(date.time.nanosecond), 0);

    // Leading zeros
    let date: DateTime<Gregorian> = parse_gregorian_from_str("2020-10-14T13:21:00.000123")
        .expect("Failed to parse a datetime.");
    assert_eq!(u32::from(date.time.nanosecond), 123_000);

    // Trailing zeros
    let date: DateTime<Gregorian> = parse_gregorian_from_str("2020-10-14T13:21:00.123000000000000")
        .expect("Failed to parse a datetime.");
    assert_eq!(u32::from(date.time.nanosecond), 123_000_000);

    // Too much precision, should truncate to nanoseconds
    let date: DateTime<Gregorian> = parse_gregorian_from_str("2020-10-14T13:21:00.123456789999999")
        .expect("Failed to parse a datetime.");
    assert_eq!(u32::from(date.time.nanosecond), 123_456_789);
}
