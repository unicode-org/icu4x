// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate icu;
extern crate icu_provider;

use icu_provider::hello_world::{HelloWorldFormatter, HelloWorldProvider};
use icu::locid::locale;

fn main() {
    let fmt = HelloWorldFormatter::try_new_unstable(
        &HelloWorldProvider,
        &locale!("eo").into(),
    )
    .expect("locale exists");
    println!("{}", fmt.format());
}
