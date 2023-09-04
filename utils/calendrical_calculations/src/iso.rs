// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

use crate::helpers::{div_rem_euclid64, i64_to_i32, quotient64, I32CastError};
use crate::rata_die::RataDie;

// The Gregorian epoch is equivalent to first day in fixed day measurement
const EPOCH: RataDie = RataDie::new(1);

pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 400 == 0 || year % 100 != 0)
}

// Fixed is day count representation of calendars starting from Jan 1st of year 1.
// The fixed calculations algorithms are from the Calendrical Calculations book.
//
// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1167-L1189
pub fn fixed_from_iso(year: i32, month: u8, day: u8) -> RataDie {
    let prev_year = (year as i64) - 1;
    // Calculate days per year
    let mut fixed: i64 = (EPOCH.to_i64_date() - 1) + 365 * prev_year;
    // Calculate leap year offset
    let offset = quotient64(prev_year, 4) - quotient64(prev_year, 100) + quotient64(prev_year, 400);
    // Adjust for leap year logic
    fixed += offset;
    // Days of current year
    fixed += quotient64(367 * (month as i64) - 362, 12);
    // Leap year adjustment for the current year
    fixed += if month <= 2 {
        0
    } else if is_leap_year(year) {
        -1
    } else {
        -2
    };
    // Days passed in current month
    fixed += day as i64;
    RataDie::new(fixed)
}

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1191-L1217
fn iso_year_from_fixed(date: RataDie) -> i64 {
    // Shouldn't overflow because it's not possbile to construct extreme values of RataDie
    let date = date - EPOCH;

    // 400 year cycles have 146097 days
    let (n_400, date) = div_rem_euclid64(date, 146097);

    // 100 year cycles have 36524 days
    let (n_100, date) = div_rem_euclid64(date, 36524);

    // 4 year cycles have 1461 days
    let (n_4, date) = div_rem_euclid64(date, 1461);

    let n_1 = quotient64(date, 365);

    let year = 400 * n_400 + 100 * n_100 + 4 * n_4 + n_1;

    if n_100 == 4 || n_1 == 4 {
        year
    } else {
        year + 1
    }
}

fn iso_new_year(year: i32) -> RataDie {
    fixed_from_iso(year, 1, 1)
}

pub fn iso_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    let year = iso_year_from_fixed(date);
    let year = i64_to_i32(year)?;
    // Calculates the prior days of the adjusted year, then applies a correction based on leap year conditions for the correct ISO date conversion.
    let prior_days = date - iso_new_year(year);
    let correction = if date < fixed_from_iso(year, 3, 1) {
        0
    } else if is_leap_year(year) {
        1
    } else {
        2
    };
    let month = quotient64(12 * (prior_days + correction) + 373, 367) as u8; // in 1..12 < u8::MAX
    let day = (date - fixed_from_iso(year, month, 1) + 1) as u8; // <= days_in_month < u8::MAX
    Ok((year, month, day))
}
