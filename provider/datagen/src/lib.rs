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
//! Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md).
//!
//! # Examples
//!
//! ## Rust API
//!
//! ```no_run
//! use icu_datagen::blob_exporter::*;
//! use icu_datagen::prelude::*;
//! use std::fs::File;
//!
//! DatagenDriver::new()
//!     .with_keys([icu::list::provider::AndListV1Marker::KEY])
//!     .with_all_locales()
//!     .export(
//!         &DatagenProvider::new_latest_tested(),
//!         BlobExporter::new_v2_with_sink(Box::new(
//!             File::create("data.postcard").unwrap(),
//!         )),
//!     )
//!     .unwrap();
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
//! $ icu4x-datagen --keys all --locales de en-AU --format blob --out data.postcard
//! ```
//!
//! More details can be found by running `--help`.
//!
//! # Cargo features
//!
//! This crate has a lot of dependencies, some of which are not required for all operating modes. These default Cargo features
//! can be disabled to reduce dependencies:
//! * `baked_exporter`
//!   * enables the [`baked_exporter`] module
//!   * enables the `--format mod` CLI argument
//! * `blob_exporter`
//!   * enables the [`blob_exporter`] module, a reexport of [`icu_provider_blob::export`]
//!   * enables the `--format blob` CLI argument
//! * `fs_exporter`
//!   * enables the [`fs_exporter`] module, a reexport of [`icu_provider_fs::export`]
//!   * enables the `--format dir` CLI argument
//! * `networking`
//!   * enables methods on [`DatagenProvider`] that fetch source data from the network
//!   * enables the `--cldr-tag`, `--icu-export-tag`, and `--segmenter-lstm-tag` CLI arguments that download data
//! * `rayon`
//!   * enables parallelism during export
//! * `use_wasm` / `use_icu4c`
//!   * see the documentation on [`icu_codepointtrie_builder`](icu_codepointtrie_builder#build-configuration)
//! * `bin`
//!   * required by the CLI and enabled by default to make `cargo install` work
//! * `legacy_api`
//!   * enables the deprecated pre-1.3 API
//!   * enabled by default for semver stability
//!   * will be removed in 2.0.
//! * `icu_experimental`
//!   * enables data generation for keys defined in the unstable `icu_experimental` crate
//!   * note that this features affects the behaviour of `all_keys`
//!
//! The meta-feature `experimental_components` is available to activate all experimental components.

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
#[cfg(feature = "provider")]
mod provider;
mod registry;

pub use driver::DatagenDriver;
pub use driver::DeduplicationStrategy;
pub use driver::FallbackOptions;
pub use driver::LocaleFamily;
pub use driver::NoFallbackOptions;
pub use driver::RuntimeFallbackLocation;

#[cfg(feature = "provider")]
pub use provider::CollationHanDatabase;
#[cfg(feature = "provider")]
pub use provider::CoverageLevel;
#[cfg(feature = "provider")]
pub use provider::DatagenProvider;

#[cfg(feature = "baked_exporter")]
pub mod baked_exporter;
#[cfg(feature = "blob_exporter")]
pub use icu_provider_blob::export as blob_exporter;
#[cfg(feature = "fs_exporter")]
pub use icu_provider_fs::export as fs_exporter;

/// A prelude for using the datagen API
pub mod prelude {
    #[doc(no_inline)]
    #[cfg(feature = "provider")]
    pub use crate::provider::{CollationHanDatabase, CoverageLevel, DatagenProvider};
    #[doc(no_inline)]
    pub use crate::{
        DatagenDriver, DeduplicationStrategy, FallbackMode, FallbackOptions, LocaleFamily,
        NoFallbackOptions, RuntimeFallbackLocation,
    };
    #[doc(no_inline)]
    pub use icu_locid::{langid, LanguageIdentifier};
    #[doc(no_inline)]
    pub use icu_provider::{datagen::DataExporter, DataKey, KeyedDataMarker};

