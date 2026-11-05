// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing of Time Values

use core::num::NonZeroU8;

use crate::{
    assert_syntax,
    encoding::EncodingType,
    parsers::{
        datetime::{parse_month_day, parse_year_month},
        grammar::{is_decimal_separator, is_time_designator, is_time_separator},
        Cursor,
    },
    records::{Annotation, Fraction, IxdtfParseRecord, TimeRecord},
    ParseError, ParserResult,
};

use super::{annotations, timezone};

/// Parse annotated time record is silently fallible returning None in the case that the
/// value does not align
pub(crate) fn parse_annotated_time_record<'a, T: EncodingType>(
    cursor: &mut Cursor<'a, T>,
    handler: impl FnMut(Annotation<'a, T>) -> Option<Annotation<'a, T>>,
) -> ParserResult<IxdtfParseRecord<'a, T>> {
    let mut start = *cursor;
    cursor.next_if(is_time_designator)?;

    let time = parse_time_record(cursor)?;

    // If Time was successfully parsed, assume from this point that this IS a
    // valid AnnotatedTimeRecord.

    let offset = timezone::parse_date_time_utc_offset(cursor)?;

    let annotations = annotations::parse_annotation_set(cursor, handler)?;

    cursor.close()?;
    check_time_ambiguity(&mut start)?;

    Ok(IxdtfParseRecord {
        date: None,
        time: Some(time),
        offset,
        tz: annotations.tz,
        calendar: annotations.calendar,
    })
}

/// Checks that an unprefixed time record is not ambiguous with a `MonthDay` or `YearMonth` date spec.
#[inline]
fn check_time_ambiguity<T: EncodingType>(cursor: &mut Cursor<T>) -> ParserResult<()> {
    // It is a Syntax Error if ParseText(Time DateTimeUTCOffset[~Z], DateSpecMonthDay) is a Parse Node.
    if cursor
        .speculative_parse(|c| Ok(parse_month_day(c).ok()))?
        .is_some()
    {
        return Err(ParseError::AmbiguousTimeMonthDay);
    }
    // It is a Syntax Error if ParseText(Time DateTimeUTCOffset[~Z], DateSpecYearMonth) is a Parse Node.
    if cursor
        .speculative_parse(|c| Ok(parse_year_month(c).ok()))?
        .is_some()
    {
        return Err(ParseError::AmbiguousTimeYearMonth);
    }
    Ok(())
}

/// Parse `TimeRecord`
pub(crate) fn parse_time_record<T: EncodingType>(
    cursor: &mut Cursor<T>,
) -> ParserResult<TimeRecord> {
    let hour = parse_hour(cursor)?;

    let separator_present = cursor.next_if(is_time_separator)?.is_some();

    let Some(minute) = parse_minute_second(cursor, false, separator_present)? else {
        return Ok(TimeRecord {
            hour,
            minute: 0,
            second: 0,
            fraction: None,
        });
    };

    let second_separator = cursor.next_if(is_time_separator)?.is_some();
    let second = match (separator_present, second_separator) {
        (true, true) | (false, false) => parse_minute_second(cursor, true, second_separator)?,
        (false, true) => return Err(ParseError::TimeSeparator),
        (true, false) => {
            assert_syntax!(
                cursor.next_if(|ch| ch.is_ascii_digit())?.is_none(),
                TimeSeparator
            );
            None
        }
    };
    let Some(second) = second else {
        return Ok(TimeRecord {
            hour,
            minute,
            second: 0,
            fraction: None,
        });
    };

    let fraction = parse_fraction(cursor)?;

    Ok(TimeRecord {
        hour,
        minute,
        second,
        fraction,
    })
}

/// Parse an hour value.
#[inline]
pub(crate) fn parse_hour<T: EncodingType>(cursor: &mut Cursor<T>) -> ParserResult<u8> {
    let first = cursor.next_digit()?.ok_or(ParseError::TimeHour)?;
    let hour_value = first * 10 + cursor.next_digit()?.ok_or(ParseError::TimeHour)?;
    if !(0..=23).contains(&hour_value) {
        return Err(ParseError::TimeHour);
    }
    Ok(hour_value)
}

/// Parses `MinuteSecond` value.
#[inline]
pub(crate) fn parse_minute_second<T: EncodingType>(
    cursor: &mut Cursor<T>,
    is_leap_second_valid: bool,
    separator_present: bool,
) -> ParserResult<Option<u8>> {
    let (valid_range, err) = if is_leap_second_valid {
        (0..=60, ParseError::TimeSecond)
    } else {
        (0..=59, ParseError::TimeMinuteSecond)
    };
    let first = if separator_present {
        cursor.next_digit()?.ok_or(err)?
    } else if let Some(ch) = cursor.next_if(|ch| ch.is_ascii_digit())? {
        ch - b'0'
    } else {
        return Ok(None);
    };
    let min_sec_value = first * 10 + cursor.next_digit()?.ok_or(err)?;
    if !valid_range.contains(&min_sec_value) {
        return Err(err);
    }
    Ok(Some(min_sec_value))
}

/// Parse a `Fraction` value
///
/// This is primarily used in ISO8601 to add percision past
/// a second.
#[inline]
pub(crate) fn parse_fraction<T: EncodingType>(
    cursor: &mut Cursor<T>,
) -> ParserResult<Option<Fraction>> {
    // Assert that the first char provided is a decimal separator.
    if cursor.next_if(is_decimal_separator)?.is_none() {
        return Ok(None);
    }

    let mut value = 0;
    let mut digits: u8 = 0;
    while let Some(ch) = cursor.next_if(|ch| ch.is_ascii_digit())? {
        let next_value = u64::from(ch - b'0');
        if digits < 18 {
            value = value * 10 + next_value;
        }
        digits = digits.saturating_add(1);
    }

    let digits = NonZeroU8::new(digits).ok_or(ParseError::FractionPart)?;

    Ok(Some(Fraction { digits, value }))
}
