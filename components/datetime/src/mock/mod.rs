// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of temporary structs and utilities to input data for tests, benchmarks,
//! and examples.

use core::str::FromStr;
use icu_calendar::{DateTime, DateTimeError, Gregorian};
use icu_timezone::CustomTimeZone;

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
    let mut datetime = DateTime::new_gregorian_datetime(year, month, day, hour, minute, second)?;
    datetime.time = icu_calendar::types::Time::try_new(hour, minute, second, fraction)?;

    Ok(datetime)
}

/// Parse a [`DateTime`] and [`CustomTimeZone`] from a string.
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
/// let (date, zone) = mock::parse_zoned_gregorian_from_str("2020-10-14T13:21:00+05:30")
///     .expect("Failed to parse a zoned datetime.");
/// ```
pub fn parse_zoned_gregorian_from_str(
    input: &str,
) -> Result<(DateTime<Gregorian>, CustomTimeZone), DateTimeError> {
    let datetime = parse_gregorian_from_str(input)?;
    let time_zone = match input
        .rfind(|c| c == '+' || /* ASCII */ c == '-' || /* U+2212 */ c == '−' || c == 'Z')
    {
        #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
        Some(index) => {
            FromStr::from_str(&input[index..]).map_err(|_| DateTimeError::InvalidTimeZoneOffset)?
        }
        None => return Err(DateTimeError::InvalidTimeZoneOffset),
    };

    Ok((datetime, time_zone))
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
