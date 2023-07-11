// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::prelude::*;
use std::fs::File;
use std::path::Path;

include!("../../locales.rs.data");

fn main() {
    #![allow(deprecated)] // want to keep old datagen code path covered

    simple_logger::SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let data_root = Path::new(concat!(
        core::env!("CARGO_MANIFEST_DIR"),
        "/../../provider/datagen/tests/data/"
    ));

    let source = SourceData::offline()
        .with_cldr(data_root.join("cldr"), Default::default())
        .unwrap()
        .with_icuexport(data_root.join("icuexport"))
        .unwrap()
        .with_segmenter_lstm(data_root.join("lstm"))
        .unwrap();

    let json_out = Out::Fs {
        output_path: data_root.join("json"),
        serializer: Box::new(syntax::Json::pretty()),
        overwrite: true,
        fingerprint: true,
    };

    let postcard_out = Out::Fs {
        output_path: data_root.join("postcard"),
        serializer: Box::<syntax::Postcard>::default(),
        overwrite: true,
        fingerprint: true,
    };

    let blob_out = Out::Blob(Box::new(
        File::create(data_root.join("testdata.postcard")).unwrap(),
    ));

    let mut options = BakedOptions::default();
    options.insert_feature_gates = true;
    options.use_separate_crates = true;
    options.overwrite = true;
    options.pretty = true;
    let mod_out = Out::Baked {
        mod_directory: Path::new("/dev/null/").into(),
        options,
    };

    icu_datagen::datagen(
        Some(LOCALES),
        &icu_datagen::all_keys_with_experimental()
            .into_iter()
            .chain([icu_provider::hello_world::HelloWorldV1Marker::KEY])
            .collect::<Vec<_>>(),
        &source,
        vec![json_out, blob_out, mod_out, postcard_out],
    )
    .unwrap();
}
