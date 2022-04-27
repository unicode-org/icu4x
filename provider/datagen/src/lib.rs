// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen` contains command-line tools to generate and process ICU4X data.
//!
//! The tools include:
//!
//! * `icu4x-datagen`: Read source data (CLDR JSON, uprops files) and dump ICU4X-format data.
//! * `icu4x-key-extract`: Extract `ResourceKey` objects present in a compiled executable.
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
//!     --cldr-tag 41.0.0 \
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
//! $ cargo build --example work_log --release --features serde
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
//!     --cldr-tag 41.0.0 \
//!     --key-file /tmp/icu4x_data/work_log+keys.txt \
//!     --locales es \
//!     --locales de \
//!     --out /tmp/icu4x_data/work_log_json
//! ```

pub mod cldr;
pub mod error;
#[cfg(feature = "experimental")]
pub mod segmenter;
pub mod uprops;

use icu_provider::prelude::*;
use std::path::Path;
use std::path::PathBuf;

/// List of all supported keys
pub fn get_all_keys() -> Vec<ResourceKey> {
    let mut v = vec![];
    v.extend(cldr::ALL_KEYS);
    v.extend(icu_properties::provider::key::ALL_MAP_KEYS);
    v.extend(icu_properties::provider::key::ALL_SCRIPT_EXTENSIONS_KEYS);
    v.extend(icu_properties::provider::key::ALL_SET_KEYS);
    #[cfg(feature = "experimental")]
    {
        v.extend(icu_segmenter::ALL_KEYS);
        v.push(icu_casemapping::provider::CaseMappingV1Marker::KEY);
    }
    v
}

/// Bag of options for datagen source data.
#[derive(Clone, Debug, Default)]
pub struct SourceData {
    cldr_paths: Option<cldr::CldrPaths>,
    uprops_root: Option<PathBuf>,
}

impl SourceData {
    /// Adds CLDR data to this `DataSource`. The `CldrPaths` should point to
    /// a local `cldr-{version}-json-full.zip` directory (see
    /// <https://github.com/unicode-org/cldr-json/releases>).
    pub fn with_cldr(self, cldr_paths: cldr::CldrPaths) -> Self {
        Self {
            cldr_paths: Some(cldr_paths),
            uprops_root: self.uprops_root,
        }
    }

    /// Adds Unicode Properties data to this `DataSource`. The path should
    /// point to a local `icuexportdata_uprops_full` directory (see
    /// <https://github.com/unicode-org/icu/releases>).
    pub fn with_uprops(self, uprops_root: PathBuf) -> Self {
        Self {
            cldr_paths: self.cldr_paths,
            uprops_root: Some(uprops_root),
        }
    }

    /// Create `SourceData` pointing to test data.
    pub fn for_test() -> Self {
        Self::default()
            .with_cldr(cldr::CldrPaths {
                cldr_json_root: icu_testdata::paths::cldr_json_root(),
                locale_subset: "full".to_string(),
            })
            .with_uprops(icu_testdata::paths::uprops_toml_root())
    }

    /// Paths to CLDR source data.
    pub fn get_cldr_paths(&self) -> Result<&cldr::CldrPaths, DataError> {
        Ok(self
            .cldr_paths
            .as_ref()
            .ok_or(error::DatagenError::MissingCldrPaths)?)
    }

    /// Path to Unicode Properties source data.
    pub fn get_uprops_root(&self) -> Result<&Path, DataError> {
        Ok(self
            .uprops_root
            .as_deref()
            .ok_or(error::DatagenError::MissingUpropsPath)?)
    }

