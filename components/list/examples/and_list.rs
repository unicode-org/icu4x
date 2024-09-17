// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
icu_benchmark_macros::instrument!();
use icu_benchmark_macros::println;

use icu::list::{ListFormatter, ListLength};
use icu::locale::locale;

fn main() {
    let list_formatter =
        ListFormatter::try_new_and_with_length(&locale!("es").into(), ListLength::Wide).unwrap();

    println!(
        "{}",
        list_formatter.format(["España", "Francia", "Suiza", "Italia"].iter())
    );
}
