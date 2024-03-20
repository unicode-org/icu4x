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
        records::{DateDurationRecord, DurationFraction, DurationParseRecord, TimeDurationRecord},
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
        DateDurationRecord::default()
    } else {
        parse_date_duration(cursor)?
    };

    let time = if cursor.check_or(false, is_time_designator) {
        cursor.advance();
        parse_time_duration(cursor)?
    } else {
        TimeDurationRecord::default()
    };

    cursor.close()?;

    Ok(DurationParseRecord {
        sign: sign.into(),
        years: date.years,
        months: date.months,
        weeks: date.weeks,
        days: date.days,
        hours: time.hours,
        minutes: time.minutes,
        seconds: time.seconds,
        fraction: time.fraction,
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
        while cursor.check_or(false, |c| c.is_ascii_digit()) {
            let digit = cursor
                .next_digit()?
                .ok_or_else(|| ParserError::abrupt_end("DateDuration"))?;
            value = value * 10 + u32::from(digit);
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

pub(crate) fn parse_time_duration(cursor: &mut Cursor) -> ParserResult<TimeDurationRecord> {
    let mut time = TimeDurationRecord::default();

    assert_syntax!(
        cursor.check_or(false, |ch| ch.is_ascii_digit()),
        TimeDurationDesignator,
    );

    let mut previous_unit = TimeUnit::None;
    while cursor.check_or(false, |ch| ch.is_ascii_digit()) {
        let mut value: u32 = 0;
        while cursor.check_or(false, |c| c.is_ascii_digit()) {
            let digit = cursor
                .next_digit()?
                .ok_or_else(|| ParserError::abrupt_end("TimeDurationDigit"))?;
            value = value * 10 + u32::from(digit);
        }

        let fraction = parse_fraction(cursor)?;

        match cursor.next() {
            Some(ch) if is_hour_designator(ch) => {
                if previous_unit > TimeUnit::Hour {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.hours = value;
                if let Some(fraction) = fraction {
                    time.fraction = Some(DurationFraction::Hours(3600 * u64::from(fraction)));
                }
                previous_unit = TimeUnit::Hour;
            }
            Some(ch) if is_minute_designator(ch) => {
                if previous_unit > TimeUnit::Minute {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.minutes = value;
                if let Some(fraction) = fraction {
                    time.fraction = Some(DurationFraction::Minutes(60 * u64::from(fraction)));
                }
                previous_unit = TimeUnit::Minute;
            }
            Some(ch) if is_second_designator(ch) => {
                if previous_unit > TimeUnit::Second {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.seconds = value;
                if let Some(fraction) = fraction {
                    // Assert: Fraction value does not exceed i32 max.
                    time.fraction = Some(DurationFraction::Seconds(fraction));
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

    Ok(time)
}
