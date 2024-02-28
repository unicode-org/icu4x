// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use resb::{binary, text};

fn main() {
    let input = File::open("examples/data/zoneinfo64.txt");
    let mut reader = BufReader::new(input.unwrap());

    let mut in_string = String::new();
    match reader.read_to_string(&mut in_string) {
        Ok(_) => (),
        Err(err) => panic!("Unable to read file: {}", err),
    };

    let (in_bundle, keys_in_discovery_order) = match text::Reader::read(&in_string) {
        Ok(result) => result,
        Err(err) => panic!("Failed to parse text bundle:\n{err}"),
    };

    let file = File::create("examples/data/zoneinfo64.res");
    let mut writer = BufWriter::new(file.unwrap());

    let bytes = binary::Serializer::to_bytes(&in_bundle, &keys_in_discovery_order)
        .expect("Failed to generate binary bundle.");

    writer
        .write_all(&bytes)
        .expect("Failed to write binary bundle.");
}
