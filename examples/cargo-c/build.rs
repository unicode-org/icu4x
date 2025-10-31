// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

fn main() {
    cc::Build::new()
        .file("src/locale.c")
        .include(std::env::var("DEP_ICU_CAPI_2_C_INCLUDE_DIR").unwrap())
        .compile("locale");
}