    // SEMVER GRAVEYARD
    #[cfg(feature = "legacy_api")]
    #[allow(deprecated)]
    #[doc(hidden)]
    pub use crate::{provider::CldrLocaleSubset, syntax, BakedOptions, Out, SourceData};
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

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        /// List of all keys that are available.
        ///
        /// Note that since v1.3, `all_keys` also contains experimental keys for which the
        /// corresponding Cargo features has been enabled.
        // Excludes the hello world key, as that generally should not be generated.
        pub fn all_keys() -> Vec<DataKey> {
            #[cfg(features = "experimental_components")]
            log::warn!("The icu_datagen crates has been built with the `experimental_components` feature, so `all_keys` returns experimental keys");
            vec![
                $(
                    <$marker>::KEY,
                )+
                $(
                    #[cfg(feature = "experimental_components")]
                    <$emarker>::KEY,
                )+
            ]
        }

        #[test]
        fn no_key_collisions() {
            let mut map = std::collections::BTreeMap::new();
            let mut failed = false;
            for key in all_keys() {
                if let Some(colliding_key) = map.insert(key.hashed(), key) {
                    println!(
                        "{:?} and {:?} collide at {:?}",
                        key.path(),
                        colliding_key.path(),
                        key.hashed()
                    );
                    failed = true;
                }
            }
            if failed {
                panic!();
            }
        }

        /// Parses a human-readable key identifier into a [`DataKey`].
        //  Supports the hello world key
        /// # Example
        /// ```
        /// # use icu_provider::KeyedDataMarker;
        /// assert_eq!(
        ///     icu_datagen::key("list/and@1"),
        ///     Some(icu_list::provider::AndListV1Marker::KEY),
        /// );
        /// ```
        pub fn key<S: AsRef<str>>(string: S) -> Option<DataKey> {
            use once_cell::sync::OnceCell;
            static LOOKUP: OnceCell<std::collections::HashMap<&'static str, Result<DataKey, &'static str>>> = OnceCell::new();
            let lookup = LOOKUP.get_or_init(|| {
                [
                    ("core/helloworld@1", Ok(icu_provider::hello_world::HelloWorldV1Marker::KEY)),
                    $(
                        ($path, Ok(<$marker>::KEY)),
                    )+
                    $(
                        #[cfg(feature = "experimental_components")]
                        ($epath, Ok(<$emarker>::KEY)),
                        #[cfg(not(feature = "experimental_components"))]
                        ($epath, Err(stringify!(feature = "experimental_components"))),
                    )+

                ]
                .into_iter()
                .collect()
            });
            let path = string.as_ref();
            match lookup.get(path).copied() {
                None => {
                    log::warn!("Unknown key {path:?}");
                    None
                },
                Some(Err(feature)) => {
                    log::warn!("Key {path:?} requires {feature}");
                    None
                },
                Some(Ok(key)) => Some(key)
            }
        }

        #[test]
        fn test_paths_correct() {
            $(
                assert_eq!(<$marker>::KEY.path().get(), $path);
            )+
            $(
                assert_eq!(<$emarker>::KEY.path().get(), $epath);
            )+
        }

        #[macro_export]
        #[doc(hidden)]
        macro_rules! make_exportable_provider {
            ($ty:ty) => {
                icu_provider::make_exportable_provider!(
                    $ty,
                    [
                        icu_provider::hello_world::HelloWorldV1Marker,
                        $(
                            $marker,
                        )+
                        $(
                            #[cfg(feature = "experimental_components")]
                            $emarker,
                        )+
                    ]
                );
            }
        }
    }
}
crate::registry!(cb);

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
/// | [`Runtime`] | Required, Automatic | Smallest |
/// | [`RuntimeManual`] | Required, Manual | Smallest |
/// | [`Preresolved`] | Unsupported | Small |
/// | [`Hybrid`] | Optional | Medium |
///
/// If you are not 100% certain of the closed set of locales you need at runtime, you should
/// use a provider with runtime fallback enabled.
///
/// [`Runtime`]: FallbackMode::Runtime
/// [`RuntimeManual`]: FallbackMode::RuntimeManual
/// [`Preresolved`]: FallbackMode::Preresolved
/// [`Hybrid`]: FallbackMode::Hybrid
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
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
#[deprecated(since = "1.3.0", note = "use Rust code")]
#[cfg(feature = "legacy_api")]
pub fn keys_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<DataKey>> {
    let file = std::fs::File::open(path.as_ref())?;
    keys_from_file_inner(&file)
}

