pub mod structs;

use icu_datetime::options::{style, DateTimeFormatOptions};
use icu_datetime::DummyDateTime;
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

#[allow(dead_code)]
pub fn parse_date(input: &str) -> Result<DummyDateTime, std::num::ParseIntError> {
    let year: usize = input[0..4].parse()?;
    let month: usize = input[5..7].parse()?;
    let day: usize = input[8..10].parse()?;
    let hour: usize = input[11..13].parse()?;
    let minute: usize = input[14..16].parse()?;
    let second: usize = input[17..19].parse()?;
    let millisecond: usize = input[20..23].parse()?;
    Ok(DummyDateTime {
        year,
        month: month - 1,
        day: day - 1,
        hour,
        minute,
        second,
        millisecond,
    })
}
