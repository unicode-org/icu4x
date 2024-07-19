// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::Path;

fn main() -> std::io::Result<()> {
    let capi = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../ffi/capi"));

    let Some(lang) = std::env::args().nth(1) else {
        panic!("Missing argument <language>");
    };

    diplomat_tool::gen(
        &capi.join("src/lib.rs"),
        lang.as_str(),
        &{
            let include = capi.join("bindings").join(&lang);
            std::fs::remove_dir_all(&include)?;
            std::fs::create_dir(&include)?;
            include
        },
        &Default::default(),
        None,
        false,
    )
}
