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
//!     --cldr-tag 39.0.0 \
//!     --key-file /tmp/icu4x_data/work_log+keys.txt \
//!     --locales es \
//!     --locales de \
//!     --out /tmp/icu4x_data/work_log_json
//! ```

pub mod cldr;
#[cfg(feature = "experimental")]
pub mod segmenter;
pub mod uprops;

use std::path::PathBuf;

use icu_provider::ResourceKey;

/// List of all supported keys
pub fn get_all_keys() -> Vec<ResourceKey> {
    let mut v = vec![];
    v.extend(cldr::ALL_KEYS);
    v.extend(icu_properties::provider::key::ALL_MAP_KEYS);
    v.extend(icu_properties::provider::key::ALL_SCRIPT_EXTENSIONS_KEYS);
    v.extend(icu_properties::provider::key::ALL_SET_KEYS);
    #[cfg(feature = "experimental")]
    {
        use icu_provider::ResourceMarker;
        v.extend(icu_segmenter::ALL_KEYS);
        v.push(icu_casemapping::provider::CaseMappingV1Marker::KEY);
    }
    v
}

/// Options for creating a datagen provider.
pub struct DatagenOptions {
    /// Paths to CLDR source data.
    ///
    /// If `None`, providers that need CLDR source data cannot be constructed.
    pub cldr_paths: Option<Box<dyn cldr::CldrPaths>>,

    /// Path to Unicode Properties source data.
    ///
    /// If `None`, providers that need Unicode Properties source data cannot be constructed.
    pub uprops_root: Option<PathBuf>,

    #[cfg(feature = "experimental")]
    /// Path to segmentation source data.
    ///
    /// If `None`, providers that need segmentation source data cannot be constructed.
    pub segmenter_data_root: Option<PathBuf>,
}

impl DatagenOptions {
    /// Create a DatagenOptions pointing to test data.
    pub fn for_test() -> Self {
        Self {
            cldr_paths: Some(Box::new(cldr::CldrPathsAllInOne {
                cldr_json_root: icu_testdata::paths::cldr_json_root(),
                locale_subset: "full".to_string(),
            })),
            uprops_root: Some(icu_testdata::paths::uprops_toml_root()),
            #[cfg(feature = "experimental")]
            segmenter_data_root: Some(segmenter::segmenter_data_root()),
        }
    }
}

/// Create a data provider reading from source files that generates data for all,
/// or a subset, of ICU4X.
///
/// The macro behaves like a function that takes the following arguments:
///
/// 1. An instance of [`DatagenOptions`] (required)
/// 2. A list of providers to instantiate (optional)
///
/// The return value is a complex type that implements all of the key data provider traits.
///
/// The macro expands to code that contains `?` operators. It is recommended to invoke this
/// macro from a function that returns an `eyre::Result`.
///
/// To create a data provider for all of ICU4X:
///
/// ```no_run
/// use icu_datagen::DatagenOptions;
///
/// // This data provider supports all keys required by ICU4X.
/// let provider = icu_datagen::create_datagen_provider!(DatagenOptions::for_test());
/// # Ok::<(), eyre::ErrReport>(())
/// ```
///
/// To create a data provider for a subset:
///
/// ```no_run
/// use icu_datagen::DatagenOptions;
///
/// // This data provider supports the keys for LocaleCanonicalizer.
/// let provider = icu_datagen::create_datagen_provider!(DatagenOptions::for_test(), [
///     icu_datagen::cldr::AliasesProvider,
///     icu_datagen::cldr::LikelySubtagsProvider,
/// ]);
/// # Ok::<(), eyre::ErrReport>(())
/// ```
#[macro_export]
#[cfg(not(feature = "experimental"))]
macro_rules! create_datagen_provider {
    ($datagen_options:expr) => {
        $crate::create_datagen_provider!(
            $datagen_options,
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
    ($datagen_options:expr, [ $($constructor:path),+, ]) => {{
        use core::convert::TryFrom;
        icu_provider_adapters::make_forking_provider!(
            icu_provider_adapters::fork::by_key::ForkByKeyProvider,
            [
                $(<$constructor>::try_from(&$datagen_options)?),+,
            ]
        )
    }};
}

/// Create a data provider reading from source files that generates data for all,
/// or a subset, of ICU4X.
///
/// The macro behaves like a function that takes the following arguments:
///
/// 1. An instance of [`DatagenOptions`] (required)
/// 2. A list of providers to instantiate (optional)
///
/// The return value is a complex type that implements all of the key data provider traits.
///
/// The macro expands to code that contains `?` operators. It is recommended to invoke this
/// macro from a function that returns an `eyre::Result`.
///
/// To create a data provider for all of ICU4X:
///
/// ```no_run
/// use icu_datagen::DatagenOptions;
///
/// // This data provider supports all keys required by ICU4X.
/// let provider = icu_datagen::create_datagen_provider!(DatagenOptions::for_test());
/// # Ok::<(), eyre::ErrReport>(())
/// ```
///
/// To create a data provider for a subset:
///
/// ```no_run
/// use icu_datagen::DatagenOptions;
///
/// // This data provider supports the keys for LocaleCanonicalizer.
/// let provider = icu_datagen::create_datagen_provider!(DatagenOptions::for_test(), [
///     icu_datagen::cldr::AliasesProvider,
///     icu_datagen::cldr::LikelySubtagsProvider,
/// ]);
/// # Ok::<(), eyre::ErrReport>(())
/// ```
#[macro_export]
#[cfg(feature = "experimental")]
macro_rules! create_datagen_provider {
    ($datagen_options:expr) => {
        $crate::create_datagen_provider!(
            $datagen_options,
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
    ($datagen_options:expr, [ $($constructor:path),+, ]) => {{
        use core::convert::TryFrom;
        icu_provider_adapters::make_forking_provider!(
            icu_provider_adapters::fork::by_key::ForkByKeyProvider,
            [
                $(<$constructor>::try_from(&$datagen_options)?),+,
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
