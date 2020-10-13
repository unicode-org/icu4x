// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::fs::File;
use std::io::{BufReader, Error};

#[allow(dead_code)]
const NUMBERS_DATA_PATH: &str = "./benches/fixtures/numbers.json";
#[allow(dead_code)]
const PLURALS_DATA_PATH: &str = "./benches/fixtures/plurals.json";

#[allow(dead_code)]
pub fn get_plurals_data() -> super::fixtures::PluralsFixture {
    read_fixture(PLURALS_DATA_PATH).expect("Failed to read a fixture")
}

#[allow(dead_code)]
pub fn get_numbers_data() -> super::fixtures::NumbersFixture {
    read_fixture(NUMBERS_DATA_PATH).expect("Failed to read a fixture")
}

fn read_fixture<T>(path: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
