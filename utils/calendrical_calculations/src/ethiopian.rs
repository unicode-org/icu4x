// This file is part of ICU4X.
//
// This file is licensed under the Apache License, Version 2.0,
// which can be found in the LICENSE file in the
// calendrical_calculations package root or online at
// <https://www.apache.org/licenses/LICENSE-2.0>.

use crate::helpers::I32CastError;
use crate::rata_die::RataDie;

const ETHIOPIC_TO_COPTIC_OFFSET: i64 =
    super::coptic::COPTIC_EPOCH.const_diff(crate::julian::fixed_from_julian(8, 8, 29));

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2017
pub fn fixed_from_ethiopian(year: i32, month: u8, day: u8) -> RataDie {
    crate::coptic::fixed_from_coptic(year, month, day) - ETHIOPIC_TO_COPTIC_OFFSET
}

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L2028
pub fn ethiopian_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    crate::coptic::coptic_from_fixed(date + ETHIOPIC_TO_COPTIC_OFFSET)
}
