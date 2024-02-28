// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which uses icu_datetime to format entries
// from a work log into human readable dates and times.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use icu_calendar::{DateTime, Gregorian};
use icu_datetime::{options::length, TypedDateTimeFormatter};
use icu_locid::locale;

const DATES_ISO: &[(i32, u8, u8, u8, u8, u8)] = &[
    (2001, 9, 8, 18, 46, 40),
    (2017, 7, 13, 19, 40, 0),
    (2020, 9, 13, 5, 26, 40),
    (2021, 1, 6, 22, 13, 20),
    (2021, 5, 2, 17, 0, 0),
    (2021, 8, 26, 10, 46, 40),
    (2021, 11, 20, 3, 33, 20),
    (2022, 4, 14, 22, 20, 0),
    (2022, 8, 8, 16, 6, 40),
    (2033, 5, 17, 20, 33, 20),
];

fn print(_input: &str, _value: Option<usize>) {
    #[cfg(debug_assertions)]
    if let Some(value) = _value {
        println!("{}", _input.replace("{}", &value.to_string()));
    } else {
        println!("{_input}");
    }
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let dates = DATES_ISO
        .iter()
        .copied()
        .map(|(y, m, d, h, min, s)| DateTime::try_new_gregorian_datetime(y, m, d, h, min, s))
        .collect::<Result<Vec<DateTime<Gregorian>>, _>>()
        .expect("Failed to parse dates.");

    let mut options = length::Bag::default();

    options.date = Some(length::Date::Medium);
    options.time = Some(length::Time::Short);

    let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(&locale!("en").into(), options.into())
        .expect("Failed to create TypedDateTimeFormatter instance.");
    {
        print("\n====== Work Log (en) example ============", None);

        for (idx, date) in dates.iter().enumerate() {
            let fdt = dtf.format(date);
            println!("{idx}) {fdt}");
        }
    }

    0
}
