// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// LiteMap is intended as a small and low-memory drop-in replacement for
// HashMap. This example demonstrates how it compares to HashMap.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use std::collections::HashMap;
use icu_locid_macros::language;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let mut map = HashMap::new();
    // https://github.com/rust-lang/rust/issues/62633 was declined :(
    map.insert(language!("ar"), "Arabic").ok_or(()).unwrap_err();
    map.insert(language!("bn"), "Bangla").ok_or(()).unwrap_err();
    map.insert(language!("ccp"), "Chakma").ok_or(()).unwrap_err();
    map.insert(language!("en"), "English").ok_or(()).unwrap_err();
    map.insert(language!("es"), "Spanish").ok_or(()).unwrap_err();
    map.insert(language!("fr"), "French").ok_or(()).unwrap_err();
    map.insert(language!("ja"), "Japanese").ok_or(()).unwrap_err();
    map.insert(language!("ru"), "Russian").ok_or(()).unwrap_err();
    map.insert(language!("sr"), "Serbian").ok_or(()).unwrap_err();
    map.insert(language!("th"), "Thai").ok_or(()).unwrap_err();
    map.insert(language!("tr"), "Turkish").ok_or(()).unwrap_err();

    debug_assert_eq!(11, map.len());
    debug_assert_eq!(Some(&"Thai"), map.get(&language!("th")));
    debug_assert_eq!(None, map.get(&language!("de")));

    0
}
