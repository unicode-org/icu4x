// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
pub mod structs;

use icu_datetime::options::{style, DateTimeFormatOptions};
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
    let style = style::Bag {
        date: input.style.date.as_ref().map(|date| match date {
            structs::TestStyleWidth::Full => style::Date::Full,
            structs::TestStyleWidth::Long => style::Date::Long,
            structs::TestStyleWidth::Medium => style::Date::Medium,
            structs::TestStyleWidth::Short => style::Date::Short,
        }),
        time: input.style.time.as_ref().map(|time| match time {
            structs::TestStyleWidth::Full => style::Time::Full,
            structs::TestStyleWidth::Long => style::Time::Long,
            structs::TestStyleWidth::Medium => style::Time::Medium,
            structs::TestStyleWidth::Short => style::Time::Short,
        }),
        ..Default::default()
    };
    style.into()
}
