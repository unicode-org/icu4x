// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// LiteMap is intended as a small and low-memory drop-in replacement for
// HashMap. This example demonstrates how it works with rkyv.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use litemap::{LiteMap, LiteMapPub};
use rkyv::{
    check_archived_root,
    ser::{Serializer, serializers::WriteSerializer},
    Aligned,
    AlignedVec,
};

const DATA: [(&'static str, &'static str); 11] = [
    ("ar", "Arabic"),
    ("bn", "Bangla"),
    ("ccp", "Chakma"),
    ("en", "English"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("ja", "Japanese"),
    ("ru", "Russian"),
    ("sr", "Serbian"),
    ("th", "Thai"),
    ("tr", "Turkish"),
];

const RKYV: Aligned<[u8; 280]> = Aligned([65, 114, 97, 98, 105, 99, 97, 114, 66, 97, 110, 103, 108, 97, 98, 110, 67, 104, 97, 107, 109, 97, 99, 99, 112, 69, 110, 103, 108, 105, 115, 104, 101, 110, 83, 112, 97, 110, 105, 115, 104, 101, 115, 70, 114, 101, 110, 99, 104, 102, 114, 74, 97, 112, 97, 110, 101, 115, 101, 106, 97, 82, 117, 115, 115, 105, 97, 110, 114, 117, 83, 101, 114, 98, 105, 97, 110, 115, 114, 84, 104, 97, 105, 116, 104, 84, 117, 114, 107, 105, 115, 104, 116, 114, 0, 0, 166, 255, 255, 255, 2, 0, 0, 0, 152, 255, 255, 255, 6, 0, 0, 0, 158, 255, 255, 255, 2, 0, 0, 0, 144, 255, 255, 255, 6, 0, 0, 0, 150, 255, 255, 255, 3, 0, 0, 0, 136, 255, 255, 255, 6, 0, 0, 0, 144, 255, 255, 255, 2, 0, 0, 0, 129, 255, 255, 255, 7, 0, 0, 0, 137, 255, 255, 255, 2, 0, 0, 0, 122, 255, 255, 255, 7, 0, 0, 0, 129, 255, 255, 255, 2, 0, 0, 0, 115, 255, 255, 255, 6, 0, 0, 0, 123, 255, 255, 255, 2, 0, 0, 0, 107, 255, 255, 255, 8, 0, 0, 0, 116, 255, 255, 255, 2, 0, 0, 0, 101, 255, 255, 255, 7, 0, 0, 0, 109, 255, 255, 255, 2, 0, 0, 0, 94, 255, 255, 255, 7, 0, 0, 0, 99, 255, 255, 255, 2, 0, 0, 0, 87, 255, 255, 255, 4, 0, 0, 0, 92, 255, 255, 255, 2, 0, 0, 0, 77, 255, 255, 255, 7, 0, 0, 0, 80, 255, 255, 255, 11, 0, 0, 0]);

type LiteMapOfStrings = LiteMap<String, String>;
type LiteMapPubOfStrings = LiteMapPub<String, String>;

/// Run this function to print new data to the console.
#[allow(dead_code)]
fn generate() {
    let mut map: LiteMapOfStrings = LiteMap::new();
    for (lang, name) in DATA.iter() {
        map.try_append(lang.to_string(), name.to_string()).ok_or(()).unwrap_err();
    }
    let map = map.into_pub();

    let mut serializer = WriteSerializer::new(AlignedVec::new());
    serializer.serialize_value(&map).expect("failed to archive test");
    let buf = serializer.into_inner();
    println!("{:?}", buf);
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    // Uncomment the following line to re-generate the binary data.
    // generate();

    let archived = check_archived_root::<LiteMapPubOfStrings>(&RKYV.0).unwrap();
    let s = archived.values[0].1.as_str();
    assert_eq!(s, "Arabic");

    0
}
