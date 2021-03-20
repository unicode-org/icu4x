// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::erased::*;
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
fn test_deserializer_borrowed() {
    // Deserialize from a local string to create non-static references.
    let local_data = DATA_JSON.clone();
    let deserializer = &mut serde_json::Deserializer::from_str(&local_data);
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
