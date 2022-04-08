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

pub mod cldr;
pub mod segmenter;
pub mod uprops;



use icu_provider::ResourceKey;



/// List of all supported keys
pub fn get_all_keys() -> Vec<ResourceKey> {
    let mut v = vec![];
    v.extend(cldr::ALL_KEYS);
    v.extend(icu_properties::provider::key::ALL_MAP_KEYS);
    v.extend(icu_properties::provider::key::ALL_SCRIPT_EXTENSIONS_KEYS);
    v.extend(icu_properties::provider::key::ALL_SET_KEYS);
    v.extend(icu_segmenter::ALL_KEYS);
    v
}

#[macro_export]
macro_rules! create_omnibus_provider {
    ($cldr_paths:expr, $uprops_root:expr, $segmenter_data_root:expr) => {{
        use core::convert::TryFrom;
        icu_provider_adapters::make_forking_provider!(
            icu_provider_adapters::fork::by_key::ForkByKeyProvider,
            [
                $crate::cldr::AliasesProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::CommonDateProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::JapaneseErasProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::LikelySubtagsProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::NumbersProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::PluralsProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::TimeZonesProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::WeekDataProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths)?,
                $crate::cldr::ListProvider::try_from(&*$cldr_paths as &dyn $crate::cldr::CldrPaths, $uprops_root.clone())?,
                $crate::uprops::EnumeratedPropertyCodePointTrieProvider::try_new(&$uprops_root)?,
                $crate::uprops::ScriptWithExtensionsPropertyProvider::try_new(&$uprops_root)?,
                $crate::uprops::EnumeratedPropertyUnicodeSetDataProvider::try_new(&$uprops_root)?,
                // Has to go last as it matches all props/ keys.
                $crate::uprops::BinaryPropertyUnicodeSetDataProvider::try_new(&$uprops_root)?,
                $crate::segmenter::SegmenterRuleProvider::try_new(&$segmenter_data_root, &$uprops_root)?,
            ]
        )
    }};
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
