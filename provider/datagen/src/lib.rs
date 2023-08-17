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
//! use icu_datagen::blob_exporter::*;
//! use std::fs::File;
//!
//! DatagenDriver::new()
//!       .with_keys([icu::list::provider::AndListV1Marker::KEY])
//!       .export(&DatagenProvider::latest_tested(), BlobExporter::new_with_sink(Box::new(File::create("data.postcard").unwrap())))
//!       .unwrap();
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

mod driver;
mod error;
mod provider;
mod registry;
mod source;
mod transform;

pub use driver::DatagenDriver;
pub use error::{
    is_missing_cldr_error, is_missing_icuexport_error, is_missing_segmenter_lstm_error,
};
pub use provider::DatagenProvider;
#[doc(hidden)] // for CLI serde
pub use provider::TrieType;
#[allow(deprecated)] // ugh
pub use registry::{all_keys, all_keys_with_experimental, deserialize_and_measure, key};

#[cfg(feature = "provider_baked")]
pub mod baked_exporter;
#[cfg(feature = "provider_blob")]
pub use icu_provider_blob::export as blob_exporter;
#[cfg(feature = "provider_fs")]
pub use icu_provider_fs::export as fs_exporter;

/// A prelude for using the datagen API
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        CollationHanDatabase, CoverageLevel, DatagenDriver, DatagenProvider, FallbackMode,
    };
    #[doc(no_inline)]
    pub use icu_locid::{langid, LanguageIdentifier};
    #[doc(no_inline)]
    pub use icu_provider::{datagen::DataExporter, DataKey, KeyedDataMarker};

    // SEMVER GRAVEYARD
    #[cfg(feature = "legacy_api")]
    #[allow(deprecated)]
    #[doc(hidden)]
    pub use crate::{syntax, BakedOptions, CldrLocaleSubset, Out, SourceData};
}

use icu_provider::prelude::*;
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

/// Defines how fallback will apply to the generated data.
///
/// If in doubt, use [`FallbackMode::PreferredForExporter`], which selects the best mode for your
/// chosen data provider.
///
/// # Fallback Mode Comparison
///
/// The modes differ primarily in their approaches to runtime fallback and data size.
///
/// | Mode | Runtime Fallback | Data Size |
/// |---|---|---|
/// | [`Runtime`] | Yes, Automatic | Smallest |
/// | [`RuntimeManual`] | Yes, Manual | Smallest |
/// | [`Preresolved`] | No | Small |
/// | [`Hybrid`] | Optional | Medium |
///
/// If you are not 100% certain of the closed set of locales you need at runtime, you should
/// use a provider with runtime fallback enabled.
///
/// [`Runtime`]: FallbackMode::Runtime
/// [`RuntimeManual`]: FallbackMode::RuntimeManual
/// [`Preresolved`]: FallbackMode::Preresolved
/// [`Hybrid`]: FallbackMode::Hybrid
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum FallbackMode {
    /// Selects the fallback mode based on [`DataExporter::supports_built_in_fallback()`](
    /// icu_provider::datagen::DataExporter::supports_built_in_fallback()), resolving to either
    /// [`Runtime`] or [`Hybrid`].
    ///
    /// [`Runtime`]: Self::Runtime
    /// [`Hybrid`]: Self::Hybrid
    #[default]
    PreferredForExporter,
    /// This mode generates the minimal set of locales that cover the requested locales when
    /// fallback is used at runtime. For example, if "en" and "en-US" are both requested but
    /// they contain the same value, only "en" will be included, since "en-US" falls back to
    /// "en" at runtime.
    ///
    /// If an explicit list of locales is used, this mode includes all ancestors and descendants
    /// (usually regional variants) of the explicitly listed locales. For example, if "pt-PT" is
    /// requested, then "pt", "pt-PT", and children like "pt-MO" will be included. Note that the
    /// children of "pt-PT" usually inherit from it and therefore don't take up a significant
    /// amount of space in the data file.
    ///
    /// This mode is only supported with the baked data provider, and it builds fallback logic
    /// into the generated code. To use this mode with other providers that don't bundle fallback
    /// logic, use [`FallbackMode::RuntimeManual`] or [`FallbackMode::Hybrid`].
    ///
    /// This is the default fallback mode for the baked provider.
    Runtime,
    /// Same as [`FallbackMode::Runtime`] except that the fallback logic is not included in the
    /// generated code. It must be enabled manually with a [`LocaleFallbackProvider`].
    ///
    /// This mode is supported on all data provider implementations.
    ///
    /// [`LocaleFallbackProvider`]: icu_provider_adapters::fallback::LocaleFallbackProvider
    RuntimeManual,
    /// This mode generates data for exactly the supplied locales. If data doesn't exist for a
    /// locale, fallback will be performed and the fallback value will be exported.
    ///
    /// Requires using an explicit list of locales.
    ///
    /// Note: in data exporters that deduplicate values (such as `BakedExporter` and
    /// `BlobDataExporter`), the impact on data size as compared to [`FallbackMode::Runtime`]
    /// is limited to the pointers in the explicitly listed locales.
    ///
    /// Data generated in this mode can be used without runtime fallback and guarantees that all
    /// locales are present. If you wish to also support locales that were not explicitly listed
    /// with runtime fallback, see [`FallbackMode::Hybrid`].
    Preresolved,
    /// This mode passes through CLDR data without performing locale deduplication.
    ///
    /// If an explicit list of locales is used, this mode includes all ancestors and descendants
    /// (usually regional variants) of the explicitly listed locales. For example, if "pt-PT" is
    /// requested, then "pt", "pt-PT", and children like "pt-MO" will be included.
    ///
    /// Note: in data exporters that deduplicate values (such as `BakedExporter` and
    /// `BlobDataExporter`), the impact on data size as compared to [`FallbackMode::Runtime`]
    /// is limited to the pointers in the explicitly listed locales.
    ///
    /// Data generated in this mode is suitable for use with or without runtime fallback. To
    /// enable runtime fallback, use a [`LocaleFallbackProvider`].
    ///
    /// This is the default fallback mode for the blob and filesystem providers.
    ///
    /// [`LocaleFallbackProvider`]: icu_provider_adapters::fallback::LocaleFallbackProvider
    Hybrid,
}

