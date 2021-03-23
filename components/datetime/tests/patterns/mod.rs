// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod structs;

use std::fs::File;
use std::io::BufReader;

pub fn get_tests(name: &str) -> std::io::Result<structs::Tests> {
    let file = File::open(format!("./tests/patterns/tests/{}.json", name))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
