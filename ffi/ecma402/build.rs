// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    icu_datagen::datagen(
        None,
        &icu_datagen::keys(&[
            "list/and@1",
            "list/or@1",
            "list/unit@1",
            "plurals/cardinal@1",
            "plurals/ordinal@1",
        ]),
        &icu_datagen::SourceData::default()
            .with_cldr_latest(icu_datagen::CldrLocaleSubset::Full)
            .unwrap()
            .with_icuexport_latest()
            .unwrap(),
        vec![icu_datagen::Out::Module {
            mod_directory: std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
                .join("baked"),
            pretty: false,
            insert_feature_gates: false,
        }],
    )
    .unwrap();
}