#[cfg(feature = "legacy_api")]
fn keys_from_file_inner<R: std::io::Read>(source: R) -> std::io::Result<Vec<DataKey>> {
    use std::io::{BufRead, BufReader};
    BufReader::new(source)
        .lines()
        .filter_map(|k| k.map(crate::key).transpose())
        .collect()
}
/// Parses a compiled binary and returns a list of [`DataKey`]s that it uses *at runtime*.
///
/// This function is intended to be used for binaries that use `AnyProvider` or `BufferProvider`,
/// for which the compiler cannot remove unused data. Using this function on a binary that only
/// uses compiled data (through the `compiled_data` feature or manual baked data) will not return
/// any keys, as the keys only exist in the type system.
///
/// # Example
///
/// #### build.rs
/// ```no_run
/// # use icu_provider::KeyedDataMarker;
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
//  Supports the hello world key
pub fn keys_from_bin<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<DataKey>> {
    let file = std::fs::read(path.as_ref())?;
    let file = file.as_slice();

    Ok(keys_from_bin_inner(file))
}

fn keys_from_bin_inner(bytes: &[u8]) -> Vec<DataKey> {
    use memchr::memmem::*;

    const LEADING_TAG: &[u8] = icu_provider::leading_tag!().as_bytes();
    const TRAILING_TAG: &[u8] = icu_provider::trailing_tag!().as_bytes();

    let trailing_tag = Finder::new(TRAILING_TAG);

    let mut result: Vec<DataKey> = find_iter(bytes, LEADING_TAG)
        .map(|tag_position| tag_position + LEADING_TAG.len())
        .map(|key_start| &bytes[key_start..])
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

    result
}

#[deprecated(since = "1.3.0", note = "use `DatagenDriver`")]
#[allow(deprecated)]
#[cfg(feature = "legacy_api")]
pub use provider::SourceData;

