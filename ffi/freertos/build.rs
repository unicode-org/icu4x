// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::env;

/// Returns whether the Rust compiler needs an `#[alloc_error_handler]`
/// set. Returns None for cases where we cannot determine the nightly version of the
/// compiler.
fn needs_alloc_error_handler() -> Option<bool> {
    use rustc_version::Channel;
    let version = rustc_version::version_meta().ok()?;
    if version.channel != Channel::Nightly {
        // Ignore custom/dev toolchains, commit date
        // may not be meaningful
        return None;
    }
    let commit_date = version.commit_date?;

    let year = commit_date.split('-').next()?.parse::<u32>().ok()?;

    // alloc_error_handler became defaulted to the panic handler
    // in December 2022. Since it still works until April 2023,
    // we can be fuzzy with our dates and just set the boundary at
    // 2022/2023
    Some(year <= 2022)
}

fn main() {
    match env::var("CARGO_CFG_TARGET_OS") {
        Ok(v) if v == "none" => (),
        // Only on target_os = none
        _ => return,
    };

    if let Some(true) = needs_alloc_error_handler() {
        println!("cargo:rustc-cfg=needs_alloc_error_handler");
    }

    // For unknown compilers, assume that the nightly is recent.
}
