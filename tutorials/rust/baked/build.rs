// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider_export::baked_exporter::*;
use icu_provider_export::prelude::*;
use icu_provider_source::SourceDataProvider;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mod_directory = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("baked_data");

    ExportDriver::new(
        [DataLocaleFamily::single(locale!("ru").into())],
        DeduplicationStrategy::None.into(),
        // We are not deduplicating, so this can be anything. If we were using runtime fallback,
        // this would need to be the same fallbacker as used at runtime (in the baked data case,
        // the compiled data one, which we could obtain here using `try_new_unstable`).
        LocaleFallbacker::new_without_data(),
    )
    // These are the markers required by `PluralRules::try_new_cardinal_unstable`. Compilation will
    // discard unused markers and fail if required markers are not generated, but explicitly listing the
    // markers will speed up the datagen.
    .with_markers([icu::plurals::provider::CardinalV1Marker::INFO])
    .export(
        &SourceDataProvider::new_latest_tested(),
        BakedExporter::new(mod_directory, {
            let mut options = Options::default();
            options.overwrite = true;
            options.use_internal_fallback = false;
            options
        })
        .unwrap(),
    )
    .expect("Datagen should be successful");
}
