// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

fn main() {
    if std::env::var("ICU4X_FFI_BAKED_ROOT").is_err() {
        // Empty data generated with
        // cargo run -p icu_datagen --features bin,icu_segmenter -- --format mod --use-separate-crates --keys none --out empty_bake
        println!("cargo:rustc-env=ICU4X_FFI_BAKED_ROOT=../empty_bake");
    }
}