    /// Path to segmenter data.
    pub fn get_segmenter_data_root(&self) -> Result<PathBuf, DataError> {
        Ok(PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data"))
    }
}

/// Create a data provider reading from source files that generates data for all,
/// or a subset, of ICU4X.
///
/// The macro behaves like a function that takes the following arguments:
///
/// 1. An instance of [`SourceData`] (required)
/// 2. A list of providers to instantiate (optional)
///
/// The return value is a complex type that implements all of the key data provider traits.
///
/// To create a data provider for all of ICU4X:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports all keys required by ICU4X.
/// let provider = icu_datagen::create_datagen_provider!(SourceData::for_test());
/// ```
///
/// To create a data provider for a subset:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports the keys for LocaleCanonicalizer.
/// let provider = icu_datagen::create_datagen_provider!(SourceData::for_test(),
/// [
///     icu_datagen::cldr::AliasesProvider,
///     icu_datagen::cldr::LikelySubtagsProvider,
/// ]);
/// ```
#[macro_export]
#[cfg(not(feature = "experimental"))]
macro_rules! create_datagen_provider {
    ($source_data:expr) => {
        $crate::create_datagen_provider!(
            $source_data,
            [
                $crate::cldr::AliasesProvider,
                $crate::cldr::CommonDateProvider,
                $crate::cldr::JapaneseErasProvider,
                $crate::cldr::LikelySubtagsProvider,
                $crate::cldr::NumbersProvider,
                $crate::cldr::PluralsProvider,
                $crate::cldr::TimeZonesProvider,
                $crate::cldr::WeekDataProvider,
                $crate::cldr::ListProvider,
                $crate::uprops::EnumeratedPropertyCodePointTrieProvider,
                $crate::uprops::ScriptWithExtensionsPropertyProvider,
                $crate::uprops::EnumeratedPropertyUnicodeSetDataProvider,
                // Has to go last as it matches all props/ keys.
                $crate::uprops::BinaryPropertyUnicodeSetDataProvider,
            ]
        )
    };
    ($source_data:expr, [ $($constructor:path),+, ]) => {{
        let __source = $source_data;
        icu_provider_adapters::make_forking_provider!(
            icu_provider_adapters::fork::by_key::ForkByKeyProvider,
            [
                $(<$constructor>::from(&__source)),+,
            ]
        )
    }};
}

/// Create a data provider reading from source files that generates data for all,
/// or a subset, of ICU4X.
///
/// The macro behaves like a function that takes the following arguments:
///
/// 1. An instance of [`SourceData`] (required)
/// 2. A list of providers to instantiate (optional)
///
/// The return value is a complex type that implements all of the key data provider traits.
///
/// To create a data provider for all of ICU4X:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports all keys required by ICU4X.
/// let provider = icu_datagen::create_datagen_provider!(SourceData::for_test());
/// ```
///
/// To create a data provider for a subset:
///
/// ```no_run
/// use icu_datagen::SourceData;
///
/// // This data provider supports the keys for LocaleCanonicalizer.
/// let provider = icu_datagen::create_datagen_provider!(SourceData::for_test(), [
///     icu_datagen::cldr::AliasesProvider,
///     icu_datagen::cldr::LikelySubtagsProvider,
/// ]);
/// ```
#[macro_export]
#[cfg(feature = "experimental")]
macro_rules! create_datagen_provider {
    ($source_data:expr) => {
        $crate::create_datagen_provider!(
            $source_data,
            [
                $crate::cldr::AliasesProvider,
                $crate::cldr::CommonDateProvider,
                $crate::cldr::JapaneseErasProvider,
                $crate::cldr::LikelySubtagsProvider,
                $crate::cldr::NumbersProvider,
                $crate::cldr::PluralsProvider,
                $crate::cldr::TimeZonesProvider,
                $crate::cldr::WeekDataProvider,
                $crate::cldr::ListProvider,
                $crate::uprops::CaseMappingDataProvider,
                $crate::uprops::EnumeratedPropertyCodePointTrieProvider,
                $crate::uprops::ScriptWithExtensionsPropertyProvider,
                $crate::uprops::EnumeratedPropertyUnicodeSetDataProvider,
                // Has to go last as it matches all props/ keys.
                $crate::uprops::BinaryPropertyUnicodeSetDataProvider,
                $crate::segmenter::SegmenterRuleProvider,
            ]
        )
    };
    ($source_data:expr, [ $($constructor:path),+, ]) => {{
        let __source = $source_data;
        icu_provider_adapters::make_forking_provider!(
            icu_provider_adapters::fork::by_key::ForkByKeyProvider,
            [
                icu_provider::hello_world::HelloWorldProvider::new_with_placeholder_data(),

                $(<$constructor>::from(&__source)),+,
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
