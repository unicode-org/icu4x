// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// These extern crates are needed for linking
extern crate icu_capi;

unsafe extern "C" {
    fn c_main() -> i32;
}

fn main() {
    std::process::exit(unsafe {c_main()})
}
