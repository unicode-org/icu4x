// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod structs;

use std::fs::File;
use std::io::BufReader;

use self::structs::{dayperiods::DayPeriodTests, time_zones::TimeZoneTests};

pub fn get_dayperiod_tests(name: &str) -> std::io::Result<DayPeriodTests> {
    let file = File::open(format!("./tests/patterns/tests/{name}.json"))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

pub fn get_time_zone_tests(name: &str) -> std::io::Result<TimeZoneTests> {
    let file = File::open(format!("./tests/patterns/tests/{name}.json"))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
