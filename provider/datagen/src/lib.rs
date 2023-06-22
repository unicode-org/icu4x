// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::needless_doctest_main)]
//! `icu_datagen` is a library to generate data files that can be used in ICU4X data providers.
//!
//! Data files can be generated either programmatically (i.e. in `build.rs`), or through a
//! command-line utility.
//!
//!
//! Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/docs/tutorials/data_management.md)
//!
//! # Examples
//!
//! ## `build.rs`
//!
//! ```no_run
//! use icu_datagen::prelude::*;
//! use icu_provider_blob::export::*;
//! use std::fs::File;
//!
//! fn main() {
//!     DatagenProvider::default()
//!         .export(
//!             [icu::list::provider::AndListV1Marker::KEY].into_iter().collect(),
//!             BlobExporter::new_with_sink(Box::new(File::create("data.postcard").unwrap())),
//!         )
//!         .unwrap();
//! }
//! ```
//!
//! ## Command line
//!
//! The command line interface can be installed through Cargo.
//!
//! ```bash
//! $ cargo install icu_datagen
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
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

mod error;
mod registry;
mod source;
#[cfg(test)]
mod testutil;
mod transform;

pub use error::{is_missing_cldr_error, is_missing_icuexport_error};
pub use registry::{all_keys, all_keys_with_experimental, deserialize_and_discard};
pub use source::SourceData;

#[cfg(feature = "provider_baked")]
pub mod baked_exporter;
#[cfg(feature = "provider_blob")]
pub use icu_provider_blob::export as blob_exporter;
#[cfg(feature = "provider_fs")]
pub use icu_provider_fs::export as fs_exporter;

pub mod options;

/// A prelude for using the datagen API
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{options, DatagenProvider, SourceData};
    #[doc(no_inline)]
    pub use icu_locid::{langid, LanguageIdentifier};
    #[doc(no_inline)]
    pub use icu_provider::{datagen::DataExporter, DataKey, KeyedDataMarker};

    // SEMVER GRAVEYARD
    #[cfg(feature = "legacy_api")]
    #[doc(hidden)]
    pub use crate::options::{CollationHanDatabase, CoverageLevel};
    #[cfg(feature = "legacy_api")]
    #[allow(deprecated)]
    #[doc(hidden)]
    pub use crate::{syntax, BakedOptions, CldrLocaleSubset, Out};
}

use icu_provider::datagen::*;
use icu_provider::prelude::*;
use memchr::memmem;
use std::collections::HashSet;
use std::path::Path;

#[cfg(feature = "rayon")]
pub(crate) use rayon::prelude as rayon_prelude;

#[cfg(not(feature = "rayon"))]
pub(crate) mod rayon_prelude {
    pub trait IntoParallelIterator: IntoIterator + Sized {
        fn into_par_iter(self) -> <Self as IntoIterator>::IntoIter {
            self.into_iter()
        }
    }
    impl<T: IntoIterator> IntoParallelIterator for T {}
}

/// [`DataProvider`] backed by [`SourceData`]
///
/// If `source` does not contain a specific data source, `DataProvider::load` will
/// error ([`is_missing_cldr_error`](crate::is_missing_cldr_error) /
/// [`is_missing_icuexport_error`](crate::is_missing_icuexport_error)) if the data is
/// required for that key.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "networking", derive(Default))]
#[cfg_attr(not(doc), allow(clippy::exhaustive_structs))]
#[cfg_attr(doc, non_exhaustive)]
pub struct DatagenProvider {
    #[doc(hidden)]
    pub source: SourceData,
}

impl DatagenProvider {
    /// Creates a new data provider with the given `source` and `options`.
    ///
    /// Fails if `options` is using CLDR locale sets and `source` does not contain CLDR data.
    pub fn try_new(options: options::Options, mut source: SourceData) -> Result<Self, DataError> {
        if source.options != Default::default() {
            log::warn!("Trie type, collation database, or collations set on SourceData. These will be ignored in favor of options.");
        }

        source.options = options;

        source.options.locales = match core::mem::take(&mut source.options.locales) {
            options::LocaleInclude::None => options::LocaleInclude::Explicit(Default::default()),
            options::LocaleInclude::CldrSet(levels) => options::LocaleInclude::Explicit(
                source
                    .locales(levels.iter().copied().collect::<Vec<_>>().as_slice())?
                    .into_iter()
                    .collect(),
            ),
            options::LocaleInclude::Explicit(set) => options::LocaleInclude::Explicit(set),
            options::LocaleInclude::All => options::LocaleInclude::All,
        };

        if source.options.fallback == options::FallbackMode::Runtime {
            if let options::LocaleInclude::Explicit(ref mut set) = source.options.locales {
                set.insert(Default::default());
            }
        }

        Ok(Self { source })
    }

