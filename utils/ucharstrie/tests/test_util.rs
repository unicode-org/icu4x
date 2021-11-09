// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;
use ucharstrie::ucharstrie::UCharsTrie;
use zerovec::ZeroVec;

#[derive(serde::Deserialize)]
pub struct TestFile {
    ucharstrie: TestUCharsTrie,
}

#[derive(serde::Deserialize)]
pub struct TestUCharsTrie {
    name: String,
    data: Vec<u16>,
    length: usize,
}

// Given a .toml file dumped from ICU4C test data for UCharsTrie, run the test
// data file deserialization into the test file struct, convert and construct
// the `UCharsTrie`, and return the constructed struct.
pub fn load_ucharstrie_data(test_file_path: &str) -> Vec<u16> {
    let path = Path::new(test_file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut toml_str = String::new();

    if let Err(err) = file.read_to_string(&mut toml_str) {
        panic!("couldn't read {}: {}", display, err)
    }

    let test_file: TestFile = toml::from_str(&toml_str).unwrap();
    test_file.ucharstrie.data
}
