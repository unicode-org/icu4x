// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_codepointtrie::codepointtrie::*;
use icu_codepointtrie::error::Error;

use core::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zerovec::ZeroVec;

pub fn check_trie<W: ValueWidth>(trie: &CodePointTrie<W>, check_ranges: &[u32]) {
    assert_eq!(
        0,
        check_ranges.len() % 2,
        "check_ranges must have an even number of 32-bit values in (limit,value) pairs"
    );

    let mut i: u32 = 0;
    let check_range_tuples = check_ranges.chunks(2);
    // Iterate over each check range
    for range_tuple in check_range_tuples {
        let range_end = range_tuple[0];
        let range_value = range_tuple[1];
        // Check all values in this range, one-by-one
        while i < range_end {
            assert_eq!(range_value, trie.get_u32(i), "trie_get({})", i,);
            i += 1;
        }
    }
}

// The following structs might be useful later for de-/serialization of the
// main `CodePointTrie` struct in the corresponding data provider.

#[cfg_attr(any(feature = "serde", test), derive(serde::Deserialize))]
pub struct UnicodeEnumeratedProperty {
    pub code_point_map: EnumPropCodePointMap,
    pub code_point_trie: EnumPropSerializedCPT,
}

#[cfg_attr(any(feature = "serde", test), derive(serde::Deserialize))]
pub struct EnumPropCodePointMap {
    pub data: EnumPropCodePointMapData,
}

#[cfg_attr(any(feature = "serde", test), derive(serde::Deserialize))]
pub struct EnumPropCodePointMapData {
    pub long_name: String,
    pub name: String,
    pub ranges: Vec<(u32, u32, u32)>,
}

#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(any(feature = "serde", test), derive(serde::Deserialize))]
pub struct EnumPropSerializedCPT {
    #[cfg_attr(any(feature = "serde", test), serde(rename = "struct"))]
    pub trie_struct: EnumPropSerializedCPTStruct,
}

// These structs support the test data dumped as TOML files from ICU.
// Because the properties CodePointMap data will also be dumped from ICU
// using similar functions, some of these structs may be useful to refactor
// into main code at a later point.

#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(any(feature = "serde", test), derive(serde::Deserialize))]
pub struct EnumPropSerializedCPTStruct {
    #[cfg_attr(any(feature = "serde", test), serde(skip))]
    pub long_name: String,
    pub name: String,
    pub index: Vec<u16>,
    pub data_8: Option<Vec<u8>>,
    pub data_16: Option<Vec<u16>>,
    pub data_32: Option<Vec<u32>>,
    #[cfg_attr(any(feature = "serde", test), serde(skip))]
    pub index_length: u32,
    #[cfg_attr(any(feature = "serde", test), serde(skip))]
    pub data_length: u32,
    #[cfg_attr(any(feature = "serde", test), serde(rename = "highStart"))]
    pub high_start: u32,
    #[cfg_attr(any(feature = "serde", test), serde(rename = "shifted12HighStart"))]
    pub shifted12_high_start: u16,
    #[cfg_attr(any(feature = "serde", test), serde(rename = "type"))]
    pub trie_type_enum_val: u8,
    #[cfg_attr(any(feature = "serde", test), serde(rename = "valueWidth"))]
    pub value_width_enum_val: u8,
    #[cfg_attr(any(feature = "serde", test), serde(rename = "index3NullOffset"))]
    pub index3_null_offset: u16,
    #[cfg_attr(any(feature = "serde", test), serde(rename = "dataNullOffset"))]
    pub data_null_offset: u32,
    #[cfg_attr(any(feature = "serde", test), serde(rename = "nullValue"))]
    pub null_value: u32,
}

// The following structs are specific to the TOML format files for dumped ICU
// test data.

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct TestFile {
    code_point_trie: TestCodePointTrie,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct TestCodePointTrie {
    // The trie_struct field for test data files is dumped from the same source
    // (ICU4C) using the same function (usrc_writeUCPTrie) as property data
    // for the provider, so we can reuse the same struct here.
    #[serde(rename(deserialize = "struct"))]
    trie_struct: EnumPropSerializedCPTStruct,
    #[serde(rename(deserialize = "testdata"))]
    test_data: TestData,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct TestData {
    #[serde(rename(deserialize = "checkRanges"))]
    check_ranges: Vec<u32>,
}

// Given a .toml file dumped from ICU4C test data for UCPTrie, run the test
// data file deserialization into the test file struct, convert and construct
// the `CodePointTrie`, and test the constructed struct against the test file's
// "check ranges" (inversion map ranges) using `check_trie` to verify the
// validity of the `CodePointTrie`'s behavior for all code points.
#[allow(dead_code)]
pub fn run_deserialize_test_from_test_data(test_file_path: &str) {
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
    let test_struct = test_file.code_point_trie.trie_struct;

    println!(
        "Running CodePointTrie reader logic test on test data file: {}",
        test_struct.name
    );

    let trie_type_enum = match TrieTypeEnum::try_from(test_struct.trie_type_enum_val) {
        Ok(enum_val) => enum_val,
        _ => {
            panic!(
                "Could not parse trie_type serialized enum value in test data file: {}",
                test_struct.name
            );
        }
    };

    let trie_header = CodePointTrieHeader {
        high_start: test_struct.high_start,
        shifted12_high_start: test_struct.shifted12_high_start,
        index3_null_offset: test_struct.index3_null_offset,
        data_null_offset: test_struct.data_null_offset,
        null_value: test_struct.null_value,
        trie_type: trie_type_enum,
    };

    let index = ZeroVec::from_slice(&test_struct.index);

    match (test_struct.data_8, test_struct.data_16, test_struct.data_32) {
        (Some(data_8), _, _) => {
            let data = ZeroVec::from_slice(&data_8);
            let trie_result: Result<CodePointTrie<u8>, Error> =
                CodePointTrie::try_new(trie_header, index, data);
            assert!(trie_result.is_ok(), "Could not construct trie");
            assert_eq!(
                test_struct.value_width_enum_val,
                ValueWidthEnum::Bits8 as u8
            );
            check_trie(
                &trie_result.unwrap(),
                &test_file.code_point_trie.test_data.check_ranges,
            );
        }

        (_, Some(data_16), _) => {
            let data = ZeroVec::from_slice(&data_16);
            let trie_result: Result<CodePointTrie<u16>, Error> =
                CodePointTrie::try_new(trie_header, index, data);
            assert!(trie_result.is_ok(), "Could not construct trie");
            assert_eq!(
                test_struct.value_width_enum_val,
                ValueWidthEnum::Bits16 as u8
            );
            check_trie(
                &trie_result.unwrap(),
                &test_file.code_point_trie.test_data.check_ranges,
            );
        }

        (_, _, Some(data_32)) => {
            let data = ZeroVec::from_slice(&data_32);
            let trie_result: Result<CodePointTrie<u32>, Error> =
                CodePointTrie::try_new(trie_header, index, data);
            assert!(trie_result.is_ok(), "Could not construct trie");
            assert_eq!(
                test_struct.value_width_enum_val,
                ValueWidthEnum::Bits32 as u8
            );
            check_trie(
                &trie_result.unwrap(),
                &test_file.code_point_trie.test_data.check_ranges,
            );
        }

        (_, _, _) => {
            panic!("Could not match test trie data to a known value width or trie type");
        }
    };
}