/// The output format for [`datagen`].
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
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
    ///
    /// This supports only version 1 of the blob format. Please use [`DatagenDriver`] with
    /// [`BlobExporter`] to export to blob format version 2.
    ///
    /// [`BlobExporter`]: crate::blob_exporter::BlobExporter
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
/// Runs data generation
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
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
        .with_additional_collations(source.collations.clone());
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
                    if let Some(model) = crate::lstm_data_locale_to_model_name(&locale) {
                        models.push(model.into());
                    }
                    if let Some(model) = crate::dictionary_data_locale_to_model_name(&locale) {
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
                        Ok(match out {
                            Out::Fs {
                                output_path,
                                serializer,
                                overwrite,
                                fingerprint,
                            } => {
                                let mut options = fs_exporter::Options::default();
                                options.root = output_path;
                                if overwrite {
                                    options.overwrite =
                                        fs_exporter::OverwriteOption::RemoveAndReplace
                                }
                                options.fingerprint = fingerprint;
                                Box::new(fs_exporter::FilesystemExporter::try_new(
                                    serializer, options,
                                )?)
                            }
                            Out::Blob(write) => {
                                // Note: no blob v2 support in legacy API
                                Box::new(blob_exporter::BlobExporter::new_with_sink(write))
                            }
                            Out::Baked {
                                mod_directory,
                                options,
                            } => Box::new(baked_exporter::BakedExporter::new(
                                mod_directory,
                                options,
                            )?),
                            #[allow(deprecated)]
                            Out::Module {
                                mod_directory,
                                pretty,
                                insert_feature_gates,
                                use_separate_crates,
                            } => Box::new(baked_exporter::BakedExporter::new(
                                mod_directory,
                                baked_exporter::Options {
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

use icu_locid::langid;

#[cfg(feature = "provider")]
fn lstm_model_name_to_data_locale(name: &str) -> Option<DataLocale> {
    match name {
        "Burmese_codepoints_exclusive_model4_heavy" => Some(langid!("my").into()),
        "Khmer_codepoints_exclusive_model4_heavy" => Some(langid!("km").into()),
        "Lao_codepoints_exclusive_model4_heavy" => Some(langid!("lo").into()),
        "Thai_codepoints_exclusive_model4_heavy" => Some(langid!("th").into()),
        _ => None,
    }
}

fn lstm_data_locale_to_model_name(locale: &DataLocale) -> Option<&'static str> {
    match locale.get_langid() {
        id if id == langid!("my") => Some("Burmese_codepoints_exclusive_model4_heavy"),
        id if id == langid!("km") => Some("Khmer_codepoints_exclusive_model4_heavy"),
        id if id == langid!("lo") => Some("Lao_codepoints_exclusive_model4_heavy"),
        id if id == langid!("th") => Some("Thai_codepoints_exclusive_model4_heavy"),
        _ => None,
    }
}

#[cfg(feature = "provider")]
fn dictionary_model_name_to_data_locale(name: &str) -> Option<DataLocale> {
    match name {
        "khmerdict" => Some(langid!("km").into()),
        "cjdict" => Some(langid!("ja").into()),
        "laodict" => Some(langid!("lo").into()),
        "burmesedict" => Some(langid!("my").into()),
        "thaidict" => Some(langid!("th").into()),
        _ => None,
    }
}

fn dictionary_data_locale_to_model_name(locale: &DataLocale) -> Option<&'static str> {
    match locale.get_langid() {
        id if id == langid!("km") => Some("khmerdict"),
        id if id == langid!("ja") => Some("cjdict"),
        id if id == langid!("lo") => Some("laodict"),
        id if id == langid!("my") => Some("burmesedict"),
        id if id == langid!("th") => Some("thaidict"),
        _ => None,
    }
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
#[cfg(feature = "legacy_api")]
fn test_keys_from_file() {
    #![allow(deprecated)]

    const BYTES: &[u8] = include_bytes!("../tests/data/tutorial_buffer+keys.txt");

    assert_eq!(
        keys_from_file_inner(BYTES).unwrap(),
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
    assert_eq!(
        keys_from_bin_inner(include_bytes!("../tests/data/tutorial_buffer.wasm")),
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

/// Same as `all_keys`.
///
/// Note that since v1.3, `all_keys` also contains experimental keys for which the
/// corresponding Cargo features has been enabled.
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
#[deprecated(
    since = "1.3.0",
    note = "use `all_keys` with the required cargo features"
)]
#[cfg(feature = "legacy_api")]
pub fn all_keys_with_experimental() -> Vec<DataKey> {
    all_keys()
}

#[cfg(feature = "legacy_api")]
#[deprecated(since = "1.3.0", note = "use methods on `DatagenProvider`")]
/// Identifies errors that are due to missing CLDR data.
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
pub fn is_missing_cldr_error(e: DataError) -> bool {
    DatagenProvider::is_missing_cldr_error(e)
}

#[cfg(feature = "legacy_api")]
#[deprecated(since = "1.3.0", note = "use methods on `DatagenProvider`")]
/// Identifies errors that are due to missing ICU export data.
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
pub fn is_missing_icuexport_error(e: DataError) -> bool {
    DatagenProvider::is_missing_icuexport_error(e)
}

#[cfg(feature = "legacy_api")]
/// [`Out::Fs`] serialization formats.
///
/// ✨ *Enabled with the `legacy_api` Cargo feature.*
#[deprecated(since = "1.3.0", note = "use `fs_exporter::serializers`")]
pub mod syntax {
    #[doc(no_inline)]
    pub use crate::fs_exporter::serializers::Bincode;
    #[doc(no_inline)]
    pub use crate::fs_exporter::serializers::Json;
    #[doc(no_inline)]
    pub use crate::fs_exporter::serializers::Postcard;
}

#[cfg(feature = "legacy_api")]
#[doc(hidden)]
pub use baked_exporter::Options as BakedOptions;

#[cfg(feature = "legacy_api")]
#[doc(hidden)]
pub use provider::CldrLocaleSubset;
