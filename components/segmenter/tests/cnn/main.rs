// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::Deserialize;

mod helper;

static JSON_STR: &str = include_str!("sample.json");

#[test]
fn hello_world() {
    println!("hello world");
    assert_eq!(helper::get_shape(), vec![4]);
}

#[derive(Deserialize)]
struct JsonStruct {
    key: String,
}

#[test]
fn test_json() {
    let json_obj: JsonStruct = serde_json::from_str(JSON_STR).unwrap();
    assert_eq!(json_obj.key, "value");
}
