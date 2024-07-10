// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which uses icu_datetime to format entries
// from a log into human readable dates and times.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
icu_benchmark_macros::instrument!();
use icu_benchmark_macros::println;

use icu_calendar::{DateError, DateTime, Iso};

const DATETIMES_ISO: &[(i32, u8, u8, u8, u8, u8)] = &[
    (1970, 1, 1, 3, 5, 12),
    (1982, 3, 11, 2, 25, 59),
    (1999, 2, 21, 13, 12, 23),
    (2000, 12, 29, 10, 50, 23),
    (2001, 9, 8, 11, 5, 5),
    (2017, 7, 12, 3, 1, 1),
    (2020, 2, 29, 23, 12, 23),
    (2021, 3, 21, 18, 35, 34),
    (2021, 6, 10, 13, 12, 23),
    (2021, 9, 2, 5, 50, 22),
    (2022, 10, 8, 9, 45, 32),
    (2022, 2, 9, 10, 32, 45),
    (2033, 6, 10, 17, 22, 22),
];

fn tuple_to_iso_datetime(date: (i32, u8, u8, u8, u8, u8)) -> Result<DateTime<Iso>, DateError> {
    DateTime::try_new_iso_datetime(date.0, date.1, date.2, date.3, date.4, date.5)
}

fn main() {
    let datetimes = DATETIMES_ISO
        .iter()
        .copied()
        .map(tuple_to_iso_datetime)
        .collect::<Result<Vec<DateTime<Iso>>, _>>()
        .expect("Failed to parse datetimes.");

    for datetime_input in datetimes {
        println!(
            "Year: {}, Month: {}, Day: {}, Hour: {}, Minute: {}, Second: {}",
            datetime_input.date.year().number,
            datetime_input.date.month().ordinal,
            datetime_input.date.day_of_month().0,
            u8::from(datetime_input.time.hour),
            u8::from(datetime_input.time.minute),
            u8::from(datetime_input.time.second),
        );
    }
}
