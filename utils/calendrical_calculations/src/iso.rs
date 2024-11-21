// This file is part of ICU4X.
//
// The contents of this file implement algorithms from
// "Euclidean Affine Functions and Applications to Calendar Algorithms"
// by Cassio Neri, Lorenz Schneider (Feb. 2021), DOI: 10.48550/arXiv.2102.06959
// which have been released as C/C++ code at
// <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp>
// under the MIT/GNU license. Accordingly, this file is released under
// the Apache License, Version 2.0 which can be found at the calendrical_calculations
// package root or at http://www.apache.org/licenses/LICENSE-2.0.

use crate::helpers::I32CastError;
use crate::rata_die::RataDie;

/// Days in 4 years (there is 1 leap day):
const FOUR_YEARS_TO_D: u64 = 365 * 4 + 1;

/// In Gregorian calendar each 400 years sequence of leap days repeated.\
/// In Gregorian calendar 400 years have 146097 days.
const ONE_PERIOD_TO_DAYS: i64 = 365 * 400 + 400 / 4 - 4 + 1; // 146097

/// In plain language, this is a reference point (in 4 centuries).
///
/// Amount of periods is calculated in way that `year + SHIFT_TO_YEARS > 0`,
/// otherwise we will have inaccuracy of 1 day around of leap days.
///
/// Minimal input for `fixed_from_iso` is `-2**31`(`i32::MIN`) and the condition
/// above start to be always true from `(1 << 31) / 400 + 1` (`5_368_710`)
///
/// But for `iso_year_from_fixed` output is `i64` and input is RataData
/// that allow to have input from `-i64::MIN / 256`(`2**55`) to `i64::MAX / 256`(`2**55`);
/// corresponding years will be in `-98643495811588..246608739529`;
/// `AMOUNT_OF_PERIODS` from which the condition always true more than `(1 << 50) / 400 + 1`
/// so this value is correct for `iso_year_from_fixed`;
///
/// We should choose max(correct_val(iso_year_from_fixed), correct_val(fixed_from_iso))
const AMOUNT_OF_PERIODS: i64 = (1 << 50) / 400 + 1;

/// In plain language, this is a reference point (in days).
///
/// Amount of days by which two calendars(Gregorian & pseudo calendar) differ.
const SHIFT: i64 = 305 + ONE_PERIOD_TO_DAYS * AMOUNT_OF_PERIODS;

/// In plain language, this is a reference point (in years).
///
/// How many years in `AMOUNT_OF_PERIODS` periods.
const SHIFT_TO_YEARS: i64 = 400 * AMOUNT_OF_PERIODS;

/// Whether or not `year` is a leap year
#[inline] // real call will be more complex operation than inner code
pub const fn is_leap_year(year: i32) -> bool {
    // ```
    // if year % 100 != 0 { year % 4 == 0 }
    // else { year % 400 == 0 }
    // ```
    // here we can change 400 to 16 because:
    // 1.  `x % 100 == 0`  =  `x % (25 *  4) == 0`  =   `(x % 25 == 0) && (x %  4 == 0)`
    // 2.  `x % 400 == 0`  =  `x % (25 * 16) == 0`  =   `(x % 25 == 0) && (x % 16 == 0)`
    // so if 1. is true then we already know that `(x % 25 == 0)` is true
    //
    // so it becomes:
    // ```
    // if year % 100 != 0 { year % 4 == 0 }
    // else { year % 16 == 0 }
    // ```
    // then we see `year % X == 0` and make transformation:
    // ```
    // divisor = year % 100 != 0 { 4 } else { 16 };
    // year % divisor == 0
    // ```
    // and for divisor that is equal to 2^N
    // `year % divisor == 0` is the same as `(year & (divisor - 1)) == 0`
    // so it becomes:
    // ```
    // divisor = year % 100 != 0 { 4 } else { 16 };
    // (year & (divisor - 1)) == 0
    // ```
    //
    // Also the compiler could potentially use CMOVcc (conditional move instr)
    // for `if ... { VAL_A } else { VAL_B }` stmt
    // in such case there will be no branching.
    // Otherwise branch predictor will mostly right.

    let divisor_sub_1 = if year % 100 != 0 { 0b11 } else { 0b1111 };
    (year & divisor_sub_1) == 0
}

