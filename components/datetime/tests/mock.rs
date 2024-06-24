// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Some useful parsing functions for tests.

use icu_calendar::{DateTime, Gregorian};
use icu_timezone::CustomTimeZone;

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
    #![allow(clippy::indexing_slicing)]
    assert!(input.len() > 20 || input.len() == 19);
    let year: i32 = input[0..4].parse().unwrap();
    assert_eq!(input.as_bytes()[4], b'-');
    let month: u8 = input[5..7].parse().unwrap();
    assert_eq!(input.as_bytes()[7], b'-');
    let day: u8 = input[8..10].parse().unwrap();
    assert_eq!(input.as_bytes()[10], b'T');
    let hour: u8 = input[11..13].parse().unwrap();
    assert_eq!(input.as_bytes()[13], b':');
    let minute: u8 = input[14..16].parse().unwrap();
    assert_eq!(input.as_bytes()[16], b':');
    let second: u8 = input[17..19].parse().unwrap();
    let mut datetime =
        DateTime::try_new_gregorian_datetime(year, month, day, hour, minute, second).unwrap();
    if input.len() > 20 {
        assert_eq!(input.as_bytes()[19], b'.');
        let fraction_str = &input[20..29.min(input.len())];
        let fraction = fraction_str.parse::<u32>().unwrap();
        let nanoseconds = fraction * (10u32.pow(9 - fraction_str.len() as u32));
        datetime.time = icu_calendar::Time::try_new(hour, minute, second, nanoseconds).unwrap();
    };

    datetime
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
/// let (date, zone) =
///     mock::parse_zoned_gregorian_from_str("2020-10-14T13:21:00+05:30")
///         .expect("Failed to parse a zoned datetime.");
/// ```
pub fn parse_zoned_gregorian_from_str(input: &str) -> (DateTime<Gregorian>, CustomTimeZone) {
    let idx = input.rfind(&['+', '-', '\u{2212}', 'Z']).unwrap();
    #[allow(clippy::indexing_slicing)] // valid index
    (
        parse_gregorian_from_str(&input[..idx]),
        input[idx..].parse().unwrap(),
    )
}
