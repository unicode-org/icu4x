// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

fn main() {
    if std::env::var("CARGO_FEATURE_DATA").is_ok() {
        match std::env::var("ICU4X_DATA_DIR") {
            Ok(path) => println!("cargo:rustc-env=MACROS_RS={}/macros.rs", path),
            Err(_) => println!("cargo:rustc-env=MACROS_RS=../data/macros.rs"),
        }
        println!("cargo:rerun-if-env-changed=ICU4X_DATA_DIR");
    }
}