// Returns years passed from Jan 1st of year 1.
//
// Fixed is day count representation of calendars starting from Jan 1st of year 1.
// The calculation algorithm is from Cassio Neri & Lorenz Schneider article.
//
/// C/C++ code reference: <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L71-L101>
pub const fn fixed_from_iso(year: i32, month: u8, day: u8) -> RataDie {
    // There is no branching.

    // ↴
    // [[SECTION]]  Map to the pseudo calendar:
    let need_to_shift_months = month <= 2;
    let day = day - 1;

    // Always more than 0:
    // `SHIFT_TO_YEARS` > any `i32` (see comment to `AMOUNT_OF_PERIODS`).
    let year = (((year as i64) + SHIFT_TO_YEARS) - (need_to_shift_months as i64)) as u64;

    // We move the leap month (February) to the end of our pseud year.
    // And we start our month from 3rd month. So:
    //
    // Gregorian:  .. | 11 | 12 \  1 |  2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 \  1 |  2 | 3 | 4 | ..
    // Pseudo:     .. | 11 | 12 | 13 | 14 \ 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 \ 3 | 4 | ..
    //
    // So mappping will be next:
    // `let month = if need_to_shift_months { month + 12 } else { month };`
    // Minimal value is 3.
    // `month + 12` is equal to `month | 12` when month is `1` or `2` (our case).
    // So we will have:
    // `let month = if need_to_shift_months { month | 12 } else { month };`
    // `mask` = `-(need_to_shift_months as i8)` = `if need_to_shift_months { 0xFF } else { 0x00 }`
    // `12 & (mask as u8)` = `if need_to_shift_months { 12 } else { 0 }`
    let mask = -(need_to_shift_months as i8);
    let month = month | ((mask as u8) & 12);

    // always more than 0. (see comment to `AMOUNT_OF_PERIODS` & `year`).
    let century = year / 100;

    // ↴
    // [[SECTION]]  Calc shifted `RataDie`:
    // Leap years every 4 year: `365 * year + year / 4` = `1461 * year / 4`
    // Except for each century: `- century`
    // But not for each 4th: `+ (century / 4)`
    let days_before_year = ((FOUR_YEARS_TO_D * year) >> 2) - century + (century >> 2);

    // Our months in `3..=14` with next days amount in it:
    // month: |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 |
    // days:  | 31 | 30 | 31 | 30 | 31 | 31 | 30 | 31 | 30 | 31 | 31 | 28 |
    // days before a month will be:
    //        |  0 | 31 | 61 | 92 | 122| 153| 184| 214| 245| 275| 306| 337|
    // it's the same as `(979 * (month as i64) - 2919) / 32`
    //
    // See `test_days_before_month`.
    //
    // See [formulas 1]:
    // [X = months] [N = days]  :  a = 5, b = 461, d = 153;
    // It's aprox for `(153 * (month as i64) -457) / 5` within month in `3..=14`.
    //
    // Instead of `2919` there can be any number in `2918..=2922`;
    // `2919` choosed because of `month * 9X9 - X9X9` (＾▽＾)
    let days_before_month = (979 * (month as i64) - 2919) >> 5;

    let shifted_rata_die = (days_before_year as i64) + days_before_month + (day as i64);

    // ↴
    // [[SECTION]]  Shift `RataDie`:
    RataDie::new(shifted_rata_die - SHIFT)
}

