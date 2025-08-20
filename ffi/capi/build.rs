// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::env;
use std::path::PathBuf;

/// Inform cargo of the include directories as metadata key value pairs
///
/// Cargo will make the values available to consumers via `DEP_ICU_CAPI_<KEY>`.
/// See <https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key>
/// for more information.
fn add_bindings_to_cargo_metadata() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bindings_dir = PathBuf::from(manifest_dir).join("bindings");
    let c_bindings_dir = bindings_dir.join("c");
    let cpp_bindings_dir = bindings_dir.join("cpp");
    let dart_bindings_dir = bindings_dir.join("dart");
    let js_bindings_dir = bindings_dir.join("js");

    println!("cargo::metadata=c_include_dir={}", c_bindings_dir.display());
    println!(
        "cargo::metadata=cpp_include_dir={}",
        cpp_bindings_dir.display()
    );
    println!(
        "cargo::metadata=dart_include_dir={}",
        dart_bindings_dir.display()
    );
    println!(
        "cargo::metadata=js_include_dir={}",
        js_bindings_dir.display()
    );
}

fn main() {
    add_bindings_to_cargo_metadata();
}