    #[cfg(test)]
    pub fn for_test() -> Self {
        // Singleton so that all instantiations share the same cache.
        lazy_static::lazy_static! {
            static ref TEST_PROVIDER: DatagenProvider = DatagenProvider {
                // This is equivalent to `latest_tested` for the files defined in
                // `tools/testdata-scripts/globs.rs.data`.
                source: SourceData::offline()
                    .with_cldr(repodata::paths::cldr(), Default::default()).unwrap()
                    .with_icuexport(repodata::paths::icuexport()).unwrap()
                    .with_segmenter_lstm(repodata::paths::lstm()).unwrap(),
            };
        }
        TEST_PROVIDER.clone()
    }

    /// Exports data for the set of keys to the given exporter.
    ///
    /// See
    /// [`BlobExporter`](icu_provider_blob::export),
    /// [`FileSystemExporter`](icu_provider_fs::export),
    /// and [`BakedExporter`](crate::baked_exporter).
    pub fn export(
        &self,
        keys: HashSet<DataKey>,
        mut exporter: impl DataExporter,
    ) -> Result<(), DataError> {
        if keys.is_empty() {
            log::warn!("No keys selected");
        }

        // Avoid multiple monomorphizations
        fn internal(
            provider: &DatagenProvider,
            keys: HashSet<DataKey>,
            exporter: &mut dyn DataExporter,
        ) -> Result<(), DataError> {
            use rayon_prelude::*;

            keys.into_par_iter().try_for_each(|key| {
                if key.metadata().singleton {
                    let payload = provider
                        .load_data(key, Default::default())
                        .and_then(DataResponse::take_payload)
                        .map_err(|e| e.with_req(key, Default::default()))?;

                    return exporter.flush_singleton(key, &payload).map_err(|e| e.with_req(key, Default::default()));
                }

                let mut supported_locales: HashSet<DataLocale> = provider
                    .supported_locales_for_key(key)
                    .map_err(|e| e.with_key(key))?.into_iter().collect();

                match provider.source.options.fallback {
                    options::FallbackMode::Legacy => {
                        supported_locales
                            .into_par_iter()
                            .try_for_each(|locale| {
                                let req = DataRequest {
                                    locale: &locale,
                                    metadata: Default::default(),
                                };
                                let payload = provider
                                    .load_data(key, req)
                                    .and_then(DataResponse::take_payload)
                                    .map_err(|e| e.with_req(key, req))?;
                                exporter.put_payload(key, &locale, &payload)
                            })
                            .map_err(|e| e.with_key(key))?;

                        exporter
                            .flush_with_fallback(key, icu_provider::datagen::FallbackMode::None)
                            .map_err(|e| e.with_key(key))
                    }
                    options::FallbackMode::Runtime => {
                        let payloads = supported_locales.into_par_iter()
                            .map(|locale| {
                                let req = DataRequest {
                                    locale: &locale,
                                    metadata: Default::default(),
                                };
                                let payload = provider
                                    .load_data(key, req)
                                    .and_then(DataResponse::take_payload)
                                    .map_err(|e| e.with_req(key, req))?;
                                Ok::<_, DataError>((locale, payload))
                            })
                            .collect::<Result<std::collections::HashMap<_, _>, _>>().map_err(|e| e.with_key(key))?;

                        // TODO(#2683): Figure out how to compare `DataPayload<ExportMarker>` for equality
                        // to actually dedupe payloads

                        payloads
                            .into_par_iter()
                            .try_for_each(|(locale, payload)| {
                                exporter.put_payload(key, &locale, &payload).map_err(|e| e.with_key(key))
                            })?;
                        exporter
                            .flush_with_fallback(key, icu_provider::datagen::FallbackMode::Full)
                            .map_err(|e| e.with_key(key))
                    }
                    options::FallbackMode::Expand => match &provider.source.options.locales {
                        options::LocaleInclude::Explicit(requested_locales) => {
                            let provider = icu_provider_adapters::fallback::LocaleFallbackProvider::try_new_unstable(provider.clone())?;
                            supported_locales.extend(requested_locales.iter().map(Into::into));
                            supported_locales
                                .into_par_iter()
                                .try_for_each(|locale| {
                                    let req = DataRequest {
                                        locale: &locale,
                                        metadata: Default::default(),
                                    };
                                    let payload = provider
                                        .load_data(key, req)
                                        .and_then(DataResponse::take_payload)
                                        .map_err(|e| e.with_req(key, req))?;
                                    exporter.put_payload(key, &locale, &payload).map_err(|e| e.with_key(key))
                                })?;
                                exporter.flush_with_fallback(key, icu_provider::datagen::FallbackMode::None)
                                .map_err(|e| e.with_key(key))
                        }
                        _ => Err(DataError::custom(
                            "FallbackMode::Expand requires LocaleInclude::Explicit or LocaleInclude::CldrSet",
                        )),
                    },
                }
            })?;

            exporter.close()
        }
        internal(self, keys, &mut exporter)
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
                    .chain([icu_provider::hello_world::HelloWorldV1Marker::KEY])
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
    use std::io::{BufRead, BufReader};
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
    let file = file.as_slice();

    const LEADING_TAG: &[u8] = icu_provider::leading_tag!().as_bytes();
    const TRAILING_TAG: &[u8] = icu_provider::trailing_tag!().as_bytes();

    let trailing_tag = memmem::Finder::new(TRAILING_TAG);

    let mut result: Vec<DataKey> = memmem::find_iter(file, LEADING_TAG)
        .map(|tag_position| tag_position + LEADING_TAG.len())
        .map(|key_start| &file[key_start..])
        .filter_map(move |key_fragment| {
            trailing_tag
                .find(key_fragment)
                .map(|end| &key_fragment[..end])
        })
        .map(std::str::from_utf8)
        .filter_map(Result::ok)
        .filter_map(crate::key)
        .collect();

    result.sort();
    result.dedup();

    Ok(result)
}

/// Requires `legacy_api` Cargo feature
///
/// The output format.
#[deprecated(
    since = "1.3.0",
    note = "use `DatagenProvider::export` with self-constructed `DataExporter`s"
)]
#[non_exhaustive]
#[cfg(feature = "legacy_api")]
pub enum Out {
    /// Output to a file system tree
    Fs {
        /// The root path.
        output_path: std::path::PathBuf,
        /// The serialization format. See [syntax].
        serializer: Box<dyn icu_provider_fs::export::serializers::AbstractSerializer + Sync>,
        /// Whether to overwrite existing data.
        overwrite: bool,
        /// Whether to create a fingerprint file with SHA2 hashes
        fingerprint: bool,
    },
    /// Output as a postcard blob to the given sink.
    Blob(Box<dyn std::io::Write + Sync>),
    /// Output a module with baked data at the given location.
    Baked {
        /// The directory of the generated module.
        mod_directory: std::path::PathBuf,
        /// Additional options to configure the generated module.
        options: BakedOptions,
    },
    /// Old deprecated configuration for databake.
    #[doc(hidden)]
    #[deprecated(since = "1.1.2", note = "please use `Out::Baked` instead")]
    Module {
        mod_directory: std::path::PathBuf,
        pretty: bool,
        insert_feature_gates: bool,
        use_separate_crates: bool,
    },
}

