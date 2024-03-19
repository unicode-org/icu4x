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
        .with_locales([langid!("ru")])
        // These are the keys required by `PluralRules::try_new_cardinal_unstable`. Compilation will
        // discard unused keys and fail if required keys are not generated, but explicitly listing the
        // keys will speed up the datagen.
        .with_keys([icu::plurals::provider::CardinalV1Marker::KEY])
        .with_fallback_mode(FallbackMode::RuntimeManual)
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
