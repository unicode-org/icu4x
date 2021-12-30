// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_list::{
    options::{Type, Width},
    ListFormatter,
};
use icu_locid_macros::langid;

fn main() {
    let provider = icu_testdata::get_static_provider();

    let list_formatter =
        ListFormatter::try_new(langid!("es"), &provider, Type::And, Width::Wide).unwrap();

    println!(
        "{}",
        list_formatter.format(&["España", "Francia", "Suiza", "Italia"])
    );
}
