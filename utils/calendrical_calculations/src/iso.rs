// This file is part of ICU4X.
//
// The contents of this file implement algorithms from Calendrical Calculations
// by Reingold & Dershowitz, Cambridge University Press, 4th edition (2018),
// which have been released as Lisp code at <https://github.com/EdReingold/calendar-code2/>
// under the Apache-2.0 license. Accordingly, this file is released under
// the Apache License, Version 2.0 which can be found at the calendrical_calculations
// package root or at http://www.apache.org/licenses/LICENSE-2.0.

use crate::helpers::{i64_to_i32, I32CastError};
use crate::rata_die::RataDie;

// The Gregorian epoch is equivalent to first day in fixed day measurement
const EPOCH: RataDie = RataDie::new(1);

/// Whether or not `year` is a leap year
///
/// Inspired by Neri-Schneider <https://arxiv.org/abs/2102.06959>
pub const fn is_leap_year(year: i32) -> bool {
    // This is branch-free, as it compiles to a conditional move
    if year % 100 != 0 {
        year % 4 == 0
    } else {
        year % 16 == 0
    }
}

// Fixed is day count representation of calendars starting from Jan 1st of year 1.
// The fixed calculations algorithms are from the Calendrical Calculations book.
//
/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1167-L1189>
pub const fn const_fixed_from_iso(year: i32, month: u8, day: u8) -> RataDie {
    day_before_year(year)
        .add(days_before_month(year, month) as i64)
        .add(day as i64)
}

/// The number of days in this year before this month starts
///
/// Inspired by Neri-Schneider <https://arxiv.org/abs/2102.06959>
pub const fn days_before_month(year: i32, month: u8) -> u16 {
    if month < 3 {
        // This compiles to a conditional move, so there's only one branch in this function
        if month == 1 {
            0
        } else {
            31
        }
    } else {
        31 + 28 + is_leap_year(year) as u16 + ((979 * (month as u32) - 2919) >> 5) as u16
    }
}

/// Non-const version of [`const_fixed_from_iso`]
pub fn fixed_from_iso(year: i32, month: u8, day: u8) -> RataDie {
    const_fixed_from_iso(year, month, day)
}

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1191-L1217>
pub const fn iso_year_from_fixed(date: RataDie) -> Result<i32, I32CastError> {
    // Shouldn't overflow because it's not possbile to construct extreme values of RataDie
    let date = date.since(EPOCH);

    // 400 year cycles have 146097 days
    let (n_400, date) = (date.div_euclid(146097), date.rem_euclid(146097));

    // 100 year cycles have 36524 days
    let (n_100, date) = (date / 36524, date % 36524);

    // 4 year cycles have 1461 days
    let (n_4, date) = (date / 1461, date % 1461);

    let n_1 = date / 365;

    let year = 400 * n_400 + 100 * n_100 + 4 * n_4 + n_1;

    if n_100 == 4 || n_1 == 4 {
        i64_to_i32(year)
    } else {
        i64_to_i32(year + 1)
    }
}

/// Calculates the day before Jan 1 of `year`.
pub const fn day_before_year(year: i32) -> RataDie {
    let prev_year = (year as i64) - 1;
    // Calculate days per year
    let mut fixed: i64 = 365 * prev_year;
    // Adjust for leap year logic. We can avoid the branch of div_euclid by making prev_year positive:
    // YEAR_SHIFT is larger (in magnitude) than any prev_year, and, being divisible by 400,
    // distributes correctly over the calculation on the next line.
    const YEAR_SHIFT: i64 = (-(i32::MIN as i64 - 1) / 400 + 1) * 400;
    fixed += (prev_year + YEAR_SHIFT) / 4 - (prev_year + YEAR_SHIFT) / 100
        + (prev_year + YEAR_SHIFT) / 400
        - const { YEAR_SHIFT / 4 - YEAR_SHIFT / 100 + YEAR_SHIFT / 400 };
    RataDie::new(fixed)
}

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1525-L1540>
pub fn iso_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    let year = iso_year_from_fixed(date)?;
    // Calculates the prior days of the adjusted year, then applies a correction based on leap year conditions for the correct ISO date conversion.
    let prior_days = date - day_before_year(year);
    let correction = if prior_days < 31 + 28 + is_leap_year(year) as i64 {
        -1
    } else {
        (!is_leap_year(year)) as i64
    };
    let month = ((12 * (prior_days + correction) + 373) / 367) as u8; // in 1..12 < u8::MAX
    let day = (date - fixed_from_iso(year, month, 1) + 1) as u8; // <= days_in_month < u8::MAX
    Ok((year, month, day))
}
