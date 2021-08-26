// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// LiteMap is intended as a small and low-memory drop-in replacement for
// HashMap. This example demonstrates how it works with rkyv.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use rkyv::{check_archived_root, util::AlignedBytes};

// The data can be re-generated in litemap_rkyv_archive.rs
const RKYV: AlignedBytes<192> = AlignedBytes([
    74, 97, 112, 97, 110, 101, 115, 101, 97, 114, 0, 0, 0, 0, 0, 2, 65, 114, 97, 98, 105, 99, 0, 6,
    98, 110, 0, 0, 0, 0, 0, 2, 66, 97, 110, 103, 108, 97, 0, 6, 99, 99, 112, 0, 0, 0, 0, 3, 67,
    104, 97, 107, 109, 97, 0, 6, 101, 110, 0, 0, 0, 0, 0, 2, 69, 110, 103, 108, 105, 115, 104, 7,
    101, 115, 0, 0, 0, 0, 0, 2, 83, 112, 97, 110, 105, 115, 104, 7, 102, 114, 0, 0, 0, 0, 0, 2, 70,
    114, 101, 110, 99, 104, 0, 6, 106, 97, 0, 0, 0, 0, 0, 2, 8, 0, 0, 0, 144, 255, 255, 255, 114,
    117, 0, 0, 0, 0, 0, 2, 82, 117, 115, 115, 105, 97, 110, 7, 115, 114, 0, 0, 0, 0, 0, 2, 83, 101,
    114, 98, 105, 97, 110, 7, 116, 104, 0, 0, 0, 0, 0, 2, 84, 104, 97, 105, 0, 0, 0, 4, 116, 114,
    0, 0, 0, 0, 0, 2, 84, 117, 114, 107, 105, 115, 104, 7, 80, 255, 255, 255, 11, 0, 0, 0,
]);

type TupleVecOfStrings = Vec<(String, String)>;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let archived = check_archived_root::<TupleVecOfStrings>(&RKYV.0).unwrap();
    let s = archived[0].1.as_str();
    assert_eq!(s, "Arabic");

    0
}
