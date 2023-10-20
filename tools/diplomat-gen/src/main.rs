// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::Path;

fn main() {
    let capi = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../ffi/capi"));

    let lang = std::env::args().nth(1).unwrap();

    let include = capi.join(&lang).join("include");
    let docs = capi.join(&lang).join("docs/source");

    std::fs::remove_dir_all(&include).unwrap();
    std::fs::create_dir(&include).unwrap();

    if lang != "c" {
        let conf = std::fs::read_to_string(docs.join("conf.py")).unwrap();
        std::fs::remove_dir_all(&docs).unwrap();
        std::fs::create_dir(&docs).unwrap();
        std::fs::write(docs.join("conf.py"), conf).unwrap();
    }

    diplomat_tool::gen(
        &capi.join("src/lib.rs"),
        &lang,
        &include,
        (lang != "c").then_some(&docs),
        &Default::default(),
        None,
        false,
    )
    .unwrap()
}
