// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which uses icu_datetime to format entries
// from a work log into human readable dates and times.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use icu_datetime::mock::datetime::MockDateTime;
use icu_datetime::{options::length, DateTimeFormat};
use icu_locid::Locale;
use icu_locid_macros::langid;

const DATES_ISO: &[&str] = &[
    "2001-09-08T18:46:40:000",
    "2017-07-13T19:40:00:000",
    "2020-09-13T05:26:40:000",
    "2021-01-06T22:13:20:000",
    "2021-05-02T17:00:00:000",
    "2021-08-26T10:46:40:000",
    "2021-11-20T03:33:20:000",
    "2022-04-14T22:20:00:000",
    "2022-08-08T16:06:40:000",
    "2033-05-17T20:33:20:000",
];

fn print(_input: &str, _value: Option<usize>) {
    #[cfg(debug_assertions)]
    if let Some(value) = _value {
        println!("{}", _input.replace("{}", &value.to_string()));
    } else {
        println!("{}", _input);
    }
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let locale: Locale = langid!("en").into();

    let provider = icu_testdata::get_provider();

    let dates = DATES_ISO
        .iter()
        .map(|date| date.parse())
        .collect::<Result<Vec<MockDateTime>, _>>()
        .expect("Failed to parse dates.");

    let options = length::Bag {
        date: Some(length::Date::Medium),
        time: Some(length::Time::Short),
        ..Default::default()
    };

    let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
        .expect("Failed to create DateTimeFormat instance.");
    {
        print("\n====== Work Log (en) example ============", None);

        for (idx, date) in dates.iter().enumerate() {
            let fdt = dtf.format(date);
            println!("{}) {}", idx, fdt);
        }
    }

    0
}
