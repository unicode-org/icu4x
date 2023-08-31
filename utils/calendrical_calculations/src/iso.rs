// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

use crate::helpers::quotient64;
use crate::rata_die::RataDie;

// The Gregorian epoch is equivalent to first day in fixed day measurement
pub const EPOCH: RataDie = RataDie::new(1);

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
