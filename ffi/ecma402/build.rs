// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    icu_datagen::datagen(
        None,
        &[
            icu::list::provider::AndListV1Marker::KEY,
            icu::list::provider::OrListV1Marker::KEY,
            icu::list::provider::UnitListV1Marker::KEY,
            icu::plurals::provider::CardinalV1Marker::KEY,
            icu::plurals::provider::OrdinalV1Marker::KEY,
        ],
        &SourceData::default()
            .with_cldr_latest(CldrLocaleSubset::Full)
            .unwrap()
            .with_icuexport_latest()
            .unwrap(),
        vec![Out::Module {
            mod_directory: std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
                .join("baked"),
            pretty: false,
            insert_feature_gates: false,
            use_separate_crates: false,
        }],
    )
    .unwrap();
}
