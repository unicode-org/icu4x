pub mod structs;

use icu_datetime::date::DateTime;
use icu_datetime::options;
use icu_datetime::DateTimeFormatOptions;
use serde_json;
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
    let style = options::style::Bag {
        date: match input.style.date {
            Some(structs::TestStyleWidth::Full) => options::style::Date::Full,
            Some(structs::TestStyleWidth::Long) => options::style::Date::Long,
            Some(structs::TestStyleWidth::Medium) => options::style::Date::Medium,
            Some(structs::TestStyleWidth::Short) => options::style::Date::Short,
            None => options::style::Date::None,
        },
        time: match input.style.time {
            Some(structs::TestStyleWidth::Full) => options::style::Time::Full,
            Some(structs::TestStyleWidth::Long) => options::style::Time::Long,
            Some(structs::TestStyleWidth::Medium) => options::style::Time::Medium,
            Some(structs::TestStyleWidth::Short) => options::style::Time::Short,
            None => options::style::Time::None,
        },
        ..Default::default()
    };
    DateTimeFormatOptions::Style(style)
}

#[allow(dead_code)]
pub fn parse_date(input: &str) -> Result<DateTime, std::num::ParseIntError> {
    let year: usize = input[0..4].parse()?;
    let month: usize = input[5..7].parse()?;
    let day: usize = input[8..10].parse()?;
    let hour: usize = input[11..13].parse()?;
    let minute: usize = input[14..16].parse()?;
    let second: usize = input[17..19].parse()?;
    let millisecond: usize = input[20..23].parse()?;
    Ok(DateTime {
        year,
        month: month - 1,
        day: day - 1,
        hour,
        minute,
        second,
        millisecond,
    })
}