#[allow(deprecated)]
#[cfg(feature = "legacy_api")]
impl core::fmt::Debug for Out {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fs {
                output_path,
                serializer,
                overwrite,
                fingerprint,
            } => f
                .debug_struct("Fs")
                .field("output_path", output_path)
                .field("serializer", serializer)
                .field("overwrite", overwrite)
                .field("fingerprint", fingerprint)
                .finish(),
            Self::Blob(_) => f.debug_tuple("Blob").field(&"[...]").finish(),
            Self::Baked {
                mod_directory,
                options,
            } => f
                .debug_struct("Baked")
                .field("mod_directory", mod_directory)
                .field("options", options)
                .finish(),
            #[allow(deprecated)]
            Self::Module {
                mod_directory,
                pretty,
                insert_feature_gates,
                use_separate_crates,
            } => f
                .debug_struct("Module")
                .field("mod_directory", mod_directory)
                .field("pretty", pretty)
                .field("insert_feature_gates", insert_feature_gates)
                .field("use_separate_crates", use_separate_crates)
                .finish(),
        }
    }
}

#[deprecated(since = "1.3.0", note = "use `DatagenProvider::export`")]
#[cfg(feature = "legacy_api")]
#[allow(deprecated)]
/// Requires `legacy_api` Cargo feature
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
    locales: Option<&[icu_locid::LanguageIdentifier]>,
    keys: &[DataKey],
    source: &SourceData,
    outs: Vec<Out>,
) -> Result<(), DataError> {
    use options::*;
    let provider = DatagenProvider::try_new(
        Options {
            locales: locales
                .map(|ls| {
                    LocaleInclude::Explicit(
                        ls.iter()
                            .cloned()
                            .chain([icu_locid::LanguageIdentifier::UND])
                            .collect(),
                    )
                })
                .unwrap_or(options::LocaleInclude::All),
            ..source.options.clone()
        },
        {
            let mut source = source.clone();
            source.options = Default::default();
            source
        },
    )?;

    provider.export(
        keys.iter().cloned().collect(),
        MultiExporter::new(
            outs.into_iter()
                .map(|out| -> Result<Box<dyn DataExporter>, DataError> {
                    use baked_exporter::*;
                    use icu_provider_blob::export::*;
                    use icu_provider_fs::export::*;

                    Ok(match out {
                        Out::Fs {
                            output_path,
                            serializer,
                            overwrite,
                            fingerprint,
                        } => {
                            let mut options = ExporterOptions::default();
                            options.root = output_path;
                            if overwrite {
                                options.overwrite = OverwriteOption::RemoveAndReplace
                            }
                            options.fingerprint = fingerprint;
                            Box::new(FilesystemExporter::try_new(serializer, options)?)
                        }
                        Out::Blob(write) => Box::new(BlobExporter::new_with_sink(write)),
                        Out::Baked {
                            mod_directory,
                            options,
                        } => Box::new(BakedExporter::new(mod_directory, options)?),
                        #[allow(deprecated)]
                        Out::Module {
                            mod_directory,
                            pretty,
                            insert_feature_gates,
                            use_separate_crates,
                        } => Box::new(BakedExporter::new(
                            mod_directory,
                            Options {
                                pretty,
                                insert_feature_gates,
                                use_separate_crates,
                                // Note: overwrite behavior was `true` in 1.0 but `false` in 1.1;
                                // 1.1.2 made it an option in Options.
                                overwrite: false,
                            },
                        )?),
                    })
                })
                .collect::<Result<_, _>>()?,
        ),
    )
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
        keys_from_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/work_log+keys.txt"
        ))
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
    // and running `cargo +nightly-2022-04-18 wasm-build-release --examples -p icu_datetime --features serde \
    // && cp target/wasm32-unknown-unknown/release-opt-size/examples/work_log.wasm provider/datagen/tests/data/`
    assert_eq!(
        keys_from_bin(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/work_log.wasm"
        ))
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

// SEMVER GRAVEYARD

#[cfg(feature = "legacy_api")]
#[doc(hidden)]
pub use source::{CollationHanDatabase, CoverageLevel};

#[cfg(feature = "legacy_api")]
#[doc(hidden)]
pub use baked_exporter::Options as BakedOptions;

#[allow(clippy::exhaustive_enums)] // exists for backwards compatibility
#[doc(hidden)]
#[derive(Debug)]
pub enum CldrLocaleSubset {
    Ignored,
}

impl Default for CldrLocaleSubset {
    fn default() -> Self {
        Self::Ignored
    }
}

impl CldrLocaleSubset {
    #[allow(non_upper_case_globals)]
    pub const Full: Self = Self::Ignored;
    #[allow(non_upper_case_globals)]
    pub const Modern: Self = Self::Ignored;
}

#[cfg(feature = "legacy_api")]
#[doc(hidden)]
pub use icu_provider_fs::export::serializers as syntax;

impl AnyProvider for DatagenProvider {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        self.as_any_provider().load_any(key, req)
    }
}
