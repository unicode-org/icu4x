// This file is part of ICU4X.
//
// The contents of this file implement algorithms from the article:
// "Euclidean Affine Functions and Applications to Calendar Algorithms"
// by Cassio Neri & Lorenz Schneider (Feb. 2021), DOI: 10.48550/arXiv.2102.06959
// which have been released as C/C++ code at
// <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp>
// under the MIT/GNU license. Accordingly, this file is released under
// the Apache License, Version 2.0 which can be found at the calendrical_calculations
// package root or at http://www.apache.org/licenses/LICENSE-2.0.

use crate::helpers::I32CastError;
use crate::rata_die::RataDie;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] const's

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

// [-] const's
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] `fixed_from_iso` & `day_of_week` & `day_of_year`

/// Returns years passed from Jan 1st of year 1.
///
/// Fixed is day count representation of calendars starting from Jan 1st of year 1.
/// The calculation algorithm is from Cassio Neri & Lorenz Schneider article.
///
/// C/C++ code reference: <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L71-L101>
pub const fn fixed_from_iso(year: i32, month: u8, day: u8) -> RataDie {
    // There is no branching.

    // [[SECTION]]  Map to the pseudo calendar:
    let day = day - 1;

    // [[SECTION]]  Calc shifted `RataDie`:
    let shifted_rata_die = calc_shifted_rata_die_wo_days(year, month) as i64 + (day as i64);

    // [[SECTION]]  Shift `RataDie`:
    RataDie::new(shifted_rata_die - SHIFT)
}

/// C/C++ code reference (except for line 89):
/// <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L71-L95C44>
#[inline(always)]
const fn calc_shifted_rata_die_wo_days(year: i32, month: u8) -> u64 {
    // There is no branching.

    // ↴
    // [[SECTION]]  Map to the pseudo calendar:
    let need_to_shift_months = month <= 2;
    // let day = day - 1; // ⚠️ deal with `day` separately

    let days_before_month = calc_day_before_month_in_pseudo_year(month, need_to_shift_months);

    // Always more than 0:
    // `SHIFT_TO_YEARS` > any `i32` (see comment to `AMOUNT_OF_PERIODS`).
    let year = (((year as i64) + SHIFT_TO_YEARS) - (need_to_shift_months as i64)) as u64;

    // always more than 0. (see comment to `AMOUNT_OF_PERIODS` & `year`).
    let century = year / 100;

    // ↴
    // [[SECTION]]  Calc shifted `RataDie`:
    // Leap years every 4 year: `365 * year + year / 4` = `1461 * year / 4`
    // Except for each century: `- century`
    // But not for each 4th: `+ (century / 4)`
    let days_before_year = ((FOUR_YEARS_TO_D * year) >> 2) - century + (century >> 2);

    // shifted_rata_die (without days)
    days_before_year + (days_before_month as u64)
}

/// C/C++ code reference:
/// <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L88>
/// <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L94>
#[inline(always)]
pub(super) const fn calc_day_before_month_in_pseudo_year(month: u8, shift_months: bool) -> u32 {
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
    let mask = -(shift_months as i8);
    let month = month | ((mask as u8) & 12);

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
    (979 * (month as u32) - 2919) >> 5
}

/// Returns IsoWeekday(as `u8`) for the Gregorian date.
/// IsoWeekday: Monday=1 .. Sunday=7
pub const fn day_of_week(year: i32, month: u8, day: u8) -> u8 {
    // Because in the Gregorian calendar everythings repeated each 400 years
    // there also repeated day of weeks. So we should only find the shift
    // within weeks and add it to `shifted_rata_die` before `(x) % 7 + 1`
    // transformation. In our case it's the shift is `1`.
    let shifted_rata_die = calc_shifted_rata_die_wo_days(year, month) + (day as u64);
    ((shifted_rata_die + 1) % 7) as u8 + 1
}

/// # Returns
/// day of the year in the Grigorian calendar:
/// + `1..=365` for a non leap year
/// + `1..=366` for a leap year
pub const fn day_of_year(year: i32, month: u8, day: u8) -> u16 {
    const DAYS_BEFORE_FEB: u8 = 31;
    const DAYS_BEFORE_MAR: u16 = DAYS_BEFORE_FEB as u16 + 28;

    #[allow(clippy::comparison_chain)]
    if month > 2 {
        let days_before_month = calc_day_before_month_in_pseudo_year(month, false);
        // shift back from pseudo calendar
        let leap = is_leap_year(year) as u16 + DAYS_BEFORE_MAR;
        days_before_month as u16 + day as u16 + leap
    } else if month == 2 {
        (day + DAYS_BEFORE_FEB) as u16
    } else {
        day as u16
    }
}

