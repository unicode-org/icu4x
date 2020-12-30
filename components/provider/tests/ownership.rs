// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_provider::erased::ErasedDataStruct;
use icu_provider::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
struct DataStruct<'a> {
    #[serde(borrow)]
    pub value: Cow<'a, str>,
}

#[allow(clippy::redundant_static_lifetimes)]
const DATA_JSON: &'static str = r#"{
    "value": "abc"
}"#;

const DATA_STRUCT: DataStruct<'static> = DataStruct {
    value: Cow::Borrowed("def"),
};

fn get_struct_with_static_references() -> DataStruct<'static> {
    // Deserialize from a string to create static references.
    serde_json::from_str(DATA_JSON).expect("Well-formed data")
}

fn get_struct_with_owned_data() -> DataStruct<'static> {
    icu_provider::resource_key!(x, "foo", "bar", 1);
    // Deserialize from a reader to create owned data.
    // NOTE: Unclear why plain `serde_json::from_reader` doesn't work here:
    //     error: implementation of `_::_serde::Deserialize` is not general enough
    // serde_json::from_reader(DATA_JSON.as_bytes()).unwrap();
    let deserializer = &mut serde_json::Deserializer::from_reader(DATA_JSON.as_bytes());
    DataStruct::deserialize(deserializer).expect("Well-formed data")
}

#[test]
fn test_deserializer_static() {
    // Deserialize from a string to create static references.
    let deserializer = &mut serde_json::Deserializer::from_str(DATA_JSON);
    let mut receiver = DataReceiver::<DataStruct>::new();
    receiver
        .receive_deserializer(&mut erased_serde::Deserializer::erase(deserializer))
        .expect("Well-formed data");

    assert!(matches!(
        receiver.payload,
        Some(Cow::Owned(DataStruct {
            value: Cow::Borrowed(_)
        }))
    ));
}

#[test]
fn test_deserializer_owned() {
    // Deserialize from a reader to create owned data.
    let deserializer = &mut serde_json::Deserializer::from_reader(DATA_JSON.as_bytes());
    let mut receiver = DataReceiver::<DataStruct>::new();
    receiver
        .receive_deserializer(&mut erased_serde::Deserializer::erase(deserializer))
        .expect("Well-formed data");

    assert!(matches!(
        receiver.payload,
        Some(Cow::Owned(DataStruct {
            value: Cow::Owned(_)
        }))
    ));
}

#[test]
fn test_borrow_static() {
    let mut receiver = DataReceiver::<DataStruct>::new();
    receiver
        .receive_erased(Cow::Borrowed(&DATA_STRUCT))
        .expect("Types should match");

    assert!(matches!(
        receiver.payload,
        Some(Cow::Borrowed(DataStruct {
            value: Cow::Borrowed(_)
        }))
    ));
}

#[test]
fn test_box_static() {
    let mut receiver = DataReceiver::<DataStruct>::new();
    receiver
        .receive_erased(Cow::Owned(
            Box::new(get_struct_with_static_references()) as Box<dyn ErasedDataStruct>
        ))
        .expect("Types should match");

    assert!(matches!(
        receiver.payload,
        Some(Cow::Owned(DataStruct {
            value: Cow::Borrowed(_)
        }))
    ));
}

#[test]
fn test_box_owned() {
    let mut receiver = DataReceiver::<DataStruct>::new();
    receiver
        .receive_erased(Cow::Owned(
            Box::new(get_struct_with_owned_data()) as Box<dyn ErasedDataStruct>
        ))
        .expect("Types should match");

    assert!(matches!(
        receiver.payload,
        Some(Cow::Owned(DataStruct {
            value: Cow::Owned(_)
        }))
    ));
}
