// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::prelude::*;
use icu_testdata::paths;
use std::fs::File;

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let source_data = SourceData::default()
        .with_cldr(paths::cldr_json_root(), Default::default())
        .unwrap()
        .with_icuexport(paths::icuexport_toml_root())
        .unwrap();

    let json_out = Out::Fs {
        output_path: paths::data_root().join("json"),
        serializer: Box::new(syntax::Json::pretty()),
        overwrite: true,
        fingerprint: true,
    };

    let postcard_out = Out::Fs {
        output_path: paths::data_root().join("postcard"),
        serializer: Box::<syntax::Postcard>::default(),
        overwrite: true,
        fingerprint: true,
    };

    let blob_out = Out::Blob(Box::new(
        File::create(paths::data_root().join("testdata.postcard")).unwrap(),
    ));

    let mut options = BakedOptions::default();
    options.pretty = true;
    options.insert_feature_gates = true;
    options.use_separate_crates = true;
    options.overwrite = true;
    let mod_out = Out::Baked {
        mod_directory: paths::data_root().join("baked"),
        options,
    };

    icu_datagen::datagen(
        Some(&icu_testdata::locales()),
        &icu_datagen::all_keys_with_experimental()
            .into_iter()
            .chain(core::iter::once(
                icu_provider::hello_world::HelloWorldV1Marker::KEY,
            ))
            .collect::<Vec<_>>(),
        &source_data,
        vec![json_out, blob_out, mod_out, postcard_out],
    )
    .unwrap();
}
