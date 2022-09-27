// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Some useful parsing functions for tests.

use either::Either;
use icu_calendar::{CalendarError, DateTime, Gregorian};
use icu_timezone::{CustomTimeZone, TimeZoneError};

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
///     parse_gregorian_from_str("2020-10-14T13:21:00")
///         .expect("Failed to parse a datetime.");
/// ```
///
/// Optionally, fractional seconds can be specified: `YYYY-MM-DDThh:mm:ss.SSS`.
///
/// ```
/// use icu::datetime::mock::parse_gregorian_from_str;
/// use icu_calendar::{DateTime, Gregorian};
///
/// let date: DateTime<Gregorian> =
///     parse_gregorian_from_str("2020-10-14T13:21:00.101")
///         .expect("Failed to parse a datetime.");
/// assert_eq!(u32::from(date.time.nanosecond), 101_000_000);
/// ```
pub fn parse_gregorian_from_str(input: &str) -> Result<DateTime<Gregorian>, CalendarError> {
    #![allow(clippy::indexing_slicing)] // all indexing is gated
    let validate = |c, i| -> Result<(), CalendarError> {
        if input.as_bytes()[i] != c {
            Err(CalendarError::Parse)
        } else {
            Ok(())
        }
    };

    if input.len() < 19 || input.len() == 20 {
        return Err(CalendarError::Parse);
    }
    let year: i32 = input[0..4].parse()?;
    validate(b'-', 4)?;
    let month: u8 = input[5..7].parse()?;
    validate(b'-', 7)?;
    let day: u8 = input[8..10].parse()?;
    validate(b'T', 10)?;
    let hour: u8 = input[11..13].parse()?;
    validate(b':', 13)?;
    let minute: u8 = input[14..16].parse()?;
    validate(b':', 16)?;
    let second: u8 = input[17..19].parse()?;
    let mut datetime =
        DateTime::try_new_gregorian_datetime(year, month, day, hour, minute, second)?;
    if input.len() > 20 {
        validate(b'.', 19)?;
        let fraction_str = &input[20..29.min(input.len())];
        let fraction = fraction_str.parse::<u32>()?;
        let nanoseconds = fraction * (10u32.pow(9 - fraction_str.len() as u32));
        datetime.time = icu_calendar::types::Time::try_new(hour, minute, second, nanoseconds)?;
    };

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
/// let (date, zone) =
///     mock::parse_zoned_gregorian_from_str("2020-10-14T13:21:00+05:30")
///         .expect("Failed to parse a zoned datetime.");
/// ```
pub fn parse_zoned_gregorian_from_str(
    input: &str,
) -> Result<(DateTime<Gregorian>, CustomTimeZone), Either<CalendarError, TimeZoneError>> {
    match input.rfind(&['+', '-', '\u{2212}', 'Z']) {
        #[allow(clippy::indexing_slicing)] // valid index
        Some(index) => Ok((
            parse_gregorian_from_str(&input[..index]).map_err(Either::Left)?,
            input[index..].parse().map_err(Either::Right)?,
        )),
        None => Err(Either::Right(TimeZoneError::InvalidOffset)),
    }
}
