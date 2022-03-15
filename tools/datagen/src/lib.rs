// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen` contains command-line tools to generate and process ICU4X data.
//!
//! The tools include:
//!
//! 1. `icu4x-datagen`: Read source data (CLDR JSON) and dump ICU4X-format data.
//! 2. `icu4x-testdata-download`: Download fresh CLDR JSON for testdata.
//! 3. `icu4x-key-extract`: Extract `ResourceKey` objects present in a compiled executable.
//!
//! More details on each tool can be found by running `--help`.
//!
//! # Examples
//!
//! Generate ICU4X Postcard blob (single file) for all keys and all locales:
//!
//! ```bash
//! # Run from the icu4x project folder
//! $ cargo run --bin icu4x-datagen -- \
//!     --cldr-tag 39.0.0 \
//!     --all-keys \
//!     --all-locales \
//!     --format blob \
//!     --out /tmp/icu4x_data/icu4x_data.postcard
//! ```
//!
//! Extract the keys used by an executable into a key file:
//!
//! ```bash
//! # Run from the icu4x project folder
//! $ cargo build --example work_log --release
//! $ cargo make icu4x-key-extract \
//!     target/release/examples/work_log \
//!     /tmp/icu4x_data/work_log+keys.txt
//! $ cat /tmp/icu4x_data/work_log+keys.txt
//! ```
//!
//! Generate ICU4X JSON file tree from the key file for Spanish and German:
//!
//! ```bash
//! # Run from the icu4x project folder
//! $ cargo run --bin icu4x-datagen -- \
//!     --cldr-tag 39.0.0 \
//!     --key-file /tmp/icu4x_data/work_log+keys.txt \
//!     --locales es \
//!     --locales de \
//!     --out /tmp/icu4x_data/work_log_json
//! ```

use icu_provider::ResourceKey;
use icu_provider::ResourceMarker;

pub fn get_all_keys() -> Vec<ResourceKey> {
    // TODO(#1512): Use central key repository
    let mut v = vec![];
    v.extend(icu_provider_cldr::ALL_KEYS);
    v.extend(icu_properties::provider::key::ALL_MAP_KEYS);
    v.extend(icu_properties::provider::key::ALL_SCRIPT_EXTENSIONS_KEYS);
    v.extend(icu_properties::provider::key::ALL_SET_KEYS);
    v.extend(icu_segmenter::ALL_KEYS);
    v.push(icu_list::provider::AndListV1Marker::KEY);
    v.push(icu_list::provider::OrListV1Marker::KEY);
    v.push(icu_list::provider::UnitListV1Marker::KEY);
    v
}

#[test]
fn no_key_collisions() {
    let mut map = std::collections::BTreeMap::new();
    let mut failed = false;
    for key in get_all_keys() {
        if let Some(colliding_key) = map.insert(key.get_hash(), key) {
            println!(
                "{:?} and {:?} collide at {:?}",
                key.get_path(),
                colliding_key.get_path(),
                key.get_hash()
            );
            failed = true;
        }
    }
    if failed {
        panic!();
    }
}
