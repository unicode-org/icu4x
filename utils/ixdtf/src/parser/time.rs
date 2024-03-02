// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//! Parsing of ISO8601 Time Values

use crate::{
    assert_syntax,
    parser::{
        grammar::{is_decimal_separator, is_time_separator},
        records::TimeRecord,
        Cursor,
    },
    ParserError, ParserResult,
};

use alloc::string::{String, ToString};

/// Parse `TimeRecord`
pub(crate) fn parse_time_record(cursor: &mut Cursor) -> ParserResult<TimeRecord> {
    let hour = parse_hour(cursor)?;

    if !cursor.check_or(false, |ch| is_time_separator(ch) || ch.is_ascii_digit()) {
        return Ok(TimeRecord {
            hour,
            minute: 0,
            second: 0,
            millisecond: 0,
            microsecond: 0,
            nanosecond: 0,
        });
    }

    let separator_present = cursor.check_or(false, is_time_separator);
    cursor.advance_if(separator_present);

    let minute = parse_minute_second(cursor, false)?;

    if !cursor.check_or(false, |ch| is_time_separator(ch) || ch.is_ascii_digit()) {
        return Ok(TimeRecord {
            hour,
            minute,
            second: 0,
            millisecond: 0,
            microsecond: 0,
            nanosecond: 0,
        });
    }

    let second_separator = cursor.check_or(false, is_time_separator);
    assert_syntax!(separator_present == second_separator, TimeSeparator);
    cursor.advance_if(second_separator);

    let second = parse_minute_second(cursor, true)?;

    let (millisecond, microsecond, nanosecond) = parse_fraction_component(cursor)?;

    Ok(TimeRecord {
        hour,
        minute,
        second,
        millisecond,
        microsecond,
        nanosecond,
    })
}

/// Parse an hour value.
#[inline]
pub(crate) fn parse_hour(cursor: &mut Cursor) -> ParserResult<u8> {
    let first = cursor.next_digit()?.ok_or(ParserError::TimeHour)?;
    let hour_value = first * 10 + cursor.next_digit()?.ok_or(ParserError::TimeHour)?;
    if !(0..=23).contains(&hour_value) {
        return Err(ParserError::TimeHour);
    }
    Ok(hour_value)
}

/// Parses `MinuteSecond` value.
#[inline]
pub(crate) fn parse_minute_second(cursor: &mut Cursor, is_second: bool) -> ParserResult<u8> {
    let first = cursor.next_digit()?.ok_or(if is_second {
        ParserError::TimeSecond
    } else {
        ParserError::TimeMinute
    })?;
    let min_sec_value = first * 10
        + cursor.next_digit()?.ok_or(if is_second {
            ParserError::TimeSecond
        } else {
            ParserError::TimeMinute
        })?;
    let valid_range = if is_second { 0..=60 } else { 0..=59 };
    if !valid_range.contains(&min_sec_value) {
        let err = if is_second {
            ParserError::TimeSecond
        } else {
            ParserError::TimeMinute
        };
        return Err(err);
    }
    Ok(min_sec_value)
}

#[inline]
pub(crate) fn parse_fraction_component(cursor: &mut Cursor) -> ParserResult<(u16, u16, u16)> {
    let Some(fraction) = parse_fraction(cursor)? else {
        return Ok((0, 0, 0));
    };
    Ok(to_fraction_components(fraction))
}

/// Parse a `Fraction` value
///
/// This is primarily used in ISO8601 to add percision past
/// a second.
#[inline]
pub(crate) fn parse_fraction(cursor: &mut Cursor) -> ParserResult<Option<f64>> {
    // Assert that the first char provided is a decimal separator.
    if !cursor.check_or(false, is_decimal_separator) {
        return Ok(None);
    }
    cursor.abrupt_next()?;
    let mut fraction = String::from('.');

    while cursor.check_or(false, |ch| ch.is_ascii_digit()) {
        fraction.push(cursor.abrupt_next()?);
    }

    assert_syntax!(fraction.len() <= 10, FractionPart);

    let fraction_value = fraction
        .parse::<f64>()
        .map_err(|e| ParserError::External(e.to_string()))?;
    Ok(Some(fraction_value))
}

#[inline]
fn to_fraction_components(raw: f64) -> (u16, u16, u16) {
    let intermediate: u32 = (raw * 1_000_000_000.0) as u32;

    let (milliseconds, rem) = div_rem(intermediate, 1_000_000);
    let (microseconds, nanoseconds) = div_rem(rem, 1_000);

    // Safety: Intermediate should only be max 999_999_999;
    (milliseconds as u16, microseconds as u16, nanoseconds as u16)
}

#[inline]
fn div_rem(num: u32, divisor: u32) -> (u32, u32) {
    (num / divisor, num % divisor)
}
