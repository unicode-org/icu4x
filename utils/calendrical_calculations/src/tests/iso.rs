use super::iso_old_algos as alg_old;
use super::iso_old_file as iso_old;
use crate::iso as iso_new;
use crate::rata_die::RataDie;
use core::ops::{Range, RangeInclusive};

const N_YEAR_BOUND: i32 = 1234; // more than one cycle (400 years)
const MIN_YEAR_BOUND_RANGE: Range<i32> = i32::MIN..(i32::MIN + N_YEAR_BOUND);
const MAX_YEAR_BOUND_RANGE: RangeInclusive<i32> = (i32::MAX - N_YEAR_BOUND)..=i32::MAX;

const MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn calc_last_month_day(year: i32, month: u8) -> u8 {
    let to = MONTH_DAYS[(month as usize) - 1];
    if iso_old::is_leap_year(year) && (month == 2) {
        to + 1
    } else {
        to
    }
}

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

    fn the_same_in_year(year: i32) {
        for month in 1..=12u8 {
            for day in 1..=calc_last_month_day(year, month) {
                assert_eq!(
                    iso_old::fixed_from_iso(year, month, day),
                    iso_new::fixed_from_iso(year, month, day),
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
fn test_year_from_fixed_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    fn the_same_in_year(year: i32) {
        for month in 1..=12u8 {
            for day in 1..=calc_last_month_day(year, month) {
                let rata_die = iso_old::fixed_from_iso(year, month, day);
                assert_eq!(
                    year as i64,
                    iso_new::iso_year_from_fixed(rata_die),
                    "{year} {month} {day} | {rata_die:?}"
                );
            }
        }
    }

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

    fn the_same_in_year(year: i32) {
        for month in 1..=12u8 {
            for day in 1..=calc_last_month_day(year, month) {
                let rata_die = iso_old::fixed_from_iso(year, month, day);
                assert_eq!(
                    iso_old::iso_from_fixed(rata_die),
                    iso_new::iso_from_fixed(rata_die),
                    "{year} {month} {day}"
                );
            }
        }
    }

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
