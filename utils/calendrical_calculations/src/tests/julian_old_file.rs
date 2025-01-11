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

// Julian epoch is equivalent to fixed_from_iso of December 30th of 0 year
// 1st Jan of 1st year Julian is equivalent to December 30th of 0th year of ISO year
const JULIAN_EPOCH: RataDie = RataDie::new(-1);

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1684-L1687>
#[inline(always)]
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0
}

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1689-L1709>
pub const fn fixed_from_julian(year: i32, month: u8, day: u8) -> RataDie {
    let mut fixed =
        JULIAN_EPOCH.to_i64_date() - 1 + 365 * (year as i64 - 1) + (year as i64 - 1).div_euclid(4);
    debug_assert!(month > 0 && month < 13, "Month should be in range 1..=12.");
    fixed += day_of_year(year, month, day) as i64;
    RataDie::new(fixed)
}

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1711-L1738>
pub fn julian_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    // incorrect for rata_data(i32::MIN, 1, 1)
    let approx = (4 * date.to_i64_date() + 1464).div_euclid(1461);
    let year = i64_to_i32(approx)?;

    let prior_days = date
        - fixed_from_julian(year, 1, 1)
        - if is_leap_year(year) && date > fixed_from_julian(year, 2, 28) {
            1
        } else {
            0
        };
    let adjusted_year = if prior_days >= 365 {
        year.saturating_add(1)
    } else {
        year
    };
    let adjusted_prior_days = prior_days.rem_euclid(365);
    debug_assert!((0..365).contains(&adjusted_prior_days));
    let month = if adjusted_prior_days < 31 {
        1
    } else if adjusted_prior_days < 59 {
        2
    } else if adjusted_prior_days < 90 {
        3
    } else if adjusted_prior_days < 120 {
        4
    } else if adjusted_prior_days < 151 {
        5
    } else if adjusted_prior_days < 181 {
        6
    } else if adjusted_prior_days < 212 {
        7
    } else if adjusted_prior_days < 243 {
        8
    } else if adjusted_prior_days < 273 {
        9
    } else if adjusted_prior_days < 304 {
        10
    } else if adjusted_prior_days < 334 {
        11
    } else {
        12
    };
    let day = (date - fixed_from_julian(adjusted_year, month, 1) + 1) as u8; // as days_in_month is < u8::MAX
    debug_assert!(day <= 31, "Day assertion failed; date: {date:?}, adjusted_year: {adjusted_year}, prior_days: {prior_days}, month: {month}, day: {day}");

    Ok((adjusted_year, month, day))
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const fn day_of_year(year: i32, month: u8, day: u8) -> u16 {
    let mut day_before_month = match month {
        1 => 0,
        2 => 31,
        3 => 59,
        4 => 90,
        5 => 120,
        6 => 151,
        7 => 181,
        8 => 212,
        9 => 243,
        10 => 273,
        11 => 304,
        12 => 334,
        _ => 0,
    };
    // Only add one if the month is after February (month > 2), since leap days are added to the end of February
    if month > 2 && is_leap_year(year) {
        day_before_month += 1;
    }
    day_before_month + day as u16
}
