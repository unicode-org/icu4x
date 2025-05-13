// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::Path;

use diplomat_tool::config::Config;

fn main() -> std::io::Result<()> {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../"));

    let Some(lang) = std::env::args().nth(1) else {
        panic!("Missing argument <language>");
    };

    let config_path = root.join("ffi/capi/config.toml");

    let mut library_config = Config::default();
    library_config.read_file(&config_path).unwrap();

    diplomat_tool::gen(
        &root.join("ffi/capi/src/lib.rs"),
        lang.as_str(),
        &{
            let include = if lang != "demo_gen" {
                root.join("ffi/capi/bindings").join(&lang)
            } else {
                root.join("tutorials/web-demo/gen")
            };
            std::fs::remove_dir_all(&include)?;
            std::fs::create_dir(&include)?;
            include
        },
        &Default::default(),
        library_config,
        false,
    )
}
