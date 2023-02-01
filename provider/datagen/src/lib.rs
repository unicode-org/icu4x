// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::needless_doctest_main)]
//! `icu_datagen` is a library to generate data files that can be used in ICU4X data providers.
//!
//! Data files can be generated either programmatically (i.e. in `build.rs`), or through a
//! command-line utility.
//!
//! # Examples
//!
//! ## `build.rs`
//!
//! ```no_run
//! use icu_datagen::prelude::*;
//! use std::fs::File;
//!
//! fn main() {
//!     icu_datagen::datagen(
//!         Some(&[langid!("de"), langid!("en-AU")]),
//!         &[icu::list::provider::AndListV1Marker::KEY],
//!         &SourceData::default(),
//!         vec![Out::Blob(Box::new(File::create("data.postcard").unwrap()))],
//!     )
//!     .unwrap();
//! }
//! ```
//!
//! ## Command line
//!
//! The command line interface can be installed with the `bin` Cargo feature.
//!
//! ```bash
//! $ cargo install icu4x-datagen
//! ```
//!
//! Once the tool is installed, you can invoke it like this:
//!
//! ```bash
//! $ icu4x-datagen \
//! >    --keys all \
//! >    --locales de en-AU \
//! >    --format blob \
//! >    --out data.postcard
//! ```

//! More details can be found by running `--help`.

#![cfg_attr(
    not(test),
    deny(
        // This is a tool, and as such we don't care about panics too much
        // clippy::indexing_slicing,
        // clippy::unwrap_used,
        // clippy::expect_used,
        // clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

mod databake;
mod error;
mod registry;
mod source;
#[cfg(test)]
mod testutil;
mod transform;

pub use error::{is_missing_cldr_error, is_missing_icuexport_error};
pub use registry::{all_keys, all_keys_with_experimental};
pub use source::{CollationHanDatabase, CoverageLevel, SourceData};

#[allow(clippy::exhaustive_enums)] // exists for backwards compatibility
#[doc(hidden)]
pub enum CldrLocaleSubset {
    Ignored,
}

impl CldrLocaleSubset {
    #[allow(non_upper_case_globals)]
    pub const Full: Self = Self::Ignored;
    #[allow(non_upper_case_globals)]
    pub const Modern: Self = Self::Ignored;
}

/// [Out::Fs] serialization formats.
pub mod syntax {
    pub use icu_provider_fs::export::serializers::bincode::Serializer as Bincode;
    pub use icu_provider_fs::export::serializers::json::Serializer as Json;
    pub use icu_provider_fs::export::serializers::postcard::Serializer as Postcard;
}

/// A prelude for using the datagen API
pub mod prelude {
    pub use super::{
        syntax, CldrLocaleSubset, CollationHanDatabase, CoverageLevel, Out, SourceData,
    };
    pub use icu_locid::{langid, LanguageIdentifier};
    pub use icu_provider::KeyedDataMarker;
}

use icu_provider::datagen::*;
use icu_provider::prelude::*;
use icu_provider_adapters::empty::EmptyDataProvider;
use icu_provider_adapters::filter::Filterable;
use icu_provider_fs::export::serializers::AbstractSerializer;
use prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// [`DataProvider`] backed by [`SourceData`]
#[allow(clippy::exhaustive_structs)] // any information will be added to SourceData
#[derive(Debug, Clone)]
pub struct DatagenProvider {
    /// The underlying raw data
    pub source: SourceData,
}

#[cfg(test)]
impl DatagenProvider {
    /// Create a `DatagenProvider` that uses test data.
    pub fn for_test() -> Self {
        lazy_static::lazy_static! {
            static ref TEST_PROVIDER: DatagenProvider = DatagenProvider {
                source: SourceData::for_test(),
            };
        }
        TEST_PROVIDER.clone()
    }
}

impl AnyProvider for DatagenProvider {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        self.as_any_provider().load_any(key, req)
    }
}

/// Parses a human-readable key identifier into a [`DataKey`].
//  Supports the hello world key
/// # Example
/// ```
/// # use icu_provider::KeyedDataMarker;
/// assert_eq!(
///     icu_datagen::key("list/and@1"),
///     Some(icu::list::provider::AndListV1Marker::KEY),
/// );
/// ```
pub fn key<S: AsRef<str>>(string: S) -> Option<DataKey> {
    lazy_static::lazy_static! {
        static ref LOOKUP: std::collections::HashMap<&'static str, DataKey> = all_keys_with_experimental()
                    .into_iter()
                    .chain(std::iter::once(
                        icu_provider::hello_world::HelloWorldV1Marker::KEY,
                    ))
                    .map(|k| (k.path().get(), k))
                    .collect();
    }
    LOOKUP.get(string.as_ref()).copied()
}

