// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//! Parsing for Temporal's ISO8601 `Date` and `DateTime`.

use crate::{
    assert_syntax,
    parser::{
        annotations,
        grammar::{
            is_annotation_open, is_date_time_separator, is_hyphen, is_sign, is_utc_designator,
        },
        records::{DateRecord, TimeRecord, TimeZone},
        time::parse_time_record,
        time_zone, Cursor, IsoParseRecord,
    },
    ParserError, ParserResult,
};

use bitflags::bitflags;

#[derive(Debug, Default, Clone)]
/// A `DateTime` Parse Node that contains the date, time, and offset info.
pub(crate) struct DateTimeRecord {
    /// Date
    pub(crate) date: DateRecord,
    /// Time
    pub(crate) time: Option<TimeRecord>,
    /// Tz Offset
    pub(crate) time_zone: Option<TimeZone>,
}

bitflags! {
    /// Parsing flags for `AnnotatedDateTime` parsing.
    #[derive(Debug, Clone, Copy)]
    pub struct DateTimeFlags: u8 {
        const ZONED = 0b0000_0001;
        const TIME_REQ = 0b0000_0010;
        const UTC_REQ = 0b0000_0100;
    }
}

/// This function handles parsing for [`AnnotatedDateTime`][datetime],
/// [`AnnotatedDateTimeTimeRequred`][time], and
/// [`TemporalInstantString.`][instant] according to the requirements
/// provided via Spec.
///
/// [datetime]: https://tc39.es/proposal-temporal/#prod-AnnotatedDateTime
/// [time]: https://tc39.es/proposal-temporal/#prod-AnnotatedDateTimeTimeRequired
/// [instant]: https://tc39.es/proposal-temporal/#prod-TemporalInstantString
pub(crate) fn parse_annotated_date_time(
    flags: DateTimeFlags,
    cursor: &mut Cursor,
) -> ParserResult<IsoParseRecord> {
    let date_time = parse_date_time(
        flags.contains(DateTimeFlags::TIME_REQ),
        flags.contains(DateTimeFlags::UTC_REQ),
        cursor,
    )?;

    // Peek Annotation presence
    // Throw error if annotation does not exist and zoned is true, else return.
    if !cursor.check_or(false, is_annotation_open) {
        if flags.contains(DateTimeFlags::ZONED) {
            return Err(ParserError::MissingRequiredTzAnnotation);
        }

        cursor.close()?;

        return Ok(IsoParseRecord {
            date: date_time.date,
            time: date_time.time,
            tz: date_time.time_zone,
            calendar: None,
        });
    }

    let mut tz = TimeZone::default();

    if let Some(tz_info) = date_time.time_zone {
        tz = tz_info;
    }

    let annotation_set =
        annotations::parse_annotation_set(flags.contains(DateTimeFlags::ZONED), cursor)?;

    if let Some(annotated_tz) = annotation_set.tz {
        tz = annotated_tz.tz;
    }

    let tz = if tz.name.is_some() || tz.offset.is_some() {
        Some(tz)
    } else {
        None
    };

    cursor.close()?;

    Ok(IsoParseRecord {
        date: date_time.date,
        time: date_time.time,
        tz,
        calendar: annotation_set.calendar,
    })
}

/// Parses a `DateTime` record.
fn parse_date_time(
    time_required: bool,
    utc_required: bool,
    cursor: &mut Cursor,
) -> ParserResult<DateTimeRecord> {
    let date = parse_date(cursor)?;

    // If there is no `DateTimeSeparator`, return date early.
    if !cursor.check_or(false, is_date_time_separator) {
        if time_required {
            return Err(ParserError::MissingRequiredTime);
        }

        return Ok(DateTimeRecord {
            date,
            time: None,
            time_zone: None,
        });
    }

    cursor.advance();

    let time = parse_time_record(cursor)?;

    let time_zone = if cursor.check_or(false, |ch| is_sign(ch) || is_utc_designator(ch)) {
        Some(time_zone::parse_date_time_utc(cursor)?)
    } else {
        if utc_required {
            return Err(ParserError::MissingUtcOffset);
        }
        None
    };

    Ok(DateTimeRecord {
        date,
        time: Some(time),
        time_zone,
    })
}

/// Parses `Date` record.
fn parse_date(cursor: &mut Cursor) -> ParserResult<DateRecord> {
    let year = parse_date_year(cursor)?;
    let hyphenated = cursor
        .check(is_hyphen)
        .ok_or_else(ParserError::abrupt_end)?;

    cursor.advance_if(hyphenated);

    let month = parse_date_month(cursor)?;

    let second_hyphen = cursor.check_or(false, is_hyphen);
    assert_syntax!(hyphenated == second_hyphen, DateSeparator);
    cursor.advance_if(second_hyphen);

    let day = parse_date_day(cursor)?;

    check_date_validity(year, month, day)?;

    Ok(DateRecord { year, month, day })
}

// ==== `YearMonth` and `MonthDay` parsing functions ====

