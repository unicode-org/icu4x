// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::blob_exporter::*;
use icu_datagen::fs_exporter::serializers::*;
use icu_datagen::fs_exporter::*;
use icu_datagen::prelude::*;
use icu_provider::datagen::MultiExporter;
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
        .unwrap();

    let json_out = Box::new(
        FilesystemExporter::try_new(Box::new(Json::pretty()), {
            let mut options = ExporterOptions::default();
            options.root = data_root.join("json");
            options.overwrite = OverwriteOption::RemoveAndReplace;
            options.fingerprint = true;
            options
        })
        .unwrap(),
    );

    let postcard_out = Box::new(
        FilesystemExporter::try_new(Box::<Postcard>::default(), {
            let mut options = ExporterOptions::default();
            options.root = data_root.join("postcard");
            options.overwrite = OverwriteOption::RemoveAndReplace;
            options.fingerprint = true;
            options
        })
        .unwrap(),
    );

    let blob_out = Box::new(BlobExporter::new_with_sink(Box::new(
        File::create(data_root.join("testdata.postcard")).unwrap(),
    )));

    let mut options = options::Options::default();
    options.locales = options::LocaleInclude::Explicit(LOCALES.iter().cloned().collect());

    DatagenProvider::try_new(options, source)
        .unwrap()
        .export(
            icu_datagen::all_keys_with_experimental()
                .into_iter()
                .chain([icu_provider::hello_world::HelloWorldV1Marker::KEY])
                .collect(),
            MultiExporter::new(vec![json_out, blob_out, postcard_out]),
        )
        .unwrap();
}
