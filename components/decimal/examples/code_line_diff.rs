// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which displays the number of lines added and
// removed from a series of pull requests.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use icu_decimal::FixedDecimalFormatter;
use icu_locid::locale;

icu_benchmark_macros::static_setup!();

const LINES_REMOVED_ADDED: [(i64, i64); 5] = [
    (-50, 72),
    (0, 3750),
    (-1201, 0),
    (-9876, 5432),
    (-5000000, 3000000),
];

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let fdf = FixedDecimalFormatter::try_new(&locale!("bn").into(), Default::default())
        .expect("locale should be present");

    for (removed, added) in LINES_REMOVED_ADDED {
        let removed = fdf.format_to_string(&removed.into());
        let added = fdf.format_to_string(&added.into());
        assert!(!removed.is_empty());
        assert!(!added.is_empty());
        #[cfg(debug_assertions)]
        println!("Added/Removed: {added}/{removed}",);
    }

    0
}
