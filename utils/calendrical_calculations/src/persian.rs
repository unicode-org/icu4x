// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

use crate::helpers::{ceil_div, div_rem_euclid64, i64_to_i32, I32CastError};
use crate::rata_die::RataDie;
// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4720
// Book states that the Persian epoch is the date: 3/19/622 and since the Persian Calendar has no year 0, the best choice was to use the Julian function.
pub const FIXED_PERSIAN_EPOCH: RataDie = crate::julian::fixed_from_julian(622, 3, 19);

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4803
pub fn fixed_from_arithmetic_persian(year: i32, month: u8, day: u8) -> RataDie {
    let p_year = i64::from(year);
    let month = i64::from(month);
    let day = i64::from(day);
    let y = if p_year > 0 {
        p_year - 474
    } else {
        p_year - 473
    };
    let x = div_rem_euclid64(y, 2820);
    let year = x.1 + 474;
    let z = div_rem_euclid64(31 * year - 5, 128);

    RataDie::new(
        FIXED_PERSIAN_EPOCH.to_i64_date() - 1
            + 1029983 * x.0
            + 365 * (year - 1)
            + z.0
            + if month <= 7 {
                31 * (month - 1)
            } else {
                30 * (month - 1) + 6
            }
            + day,
    )
}

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4857
pub fn arithmetic_persian_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    let year = arithmetic_persian_year_from_fixed(date);
    let year = i64_to_i32(year)?;
    #[allow(clippy::unwrap_used)] // valid month,day
    let day_of_year = 1_i64 + (date - fixed_from_arithmetic_persian(year, 1, 1));
    let month = if day_of_year <= 186 {
        ceil_div(day_of_year, 31) as u8
    } else {
        ceil_div(day_of_year - 6, 30) as u8
    };
    let day = (date - fixed_from_arithmetic_persian(year, month, 1) + 1) as u8;
    Ok((year, month, day))
}

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4829
fn arithmetic_persian_year_from_fixed(date: RataDie) -> i64 {
    let d0 = date - fixed_from_arithmetic_persian(475, 1, 1);
    let d = div_rem_euclid64(d0, 1029983);
    let n2820 = d.0;
    let d1 = d.1;
    let y2820 = if d1 == 1029982 {
        2820
    } else {
        div_rem_euclid64(128 * d1 + 46878, 46751).0
    };
    let year = 474 + n2820 * 2820 + y2820;
    if year > 0 {
        year
    } else {
        year - 1
    }
}
