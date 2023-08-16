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
//!             {
//!                 let mut options = options::Options::default();
//!                 options.keys = [icu::list::provider::AndListV1Marker::KEY].into_iter().collect();
//!                 options
//!             },
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

pub use error::{is_missing_cldr_error, is_missing_icuexport_error};
#[allow(deprecated)] // ugh
pub use registry::{all_keys, all_keys_with_experimental, deserialize_and_measure, key};
pub use source::CollationHanDatabase;
pub use source::SourceData;
#[doc(hidden)] // for CLI serde
pub use source::TrieType;

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
    pub use crate::options::CoverageLevel;
    #[cfg(feature = "legacy_api")]
    #[doc(hidden)]
    pub use crate::source::CollationHanDatabase;
    #[cfg(feature = "legacy_api")]
    #[allow(deprecated)]
    #[doc(hidden)]
    pub use crate::{syntax, BakedOptions, CldrLocaleSubset, Out};
}

use icu_locid::LanguageIdentifier;
use icu_locid_transform::fallback::LocaleFallbackIterator;
use icu_locid_transform::fallback::LocaleFallbacker;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use memchr::memmem;
use once_cell::sync::Lazy;
use options::{FallbackMode, LocaleInclude};
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use writeable::Writeable;

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
    /// Creates a new data provider with the given `source`.
    pub fn new(source: SourceData) -> Self {
        Self { source }
    }

    #[cfg(test)]
    pub fn for_test() -> Self {
        use once_cell::sync::OnceCell;

        static TEST_PROVIDER: OnceCell<DatagenProvider> = OnceCell::new();
        // Singleton so that all instantiations share the same cache.
        TEST_PROVIDER
            .get_or_init(|| {
                let data_root =
                    std::path::Path::new(core::env!("CARGO_MANIFEST_DIR")).join("tests/data");
                DatagenProvider {
                    // This is equivalent to `latest_tested` for the files defined in
                    // `tools/testdata-scripts/globs.rs.data`.
                    source: SourceData::offline()
                        .with_cldr(data_root.join("cldr"), Default::default())
                        .unwrap()
                        .with_icuexport(data_root.join("icuexport"))
                        .unwrap(),
                }
            })
            .clone()
    }

    /// Selects the maximal set of locales to export based on a [`DataKey`] and this datagen
    /// provider's options bag. The locales may be later optionally deduplicated for fallback.
    pub(crate) fn select_locales_for_key(
        &self,
        key: DataKey,
        options: &options::Options,
        fallbacker: &Lazy<
            Result<LocaleFallbacker, DataError>,
            impl FnOnce() -> Result<LocaleFallbacker, DataError>,
        >,
    ) -> Result<HashSet<icu_provider::DataLocale>, DataError> {
        let mut locales = self
            .supported_locales_for_key(key)
            .map_err(|e| e.with_key(key))?
            .into_iter()
            .collect::<HashSet<DataLocale>>();

        if key == icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker::KEY
            || key == icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker::KEY
        {
            // Segmenter: filter only by segmenter_models
            return Ok(transform::segmenter::dictionary::filter_data_locales(
                locales,
                &options.segmenter_models,
            ));
        } else if key == icu_segmenter::provider::LstmForWordLineAutoV1Marker::KEY {
            // Segmenter: filter only by segmenter_models
            return Ok(transform::segmenter::lstm::filter_data_locales(
                locales,
                &options.segmenter_models,
            ));
        } else if key == icu_collator::provider::CollationDataV1Marker::KEY
            || key == icu_collator::provider::CollationDiacriticsV1Marker::KEY
            || key == icu_collator::provider::CollationJamoV1Marker::KEY
            || key == icu_collator::provider::CollationMetadataV1Marker::KEY
            || key == icu_collator::provider::CollationReorderingV1Marker::KEY
            || key == icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY
        {
            // Collator: filter by collations, but also by locales/fallback
            locales =
                transform::icuexport::collator::filter_data_locales(locales, &options.collations);
        }

        locales = match (&options.locales, options.fallback) {
            // Case 1: `LocaleInclude::All` simply exports all supported locales for this key.
            (LocaleInclude::All, _) => locales,
            // Case 2: `FallbackMode::Preresolved` exports all supported locales whose langid matches
            // one of the explicit locales. This ensures extensions are included. In addition, any
            // explicit locales are added to the list, even if they themselves don't contain data;
            // fallback should be performed upon exporting.
            (LocaleInclude::Explicit(explicit), FallbackMode::Preresolved) => locales
                .into_iter()
                .chain(explicit.iter().map(|langid| langid.into()))
                .filter(|locale| explicit.contains(&locale.get_langid()))
                .collect(),
            // Case 3: All other modes resolve to the "ancestors and descendants" strategy.
            (LocaleInclude::Explicit(explicit), _) => {
                let include_und = explicit.contains(&LanguageIdentifier::UND);
                let explicit: HashSet<DataLocale> = explicit.iter().map(DataLocale::from).collect();
                let mut implicit = HashSet::new();
                // TODO: Make including the default locale configurable
                implicit.insert(DataLocale::default());
                let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
                let fallbacker_with_config = fallbacker.for_config(key.fallback_config());

                for locale in explicit.iter() {
                    let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                    while !iter.get().is_empty() {
                        implicit.insert(iter.get().clone());
                        iter.step();
                    }
                }

                locales
                    .into_iter()
                    .chain(explicit.iter().cloned())
                    .filter(|locale| {
                        if implicit.contains(locale) {
                            return true;
                        }
                        if explicit.contains(locale) {
                            return true;
                        }
                        if locale.is_langid_und() && include_und {
                            return true;
                        }
                        if locale.language().is_empty()
                            && matches!(
                                key.fallback_config().priority,
                                icu_provider::FallbackPriority::Region
                            )
                        {
                            return true;
                        }
                        // Special case: skeletons *require* the -u-ca keyword, so don't export locales that don't have it
                        // This would get caught later on, but it makes datagen faster and quieter to catch it here
                        if key
                            == icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY
                            && !locale.has_unicode_ext()
                        {
                            return false;
                        }
                        let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                        while !iter.get().is_empty() {
                            if explicit.contains(iter.get()) {
                                return true;
                            }
                            iter.step();
                        }
                        log::trace!("Filtered out: {key}/{locale}");
                        false
                    })
                    .collect()
            }
            _ => unreachable!("Pre-processed LocaleInclude has only 2 variants"),
        };

        Ok(locales)
    }

    /// Loads a `DataPayload<ExportMarker>` with locale fallback enabled.
    fn load_with_fallback(
        &self,
        key: DataKey,
        locale: &DataLocale,
        fallbacker: &Lazy<
            Result<LocaleFallbacker, DataError>,
            impl FnOnce() -> Result<LocaleFallbacker, DataError>,
        >,
    ) -> Result<Option<DataPayload<ExportMarker>>, DataError> {
        log::trace!("Generating key/locale: {key}/{locale:}");
        let mut metadata = DataRequestMetadata::default();
        metadata.silent = true;
        // Lazy-compute the fallback iterator so that we don't always require CLDR data
        let mut option_iter: Option<LocaleFallbackIterator> = None;
        loop {
            let req = DataRequest {
                locale: match option_iter.as_ref() {
                    Some(iter) => iter.get(),
                    None => locale,
                },
                metadata,
            };
            let result = self.load_data(key, req);
            match result {
                Ok(data_response) => {
                    if let Some(iter) = option_iter.as_ref() {
                        if iter.get().is_empty() && !locale.is_empty() {
                            log::debug!("Falling back to und: {key}/{locale}");
                        }
                    }
                    return Ok(Some(data_response.take_payload()?));
                }
                Err(DataError {
                    kind: DataErrorKind::MissingLocale,
                    ..
                }) => {
                    if let Some(iter) = option_iter.as_mut() {
                        if iter.get().is_empty() {
                            log::debug!("Could not find data for: {key}/{locale}");
                            return Ok(None);
                        }
                        iter.step();
                    } else {
                        let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
                        let iter = fallbacker
                            .for_config(key.fallback_config())
                            .fallback_for(locale.clone());
                        option_iter.replace(iter);
                    }
                }
                Err(e) => return Err(e.with_req(key, req)),
            }
        }
    }

    /// Exports data for the given options to the given exporter.
    ///
    /// See
    /// [`BlobExporter`](icu_provider_blob::export),
    /// [`FileSystemExporter`](icu_provider_fs::export),
    /// and [`BakedExporter`](crate::baked_exporter).
    pub fn export(
        &self,
        mut options: options::Options,
        mut exporter: impl DataExporter,
    ) -> Result<(), DataError> {
        if options.keys.is_empty() {
            log::warn!("No keys selected");
        }

        if !self.source.collations.is_empty()
            && options.collations
                != self
                    .source
                    .collations
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>()
        {
            log::warn!("SourceData::with_collations was used and differs from Options#collations (which will be used).")
        }

        if matches!(options.fallback, options::FallbackMode::Preresolved)
            && !matches!(options.locales, options::LocaleInclude::Explicit(_))
        {
            return Err(DataError::custom(
                "FallbackMode::Preresolved requires LocaleInclude::Explicit",
            ));
        }

        options.locales = match core::mem::take(&mut options.locales) {
            options::LocaleInclude::None => options::LocaleInclude::Explicit(Default::default()),
            options::LocaleInclude::CldrSet(levels) => options::LocaleInclude::Explicit(
                self.source
                    .locales(levels.iter().copied().collect::<Vec<_>>().as_slice())?
                    .into_iter()
                    .chain(core::iter::once(LanguageIdentifier::UND))
                    .collect(),
            ),
            options::LocaleInclude::Explicit(set) => options::LocaleInclude::Explicit(set),
            options::LocaleInclude::All => options::LocaleInclude::All,
            options::LocaleInclude::Recommended => options::LocaleInclude::Explicit(
                self.source
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

        options.fallback = match options.fallback {
            options::FallbackMode::PreferredForExporter => {
                if exporter.supports_built_in_fallback() {
                    options::FallbackMode::Runtime
                } else {
                    options::FallbackMode::Hybrid
                }
            }
            f => f,
        };

        log::info!(
            "Datagen configured with fallback mode {:?} and these locales: {}",
            options.fallback,
            match options.locales {
                options::LocaleInclude::All => "ALL".to_string(),
                options::LocaleInclude::Explicit(ref set) => {
                    let mut list: Vec<Cow<str>> =
                        set.iter().map(Writeable::write_to_string).collect();
                    list.sort();
                    format!("{:?}", list)
                }
                _ => unreachable!(),
            }
        );

        // Avoid multiple monomorphizations
        fn internal(
            provider: &DatagenProvider,
            mut options: options::Options,
            exporter: &mut dyn DataExporter,
        ) -> Result<(), DataError> {
            use rayon_prelude::*;

            let fallbacker =
                once_cell::sync::Lazy::new(|| LocaleFallbacker::try_new_unstable(provider));

            core::mem::take(&mut options.keys)
                .into_par_iter()
                .try_for_each(|key| {
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

                    let locales_to_export =
                        provider.select_locales_for_key(key, &options, &fallbacker)?;

                    match options.fallback {
                        options::FallbackMode::Runtime | options::FallbackMode::RuntimeManual => {
                            let payloads = locales_to_export
                                .into_par_iter()
                                .flat_map(|locale| {
                                    match provider.load_with_fallback(key, &locale, &fallbacker) {
                                        Ok(Some(payload)) => Some(Ok((locale, Box::new(payload)))),
                                        Ok(None) => None,
                                        Err(e) => Some(Err(e)),
                                    }
                                })
                                .collect::<Result<HashMap<_, _>, _>>()?;
                            let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
                            let fallbacker_with_config =
                                fallbacker.for_config(key.fallback_config());
                            'outer: for (locale, payload) in payloads.iter() {
                                let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                                while !iter.get().is_empty() {
                                    iter.step();
                                    if let Some(parent_payload) = payloads.get(iter.get()) {
                                        if parent_payload == payload && locale != iter.get() {
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
                        options::FallbackMode::Hybrid | options::FallbackMode::Preresolved => {
                            locales_to_export.into_par_iter().try_for_each(|locale| {
                                let payload =
                                    provider.load_with_fallback(key, &locale, &fallbacker)?;
                                if let Some(payload) = payload {
                                    exporter.put_payload(key, &locale, &payload)?;
                                }
                                Ok::<(), DataError>(())
                            })?;
                        }
                        options::FallbackMode::PreferredForExporter => unreachable!("resolved"),
                    };

                    match options.fallback {
                        options::FallbackMode::Runtime => exporter
                            .flush_with_built_in_fallback(key, BuiltInFallbackMode::Standard),
                        _ => exporter.flush(key),
                    }
                    .map_err(|e| e.with_key(key))
                })?;

            exporter.close()
        }
        internal(self, options, &mut exporter)
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

    DatagenProvider::new(source.clone()).export(
        Options {
            keys: keys.iter().cloned().collect(),
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
            collations: source.collations.iter().cloned().collect(),
            fallback: FallbackMode::Hybrid,
        },
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
pub use source::CoverageLevel;

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
