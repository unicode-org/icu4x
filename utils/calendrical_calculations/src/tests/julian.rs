use super::helpful_consts::*;
use super::julian_old_file as old;
use crate::{helpers::I32CastError, julian as new, rata_die::RataDie};
use core::ops::RangeInclusive;

fn calc_last_month_day(year: i32, month: u8) -> u8 {
    let to = MONTH_DAYS[(month as usize) - 1];
    if new::is_leap_year(year) && (month == 2) {
        to + 1
    } else {
        to
    }
}

#[test]
fn day_of_year_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    fn the_same_in_year(year: i32) {
        for month in 1..=12u8 {
            for day in 1..=calc_last_month_day(year, month) {
                assert_eq!(
                    old::day_of_year(year, month, day),
                    new::day_of_year(year, month, day),
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
fn fixed_from_julian_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    fn the_same_in_year(year: i32) {
        for month in 1..=12u8 {
            for day in 1..=calc_last_month_day(year, month) {
                assert_eq!(
                    old::fixed_from_julian(year, month, day),
                    new::fixed_from_julian(year, month, day),
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
fn julian_from_fixed_the_same() {
    const N_YEAR: i32 = 9_999;
    const N_RANGE: RangeInclusive<i32> = (-N_YEAR)..=N_YEAR;

    fn the_same_in_year(year: i32) {
        for month in 1..=12u8 {
            for day in 1..=calc_last_month_day(year, month) {
                let date = new::fixed_from_julian(year, month, day);
                assert_eq!(
                    old::julian_from_fixed(date),
                    new::julian_from_fixed(date),
                    "YMD: {year} {month} {day}"
                );
            }
        }
    }

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
