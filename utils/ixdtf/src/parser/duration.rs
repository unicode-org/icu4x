// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    assert_syntax,
    parser::{
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

use alloc::string::ToString;

pub(crate) fn parse_duration(cursor: &mut Cursor) -> ParserResult<DurationParseRecord> {
    let sign = if cursor.check(is_sign).ok_or_else(ParserError::abrupt_end)? {
        cursor.abrupt_next()? == '+'
    } else {
        true
    };

    assert_syntax!(
        is_duration_designator(cursor.abrupt_next()?),
        DurationDisgnator,
    );

    let date = if cursor.check_or(false, is_time_designator) {
        Some(DateDurationRecord::default())
    } else {
        Some(parse_date_duration(cursor)?)
    };

    let time = if cursor.check_or(false, is_time_designator) {
        cursor.advance();
        Some(parse_time_duration(cursor)?)
    } else {
        None
    };

    cursor.close()?;

    Ok(DurationParseRecord::from_records(
        sign,
        date.unwrap_or_default(),
        time.unwrap_or_default(),
    ))
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
        let digit_start = cursor.pos();

        while cursor.next().is_some() {
            if !cursor.check_or(false, |ch| ch.is_ascii_digit()) {
                break;
            }
        }

        let value = cursor
            .slice(digit_start, cursor.pos())
            .parse::<i32>()
            .map_err(|err| ParserError::External(err.to_string()))?;

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
            Some(_) | None => return Err(ParserError::abrupt_end()),
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
    let mut fraction_present = false;
    while cursor.check_or(false, |ch| ch.is_ascii_digit()) {
        let digit_start = cursor.pos();

        while cursor.next().is_some() {
            if !cursor.check_or(false, |ch| ch.is_ascii_digit()) {
                break;
            }
        }

        let value = cursor
            .slice(digit_start, cursor.pos())
            .parse::<i32>()
            .map_err(|err| ParserError::External(err.to_string()))?;

        let fraction = if let Some(fraction) = parse_fraction(cursor)? {
            fraction_present = true;
            fraction
        } else {
            0.0
        };

        match cursor.next() {
            Some(ch) if is_hour_designator(ch) => {
                if previous_unit > TimeUnit::Hour {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.hours = value;
                time.fhours = fraction;
                previous_unit = TimeUnit::Hour;
            }
            Some(ch) if is_minute_designator(ch) => {
                if previous_unit > TimeUnit::Minute {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.minutes = value;
                time.fminutes = fraction;
                previous_unit = TimeUnit::Minute;
            }
            Some(ch) if is_second_designator(ch) => {
                if previous_unit > TimeUnit::Second {
                    return Err(ParserError::TimeDurationPartOrder);
                }
                time.seconds = value;
                time.fseconds = fraction;
                previous_unit = TimeUnit::Second;
            }
            Some(_) | None => return Err(ParserError::AbruptEnd),
        }

        if fraction_present {
            assert_syntax!(cursor.check_or(true, |ch| !ch.is_ascii_digit()), InvalidEnd,);
            break;
        }
    }

    Ok(time)
}
