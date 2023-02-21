// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "serde")]

pub mod structs;

use icu_datetime::DateTimeFormatterOptions;
use std::fs::File;
use std::io::BufReader;

pub fn get_fixture(name: &str) -> std::io::Result<structs::Fixture> {
    let file = File::open(format!("./tests/fixtures/tests/{name}.json"))?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

pub fn get_options(input: &structs::TestOptions) -> Option<DateTimeFormatterOptions> {
    match input {
        structs::TestOptions::Length(bag) => Some((*bag).into()),
        #[cfg(feature = "experimental")]
        structs::TestOptions::Components(bag) => Some((*bag).into()),
        #[cfg(not(feature = "experimental"))]
        structs::TestOptions::Components(_) => None,
    }
}
