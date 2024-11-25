use super::helpful_consts::*;
use super::iso_old_algos as alg_old;
use super::iso_old_file as iso_old;
use crate::iso as iso_new;
use crate::rata_die::RataDie;
use core::ops::RangeInclusive;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] helpful fns

fn calc_last_month_day(year: i32, month: u8) -> u8 {
    let to = MONTH_DAYS[(month as usize) - 1];
    if iso_old::is_leap_year(year) && (month == 2) {
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
            let rata_die = iso_old::fixed_from_iso(year, month, day);
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
    // 1st Jan of 1 year:
    let rata_die_initial = RataDie::new(1);

    // Now we know that `iso_new::iso_from_fixed` is correct for YMD(1, 1, 1):
    assert_eq!(iso_new::iso_from_fixed(rata_die_initial), Ok((1, 1, 1)));
    // Now we know that `iso_new::day_of_week` is correct for YMD(2024, 11, 25):
    assert_eq!(iso_new::day_of_week(2024, 11, 25), 1);

    let delta = 9_999; // 3_999_999;

    // This can be incorrect, but if it so, then on YMD(1, 1, 1) we will figured that out,
    // because if it is incorrect we get incorrect value for `rata_die_i64` for the YMD(1, 1, 1)
    // and `assert_eq!(iso_new::iso_from_fixed(rata_die), Ok((year, month, day)))` will panic
    let mut rata_die_i64 = iso_new::fixed_from_iso(-delta, 1, 1).to_i64_date();

    // This can be incorrect, but if it so, then on YMD(2024, 11, 25) we will figured that out
    // (days of weeks goes cyclic on each other)
    let mut day_of_week = iso_new::day_of_week(2024, 11, 25);

    let mut year_day = 1;
    // Because rata die's stay each after each we can verify that in a range
    // all of `iso_new::iso_from_fixed` is correct, if that range contains the YMD(1, 1, 1):
    for year in -delta..=delta {
        assert_year(year, |year, month, day| {
            if (month == 1) && (day == 1) {
                year_day = 1;
            }

            let rata_die = RataDie::new(rata_die_i64);
            assert_eq!(iso_new::iso_from_fixed(rata_die), Ok((year, month, day)));
            assert_eq!(iso_new::iso_year_from_fixed(rata_die), year as i64);
            assert_eq!(iso_new::fixed_from_iso(year, month, day), rata_die);
            assert_eq!(iso_new::day_of_week(year, month, day), day_of_week);

            assert_eq!(iso_new::iso_from_year_day(year, year_day), (month, day));
            assert_eq!(iso_new::day_of_year(year, month, day), year_day);

            if (month == 12) && (day == 31) {
                assert_eq!(
                    year_day,
                    365 + iso_old::is_leap_year(year) as u16,
                    "YMD: {year} {month} {day}"
                );
            }

            rata_die_i64 += 1;

            day_of_week += 1;
            if day_of_week == 8 {
                day_of_week = 1;
            }

            year_day += 1;
        });
    }

    // FNS: {iso_from_fixed, iso_year_from_fixed, fixed_from_iso, day_of_week, iso_from_year_day, day_of_year};
    // So now we sure that all fns from FNS is correct (at least in range -delta..=delta)
}

// [-] test correctness
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// [+] test the same result as prev algo

#[test]
fn test_is_leap_year_the_same() {
    const N_YEAR: i32 = 99_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR; // i32::MIN..=i32::MAX

    fn the_same_in_year(year: i32) {
        assert_eq!(
            iso_old::is_leap_year(year),
            iso_new::is_leap_year(year),
            "year = {year}"
        );
    }

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);
}

#[test]
fn test_fixed_from_iso_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    let the_same_in_year = |year| {
        assert_year(year, |year, month, day| {
            assert_eq!(
                iso_old::fixed_from_iso(year, month, day),
                iso_new::fixed_from_iso(year, month, day),
                "YMD: {year} {month} {day}"
            );
        })
    };

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);
}

