// This file is part of ICU4X.
//
// The contents of this file implement algorithms from the article:
// "Euclidean affine functions and their application to calendar algorithms"
// by Cassio Neri & Lorenz Schneider (Dec. 2022), DOI: 10.1002/spe.3172

use crate::helpers::I32CastError;
use crate::rata_die::RataDie;

/// In Julian calendar each 4 years sequence of leap days repeated.\
const ONE_PERIOD_TO_DAYS: i64 = 365 * 4 + 1;

/// Lisp code reference: <https://github.com/EdReingold/calendar-code2/blob/1ee51ecfaae6f856b0d7de3e36e9042100b4f424/calendar.l#L1684-L1687>
#[inline(always)]
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0
}

/// # Returns
/// day of the year in the Gregorian calendar:
/// + `1..=365` for a non leap year
/// + `1..=366` for a leap year
pub const fn day_of_year(year: i32, month: u8, day: u8) -> u16 {
    const DAYS_BEFORE_FEB: u8 = 31;
    const DAYS_BEFORE_MAR: u16 = DAYS_BEFORE_FEB as u16 + 28;

    #[allow(clippy::comparison_chain)]
    if month > 2 {
        let days_before_month = super::iso::calc_day_before_month_in_pseudo_year(month, false);
        // shift back from pseudo calendar
        // ⚠️ `is_leap_year` differ from `iso` so we need to copy-paste ＼(＾∇＾)／ this fn fully
        //  | also can be used macro/fn that take fn but .. readability!
        let leap = is_leap_year(year) as u16 + DAYS_BEFORE_MAR;
        days_before_month as u16 + day as u16 + leap
    } else if month == 2 {
        (day + DAYS_BEFORE_FEB) as u16
    } else {
        day as u16
    }
}

/// Returns years passed from Jan 1st of year 1.
///
/// The Algo based on Cassio Neri & Lorenz Schneider algo for Gregorian calendar.
pub const fn fixed_from_julian(year: i32, month: u8, day: u8) -> RataDie {
    debug_assert!(month > 0 && month < 13, "Month should be in range 1..=12.");

    let day_of_year = day_of_year(year, month, day) as i64;

    let year = year as i64;
    let non_leap = ((year & 0b11) != 0) as i64;
    // Leap years every 4 year: `365 * year + year / 4`
    let rata_die_year = non_leap + ((ONE_PERIOD_TO_DAYS * year) >> 2);

    const SHIFT: i64 = -368;
    RataDie::new(rata_die_year + day_of_year + SHIFT)
}

/// Returns (years, months, days) passed from Jan 1st of year 1.
///
/// The Algo based on Cassio Neri & Lorenz Schneider algo for Gregorian calendar  
pub const fn julian_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    // ⚠️ To better understand the algo see `iso::iso_from_fixed`

    const ONE_PERIOD_TO_DAYS: i64 = 365 * 4 + 1;
    const AMOUNT_OF_PERIODS: i64 = (1 << 50) / 4 + 1;
    const YEAR_SHIFT: i64 = 1;
    const SHIFT: i64 =
        307 + ((YEAR_SHIFT + 3) / 4) + YEAR_SHIFT * 365 + ONE_PERIOD_TO_DAYS * AMOUNT_OF_PERIODS;
    const SHIFT_TO_YEARS: i64 = 4 * AMOUNT_OF_PERIODS + YEAR_SHIFT;

    if let Some(err) = check_rata_die_have_i32_year(date) {
        return Err(err);
    }

    let date = date.to_i64_date();

    let shifted_rata_die = (date + SHIFT) as u64;
    // ⚠️ in the initial algo there stay `(shifted_rata_die << 2) | 3`
    // But because of the SHIFT choose (`YEAR_SHIFT`)
    // We can remove `| 3`
    let prepared_days = shifted_rata_die << 2;
    let year = (prepared_days / (ONE_PERIOD_TO_DAYS as u64)) as i64;
    let day_of_period = prepared_days % (ONE_PERIOD_TO_DAYS as u64);

    // In our case we should not do `| 3`
    let prepared = day_of_period;
    const APPROX_NUM_C: u64 = 2939745;
    let approx_prepared = APPROX_NUM_C * prepared;
    let day_of_year = (approx_prepared as u32) / (APPROX_NUM_C as u32) / 4;

    let need_to_shift_months = day_of_year >= 306;
    let year = (year - SHIFT_TO_YEARS) + (need_to_shift_months as i64);
    let (month, day) = crate::iso::calc_md_for_pseudo_year(day_of_year, need_to_shift_months);

    Ok((year as i32, month, day))
}

/// Get a fixed date from the ymd of a Julian date.
///
/// Years are counted as in _Calendrical Calculations_ by Reingold & Dershowitz,
/// meaning there is no year 0. For instance, near the epoch date, years are counted: -3, -2, -1, 1, 2, 3 instead of -2, -1, 0, 1, 2, 3.
///
/// Primarily useful for use with code constructing epochs specified in the bookg
pub const fn fixed_from_julian_book_version(book_year: i32, month: u8, day: u8) -> RataDie {
    debug_assert!(book_year != 0);
    // TODO: Should we check the bounds here?
    fixed_from_julian(
        if book_year < 0 {
            book_year + 1
        } else {
            book_year
        },
        month,
        day,
    )
}

#[inline(always)]
const fn check_rata_die_have_i32_year(input: RataDie) -> Option<I32CastError> {
    const MIN: i64 = fixed_from_julian(i32::MIN, 1, 1).to_i64_date();
    const MAX: i64 = fixed_from_julian(i32::MAX, 12, 31).to_i64_date();

    let input = input.to_i64_date();
    if input < MIN {
        Some(I32CastError::BelowMin)
    } else if input > MAX {
        Some(I32CastError::AboveMax)
    } else {
        None
    }
}
