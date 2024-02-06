// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::prelude::*;
use icu_provider_blob::export::BlobExporter;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    DatagenDriver::new()
        .with_keys(if std::env::var("PROFILE").unwrap() == "release" {
            icu_datagen::all_keys()
        } else {
            Default::default()
        })
        .with_all_locales()
        .export(
            &DatagenProvider::new_latest_tested(),
            BlobExporter::new_with_sink(Box::new(
                File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("all.blob")).unwrap(),
            )),
        )
        .unwrap();
}
