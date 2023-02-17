// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::prelude::*;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mod_directory = PathBuf::from(out_dir).join("baked_data");
    
    // TODO(#3046): We shouldn't need to do this since it worked in 1.0
    let _ = std::fs::remove_dir_all(&mod_directory);


    icu_datagen::datagen(
        Some(&[langid!("ru")]),
        &[icu::plurals::provider::CardinalV1Marker::KEY],
        &SourceData::default()
            .with_cldr_for_tag(SourceData::LATEST_TESTED_CLDR_TAG, CldrLocaleSubset::Modern)
            .expect("Source data should download successfully"),
        vec![icu_datagen::Out::Module {
            // For more information on these options, see:
            // <https://icu4x.unicode.org/doc/icu_datagen/enum.Out.html#variant.Module>
            mod_directory,
            pretty: false,
            insert_feature_gates: false,
            use_separate_crates: false,
        }],
    )
    .expect("Datagen should be successful");
}
