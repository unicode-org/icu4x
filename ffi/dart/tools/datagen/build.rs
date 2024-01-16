use icu_datagen::prelude::*;
use icu_provider_blob::export::BlobExporter;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    DatagenDriver::new()
        .with_keys(icu_datagen::all_keys())
        .with_all_locales()
        .export(
            &DatagenProvider::new_latest_tested(),
            BlobExporter::new_with_sink(Box::new(
                File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("all.blob")).unwrap(),
            )),
        )
        .unwrap();
}