// Returns (years, months, days) passed from Jan 1st of year 1.
//
// Fixed is day count representation of calendars starting from Jan 1st of year 1.
// The calculation algorithm is from Cassio Neri & Lorenz Schneider article.
//
/// C/C++ code reference: <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L40-L69>
pub const fn iso_from_fixed(date: RataDie) -> Result<(i32, u8, u8), I32CastError> {
    if let Some(err) = check_rata_die_have_i32_year(date) {
        return Err(err);
    }

    // There is no branching after:

    let shifted_rata_die = (date.to_i64_date() + SHIFT) as u64;

    // Let say that X & N can be any period of time that
    // can be represented in the form `X = (a * N + b) / d`.
    //
    // For example:
    // * X can be centuries & N can be days;
    // * X can be years & N can be weeks;
    // * X can be months & N can be days;
    //
    //
    //
    //                       [←----x_from_n(N)-----→]
    //                         /                  \
    //                        /                    \
    //                       |               N      |
    //                       ↓               ↓      ↓
    // ----------------------[←--------------|-----→]--------------  ← the time
    //           ↑                   ↑
    //           |                   |
    //           |           [←-n_of_the_x-→]
    //           |
    // [←----n_before_x----→]
    //
    //
    // Then:                                                  [formulas 1]:
    //
    // n_before_x(X) = (d * X + a - b - 1) / a;
    // x_from_n(N)   = (a * N + b) / d     = X;
    // n_of_the_x(N) = (a * N + b) % d / a = Nx; where Nx within of the X
    //
    // `(a * N + b)` will call `prepared_n_to_x`

    // ↴
    // [[SECTION]]  Century:
    // [X = centries] [N = days]  :  a = 4, b = 3, d = ONE_PERIOD_TO_DAYS;
    let prepared_days_to_cent = shifted_rata_die * 4 + 3;
    let century = prepared_days_to_cent / (ONE_PERIOD_TO_DAYS as u64);
    // We have 4 centuries in each period
    // `let day_of_century = (prepared_days_to_cent % (ONE_PERIOD_TO_DAYS as u64)) / 4;`
    let day_of_4_century = prepared_days_to_cent % (ONE_PERIOD_TO_DAYS as u64);

    // ↴
    // [[Section]]  Year:
    // [X = year] [N = days]  :  a = 4, b = 3, d = FOUR_YEARS_TO_D;

    // prepared_days_to_year = day_of_century * 4 + 3 = day_of_4_century / 4 * 4 + 3 = day_of_4_century | 3;
    let prepared_days_to_year = day_of_4_century | 3;

    // [approx comment 1]:
    // ```
    // let day_of_year = (prepared_days_to_year % FOUR_YEARS_TO_D) / 4;
    // let year_of_century = prepared_days_to_year / FOUR_YEARS_TO_D;
    // ```
    // And
    // `prepared_days_to_year` in `0..((ONE_PERIOD_TO_DAYS - 1) | 3)`
    //
    // For such `prepared_days_to_year` initial values the same as nexts
    // ```
    // let approx_prepared = 2939745 * prepared_days_to_year;
    // let day_of_year = (approx_prepared % (1 << 32)) / 2939745 / 4;
    // let year_of_century = approx_prepared >> 32;
    // ```
    //
    // See `test_approx_1` below.
    const APPROX_NUM_C: u64 = 2939745;
    let approx_prepared = APPROX_NUM_C * prepared_days_to_year;
    let year_of_century = approx_prepared >> 32;
    // `(approx_prepared % (1 << 32)) / 2939745 / 4` :
    //      `approx_prepared % (1 << 32)` is equal to `approx_prepared as u32`
    let day_of_year = (approx_prepared as u32) / (APPROX_NUM_C as u32) / 4;
    let year = (100 * century + year_of_century) as i64;

    // ↴
    // [[Section]]  Month & Day:
    // [X = months] [N = days]  :  a = 5, b = 461, d = 153;
    // [approx comment 2]:
    // for any `day_of_year` in `0..734`:
    // prep_a = (5 * day_of_year + 461)
    // prep_b = (2141 * day_of_year + 197913)
    // `prep_a / 153     == prep_b >> 16`
    // `prep_a % 153 / 5 == prep_b % (1 << 16) / 2141`
    //
    // See `test_approx_2` below.
    const APPROX_NUM_A: u32 = 2141;
    const APPROX_NUM_B: u32 = 197913;
    const DIV_MASK_02: u32 = (1 << 16) - 1;
    let approx_day = APPROX_NUM_A * day_of_year + APPROX_NUM_B;
    // Month from days: see [formulas 1] & [approx comment 2].
    let month = (approx_day >> 16) as u8;
    // Day of the month: see [formulas 1] & [approx comment 2].
    let day = ((approx_day & DIV_MASK_02) as u16 / APPROX_NUM_A as u16) as u8;

    // ↴
    // [[Section]]  Map from the pseudo calendar to the Gregorian:
    let need_to_shift_months = day_of_year >= 306;
    let year = (year - SHIFT_TO_YEARS) + (need_to_shift_months as i64);
    let day = day + 1;
    // `let month = if need_to_shift_months { month & 0b11 } else { month };`
    // mask = if need_to_shift_months { 0x03 } else { 0xFF }:
    let mask = (!(-(need_to_shift_months as i8) as u8)) | 0b11;
    let month = month & mask;

    Ok((year as i32, month, day))
}