/// Specifies the collation Han database to use.
///
/// Unihan is more precise but significantly increases data size. See
/// <https://github.com/unicode-org/icu/blob/main/docs/userguide/icu_data/buildtool.md#collation-ucadata>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum CollationHanDatabase {
    /// Implicit
    #[serde(rename = "implicit")]
    #[default]
    Implicit,
    /// Unihan
    #[serde(rename = "unihan")]
    Unihan,
}

impl std::fmt::Display for CollationHanDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CollationHanDatabase::Implicit => write!(f, "implicithan"),
            CollationHanDatabase::Unihan => write!(f, "unihan"),
        }
    }
}

/// A language's CLDR coverage level.
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum CoverageLevel {
    /// Locales listed as modern coverage targets by the CLDR subcomittee.
    ///
    /// This is the highest level of coverage.
    Modern,
    /// Locales listed as moderate coverage targets by the CLDR subcomittee.
    ///
    /// This is a medium level of coverage.
    Moderate,
    /// Locales listed as basic coverage targets by the CLDR subcomittee.
    ///
    /// This is the lowest level of coverage.
    Basic,
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
    use memchr::memmem::*;

    let file = std::fs::read(path.as_ref())?;
    let file = file.as_slice();

    const LEADING_TAG: &[u8] = icu_provider::leading_tag!().as_bytes();
    const TRAILING_TAG: &[u8] = icu_provider::trailing_tag!().as_bytes();

    let trailing_tag = Finder::new(TRAILING_TAG);

    let mut result: Vec<DataKey> = find_iter(file, LEADING_TAG)
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

#[deprecated(since = "1.3.0", note = "use `DatagenDriver`")]
#[allow(deprecated)]
#[cfg(feature = "legacy_api")]
pub use provider::SourceData;

/// Requires `legacy_api` Cargo feature
///
/// The output format.
#[deprecated(since = "1.3.0", note = "use `DatagenDriver`")]
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
        use_separate_crates: bool,
        insert_feature_gates: bool,
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
                use_separate_crates,
                insert_feature_gates,
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

#[deprecated(since = "1.3.0", note = "use `DatagenDriver`")]
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
    let exporter = DatagenDriver::new()
        .with_keys(keys.iter().cloned())
        .with_fallback_mode(FallbackMode::Hybrid)
        .with_collations(source.collations.clone());
    match locales {
        Some(locales) => exporter
            .with_locales(
                locales
                    .iter()
                    .cloned()
                    .chain([icu_locid::LanguageIdentifier::UND]),
            )
            .with_segmenter_models({
                let mut models = vec![];
                for locale in locales {
                    let locale = locale.into();
                    if let Some(model) =
                        transform::segmenter::lstm::data_locale_to_model_name(&locale)
                    {
                        models.push(model.into());
                    }
                    if let Some(model) =
                        transform::segmenter::dictionary::data_locale_to_model_name(&locale)
                    {
                        models.push(model.into());
                    }
                }
                models
            }),
        _ => exporter.with_all_locales(),
    }
    .export(
        &DatagenProvider {
            source: source.clone(),
        },
        icu_provider::datagen::MultiExporter::new(
            outs.into_iter()
                .map(
                    |out| -> Result<Box<dyn icu_provider::datagen::DataExporter>, DataError> {
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
                    },
                )
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
            "/tests/data/tutorial_buffer+keys.txt"
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
    // File obtained by running
    // cargo +nightly --config docs/tutorials/testing/patch.toml build -p tutorial_buffer --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --manifest-path docs/tutorials/crates/buffer/Cargo.toml && cp docs/tutorials/target/wasm32-unknown-unknown/release/tutorial_buffer.wasm provider/datagen/tests/data/
    assert_eq!(
        keys_from_bin(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/tutorial_buffer.wasm"
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
