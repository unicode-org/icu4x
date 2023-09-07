// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

use crate::helpers::{i64_to_i32, quotient, quotient64, I32CastError};
use crate::rata_die::RataDie;

pub(crate) const COPTIC_EPOCH: RataDie = crate::julian::fixed_from_julian(284, 8, 29);

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1978>
pub fn fixed_from_coptic(year: i32, month: u8, day: u8) -> RataDie {
    COPTIC_EPOCH - 1
        + 365 * (year as i64 - 1)
        + quotient(year, 4) as i64
        + 30 * (month as i64 - 1)
        + day as i64
}

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1990>
pub fn coptic_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    let year = quotient64(4 * (date - COPTIC_EPOCH) + 1463, 1461);
    let year = i64_to_i32(year)?;
    let month = (quotient64(date - fixed_from_coptic(year, 1, 1), 30) + 1) as u8; // <= 12 < u8::MAX
    let day = (date + 1 - fixed_from_coptic(year, month, 1)) as u8; // <= days_in_month < u8::MAX

    Ok((year, month, day))
}