/// Parses a list of human-readable key identifiers and returns a
/// list of [`DataKey`]s.
///
/// Unknown key names are ignored.
//  Supports the hello world key
/// # Example
/// ```
/// # use icu_provider::KeyedDataMarker;
/// assert_eq!(
///     icu_datagen::keys(&["list/and@1", "list/or@1"]),
///     vec![
///         icu::list::provider::AndListV1Marker::KEY,
///         icu::list::provider::OrListV1Marker::KEY,
///     ],
/// );
/// ```
pub fn keys<S: AsRef<str>>(strings: &[S]) -> Vec<DataKey> {
    strings.iter().filter_map(crate::key).collect()
}

/// Parses a file of human-readable key identifiers and returns a
/// list of [`DataKey`]s.
///
/// Unknown key names are ignored.
//  Supports the hello world key
/// # Example
///
/// #### keys.txt
/// ```text
/// list/and@1
/// list/or@1
/// ```
/// #### build.rs
/// ```no_run
/// # use icu_provider::KeyedDataMarker;
/// # use std::fs::File;
/// # fn main() -> std::io::Result<()> {
/// assert_eq!(
///     icu_datagen::keys_from_file("keys.txt")?,
///     vec![
///         icu::list::provider::AndListV1Marker::KEY,
///         icu::list::provider::OrListV1Marker::KEY,
///     ],
/// );
/// # Ok(())
/// # }
/// ```
pub fn keys_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<DataKey>> {
    BufReader::new(std::fs::File::open(path.as_ref())?)
        .lines()
        .filter_map(|k| k.map(crate::key).transpose())
        .collect()
}

/// Parses a compiled binary and returns a list of [`DataKey`]s used by it.
///
/// Unknown key names are ignored.
//  Supports the hello world key
/// # Example
///
/// #### build.rs
/// ```no_run
/// # use icu_provider::KeyedDataMarker;
/// # use std::fs::File;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert_eq!(
///     icu_datagen::keys_from_bin("target/release/my-app")?,
///     vec![
///         icu::list::provider::AndListV1Marker::KEY,
///         icu::list::provider::OrListV1Marker::KEY,
///     ],
/// );
/// # Ok(())
/// # }
/// ```
pub fn keys_from_bin<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<DataKey>> {
    let file = std::fs::read(path.as_ref())?;
    let mut result = Vec::new();
    let mut i = 0;
    let mut last_start = None;
    while i < file.len() {
        if file[i..].starts_with(icu_provider::leading_tag!().as_bytes()) {
            i += icu_provider::leading_tag!().len();
            last_start = Some(i);
        } else if file[i..].starts_with(icu_provider::trailing_tag!().as_bytes())
            && last_start.is_some()
        {
            if let Some(key) = std::str::from_utf8(&file[last_start.unwrap()..i])
                .ok()
                .and_then(crate::key)
            {
                result.push(key);
            }
            i += icu_provider::trailing_tag!().len();
            last_start = None;
        } else {
            i += 1;
        }
    }
    result.sort();
    result.dedup();
    Ok(result)
}

/// The output format.
#[non_exhaustive]
pub enum Out {
    /// Output to a file system tree
    Fs {
        /// The root path.
        output_path: PathBuf,
        /// The serialization format. See [syntax].
        serializer: Box<dyn AbstractSerializer + Sync>,
        /// Whether to overwrite existing data.
        overwrite: bool,
        /// Whether to create a fingerprint file with SHA2 hashes
        fingerprint: bool,
    },
    /// Output as a postcard blob to the given sink.
    Blob(Box<dyn std::io::Write + Sync>),
    /// Output a module at the given location.
    Module {
        /// The directory of the generated module.
        mod_directory: PathBuf,
        /// Whether to run `rustfmt` on the generated files.
        pretty: bool,
        /// Whether to gate each key on its crate name. This allows using the module
        /// even if some keys are not required and their dependencies are not included.
        /// Requires use_separate_crates.
        insert_feature_gates: bool,
        /// Whether to use separate crates to name types instead of the `icu` metacrate
        use_separate_crates: bool,
    },
}

