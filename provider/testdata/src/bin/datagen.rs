// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::{Out, SourceData};
use icu_locid::langid;
use icu_provider_fs::export::serializers::json;
use icu_testdata::{metadata, paths};
use std::fs::File;

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let source_data = SourceData::default()
        .with_cldr(paths::cldr_json_root(), "full".to_string())
        .with_uprops(paths::uprops_toml_root());
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
        mod_directory: paths::data_root().join("baked/src/data"),
        pretty: false,
        insert_feature_gates: false,
    };

    for out in [json_out, blob_out, mod_out] {
        icu_datagen::datagen(
            Some(&locales),
            &icu_datagen::get_all_keys(),
            &source_data,
            out,
            true,
        )
        .unwrap();
    }

    icu_datagen::datagen(
        Some(&[langid!("en"), langid!("bn")]),
        &icu_datagen::keys(&["decimal/symbols@1"]),
        &source_data,
        Out::Blob(Box::new(
            File::create(paths::data_root().join("decimal-bn-en.postcard")).unwrap(),
        )),
        true,
    )
    .unwrap();
}
