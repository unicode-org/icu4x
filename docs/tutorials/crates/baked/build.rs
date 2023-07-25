// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::prelude::*;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mod_directory = PathBuf::from(out_dir).join("baked_data");

    let mut options = BakedOptions::default();
    // Overwrite the baked data if it was already present:
    options.overwrite = true;
    options.use_separate_crates = false;


    icu_datagen::datagen(
        Some(&[langid!("ru")]),
        // Note: These are the keys required by `PluralRules::try_new_cardinal_unstable`
        &[icu::plurals::provider::CardinalV1Marker::KEY],
        &SourceData::default()
            .with_cldr_for_tag(SourceData::LATEST_TESTED_CLDR_TAG, CldrLocaleSubset::Modern)
            .expect("Infallible"),
        vec![icu_datagen::Out::Baked {
            mod_directory,
            options,
        }],
    )
    .expect("Datagen should be successful");
}