/// Parses a `DateSpecYearMonth`
pub(crate) fn parse_year_month(cursor: &mut Cursor) -> ParserResult<(i32, u8)> {
    let year = parse_date_year(cursor)?;

    cursor.advance_if(cursor.check_or(false, is_hyphen));

    let month = parse_date_month(cursor)?;

    assert_syntax!(cursor.check_or(true, is_annotation_open), InvalidEnd);

    Ok((year, month))
}

/// Parses a `DateSpecMonthDay`
pub(crate) fn parse_month_day(cursor: &mut Cursor) -> ParserResult<(u8, u8)> {
    let dash_one = cursor
        .check(is_hyphen)
        .ok_or_else(ParserError::abrupt_end)?;
    let dash_two = cursor
        .peek()
        .map(is_hyphen)
        .ok_or_else(ParserError::abrupt_end)?;

    if dash_two && dash_one {
        cursor.advance_n(2);
    } else if dash_two && !dash_one {
        return Err(ParserError::DateSeparator);
    }

    let month = parse_date_month(cursor)?;

    cursor.advance_if(cursor.check_or(false, is_hyphen));

    let day = parse_date_day(cursor)?;

    assert_syntax!(cursor.check_or(true, is_annotation_open), InvalidEnd);

    Ok((month, day))
}

// ==== Unit Parsers ====

#[inline]
fn parse_date_year(cursor: &mut Cursor) -> ParserResult<i32> {
    if cursor.check_or(false, is_sign) {
        let sign = if cursor.abrupt_next()? == '+' { 1 } else { -1 };

        let first = cursor.next_digit()?.ok_or(ParserError::DateExtendedYear)? as i32 * 100_000;
        let second = cursor.next_digit()?.ok_or(ParserError::DateExtendedYear)? as i32 * 10_000;
        let third = cursor.next_digit()?.ok_or(ParserError::DateExtendedYear)? as i32 * 1000;
        let fourth = cursor.next_digit()?.ok_or(ParserError::DateExtendedYear)? as i32 * 100;
        let fifth = cursor.next_digit()?.ok_or(ParserError::DateExtendedYear)? as i32 * 10;

        let year_value = first
            + second
            + third
            + fourth
            + fifth
            + cursor.next_digit()?.ok_or(ParserError::DateExtendedYear)? as i32;

        // 13.30.1 Static Semantics: Early Errors
        //
        // It is a Syntax Error if DateYear is "-000000" or "−000000" (U+2212 MINUS SIGN followed by 000000).
        if sign == -1 && year_value == 0 {
            return Err(ParserError::DateExtendedYear);
        }

        let year = sign * year_value;

        // NOTE: Below is a RangeError check -> validate.

        return Ok(year);
    }

    let first = cursor.next_digit()?.ok_or(ParserError::DateYear)? as i32 * 1000;
    let second = cursor.next_digit()?.ok_or(ParserError::DateYear)? as i32 * 100;
    let third = cursor.next_digit()?.ok_or(ParserError::DateYear)? as i32 * 10;
    let year_value =
        first + second + third + cursor.next_digit()?.ok_or(ParserError::DateYear)? as i32;

    Ok(year_value)
}

#[inline]
fn parse_date_month(cursor: &mut Cursor) -> ParserResult<u8> {
    let first = cursor.next_digit()?.ok_or(ParserError::DateMonth)?;
    let month_value = first * 10 + cursor.next_digit()?.ok_or(ParserError::DateMonth)?;
    if !(1..=12).contains(&month_value) {
        return Err(ParserError::InvalidMonthRange);
    }
    Ok(month_value)
}

#[inline]
fn parse_date_day(cursor: &mut Cursor) -> ParserResult<u8> {
    let first = cursor.next_digit()?.ok_or(ParserError::DateDay)?;
    let day_value = first * 10 + cursor.next_digit()?.ok_or(ParserError::DateDay)?;
    Ok(day_value)
}

#[inline]
fn check_date_validity(year: i32, month: u8, day: u8) -> ParserResult<()> {
    if !(-271_820..=275_760).contains(&year) {
        return Err(ParserError::InvalidYearRange);
    }
    let Some(days_in_month) = days_in_month(year, month) else {
        // NOTE: This should never through due to check in `parse_date_month`
        return Err(ParserError::InvalidMonthRange);
    };
    if !(1..=days_in_month).contains(&day) {
        return Err(ParserError::InvalidDayRange);
    }
    Ok(())
}

/// Utilty to return the days in month, returns None if month is invalid
#[inline]
fn days_in_month(year: i32, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some((28 + in_leap_year(year)) as u8),
        _ => None,
    }
}

/// Utility that returns whether a year is a leap year.
#[inline]
fn in_leap_year(year: i32) -> i8 {
    if year % 4 != 0 {
        0
    } else if year % 4 == 0 && year % 100 != 0 {
        1
    } else if year % 100 == 0 && year % 400 != 0 {
        0
    } else {
        assert_eq!(year % 400, 0);
        0
    }
}
