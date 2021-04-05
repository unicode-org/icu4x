// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod structs;

use icu_datetime::options::{length, DateTimeFormatOptions};
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub fn get_fixture(name: &str) -> std::io::Result<structs::Fixture> {
    let file = File::open(format!("./benches/fixtures/tests/{}.json", name))?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

#[allow(dead_code)]
pub fn get_patterns_fixture() -> std::io::Result<structs::PatternsFixture> {
    let file = File::open("./benches/fixtures/tests/patterns.json")?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

#[allow(dead_code)]
pub fn get_options(input: &structs::TestOptions) -> DateTimeFormatOptions {
    let length = length::Bag {
        date: input.length.date.as_ref().map(|date| match date {
            structs::TestLength::Full => length::Date::Full,
            structs::TestLength::Long => length::Date::Long,
            structs::TestLength::Medium => length::Date::Medium,
            structs::TestLength::Short => length::Date::Short,
        }),
        time: input.length.time.as_ref().map(|time| match time {
            structs::TestLength::Full => length::Time::Full,
            structs::TestLength::Long => length::Time::Long,
            structs::TestLength::Medium => length::Time::Medium,
            structs::TestLength::Short => length::Time::Short,
        }),
        ..Default::default()
    };
    length.into()
}
