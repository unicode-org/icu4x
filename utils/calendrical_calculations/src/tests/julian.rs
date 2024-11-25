use super::helpful_consts::*;
use super::julian_old_file as old;
use crate::{helpers::I32CastError, julian as new, rata_die::RataDie};
use core::ops::RangeInclusive;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] helpful fns

fn calc_last_month_day(year: i32, month: u8) -> u8 {
    let to = MONTH_DAYS[(month as usize) - 1];
    if new::is_leap_year(year) && (month == 2) {
        to + 1
    } else {
        to
    }
}

pub(crate) fn assert_year(year: i32, mut assert_f: impl FnMut(i32, u8, u8)) {
    for month in 1..=12u8 {
        for day in 1..=calc_last_month_day(year, month) {
            assert_f(year, month, day)
        }
    }
}

fn assert_year_rata_die(year: i32, mut assert_f: impl FnMut(i32, u8, u8, RataDie)) {
    for month in 1..=12u8 {
        for day in 1..=calc_last_month_day(year, month) {
            let rata_die = old::fixed_from_julian(year, month, day);
            assert_f(year, month, day, rata_die)
        }
    }
}

// [-] helpful fns
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] test correctness

#[test]
fn test_algo_correctness() {
    // In old algo there was a comment:
    // ```
    // Julian epoch is equivalent to fixed_from_iso of December 30th of 0 year
    // 1st Jan of 1st year Julian is equivalent to December 30th of 0th year of ISO year
    // ```
    // ⚠️ IS IT TRUE? ⚠️

    // firstly we must be sure that `crate::iso::fixed_from_iso` is correct
    // (see `super::iso::test_algo_correctness`)
    //
    // 1st Jan of 1 year:
    let rata_die_initial = crate::iso::fixed_from_iso(0, 12, 30);

    // Now we know that `iso_new::julian_from_fixed` is correct for jYMD(1, 1, 1):
    assert_eq!(new::julian_from_fixed(rata_die_initial), Ok((1, 1, 1)));

    let delta = 9_999; // 3_999_999;

    // This can be incorrect, but if it so, then on jYMD(1, 1, 1) we will figured that out,
    // because if it is incorrect we get incorrect value for `rata_die_i64` for the jYMD(1, 1, 1)
    // and `assert_eq!(iso_new::julian_from_fixed(rata_die), Ok((year, month, day)))` will panic
    let mut rata_die_i64 = new::fixed_from_julian(-delta, 1, 1).to_i64_date();

    let mut year_day = 1;
    // Because rata die's stay each after each we can verify that in a range
    // all of `iso_new::julian_from_fixed` is correct, if that range contains the jYMD(1, 1, 1):
    for year in -delta..=delta {
        assert_year(year, |year, month, day| {
            if (month == 1) && (day == 1) {
                year_day = 1;
            }

            let rata_die = RataDie::new(rata_die_i64);
            assert_eq!(new::julian_from_fixed(rata_die), Ok((year, month, day)));
            assert_eq!(new::fixed_from_julian(year, month, day), rata_die);

            assert_eq!(new::day_of_year(year, month, day), year_day);

            if (month == 12) && (day == 31) {
                assert_eq!(
                    year_day,
                    365 + old::is_leap_year(year) as u16,
                    "YMD: {year} {month} {day}"
                );
            }

            rata_die_i64 += 1;
            year_day += 1;
        });
    }

    // FNS: {julian_from_fixed, fixed_from_julian, day_of_year};
    // So now we sure that all fns from FNS is correct (at least in range -delta..=delta)
}

// [-] test correctness
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] test the same result as prev algo

#[test]
fn day_of_year_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    let the_same_in_year = |year| {
        assert_year(year, |year, month, day| {
            assert_eq!(
                old::day_of_year(year, month, day),
                new::day_of_year(year, month, day),
                "YMD: {year} {month} {day}"
            );
        })
    };

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);
}

#[test]
fn fixed_from_julian_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    let the_same_in_year = |year| {
        assert_year(year, |year, month, day| {
            assert_eq!(
                old::fixed_from_julian(year, month, day),
                new::fixed_from_julian(year, month, day),
                "YMD: {year} {month} {day}"
            );
        })
    };

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);
}

#[test]
fn julian_from_fixed_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    let the_same_in_year = |year| {
        assert_year_rata_die(year, |year, month, day, date| {
            assert_eq!(
                old::julian_from_fixed(date),
                new::julian_from_fixed(date),
                "YMD: {year} {month} {day}"
            );
        })
    };

    N_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);

    // There was a mistake in the prev algo for the date (i32::MIN, 1, 1):
    ((i32::MIN + 1)..=(i32::MIN + N_YEAR_BOUND)).for_each(the_same_in_year);
    let rata_die = new::fixed_from_julian(i32::MIN, 1, 1);
    let date = rata_die.to_i64_date();
    for date in (date + 1)..=(date + 366) {
        let date = RataDie::new(date);
        assert_eq!(old::julian_from_fixed(date), new::julian_from_fixed(date),);
    }

    // The mistake:
    assert_eq!(
        old::julian_from_fixed(rata_die),
        Err(I32CastError::BelowMin)
    );
    assert_eq!(new::julian_from_fixed(rata_die), Ok((i32::MIN, 1, 1)));

    // Ok (should be BelowMin):
    assert_eq!(
        old::julian_from_fixed(RataDie::new(date - 1)),
        Err(I32CastError::BelowMin)
    );
    assert_eq!(
        new::julian_from_fixed(RataDie::new(date - 1)),
        Err(I32CastError::BelowMin)
    );
}

// [-] test the same result as prev algo
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
