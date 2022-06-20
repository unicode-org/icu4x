// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::*;
use icu_locid::langid;
use icu_provider_fs::export::serializers::json;
use icu_testdata::{metadata, paths};
use std::fs::File;

// icuexport test data isn't complete, so we don't test these keys.
const IGNORED_KEYS: &[&str] = &[
    "props/alnum@1",
    "props/blank@1",
    "props/Comp_Ex@1",
    "props/CWCM@1",
    "props/Gr_Link@1",
    "props/graph@1",
    "props/Hyphen@1",
    "props/nfcinert@1",
    "props/nfdinert@1",
    "props/nfkcinert@1",
    "props/nfkdinert@1",
    "props/PCM@1",
    "props/print@1",
    "props/segstart@1",
    "props/Sensitive@1",
    "props/xdigit@1",
];

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let source_data = SourceData::default()
        .with_cldr(paths::cldr_json_root(), CldrLocaleSubset::Full)
        .unwrap()
        .with_icuexport(paths::icuexport_toml_root(), IcuTrieType::Small)
        .unwrap();
    let locales = metadata::load().unwrap().package_metadata.locales;

    let json_out = Out::Fs {
        output_path: paths::data_root().join("json"),
        serializer: Box::new(json::Serializer::pretty()),
        overwrite: true,
    };

    let blob_out = Out::Blob(Box::new(
        File::create(paths::data_root().join("testdata.postcard")).unwrap(),
    ));

    let mod_out = icu_datagen::Out::Module {
        mod_directory: paths::data_root().join("baked"),
        pretty: true,
        insert_feature_gates: false,
    };

    icu_datagen::datagen(
        Some(&locales),
        &icu_datagen::all_keys()
            .into_iter()
            .filter(|k| !IGNORED_KEYS.contains(&k.get_path()))
            .collect::<Vec<_>>(),
        &source_data,
        vec![json_out, blob_out, mod_out],
    )
    .unwrap();

    icu_datagen::datagen(
        Some(&[langid!("en"), langid!("bn")]),
        &icu_datagen::keys(&["decimal/symbols@1"]),
        &source_data,
        vec![Out::Blob(Box::new(
            File::create(paths::data_root().join("decimal-bn-en.postcard")).unwrap(),
        ))],
    )
    .unwrap();
}