// [-] `fixed_from_iso` & `day_of_week` & `day_of_year`
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] `iso_from_fixed` & `iso_from_year_day`

/// Returns (years, months, days) passed from Jan 1st of year 1.
///
/// Fixed is day count representation of calendars starting from Jan 1st of year 1.
/// The calculation algorithm is from Cassio Neri & Lorenz Schneider article.
///
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
    // [[Section]]  Map from the pseudo calendar to the Gregorian:
    let need_to_shift_months = day_of_year >= 306;
    let year = (year - SHIFT_TO_YEARS) + (need_to_shift_months as i64);
    let (month, day) = calc_md_for_pseudo_year(day_of_year, need_to_shift_months);

    Ok((year as i32, month, day))
}

/// # Input
/// `day_of_year` for the pseudo year:
/// ```plain
/// month: |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 |
/// days:  | 31 | 30 | 31 | 30 | 31 | 31 | 30 | 31 | 30 | 31 | 31 | 28 |
/// `day_of_year`:
///        |  0 | 31 | 61 | 92 | 122| 153| 184| 214| 245| 275| 306| 337|
/// ```
///
/// # Return:
/// `(month, day)` in the pseudo year:
/// * `month` in `3..=14`
/// * `day` in `1..=31`
///
/// C/C++ code reference:
/// * <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L58-L60>
/// * <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L66>
#[inline(always)]
pub(crate) const fn calc_pseudo_md_for_pseudo_year(day_of_year: u32) -> (u8, u8) {
    // See [formulas 1]:
    // [X = months] [N = days]  :  a = 5, b = 461, d = 153;
    //
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
    (month, day + 1)
}

/// # Input
/// `day_of_year` for the pseudo year:
/// ```plain
/// month: |  3 |  4 |  5 |  6 |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 |
/// days:  | 31 | 30 | 31 | 30 | 31 | 31 | 30 | 31 | 30 | 31 | 31 | 28 |
/// `day_of_year`:
///        |  0 | 31 | 61 | 92 | 122| 153| 184| 214| 245| 275| 306| 337|
/// ```
///
/// # Return:
/// `(month, day)` in the real year:
/// * `month` in `1..=12`
/// * `day` in `1..=31`
///
/// C/C++ code reference:
/// * <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L58-L60>
/// * <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L63>
/// * <https://github.com/cassioneri/eaf/blob/main/algorithms/neri_schneider.hpp#L65-L66>
#[inline(always)]
pub(crate) const fn calc_md_for_pseudo_year(day_of_year: u32, shift_months: bool) -> (u8, u8) {
    let (month, day) = calc_pseudo_md_for_pseudo_year(day_of_year);

    // `let month = if shift_months { month & 0b11 } else { month };`
    // mask = if shift_months { 0x03 } else { 0xFF }:
    let mask = (!(-(shift_months as i8) as u8)) | 0b11;
    let month = month & mask;

    // let day = day + 1; // ⚠️ Done in `calc_md_for_pseudo_year`

    (month, day)
}
/// # Input
/// * `year` in the Gregorian calendar
/// * `day_of_year` for the Gregorian calendar\
///   Valid values is:
///   + `1..=365` for a non leap year
///   + `1..=366` for a leap year
///
/// # Return:
/// `(month, day)` in
/// * `month` in `3..=14`
/// * `day` in `1..=31`
pub const fn iso_from_year_day(year: i32, day_of_year: u16) -> (u8, u8) {
    const DAYS_BEFORE_FEB: u8 = 31;
    const DAYS_BEFORE_MAR: u32 = DAYS_BEFORE_FEB as u32 + 28;
    let day_of_year = day_of_year as u32;
    let shift = is_leap_year(year) as u32 + DAYS_BEFORE_MAR;

    if day_of_year > shift {
        let day_of_year = day_of_year - shift - 1;
        calc_pseudo_md_for_pseudo_year(day_of_year)
    } else {
        // FEB have days in `32..=60` whish is `0b10_0000..=0b11_1100`
        // So `second_month` is:
        // * `0` for JAN
        // * `1` for FEB
        let second_month = ((day_of_year as u8) & (1 << 5)) >> 5;
        let month = 1 + second_month;
        let day = (day_of_year as u8) - (DAYS_BEFORE_FEB * second_month);
        (month, day)
    }
}

// [-] `iso_from_fixed` & `iso_from_year_day`
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
