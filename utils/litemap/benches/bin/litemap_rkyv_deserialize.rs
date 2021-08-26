// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// LiteMap is intended as a small and low-memory drop-in replacement for
// HashMap. This example demonstrates how it works with rkyv.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use litemap::LiteMap;
use rkyv::{archived_root, util::AlignedBytes, Deserialize, Infallible};

// The data can be re-generated in litemap_rkyv_archive.rs
const RKYV: AlignedBytes<280> = AlignedBytes([
    65, 114, 97, 98, 105, 99, 97, 114, 66, 97, 110, 103, 108, 97, 98, 110, 67, 104, 97, 107, 109,
    97, 99, 99, 112, 69, 110, 103, 108, 105, 115, 104, 101, 110, 83, 112, 97, 110, 105, 115, 104,
    101, 115, 70, 114, 101, 110, 99, 104, 102, 114, 74, 97, 112, 97, 110, 101, 115, 101, 106, 97,
    82, 117, 115, 115, 105, 97, 110, 114, 117, 83, 101, 114, 98, 105, 97, 110, 115, 114, 84, 104,
    97, 105, 116, 104, 84, 117, 114, 107, 105, 115, 104, 116, 114, 0, 0, 166, 255, 255, 255, 2, 0,
    0, 0, 152, 255, 255, 255, 6, 0, 0, 0, 158, 255, 255, 255, 2, 0, 0, 0, 144, 255, 255, 255, 6, 0,
    0, 0, 150, 255, 255, 255, 3, 0, 0, 0, 136, 255, 255, 255, 6, 0, 0, 0, 144, 255, 255, 255, 2, 0,
    0, 0, 129, 255, 255, 255, 7, 0, 0, 0, 137, 255, 255, 255, 2, 0, 0, 0, 122, 255, 255, 255, 7, 0,
    0, 0, 129, 255, 255, 255, 2, 0, 0, 0, 115, 255, 255, 255, 6, 0, 0, 0, 123, 255, 255, 255, 2, 0,
    0, 0, 107, 255, 255, 255, 8, 0, 0, 0, 116, 255, 255, 255, 2, 0, 0, 0, 101, 255, 255, 255, 7, 0,
    0, 0, 109, 255, 255, 255, 2, 0, 0, 0, 94, 255, 255, 255, 7, 0, 0, 0, 99, 255, 255, 255, 2, 0,
    0, 0, 87, 255, 255, 255, 4, 0, 0, 0, 92, 255, 255, 255, 2, 0, 0, 0, 77, 255, 255, 255, 7, 0, 0,
    0, 80, 255, 255, 255, 11, 0, 0, 0,
]);

type LiteMapOfStrings = LiteMap<String, String>;
type TupleVecOfStrings = Vec<(String, String)>;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let archived = unsafe { archived_root::<TupleVecOfStrings>(&RKYV.0) };
    let deserialized = archived.deserialize(&mut Infallible).unwrap();
    // Safe because we are deserializing a buffer from a trusted source
    let deserialized: LiteMapOfStrings = unsafe { LiteMap::from_tuple_vec_unchecked(deserialized) };
    assert_eq!(deserialized.get("tr"), Some(&"Turkish".to_string()));

    0
}
