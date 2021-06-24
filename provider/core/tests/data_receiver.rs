// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::hello_world::*;
use icu_provider::prelude::*;
use icu_provider::serde::SerdeDeDataReceiver;
use std::borrow::Cow;
use std::rc::Rc;

#[allow(clippy::redundant_static_lifetimes)]
const DATA_JSON: &'static str = r#"{
    "message": "abc"
}"#;

#[test]
fn test_deserializer_static() {
    // Deserialize from a string to create static references.
    let deserializer = &mut serde_json::Deserializer::from_str(DATA_JSON);
    let mut receiver: Option<DataPayload<HelloWorldV1Marker>> = None;
    receiver
        .receive_static(&mut erased_serde::Deserializer::erase(deserializer))
        .expect("Well-formed data");
    let payload = receiver.expect("Data is present");

    assert!(matches!(
        payload.get(),
        &HelloWorldV1 {
            message: Cow::Borrowed(_)
        }
    ));
}

#[test]
fn test_deserializer_owned() {
    // Deserialize from a reference-counted buffer.
    let rc_buffer: Rc<[u8]> = DATA_JSON.as_bytes().into();
    let mut receiver: Option<DataPayload<HelloWorldV1Marker>> = None;
    receiver
        .receive_rc_buffer(rc_buffer, |bytes, f2| {
            let mut d = serde_json::Deserializer::from_slice(bytes);
            f2(&mut erased_serde::Deserializer::erase(&mut d))
        })
        .expect("Well-formed data");
    let payload = receiver.expect("Data is present");

    assert!(matches!(
        payload.get(),
        &HelloWorldV1 {
            message: Cow::Borrowed(_)
        }
    ));
}
