// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which uses icu_datetime to format entries
// from a log into human readable dates and times.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use icu_calendar::{Date, DateTimeError, Iso};

const DATES_ISO: &[(i32, u8, u8)] = &[
    (1970, 1, 1),
    (1982, 3, 11),
    (1999, 2, 21),
    (2000, 12, 29),
    (2001, 9, 8),
    (2017, 7, 12),
    (2020, 2, 29),
    (2021, 3, 21),
    (2021, 6, 10),
    (2021, 9, 2),
    (2022, 10, 8),
    (2022, 2, 9),
    (2033, 6, 10),
];

fn print(_input: &str) {
    #[cfg(debug_assertions)]
    println!("{}", _input);
}

fn tuple_to_iso_date(date: (i32, u8, u8)) -> Result<Date<Iso>, DateTimeError> {
    Date::new_iso_date_from_integers(date.0, date.1, date.2)
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let dates = DATES_ISO
        .iter()
        .copied()
        .map(tuple_to_iso_date)
        .collect::<Result<Vec<Date<Iso>>, _>>()
        .expect("Failed to parse dates.");

    for date in dates.iter() {
        let formatted_date = format!(
            "Year: {}, Month: {}, Day: {}",
            date.year().number.to_string(),
            date.month().number.to_string(),
            date.day_of_month().0.to_string()
        );
        print(&formatted_date);
    }

    0
}
