// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

fn main() {
    println!("cargo:rustc-check-cfg=cfg(icu4x_gio_available)");
    if std::fs::exists("/usr/include/gio/gio.h").ok() == Some(true) {
        println!("cargo:rustc-cfg=icu4x_gio_available");
    }
}