/// Runs ICU4X datagen.
///
/// The argument are used as follows:
/// * `locales`: If this is present, only locales that are either `und` or
///   contained (strictly, i.e. `en` != `en-US`) in the slice will be generated.
///   Otherwise, all locales supported by the source data will be generated.
/// * `keys`: The keys for which to generate data. See [`all_keys`], [`keys`], [`keys_from_file`], [`keys_from_bin`].
/// * `sources`: The underlying source data. CLDR and/or ICU data can be missing if no
///   requested key requires them, otherwise an error satisfying [`is_missing_cldr_error`]
///   or [`is_missing_icuexport_error`] will be returned.
/// * `out`: The output format and location. See the documentation on [`Out`]
pub fn datagen(
    locales: Option<&[LanguageIdentifier]>,
    keys: &[DataKey],
    source: &SourceData,
    outs: Vec<Out>,
) -> Result<(), DataError> {
    let exporters = outs
        .into_iter()
        .map(|out| -> Result<Box<dyn DataExporter>, DataError> {
            Ok(match out {
                Out::Fs {
                    output_path,
                    serializer,
                    overwrite,
                    fingerprint,
                } => {
                    let mut options = icu_provider_fs::export::ExporterOptions::default();
                    options.root = output_path;
                    if overwrite {
                        options.overwrite =
                            icu_provider_fs::export::OverwriteOption::RemoveAndReplace
                    }
                    options.fingerprint = fingerprint;
                    Box::new(icu_provider_fs::export::FilesystemExporter::try_new(
                        serializer, options,
                    )?)
                }
                Out::Blob(write) => Box::new(
                    icu_provider_blob::export::BlobExporter::new_with_sink(write),
                ),
                Out::Module {
                    mod_directory,
                    pretty,
                    insert_feature_gates,
                    use_separate_crates,
                } => Box::new(databake::BakedDataExporter::new(
                    mod_directory,
                    pretty,
                    insert_feature_gates,
                    use_separate_crates,
                )?),
            })
        })
        .collect::<Result<Vec<_>, DataError>>()?;

    let provider: Box<dyn ExportableProvider> = match locales {
        Some(&[]) => Box::new(EmptyDataProvider::default()),
        Some(locales) => Box::new(
            DatagenProvider {
                source: source.clone(),
            }
            .filterable("icu4x-datagen locales")
            .filter_by_langid(move |lid| lid.language.is_empty() || locales.contains(lid)),
        ),
        None => Box::new(DatagenProvider {
            source: source.clone(),
        }),
    };

    let keys: HashSet<_> = keys.iter().collect();

    keys.into_par_iter().try_for_each(|&key| {
        let locales = provider
            .supported_locales_for_key(key)
            .map_err(|e| e.with_key(key))?;
        let res = locales.into_par_iter().try_for_each(|locale| {
            let req = DataRequest {
                locale: &locale,
                metadata: Default::default(),
            };
            let payload = provider
                .load_data(key, req)
                .and_then(DataResponse::take_payload)
                .map_err(|e| e.with_req(key, req))?;
            exporters.par_iter().try_for_each(|e| {
                e.put_payload(key, &locale, &payload)
                    .map_err(|e| e.with_req(key, req))
            })
        });

        log::info!("Writing key: {}", key);
        for e in &exporters {
            e.flush(key).map_err(|e| e.with_key(key))?;
        }

        res
    })?;

    for mut e in exporters {
        e.close()?;
    }

    Ok(())
}

#[test]
fn test_keys() {
    assert_eq!(
        keys(&[
            "list/and@1",
            "datetime/gregory/datelengths@1",
            "decimal/symbols@1",
            "trash",
        ]),
        vec![
            icu_list::provider::AndListV1Marker::KEY,
            icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::KEY,
            icu_decimal::provider::DecimalSymbolsV1Marker::KEY,
        ]
    );
}

#[test]
fn test_keys_from_file() {
    assert_eq!(
        keys_from_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/work_log+keys.txt")
        )
        .unwrap(),
        vec![
            icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::KEY,
            icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker::KEY,
            icu_datetime::provider::calendar::TimeSymbolsV1Marker::KEY,
            icu_calendar::provider::WeekDataV1Marker::KEY,
            icu_decimal::provider::DecimalSymbolsV1Marker::KEY,
            icu_plurals::provider::OrdinalV1Marker::KEY,
        ]
    );
}

#[test]
fn test_keys_from_bin() {
    // File obtained by changing work_log.rs to use `try_new_with_buffer_provider` & `icu_testdata::small_buffer`
    // and running `cargo +nightly-2022-04-05 wasm-build-release --examples -p icu_datetime --features serde \
    // && cp target/wasm32-unknown-unknown/release-opt-size/examples/work_log.wasm provider/datagen/tests/data/`
    assert_eq!(
        keys_from_bin(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/work_log.wasm"))
            .unwrap(),
        vec![
            icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::KEY,
            icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker::KEY,
            icu_datetime::provider::calendar::TimeLengthsV1Marker::KEY,
            icu_datetime::provider::calendar::TimeSymbolsV1Marker::KEY,
            icu_calendar::provider::WeekDataV1Marker::KEY,
            icu_decimal::provider::DecimalSymbolsV1Marker::KEY,
            icu_plurals::provider::OrdinalV1Marker::KEY,
        ]
    );
}
