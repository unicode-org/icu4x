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
mod transform;

use elsa::sync::FrozenMap;
pub use error::{is_missing_cldr_error, is_missing_icuexport_error};
use icu_locid::LanguageIdentifier;
use icu_locid_transform::fallback::LocaleFallbackConfig;
use icu_locid_transform::fallback::LocaleFallbacker;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use memchr::memmem;
use options::{FallbackMode, LocaleInclude};
#[allow(deprecated)] // ugh
pub use registry::{all_keys, all_keys_with_experimental, deserialize_and_measure, key};
pub use source::SourceData;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

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

        if matches!(source.options.fallback, options::FallbackMode::Preresolved)
            && !matches!(source.options.locales, options::LocaleInclude::Explicit(_))
        {
            return Err(DataError::custom(
                "FallbackMode::Expand requires LocaleInclude::Explicit",
            ));
        }

        source.options.locales = match core::mem::take(&mut source.options.locales) {
            options::LocaleInclude::None => options::LocaleInclude::Explicit(Default::default()),
            options::LocaleInclude::CldrSet(levels) => options::LocaleInclude::Explicit(
                source
                    .locales(levels.iter().copied().collect::<Vec<_>>().as_slice())?
                    .into_iter()
                    .chain(core::iter::once(LanguageIdentifier::UND))
                    .collect(),
            ),
            options::LocaleInclude::Explicit(set) => options::LocaleInclude::Explicit(set),
            options::LocaleInclude::All => options::LocaleInclude::All,
            options::LocaleInclude::Recommended => options::LocaleInclude::Explicit(
                source
                    .locales(&[
                        options::CoverageLevel::Modern,
                        options::CoverageLevel::Moderate,
                        options::CoverageLevel::Basic,
                    ])?
                    .into_iter()
                    .chain(core::iter::once(LanguageIdentifier::UND))
                    .collect(),
            ),
        };

        log::debug!(
            "Datagen configured with fallback mode: {:?}",
            source.options.fallback
        );

        let mut provider = Self { source };

        // TODO: Consider figuring out the cases where we don't need the fallbacker.
        // We need it most of the time, so just pre-compute it here.
        provider.source.fallbacker = Some(LocaleFallbacker::try_new_unstable(&provider)?);

        Ok(provider)
    }

    #[cfg(test)]
    pub fn for_test() -> Self {
        // Singleton so that all instantiations share the same cache.
        lazy_static::lazy_static! {
            static ref TEST_PROVIDER: DatagenProvider = {
                let data_root = std::path::Path::new(core::env!("CARGO_MANIFEST_DIR")).join("tests/data");
                DatagenProvider {
                    // This is equivalent to `latest_tested` for the files defined in
                    // `tools/testdata-scripts/globs.rs.data`.
                    source: SourceData::offline()
                        .with_cldr(data_root.join("cldr"), Default::default()).unwrap()
                        .with_icuexport(data_root.join("icuexport")).unwrap(),
                }
            };
        }
        TEST_PROVIDER.clone()
    }

    // TODO: Get rid of this function and do all filtering in a centralized location
    pub(crate) fn filter_data_locales(
        &self,
        supported: Vec<icu_provider::DataLocale>,
    ) -> Vec<icu_provider::DataLocale> {
        supported
    }

    /// Selects the maximal set of locales to export based on a [`DataKey`] and this datagen
    /// provider's options bag. The locales may be later optionally deduplicated for fallback.
    fn select_locales_for_key(&self, key: DataKey) -> Result<HashSet<DataLocale>, DataError> {
        let supported_locales: HashSet<DataLocale> = self
            .supported_locales_for_key(key)
            .map_err(|e| e.with_key(key))?
            .into_iter()
            .collect();

        let locale_include = self.source.options.locales.clone();
        let fallback_mode = self.source.options.fallback;

        // Case 1: `LocaleInclude::All` simply exports all supported locales for this key.
        // Also include special cases like Segmenter that should skip the rest of this function.
        if matches!(locale_include, LocaleInclude::All)
            || key == icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker::KEY
            || key == icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker::KEY
            || key == icu_segmenter::provider::LstmForWordLineAutoV1Marker::KEY
        {
            return Ok(supported_locales);
        }

        let LocaleInclude::Explicit(explicit) = locale_include else {
            unreachable!("Pre-processed LocaleInclued has only 2 variants")
        };

        // Case 2: `FallbackMode::Preresolved` exports all supported locales whose langid matches
        // one of the explicit locales. This ensures extensions are included. In addition, any
        // explicit locales are added to the list, even if they themselves don't contain data;
        // fallback should be performed upon exporting.
        if matches!(fallback_mode, FallbackMode::Preresolved) {
            let result = supported_locales
                .into_iter()
                .chain(explicit.iter().map(|langid| langid.into()))
                .filter(|locale| explicit.contains(&locale.get_langid()))
                .collect();
            return Ok(result);
        }

        // Case 3: All other modes resolve to the "ancestors and descendants" strategy.
        let fallbacker_with_config = self
            .source
            .fallbacker
            .as_ref()
            .unwrap()
            .for_config(LocaleFallbackConfig::from_key(key));
        let include_und = explicit.contains(&LanguageIdentifier::UND);
        let explicit: HashSet<DataLocale> = explicit.into_iter().map(|d| d.into()).collect();
        let mut implicit = HashSet::new();
        // TODO: Make including the default locale configurable
        implicit.insert(DataLocale::default());
        for locale in explicit.iter() {
            let mut iter = fallbacker_with_config.fallback_for(locale.clone());
            while !iter.get().is_empty() {
                implicit.insert(iter.get().clone());
                iter.step();
            }
        }

        let result = supported_locales
            .into_iter()
            .chain(explicit.iter().cloned())
            .filter(|locale| {
                if implicit.contains(locale) {
                    return true;
                }
                if locale.is_langid_und() && include_und {
                    return true;
                }
                let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                while !iter.get().is_empty() {
                    if explicit.contains(iter.get()) {
                        return true;
                    }
                    iter.step();
                }
                false
            })
            .collect();

        Ok(result)
    }

    /// Loads a `DataPayload<ExportMarker>` with locale fallback enabled.
    fn load_with_fallback(
        &self,
        key: DataKey,
        locale: &DataLocale,
    ) -> Result<Option<DataPayload<ExportMarker>>, DataError> {
        log::trace!("Generating key/locale: {key}/{locale:}");
        struct DatagenProviderForFallback<'a>(&'a DatagenProvider);
        impl DynamicDataProvider<ExportMarker> for DatagenProviderForFallback<'_> {
            fn load_data(
                &self,
                key: DataKey,
                req: DataRequest,
            ) -> Result<DataResponse<ExportMarker>, DataError> {
                self.0.load_data(key, req)
            }
        }
        let provider_with_fallback = LocaleFallbackProvider::new_with_fallbacker(
            DatagenProviderForFallback(self),
            self.source.fallbacker.clone().unwrap(),
        );
        let req = DataRequest {
            locale,
            metadata: Default::default(),
        };
        match provider_with_fallback.load_data(key, req) {
            Ok(data_response) => {
                #[allow(clippy::unwrap_used)] // LocaleFallbackProvider populates it
                if data_response.metadata.locale.as_ref().unwrap().is_empty() && !locale.is_empty()
                {
                    log::debug!("Falling back to und: {key}/{locale}");
                }
                Ok(Some(data_response.take_payload()?))
            }
            Err(DataError {
                kind: DataErrorKind::MissingLocale,
                ..
            }) => {
                log::debug!("Could not find data for: {key}/{locale}");
                Ok(None)
            }
            Err(e) => Err(e.with_req(key, req)),
        }
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
                log::info!("Generating key {key}");

                if key.metadata().singleton {
                    let payload = provider
                        .load_data(key, Default::default())
                        .and_then(DataResponse::take_payload)
                        .map_err(|e| e.with_req(key, Default::default()))?;

                    return exporter
                        .flush_singleton(key, &payload)
                        .map_err(|e| e.with_req(key, Default::default()));
                }

                let locales_to_export = provider.select_locales_for_key(key)?;

                match (
                    provider.source.options.fallback,
                    exporter.supports_built_in_fallback(),
                ) {
                    (options::FallbackMode::Runtime, _)
                    | (options::FallbackMode::RuntimeManual, _)
                    | (options::FallbackMode::PreferredForExporter, true) => {
                        let payloads =
                            FrozenMap::<DataLocale, Box<DataPayload<ExportMarker>>>::new();
                        locales_to_export.into_par_iter().try_for_each(|locale| {
                            let payload = provider.load_with_fallback(key, &locale)?;
                            if let Some(payload) = payload {
                                payloads.insert(locale, Box::new(payload));
                            }
                            Ok::<(), DataError>(())
                        })?;
                        let payloads = payloads
                            .into_tuple_vec()
                            .into_iter()
                            .collect::<HashMap<_, _>>();
                        let fallbacker_with_config = provider
                            .source
                            .fallbacker
                            .as_ref()
                            .unwrap()
                            .for_config(LocaleFallbackConfig::from_key(key));
                        'outer: for (locale, payload) in payloads.iter() {
                            let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                            while !iter.get().is_empty() {
                                iter.step();
                                if let Some(parent_payload) = payloads.get(iter.get()) {
                                    if parent_payload == payload {
                                        // Found a match: don't need to write anything
                                        log::trace!(
                                            "Deduplicating {key}/{locale} (inherits from {})",
                                            iter.get()
                                        );
                                        continue 'outer;
                                    }
                                }
                            }
                            // Did not find a match: export this payload
                            exporter.put_payload(key, locale, payload)?;
                        }
                    }
                    (options::FallbackMode::Hybrid, _)
                    | (options::FallbackMode::PreferredForExporter, false)
                    | (options::FallbackMode::Preresolved, _) => {
                        locales_to_export.into_par_iter().try_for_each(|locale| {
                            let payload = provider.load_with_fallback(key, &locale)?;
                            if let Some(payload) = payload {
                                exporter.put_payload(key, &locale, &payload)?;
                            }
                            Ok::<(), DataError>(())
                        })?;
                    }
                };

                match (
                    provider.source.options.fallback,
                    exporter.supports_built_in_fallback(),
                ) {
                    (options::FallbackMode::Runtime, _)
                    | (options::FallbackMode::PreferredForExporter, true) => {
                        exporter.flush_with_built_in_fallback(key, BuiltInFallbackMode::Standard)
                    }
                    (_, _) => exporter.flush(key),
                }
                .map_err(|e| e.with_key(key))
            })?;

            exporter.close()
        }
        internal(self, keys, &mut exporter)
    }
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
            segmenter_models: match locales {
                None => options::SegmenterModelInclude::Recommended,
                Some(list) => options::SegmenterModelInclude::Explicit({
                    let mut models = vec![];
                    for locale in list {
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
            },
            fallback: FallbackMode::Hybrid,
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