// Returns years passed from Jan 1st of year 1.
//
// Fixed is day count representation of calendars starting from Jan 1st of year 1.
// The calculation algorithm is from Cassio Neri & Lorenz Schneider article.
//
/// C/C++ code reference: <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L40-L64>
pub(crate) const fn iso_year_from_fixed(date: RataDie) -> i64 {
    // ⚠️ To understand this code please see `iso_from_fixed`.

    let shifted_rata_die = (date.to_i64_date() + SHIFT) as u64;

    // [[SECTION]]  Century:
    let prepared_days_to_cent = shifted_rata_die * 4 + 3;
    let century = prepared_days_to_cent / (ONE_PERIOD_TO_DAYS as u64);
    let day_of_4_century = prepared_days_to_cent % (ONE_PERIOD_TO_DAYS as u64);

    // [[Section]]  Year:
    let prepared_days_to_year = day_of_4_century | 3;
    const APPROX_NUM_C: u64 = 2939745;
    let approx_prepared = APPROX_NUM_C * prepared_days_to_year;
    let year_of_century = approx_prepared >> 32;
    let day_of_year = (approx_prepared as u32) / (APPROX_NUM_C as u32) / 4;
    let year = (100 * century + year_of_century) as i64;

    // [[Section]]  Map year from the pseudo calendar to the Gregorian:
    let need_to_shift_months = day_of_year >= 306;

    (year - SHIFT_TO_YEARS) + (need_to_shift_months as i64)
}

#[inline(always)]
const fn check_rata_die_have_i32_year(input: RataDie) -> Option<I32CastError> {
    const MIN: i64 = fixed_from_iso(i32::MIN, 1, 1).to_i64_date();
    const MAX: i64 = fixed_from_iso(i32::MAX, 12, 31).to_i64_date();

    let input = input.to_i64_date();
    if input < MIN {
        Some(I32CastError::BelowMin)
    } else if input > MAX {
        Some(I32CastError::AboveMax)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::FOUR_YEARS_TO_D;
    use super::ONE_PERIOD_TO_DAYS;

    #[test]
    fn test_days_before_month() {
        pub const DAYS: [i64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 28];
        const SHIFT: usize = 2;

        let mut sum = 0;
        for month in (1 + SHIFT)..=(12 + SHIFT) {
            let days_before_month = (979 * (month as i64) - 2919) >> 5;
            assert_eq!(days_before_month, sum);
            sum += DAYS[month - SHIFT - 1];
        }
    }

    #[test]
    fn test_approx_1() {
        for prepared_days_to_year in 0..=((ONE_PERIOD_TO_DAYS - 1) | 3) as u64 {
            let day_of_year_a = ((prepared_days_to_year % FOUR_YEARS_TO_D) / 4) as u32;
            let year_of_century_a = prepared_days_to_year / FOUR_YEARS_TO_D;

            let x = prepared_days_to_year * 2939745;
            let day_of_year_b = x as u32 / 2939745 / 4;
            let year_of_century_b = x >> 32;

            assert_eq!(day_of_year_a, day_of_year_b);
            assert_eq!(year_of_century_a, year_of_century_b);
        }
    }

    #[test]
    fn test_approx_2() {
        for day_of_year in 0..734 {
            let prep = 5 * day_of_year + 461;
            let month_must = (prep / 153) as u8;
            let day_must = (prep % 153 / 5) as u8;

            const APPROX_NUM_A: u32 = 2141;
            const APPROX_NUM_B: u32 = 197913;
            const DIV_MASK_02: u32 = (1 << 16) - 1;
            let approx_day = APPROX_NUM_A * day_of_year + APPROX_NUM_B;
            let month = (approx_day >> 16) as u8;
            let day = ((approx_day & DIV_MASK_02) as u16 / APPROX_NUM_A as u16) as u8;

            assert_eq!(month_must, month);
            assert_eq!(day_must, day);
        }
    }
}
