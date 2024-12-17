// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which uses date range iteration.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
icu_benchmark_macros::instrument!();
use icu_benchmark_macros::println;

use icu_calendar::range::ToDateIter;
use icu_calendar::{Date, Iso};

fn print_date(date: Date<Iso>) {
    let year = date.year().era_year_or_extended();
    let mon = date.month().ordinal;
    let day = date.day_of_month().0;
    println!("Date: {day:02} {mon:02} {year}");
}

fn print_space() {
    println!("");
}

fn main() {
    let date_from = Date::try_new_iso(2024, 11, 29).unwrap();
    let date_to = Date::try_new_iso(2025, 3, 31).unwrap();

    // Let's print 10 dates from infinity date range:
    let inf_range = date_from..;
    inf_range.to_date_iter().take(10).for_each(print_date);

    print_space();

    // Let's print each 13th date from non-infinity range:
    let range = date_from..date_to;
    let iter = range.to_date_iter().step_by(13);
    iter.for_each(print_date);

    print_space();

    // Let's print each 13th date from reversed non-infinity range:
    let range_inclusive = date_from..=date_to;
    let iter = range_inclusive.to_date_iter().rev().step_by(13);
    iter.for_each(print_date);
}
