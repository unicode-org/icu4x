// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datagen::baked_exporter::*;
use icu_datagen::prelude::*;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mod_directory = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("baked_data");

    DatagenDriver::new()
        .with_locales_and_fallback([LocaleFamily::single(langid!("ru"))], {
            let mut options = FallbackOptions::default();
            options.runtime_fallback_location = Some(RuntimeFallbackLocation::External);
            options
        })
        // These are the markers required by `PluralRules::try_new_cardinal_unstable`. Compilation will
        // discard unused markers and fail if required markers are not generated, but explicitly listing the
        // markers will speed up the datagen.
        .with_markers([icu::plurals::provider::CardinalV1Marker::INFO])
        .export(
            &DatagenProvider::new_latest_tested(),
            BakedExporter::new(mod_directory, {
                let mut options = Options::default();
                options.overwrite = true;
                options
            })
            .unwrap(),
        )
        .expect("Datagen should be successful");
}
