// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::Path;

fn main() -> std::io::Result<()> {
    let capi = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../ffi/capi"));

    let Some(lang) = std::env::args().nth(1) else {
        panic!("Missing argument <language>");
    };

    diplomat_tool::gen(
        &capi.join("src/lib.rs"),
        &lang,
        &{
            let include = capi.join(&lang).join(match lang.as_str() {
                "js" => "package/lib",
                "dart" => "package/lib/src",
                _ => "include",
            });
            std::fs::remove_dir_all(&include)?;
            std::fs::create_dir(&include)?;
            include
        },
        None,
        &Default::default(),
        None,
        false,
        Some("ICU4X".into()),
    )
}
