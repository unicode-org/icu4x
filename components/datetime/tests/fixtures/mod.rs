// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
pub mod structs;

use icu_datetime::options::length;
use icu_datetime::DateTimeFormatOptions;
use std::fs::File;
use std::io::BufReader;

pub fn get_fixture(name: &str) -> std::io::Result<structs::Fixture> {
    let file = File::open(format!("./tests/fixtures/tests/{}.json", name))?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

pub fn get_options(input: &structs::TestOptions) -> DateTimeFormatOptions {
    let length = length::Bag {
        date: input.length.date.as_ref().map(|date| match date {
            structs::TestStyleWidth::Full => length::Date::Full,
            structs::TestStyleWidth::Long => length::Date::Long,
            structs::TestStyleWidth::Medium => length::Date::Medium,
            structs::TestStyleWidth::Short => length::Date::Short,
        }),
        time: input.length.time.as_ref().map(|time| match time {
            structs::TestStyleWidth::Full => length::Time::Full,
            structs::TestStyleWidth::Long => length::Time::Long,
            structs::TestStyleWidth::Medium => length::Time::Medium,
            structs::TestStyleWidth::Short => length::Time::Short,
        }),
        ..Default::default()
    };
    length.into()
}
