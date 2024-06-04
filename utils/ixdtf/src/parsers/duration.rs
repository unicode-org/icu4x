// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module implements logic for Duration parsing.

use crate::{
    assert_syntax,
    parsers::{
        grammar::{
            is_day_designator, is_duration_designator, is_hour_designator, is_minute_designator,
            is_month_designator, is_second_designator, is_sign, is_time_designator,
            is_week_designator, is_year_designator,
        },
        records::{DateDurationRecord, DurationParseRecord, TimeDurationRecord},
        time::parse_fraction,
        Cursor,
    },
    ParserError, ParserResult,
};

pub(crate) fn parse_duration(cursor: &mut Cursor) -> ParserResult<DurationParseRecord> {
    let sign = if cursor
        .check(is_sign)
        .ok_or_else(|| ParserError::abrupt_end("DurationStart"))?
    {
        cursor.next_or(ParserError::ImplAssert)? == '+'
    } else {
        true
    };

    assert_syntax!(
        is_duration_designator(cursor.next_or(ParserError::abrupt_end("DurationDesignator"))?),
        DurationDisgnator,
    );

    let date = if cursor.check_or(false, is_time_designator) {
        None
    } else {
        Some(parse_date_duration(cursor)?)
    };

    let time = parse_time_duration(cursor)?;

    cursor.close()?;

    Ok(DurationParseRecord {
        sign: sign.into(),
        date,
        time,
    })
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum DateUnit {
    None = 0,
    Year,
    Month,
    Week,
    Day,
}

pub(crate) fn parse_date_duration(cursor: &mut Cursor) -> ParserResult<DateDurationRecord> {
    let mut date = DateDurationRecord::default();
    let mut previous_unit = DateUnit::None;

    while cursor.check_or(false, |ch| ch.is_ascii_digit()) {
        let mut value: u32 = 0;
        while cursor.check_or(false, |ch| ch.is_ascii_digit()) {
            let digit = cursor
                .next_digit()?
                .ok_or_else(|| ParserError::abrupt_end("DateDuration"))?;
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(u32::from(digit)))
                .ok_or(ParserError::DurationValueExceededRange)?
        }

        match cursor.next() {
            Some(ch) if is_year_designator(ch) => {
                if previous_unit > DateUnit::Year {
                    return Err(ParserError::DateDurationPartOrder);
                }
                date.years = value;
                previous_unit = DateUnit::Year;
            }
            Some(ch) if is_month_designator(ch) => {
                if previous_unit > DateUnit::Month {
                    return Err(ParserError::DateDurationPartOrder);
                }
                date.months = value;
                previous_unit = DateUnit::Month;
            }
            Some(ch) if is_week_designator(ch) => {
                if previous_unit > DateUnit::Week {
                    return Err(ParserError::DateDurationPartOrder);
                }
                date.weeks = value;
                previous_unit = DateUnit::Week;
            }
            Some(ch) if is_day_designator(ch) => {
                if previous_unit > DateUnit::Day {
                    return Err(ParserError::DateDurationPartOrder);
                }
                date.days = value;
                previous_unit = DateUnit::Day;
            }
            Some(_) | None => return Err(ParserError::abrupt_end("DateDurationDesignator")),
        }
    }

    Ok(date)
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum TimeUnit {
    None = 0,
    Hour,
    Minute,
    Second,
}

pub(crate) fn parse_time_duration(cursor: &mut Cursor) -> ParserResult<Option<TimeDurationRecord>> {
    if !cursor.check_or(false, is_time_designator) {
        return Ok(None);
    };

    cursor.advance();
    assert_syntax!(
        cursor.check_or(false, |c| c.is_ascii_digit()),
        TimeDurationDesignator,
    );

    let mut time: (u32, u32, u32, Option<u32>) = (0, 0, 0, None);
    let mut previous_unit = TimeUnit::None;
    while cursor.check_or(false, |c| c.is_ascii_digit()) {
        let mut value: u32 = 0;
        while cursor.check_or(false, |c| c.is_ascii_digit()) {
            let digit = cursor
                .next_digit()?
                .ok_or_else(|| ParserError::abrupt_end("TimeDurationDigit"))?;
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(u32::from(digit)))
                .ok_or(ParserError::DurationValueExceededRange)?
        }

        let fraction = parse_fraction(cursor)?;

        match cursor.next() {
            Some(ch) if is_hour_designator(ch) => {
                if previous_unit > TimeUnit::Hour {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.0 = value;
                if let Some(fraction) = fraction {
                    time.3 = Some(fraction);
                };
                previous_unit = TimeUnit::Hour;
            }
            Some(ch) if is_minute_designator(ch) => {
                if previous_unit > TimeUnit::Minute {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.1 = value;
                if let Some(fraction) = fraction {
                    time.3 = Some(fraction);
                }
                previous_unit = TimeUnit::Minute;
            }
            Some(ch) if is_second_designator(ch) => {
                if previous_unit > TimeUnit::Second {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.2 = value;
                if let Some(fraction) = fraction {
                    time.3 = Some(fraction);
                }
                previous_unit = TimeUnit::Second;
            }
            Some(_) | None => return Err(ParserError::abrupt_end("TimeDurationDesignator")),
        }

        if fraction.is_some() {
            assert_syntax!(cursor.check_or(true, |ch| !ch.is_ascii_digit()), InvalidEnd,);
            break;
        }
    }

    match previous_unit {
        // Safety: Max fraction * 3600 is within u64 -> see test maximum_duration_fraction
        TimeUnit::Hour => Ok(Some(TimeDurationRecord::Hours {
            hours: time.0,
            fraction: time.3.map(|f| 3600 * u64::from(f)).unwrap_or(0),
        })),
        // Safety: Max fraction * 60 is within u64 -> see test maximum_duration_fraction
        TimeUnit::Minute => Ok(Some(TimeDurationRecord::Minutes {
            hours: time.0,
            minutes: time.1,
            fraction: time.3.map(|f| 60 * u64::from(f)).unwrap_or(0),
        })),
        TimeUnit::Second => Ok(Some(TimeDurationRecord::Seconds {
            hours: time.0,
            minutes: time.1,
            seconds: time.2,
            fraction: time.3.unwrap_or(0),
        })),
        TimeUnit::None => Err(ParserError::abrupt_end("TimeDUrationDesignator")),
    }
}
