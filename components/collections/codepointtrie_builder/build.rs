// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::env;

fn main() {
    if env::var("CARGO_FEATURE_ICU4C").is_ok() && env::var("CARGO_FEATURE_WASM").is_err() {
        let lib_path = env::var("ICU4C_LIB_PATH").expect("Must have `ICU4C_LIB_PATH` environment variable set when building icu_codepointtrie_builder with the `icu4c` feature");
        let kind = match env::var("ICU4C_LINK_STATICALLY").is_ok() {
            true => "static",
            false => "dylib",
        };

        if env::var("ICU4C_RENAME_VERSION").is_ok() {
            println!("cargo:rustc-cfg=icu4c_enable_renaming");
        }

        println!("cargo:rustc-link-search={lib_path}");
        println!("cargo:rustc-link-lib={kind}=icuuc");
        println!("cargo:rustc-link-lib={kind}=icui18n");
        println!("cargo:rustc-link-lib={kind}=icudata");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rerun-if-env-changed=ICU4C_LINK_STATICALLY");
        println!("cargo:rerun-if-env-changed=ICU4C_LIB_PATH");
        println!("cargo:rerun-if-env-changed=ICU4C_RENAME_VERSION");
    }
}