#[test]
fn test_year_from_fixed_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    let the_same_in_year = |year| {
        assert_year_rata_die(year, |year, month, day, rata_die| {
            assert_eq!(
                year as i64,
                iso_new::iso_year_from_fixed(rata_die),
                "{year} {month} {day} | {rata_die:?}"
            );
        })
    };

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);

    const RATA_MIN: i64 = i64::MIN / 256;
    const RATA_MAX: i64 = i64::MAX / 256;
    const N_DAYS: i64 = 9_999;

    for rata_die in RATA_MIN..(RATA_MIN + N_DAYS) {
        let rata_die = RataDie::new(rata_die);
        assert_eq!(
            iso_old::iso_year_from_fixed(rata_die),
            iso_new::iso_year_from_fixed(rata_die),
            "{rata_die:?}"
        );
    }
    for rata_die in (RATA_MAX - N_DAYS)..=RATA_MAX {
        let rata_die = RataDie::new(rata_die);
        assert_eq!(
            iso_old::iso_year_from_fixed(rata_die),
            iso_new::iso_year_from_fixed(rata_die),
            "{rata_die:?}"
        );
    }
}

#[test]
fn test_from_fixed_eq() {
    const N_YEAR: i32 = 1_999; // `iso_old::iso_from_fixed` is slow
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    let the_same_in_year = |year| {
        assert_year_rata_die(year, |year, month, day, rata_die| {
            assert_eq!(
                iso_old::iso_from_fixed(rata_die),
                iso_new::iso_from_fixed(rata_die),
                "{year} {month} {day}"
            );
        })
    };

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);

    const RATA_MIN: i64 = i64::MIN / 256;
    const RATA_MAX: i64 = i64::MAX / 256;
    const N_DAYS: i64 = 99_999;

    for rata_die in RATA_MIN..(RATA_MIN + N_DAYS) {
        let rata_die = RataDie::new(rata_die);
        assert_eq!(
            iso_old::iso_from_fixed(rata_die),
            iso_new::iso_from_fixed(rata_die),
            "{rata_die:?}"
        );
    }
    for rata_die in (RATA_MAX - N_DAYS)..=RATA_MAX {
        let rata_die = RataDie::new(rata_die);
        assert_eq!(
            iso_old::iso_from_fixed(rata_die),
            iso_new::iso_from_fixed(rata_die),
            "{rata_die:?}"
        );
    }
}

// New algos in the `iso` and prev. algos from others files:

#[test]
fn test_day_of_week_the_same() {
    const N_YEAR: i32 = 2_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    fn the_same_in_year(year: i32) {
        for month in 1..=12u8 {
            for day in 1..=calc_last_month_day(year, month) {
                assert_eq!(
                    alg_old::day_of_week(year, month, day),
                    iso_new::day_of_week(year, month, day),
                    "YMD: {year} {month} {day}"
                );
            }
        }
    }

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);
}

#[test]
fn test_from_year_day_the_same() {
    const N_YEAR: i32 = 2_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    fn the_same_in_year(year: i32) {
        let days = 365 + iso_new::is_leap_year(year) as u16;
        for day_of_year in 1..=days {
            assert_eq!(
                alg_old::iso_from_year_day(year, day_of_year),
                iso_new::iso_from_year_day(year, day_of_year),
                "Y & DofY: {year} {day_of_year}"
            );
        }
    }

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);
}

#[test]
fn test_day_of_year_the_same() {
    const N_YEAR: i32 = 2_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    let the_same_in_year = |year| {
        assert_year(year, |year, month, day| {
            assert_eq!(
                alg_old::day_of_year(year, month, day),
                iso_new::day_of_year(year, month, day),
                "YMD: {year} {month} {day}"
            );
        })
    };

    N_RANGE.for_each(the_same_in_year);
    MIN_YEAR_BOUND_RANGE.for_each(the_same_in_year);
    MAX_YEAR_BOUND_RANGE.for_each(the_same_in_year);
}

// [-] test the same result as prev algo
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
