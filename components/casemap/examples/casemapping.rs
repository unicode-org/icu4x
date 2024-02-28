// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use icu::casemap::CaseMapper;
use icu_locid::langid;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let cm = CaseMapper::new();

    println!(
        r#"The uppercase of "hello world" is "{}""#,
        cm.uppercase_to_string("hello world", &langid!("und"))
    );
    println!(
        r#"The lowercase of "Γειά σου Κόσμε" is "{}""#,
        cm.lowercase_to_string("Γειά σου Κόσμε", &langid!("und"))
    );

    0
}
