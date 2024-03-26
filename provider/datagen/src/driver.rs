// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rayon_prelude::*;
use crate::FallbackMode;
use icu_locid::extensions::unicode::key;
use icu_locid::langid;
use icu_locid::LanguageIdentifier;
use icu_locid_transform::fallback::LocaleFallbackIterator;
use icu_locid_transform::LocaleFallbacker;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::time::Duration;
use std::time::Instant;
use writeable::Writeable;

/// Options bag configuring locale inclusion and behavior when runtime fallback is disabled.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct NoFallbackOptions {}

#[derive(Debug, Clone)]
struct LocalesAndNoFallbackOptions {
    locales: HashSet<LanguageIdentifier>,
    _options: NoFallbackOptions,
}

impl LocalesAndNoFallbackOptions {
    fn describe(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_locales = self
            .locales
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        sorted_locales.sort();
        write!(
            f,
            "without fallback and these locales: {:?}",
            sorted_locales
        )
    }
}

/// Choices for the code location of runtime fallback.
///
/// Some data providers support "internal" fallback in which all data requests automatically run
/// the locale fallback algorithm. If internal fallback is requested for an exporter that does
/// not support it, an error will occur.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub enum RuntimeFallbackLocation {
    /// Include fallbacking code in the exported data provider.
    Internal,
    /// Do not include fallbacking code in the exported data provider.
    ///
    /// The client is responsible for manually configuring [`LocaleFallbackProvider`] in their
    /// runtime data pipeline.
    ///
    /// [`LocaleFallbackProvider`]: icu_provider_adapters::fallback::LocaleFallbackProvider
    External,
}

/// Choices for determining the deduplication of locales for exported data payloads.
///
/// Deduplication affects the lookup table from locales to data payloads. If a child locale
/// points to the same payload as its parent locale, then the child locale can be removed from
/// the lookup table. Therefore, all deduplication strategies guarantee that data requests for
/// selected locales will succeed so long as fallback is enabled at runtime (either internally
/// or externally). They also do not impact which _payloads_ are included: only the lookup table.
///
/// Comparison of the deduplication strategies:
///
/// | Name | Data file size | Supported locale queries? | Needs runtime fallback? |
/// |---|---|---|---|
/// | [`Maximal`] | Smallest | No | Yes |
/// | [`RetainBaseLanguages`] | Small | Yes | Yes |
/// | [`NoDeduplication`] | Medium/Small | Yes | No |
///
/// [`Maximal`]: DeduplicationStrategy::Maximal
/// [`RetainBaseLanguages`]: DeduplicationStrategy::RetainBaseLanguages
/// [`NoDeduplication`]: DeduplicationStrategy::NoDeduplication
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub enum DeduplicationStrategy {
    /// Removes from the lookup table any locale whose parent maps to the same data.
    Maximal,
    // TODO(#58): Add `RetainBaseLanguages`
    /*
    /// Removes from the lookup table any locale whose parent maps to the same data, except if
    /// the parent is `und`.
    RetainBaseLanguages,
    */
    /// Keeps all selected locales in the lookup table.
    NoDeduplication,
}

impl DeduplicationStrategy {
    fn describe(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Maximal => "maximal deduplication",
            // TODO(#58): Add `RetainBaseLanguages`
            // Self::RetainBaseLanguages => "deduplication retaining base languages",
            Self::NoDeduplication => "no deduplication",
        };
        write!(f, "{s}")
    }
}

/// A family of locales to export.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocaleFamily {
    langid: LanguageIdentifier,
    include_ancestors: bool,
    include_descendants: bool,
}

impl LocaleFamily {
    /// The family containing all ancestors and descendants of the selected locale.
    ///
    /// This is the recommended family type since it reflects regional preferences.
    ///
    /// For example, the family `::with_descendants("en-001")` contains:
    ///
    /// - Self: "en-001"
    /// - Ancestors: "und", "en"
    /// - Descendants: "en-GB", "en-ZA", ...
    ///
    /// Stylized on the CLI as: "en-US"
    pub fn with_descendants(langid: LanguageIdentifier) -> Self {
        Self {
            langid,
            include_ancestors: true,
            include_descendants: true,
        }
    }

    /// The family containing all ancestors of the selected locale.
    ///
    /// This family type does not include regional variants unless the selected locale is itself
    /// a regional variant.
    ///
    /// For example, the family `::without_descendants("en-001")` contains:
    ///
    /// - Self: "en-001"
    /// - Ancestors: "und", "en"
    ///
    /// Stylized on the CLI as: "^en-US"
    pub fn without_descendants(langid: LanguageIdentifier) -> Self {
        Self {
            langid,
            include_ancestors: true,
            include_descendants: false,
        }
    }

    /// The family containing only the selected locale.
    ///
    /// For example, the family `::single("en-001")` contains only "en-001".
    ///
    /// Stylized on the CLI as: "@en-US"
    pub fn single(langid: LanguageIdentifier) -> Self {
        Self {
            langid,
            include_ancestors: false,
            include_descendants: false,
        }
    }

    /// The family containing all locales.
    ///
    /// Stylized on the CLI as: "full"
    pub fn full() -> Self {
        Self {
            langid: langid!("und"),
            include_ancestors: false,
            include_descendants: true,
        }
    }
}

impl fmt::Display for LocaleFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.include_ancestors, self.include_descendants) {
            (true, true) => write!(f, "{}", self.langid),
            (true, false) => write!(f, "^{}", self.langid),
            (false, false) => write!(f, "@{}", self.langid),
            (false, true) => write!(f, "full"),
        }
    }
}

/// Options bag configuring locale inclusion and behavior when runtime fallback is enabled.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct FallbackOptions {
    /// The location in code where fallback will be performed at runtime.
    ///
    /// If not set, determined by the exporter: internal fallback is used if the exporter
    /// supports internal fallback.
    pub runtime_fallback_location: Option<RuntimeFallbackLocation>,
    /// The aggressiveness of deduplication of data payloads.
    ///
    /// If not set, determined by `runtime_fallback_location`. If internal fallback is enabled,
    /// a more aggressive deduplication strategy is used.
    pub deduplication_strategy: Option<DeduplicationStrategy>,
}

#[derive(Debug, Clone)]
struct LocalesAndFallbackOptions {
    locales: HashSet<LocaleFamily>,
    options: FallbackOptions,
}

impl LocalesAndFallbackOptions {
    fn describe(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_locales = self
            .locales
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        sorted_locales.sort();
        let start = match self.options.runtime_fallback_location {
            None => "with fallback",
            Some(RuntimeFallbackLocation::Internal) => "with internal fallback",
            Some(RuntimeFallbackLocation::External) => "with external fallback",
        };
        write!(f, "{start}, ")?;
        match self.options.deduplication_strategy {
            Some(x) => x.describe(f)?,
            None => write!(f, "default deduplication")?,
        }
        write!(f, ", and these locales: {:?}", sorted_locales)
    }
}

#[derive(Debug, Clone)]
enum LocalesWithOrWithoutFallback {
    WithFallback(LocalesAndFallbackOptions),
    WithoutFallback(LocalesAndNoFallbackOptions),
}

impl fmt::Display for LocalesWithOrWithoutFallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WithFallback(config) => config.describe(f),
            Self::WithoutFallback(config) => config.describe(f),
        }
    }
}

/// Configuration for a data export operation.
///
/// Note that this only configures *which data* is exported. The input provider, usually
/// `DatagenProvider`, might expose more options about the data itself.
///
/// # Examples
///
/// ```no_run
/// use icu_datagen::blob_exporter::*;
/// use icu_datagen::prelude::*;
///
/// DatagenDriver::new()
///     .with_keys([icu::list::provider::AndListV1Marker::KEY])
///     .with_all_locales()
///     .export(
///         &DatagenProvider::new_latest_tested(),
///         BlobExporter::new_with_sink(Box::new(&mut Vec::new())),
///     )
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct DatagenDriver {
    keys: Option<HashSet<DataKey>>,
    locales_fallback: Option<LocalesWithOrWithoutFallback>,
    // `None` means not set, `Some(None)` means all
    legacy_locales: Option<Option<Vec<LanguageIdentifier>>>,
    legacy_fallback_mode: FallbackMode,
    additional_collations: HashSet<String>,
    segmenter_models: Vec<String>,
}

impl DatagenDriver {
    /// Creates an empty [`DatagenDriver`].
    ///
    /// Note that keys and locales need to be set before calling [`export`](Self::export).
    #[allow(clippy::new_without_default)] // this is not directly usable
    pub fn new() -> Self {
        Self {
            keys: None,
            locales_fallback: None,
            legacy_fallback_mode: FallbackMode::default(),
            legacy_locales: None,
            additional_collations: HashSet::new(),
            segmenter_models: Vec::new(),
        }
        .with_recommended_segmenter_models()
    }

    /// Sets this driver to generate the given keys.
    ///
    /// See [`icu_datagen::keys`], [`icu_datagen::all_keys`], [`icu_datagen::key`] and [`icu_datagen::keys_from_bin`].
    ///
    /// [`icu_datagen::keys`]: crate::keys
    /// [`icu_datagen::all_keys`]: crate::all_keys
    /// [`icu_datagen::key`]: crate::key
    /// [`icu_datagen::keys_from_bin`]: crate::keys_from_bin
    pub fn with_keys(self, keys: impl IntoIterator<Item = DataKey>) -> Self {
        Self {
            keys: Some(keys.into_iter().collect()),
            ..self
        }
    }

    /// Sets this driver to generate the given locales.
    ///
    /// Use the [`langid!`] macro from the prelude to create an
    /// explicit list, or [`DatagenProvider::locales_for_coverage_levels`] for CLDR coverage levels.
    ///
    /// [`langid!`]: crate::prelude::langid
    /// [`DatagenProvider::locales_for_coverage_levels`]: crate::DatagenProvider::locales_for_coverage_levels
    #[deprecated(
        since = "1.5.0",
        note = "use `with_locales_and_fallback` or `with_locales_no_fallback`"
    )]
    pub fn with_locales(self, locales: impl IntoIterator<Item = LanguageIdentifier>) -> Self {
        Self {
            legacy_locales: Some(Some(locales.into_iter().collect())),
            ..self
        }
    }

    /// Sets this driver to generate all available locales.
    #[deprecated(since = "1.5.0", note = "use `with_locales_and_fallback`")]
    pub fn with_all_locales(self) -> Self {
        Self {
            legacy_locales: Some(None),
            ..self
        }
    }

    /// Sets this driver to generate the given locales assuming no runtime fallback.
    ///
    /// Use the [`langid!`] macro from the prelude to create an
    /// explicit list, or [`DatagenProvider::locales_for_coverage_levels`] for CLDR coverage levels.
    ///
    /// [`langid!`]: crate::prelude::langid
    /// [`DatagenProvider::locales_for_coverage_levels`]: crate::DatagenProvider::locales_for_coverage_levels
    pub fn with_locales_no_fallback(
        self,
        locales: impl IntoIterator<Item = LanguageIdentifier>,
        options: NoFallbackOptions,
    ) -> Self {
        Self {
            locales_fallback: Some(LocalesWithOrWithoutFallback::WithoutFallback(
                LocalesAndNoFallbackOptions {
                    locales: locales.into_iter().collect(),
                    _options: options,
                },
            )),
            ..self
        }
    }

    /// Sets this driver to generate the given locales assuming runtime fallback.
    ///
    /// Use the [`langid!`] macro from the prelude to create an
    /// explicit list, or [`DatagenProvider::locales_for_coverage_levels`] for CLDR coverage levels.
    ///
    /// [`langid!`]: crate::prelude::langid
    /// [`DatagenProvider::locales_for_coverage_levels`]: crate::DatagenProvider::locales_for_coverage_levels
    pub fn with_locales_and_fallback(
        self,
        locales: impl IntoIterator<Item = LocaleFamily>,
        options: FallbackOptions,
    ) -> Self {
        Self {
            locales_fallback: Some(LocalesWithOrWithoutFallback::WithFallback(
                LocalesAndFallbackOptions {
                    locales: locales.into_iter().collect(),
                    options,
                },
            )),
            ..self
        }
    }

    /// Sets the fallback type that the data should be generated for.
    ///
    /// If locale fallback is used at runtime, smaller data can be generated.
    #[deprecated(
        since = "1.5.0",
        note = "use `with_locales_and_fallback` or `with_locales_no_fallback`"
    )]
    pub fn with_fallback_mode(self, fallback: FallbackMode) -> Self {
        Self {
            legacy_fallback_mode: fallback,
            ..self
        }
    }

    /// This option is only relevant if using `icu::collator`.
    ///
    /// By default, the collations `big5han`, `gb2312`, and those starting with `search`
    /// are excluded. This method can be used to reennable them.
    ///
    /// The special string `"search*"` causes all search collation tables to be included.
    pub fn with_additional_collations(
        self,
        additional_collations: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            additional_collations: additional_collations.into_iter().collect(),
            ..self
        }
    }

    /// This option is only relevant if using `icu::segmenter`.
    ///
    /// Sets this driver to generate the recommended segmentation models, to the extent required by the
    /// chosen data keys.
    pub fn with_recommended_segmenter_models(self) -> Self {
        self.with_segmenter_models([
            "cjdict".into(),
            "burmesedict".into(),
            "khmerdict".into(),
            "laodict".into(),
            "thaidict".into(),
            "Burmese_codepoints_exclusive_model4_heavy".into(),
            "Khmer_codepoints_exclusive_model4_heavy".into(),
            "Lao_codepoints_exclusive_model4_heavy".into(),
            "Thai_codepoints_exclusive_model4_heavy".into(),
        ])
    }

    /// This option is only relevant if using `icu::segmenter`.
    ///
    /// Sets this driver to generate the given segmentation models, to the extent required by the
    /// chosen data keys.
    ///
    /// The currently supported dictionary models are
    /// * `cjdict`
    /// * `burmesedict`
    /// * `khmerdict`
    /// * `laodict`
    /// * `thaidict`
    ///
    /// The currently supported LSTM models are
    /// * `Burmese_codepoints_exclusive_model4_heavy`
    /// * `Khmer_codepoints_exclusive_model4_heavy`
    /// * `Lao_codepoints_exclusive_model4_heavy`
    /// * `Thai_codepoints_exclusive_model4_heavy`
    ///
    /// If a model is not included, the resulting line or word segmenter will apply rule-based
    /// segmentation when encountering text in a script that requires the model, which will be
    /// incorrect.
    ///
    /// If multiple models for the same language and segmentation type (dictionary/LSTM) are
    /// listed, the first one will be used.
    pub fn with_segmenter_models(self, models: impl IntoIterator<Item = String>) -> Self {
        Self {
            segmenter_models: models.into_iter().collect(),
            ..self
        }
    }

    /// Exports data from the given provider to the given exporter.
    ///
    /// See
    /// [`DatagenProvider`](crate::DatagenProvider),
    /// [`make_exportable_provider!`](icu_provider::make_exportable_provider),
    /// [`BlobExporter`](icu_provider_blob::export),
    /// [`FileSystemExporter`](icu_provider_fs::export),
    /// and [`BakedExporter`](crate::baked_exporter).
    pub fn export(
        self,
        provider: &impl ExportableProvider,
        mut sink: impl DataExporter,
    ) -> Result<(), DataError> {
        self.export_dyn(provider, &mut sink)
    }

    // Avoids multiple monomorphizations
    fn export_dyn(
        self,
        provider: &dyn ExportableProvider,
        sink: &mut dyn DataExporter,
    ) -> Result<(), DataError> {
        let Self {
            keys,
            locales_fallback,
            legacy_locales,
            legacy_fallback_mode,
            additional_collations,
            segmenter_models,
        } = self;

        let Some(keys) = keys else {
            return Err(DataError::custom(
                "`DatagenDriver::with_keys` needs to be called",
            ));
        };

        let map_legacy_locales_to_locales_with_expansion =
            |legacy_locales: Option<Vec<LanguageIdentifier>>| match legacy_locales {
                Some(v) => v.into_iter().map(LocaleFamily::with_descendants).collect(),
                None => [LocaleFamily::full()].into_iter().collect(),
            };

        let locales_fallback = match (locales_fallback, legacy_locales, legacy_fallback_mode) {
            // 1.5 API
            (Some(locales_fallback), _, _) => locales_fallback,
            // 1.4 API
            (_, Some(legacy_locales), FallbackMode::PreferredForExporter) => {
                LocalesWithOrWithoutFallback::WithFallback(LocalesAndFallbackOptions {
                    locales: map_legacy_locales_to_locales_with_expansion(legacy_locales),
                    options: FallbackOptions {
                        runtime_fallback_location: None,
                        deduplication_strategy: None,
                    },
                })
            }
            (_, Some(legacy_locales), FallbackMode::Runtime) => {
                LocalesWithOrWithoutFallback::WithFallback(LocalesAndFallbackOptions {
                    locales: map_legacy_locales_to_locales_with_expansion(legacy_locales),
                    options: FallbackOptions {
                        runtime_fallback_location: Some(RuntimeFallbackLocation::Internal),
                        deduplication_strategy: Some(DeduplicationStrategy::Maximal),
                    },
                })
            }
            (_, Some(legacy_locales), FallbackMode::RuntimeManual) => {
                LocalesWithOrWithoutFallback::WithFallback(LocalesAndFallbackOptions {
                    locales: map_legacy_locales_to_locales_with_expansion(legacy_locales),
                    options: FallbackOptions {
                        runtime_fallback_location: Some(RuntimeFallbackLocation::External),
                        deduplication_strategy: Some(DeduplicationStrategy::Maximal),
                    },
                })
            }
            (_, Some(Some(locales)), FallbackMode::Preresolved) => {
                LocalesWithOrWithoutFallback::WithoutFallback(LocalesAndNoFallbackOptions {
                    locales: locales.into_iter().collect(),
                    _options: NoFallbackOptions {},
                })
            }
            (_, Some(None), FallbackMode::Preresolved) => {
                return Err(DataError::custom(
                    "FallbackMode::Preresolved requires an explicit locale set",
                ));
            }
            (_, Some(legacy_locales), FallbackMode::Hybrid) => {
                LocalesWithOrWithoutFallback::WithFallback(LocalesAndFallbackOptions {
                    locales: map_legacy_locales_to_locales_with_expansion(legacy_locales),
                    options: FallbackOptions {
                        runtime_fallback_location: Some(RuntimeFallbackLocation::External),
                        deduplication_strategy: Some(DeduplicationStrategy::NoDeduplication),
                    },
                })
            }
            // Failure case
            _ => {
                return Err(DataError::custom(
                    "`DatagenDriver::with_locales` or `with_all_locales` or `with_locales_and_fallback` or `with_locales_no_fallback` needs to be called",
                ));
            }
        };

        if keys.is_empty() {
            log::warn!("No keys selected");
        }

        log::info!("Datagen configured {locales_fallback}");

        let deduplication_strategy = match &locales_fallback {
            LocalesWithOrWithoutFallback::WithoutFallback(_) => {
                DeduplicationStrategy::NoDeduplication
            }
            LocalesWithOrWithoutFallback::WithFallback(config) => {
                match config.options.deduplication_strategy {
                    // TODO: Default to RetainBaseLanguages here
                    None => {
                        if sink.supports_built_in_fallback() {
                            DeduplicationStrategy::Maximal
                        } else {
                            DeduplicationStrategy::NoDeduplication
                        }
                    }
                    Some(x) => x,
                }
            }
        };

        let uses_internal_fallback = match &locales_fallback {
            LocalesWithOrWithoutFallback::WithoutFallback(_) => false,
            LocalesWithOrWithoutFallback::WithFallback(config) => {
                match config.options.runtime_fallback_location {
                    None => sink.supports_built_in_fallback(),
                    Some(RuntimeFallbackLocation::Internal) => true,
                    Some(RuntimeFallbackLocation::External) => false,
                }
            }
        };

        let fallbacker =
            Lazy::new(|| LocaleFallbacker::try_new_with_any_provider(&provider.as_any_provider()));

        let load_with_fallback = |key, locale: &_| {
            log::trace!("Generating key/locale: {key}/{locale:}");
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            // Lazy-compute the fallback iterator so that we don't always require CLDR data
            let mut locale_iter: Option<LocaleFallbackIterator> = None;
            loop {
                let req = DataRequest {
                    locale: locale_iter.as_ref().map(|i| i.get()).unwrap_or(locale),
                    metadata,
                };
                match provider.load_data(key, req) {
                    Ok(data_response) => {
                        if let Some(iter) = locale_iter.as_ref() {
                            if iter.get().is_und() && !locale.is_und() {
                                log::debug!("Falling back to und: {key}/{locale}");
                            }
                        }
                        return Some(
                            data_response
                                .take_payload()
                                .map_err(|e| e.with_req(key, req)),
                        );
                    }
                    Err(DataError {
                        kind: DataErrorKind::MissingLocale,
                        ..
                    }) => {
                        if let Some(iter) = locale_iter.as_mut() {
                            if iter.get().is_und() {
                                log::debug!("Could not find data for: {key}/{locale}");
                                return None;
                            }
                            iter.step();
                        } else {
                            match fallbacker.as_ref() {
                                Ok(fallbacker) => {
                                    locale_iter = Some(
                                        fallbacker
                                            .for_config(key.fallback_config())
                                            .fallback_for(locale.clone()),
                                    )
                                }
                                Err(e) => return Some(Err(*e)),
                            }
                        }
                    }
                    Err(e) => return Some(Err(e.with_req(key, req))),
                }
            }
        };

        keys.clone().into_par_iter().try_for_each(|key| {
            log::trace!("Generating key {key}");
            let instant1 = Instant::now();

            if key.metadata().singleton {
                if provider.supported_locales_for_key(key)? != [Default::default()] {
                    return Err(
                        DataError::custom("Invalid supported locales for singleton key")
                            .with_key(key),
                    );
                }

                let payload = provider
                    .load_data(key, Default::default())
                    .and_then(DataResponse::take_payload)
                    .map_err(|e| e.with_req(key, Default::default()))?;

                let transform_duration = instant1.elapsed();

                sink.flush_singleton(key, &payload)
                    .map_err(|e| e.with_req(key, Default::default()))?;

                let final_duration = instant1.elapsed();
                let flush_duration = final_duration - transform_duration;

                if final_duration > Duration::new(0, 500_000_000) {
                    // Print durations if the key took longer than 500 ms
                    log::info!(
                        "Generated key {key} ({}, flushed in {})",
                        DisplayDuration(final_duration),
                        DisplayDuration(flush_duration)
                    );
                } else {
                    log::info!("Generated key {key}");
                }
                return Ok(());
            }

            let locales_to_export = select_locales_for_key(
                provider,
                key,
                &locales_fallback,
                &additional_collations,
                &segmenter_models,
                &fallbacker,
            )?;

            let (slowest_duration, slowest_locale) = match deduplication_strategy {
                // TODO(#58): Add `RetainBaseLanguages`
                // DeduplicationStrategy::RetainBaseLanguages => todo!(),
                DeduplicationStrategy::Maximal => {
                    let payloads = locales_to_export
                        .into_par_iter()
                        .filter_map(|locale| {
                            let instant2 = Instant::now();
                            load_with_fallback(key, &locale)
                                .map(|r| r.map(|payload| (locale, (payload, instant2.elapsed()))))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?;
                    let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
                    let fallbacker_with_config = fallbacker.for_config(key.fallback_config());
                    payloads
                        .iter()
                        .try_for_each(|(locale, (payload, _duration))| {
                            let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                            while !iter.get().is_und() {
                                iter.step();
                                if let Some((inherited_payload, _duration)) =
                                    payloads.get(iter.get())
                                {
                                    if inherited_payload == payload {
                                        // Found a match: don't need to write anything
                                        log::trace!(
                                            "Deduplicating {key}/{locale} (inherits from {})",
                                            iter.get()
                                        );
                                        return Ok(());
                                    } else {
                                        // Not a match: we must include this
                                        break;
                                    }
                                }
                            }
                            // Did not find a match: export this payload
                            sink.put_payload(key, locale, payload).map_err(|e| {
                                e.with_req(
                                    key,
                                    DataRequest {
                                        locale,
                                        metadata: Default::default(),
                                    },
                                )
                            })
                        })?;

                    // Slowest locale calculation:
                    payloads
                        .iter()
                        .map(|(locale, (_payload, duration))| {
                            (*duration, locale.write_to_string().into_owned())
                        })
                        .max()
                }
                DeduplicationStrategy::NoDeduplication => locales_to_export
                    .into_par_iter()
                    .filter_map(|locale| {
                        let instant2 = Instant::now();
                        let result = load_with_fallback(key, &locale)?;
                        let result = result
                            .and_then(|payload| sink.put_payload(key, &locale, &payload))
                            // Note: in Hybrid mode the elapsed time includes sink.put_payload.
                            // In Runtime mode the elapsed time is only load_with_fallback.
                            .map(|_| (instant2.elapsed(), locale.write_to_string().into_owned()))
                            .map_err(|e| {
                                e.with_req(
                                    key,
                                    DataRequest {
                                        locale: &locale,
                                        metadata: Default::default(),
                                    },
                                )
                            });
                        Some(result)
                    })
                    .collect::<Result<Vec<_>, DataError>>()?
                    .into_iter()
                    .max(),
            }
            .unwrap_or_default();

            let transform_duration = instant1.elapsed();

            // segmenter uses hardcoded locales internally, so fallback is not necessary.
            // TODO(#4511): Use auxiliary keys for segmenter
            if uses_internal_fallback && !key.path().get().starts_with("segmenter") {
                sink.flush_with_built_in_fallback(key, BuiltInFallbackMode::Standard)
            } else {
                sink.flush(key)
            }
            .map_err(|e| e.with_key(key))?;

            let final_duration = instant1.elapsed();
            let flush_duration = final_duration - transform_duration;

            if final_duration > Duration::new(0, 500_000_000) {
                // Print durations if the key took longer than 500 ms
                log::info!(
                    "Generated key {key} ({}, '{slowest_locale}' in {}, flushed in {})",
                    DisplayDuration(final_duration),
                    DisplayDuration(slowest_duration),
                    DisplayDuration(flush_duration)
                );
            } else {
                log::info!("Generated key {key}");
            }
            Ok(())
        })?;

        sink.close()
    }
}

struct ExplicitImplicitLocaleSets {
    explicit: HashSet<DataLocale>,
    implicit: HashSet<DataLocale>,
}

/// Resolves the set of explicit langids and the supported locales into two sets of locales:
///
/// - `explicit` contains the explicit langids but with aux keys and extension keywords included.
///   For example, if `ar-SA` is requested (explicit langid), and `ar` and `ar-u-nu-latn` are supported,
///   then `ar-SA` and `ar-SA-u-nu-latn` will be returned as `explicit`.
/// - `implcit` contains all supported locales reachable by fallback from an `explicit` locale.
///   These locales can be included without increasing data payload size.
fn make_explicit_implicit_sets(
    key: DataKey,
    explicit_langids: &mut dyn Iterator<Item = (&LanguageIdentifier, bool)>,
    supported_map: &HashMap<LanguageIdentifier, HashSet<DataLocale>>,
    fallbacker: &Lazy<
        Result<LocaleFallbacker, DataError>,
        impl FnOnce() -> Result<LocaleFallbacker, DataError>,
    >,
) -> Result<ExplicitImplicitLocaleSets, DataError> {
    let mut implicit = HashSet::new();
    let mut explicit: HashSet<DataLocale> = Default::default();
    for (explicit_langid, include_ancestors) in explicit_langids {
        explicit.insert(explicit_langid.into());
        if let Some(locales) = supported_map.get(explicit_langid) {
            explicit.extend(locales.iter().cloned()); // adds ar-EG-u-nu-latn
        }
        if explicit_langid == &LanguageIdentifier::UND {
            continue;
        }
        let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
        let fallbacker_with_config = fallbacker.for_config(key.fallback_config());
        let mut iter = fallbacker_with_config.fallback_for(explicit_langid.into());
        loop {
            if include_ancestors {
                implicit.insert(iter.get().clone());
            }
            if iter.get().is_und() {
                break;
            }
            // Inherit aux keys and extension keywords from parent locales
            let iter_langid = iter.get().get_langid();
            if let Some(locales) = supported_map.get(&iter_langid) {
                if include_ancestors {
                    implicit.extend(locales.iter().cloned()); // adds ar-u-nu-latn
                }
                for locale in locales {
                    let mut morphed_locale = locale.clone();
                    morphed_locale.set_langid(explicit_langid.clone());
                    explicit.insert(morphed_locale); // adds ar-SA-u-nu-latn
                }
            }
            iter.step();
        }
    }
    Ok(ExplicitImplicitLocaleSets { explicit, implicit })
}

/// Selects the maximal set of locales to export based on a [`DataKey`] and this datagen
/// provider's options bag. The locales may be later optionally deduplicated for fallback.
fn select_locales_for_key(
    provider: &dyn ExportableProvider,
    key: DataKey,
    locales_fallback: &LocalesWithOrWithoutFallback,
    additional_collations: &HashSet<String>,
    segmenter_models: &[String],
    fallbacker: &Lazy<
        Result<LocaleFallbacker, DataError>,
        impl FnOnce() -> Result<LocaleFallbacker, DataError>,
    >,
) -> Result<HashSet<icu_provider::DataLocale>, DataError> {
    // A map from langid to data locales. Keys that have aux keys or extension keywords
    // may have multiple data locales per langid.
    let mut supported_map: HashMap<LanguageIdentifier, HashSet<DataLocale>> = Default::default();
    for locale in provider
        .supported_locales_for_key(key)
        .map_err(|e| e.with_key(key))?
    {
        use std::collections::hash_map::Entry;
        match supported_map.entry(locale.get_langid()) {
            Entry::Occupied(mut entry) => entry.get_mut().insert(locale),
            Entry::Vacant(entry) => entry.insert(Default::default()).insert(locale),
        };
    }

    if key == icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker::KEY
        || key == icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker::KEY
    {
        supported_map.retain(|_, locales| {
            locales.retain(|locale| {
                let model =
                    crate::transform::segmenter::dictionary::data_locale_to_model_name(locale);
                segmenter_models.iter().any(|m| Some(m.as_ref()) == model)
            });
            !locales.is_empty()
        });
        // Don't perform additional locale filtering
        return Ok(supported_map.into_values().flatten().collect());
    } else if key == icu_segmenter::provider::LstmForWordLineAutoV1Marker::KEY {
        supported_map.retain(|_, locales| {
            locales.retain(|locale| {
                let model = crate::transform::segmenter::lstm::data_locale_to_model_name(locale);
                segmenter_models.iter().any(|m| Some(m.as_ref()) == model)
            });
            !locales.is_empty()
        });
        // Don't perform additional locale filtering
        return Ok(supported_map.into_values().flatten().collect());
    } else if key == icu_collator::provider::CollationDataV1Marker::KEY
        || key == icu_collator::provider::CollationDiacriticsV1Marker::KEY
        || key == icu_collator::provider::CollationJamoV1Marker::KEY
        || key == icu_collator::provider::CollationMetadataV1Marker::KEY
        || key == icu_collator::provider::CollationReorderingV1Marker::KEY
        || key == icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY
    {
        supported_map.retain(|_, locales| {
            locales.retain(|locale| {
                let Some(collation) = locale
                    .get_unicode_ext(&key!("co"))
                    .and_then(|co| co.as_single_subtag().copied())
                else {
                    return true;
                };
                additional_collations.contains(collation.as_str())
                    || if collation.starts_with("search") {
                        additional_collations.contains("search*")
                    } else {
                        !["big5han", "gb2312"].contains(&collation.as_str())
                    }
            });
            !locales.is_empty()
        });
    }

    let config = match locales_fallback {
        // `FallbackMode::Preresolved` exports all supported locales whose langid matches
        // one of the explicit locales. This ensures extensions are included. In addition, any
        // explicit locales are added to the list, even if they themselves don't contain data;
        // fallback should be performed upon exporting.
        LocalesWithOrWithoutFallback::WithoutFallback(config) => {
            let mut it = config.locales.iter().map(|langid| (langid, false));
            let ExplicitImplicitLocaleSets { explicit, .. } =
                make_explicit_implicit_sets(key, &mut it, &supported_map, fallbacker)?;
            return Ok(explicit);
        }
        // All other modes resolve to fallback-aware inclusion.
        LocalesWithOrWithoutFallback::WithFallback(config) => config,
    };

    let mut it = config
        .locales
        .iter()
        .map(|x| (&x.langid, x.include_ancestors));
    let ExplicitImplicitLocaleSets { explicit, implicit } =
        make_explicit_implicit_sets(key, &mut it, &supported_map, fallbacker)?;

    // TODO(#4629): Allow the exclusion of und
    let include_und = true;

    let include_full = config.locales.contains(&LocaleFamily::full());

    let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
    let fallbacker_with_config = fallbacker.for_config(key.fallback_config());

    let result = supported_map
        .into_values()
        .flatten()
        .chain(explicit.iter().cloned())
        .filter(|locale_orig| {
            if include_full {
                return true;
            }
            let mut locale = locale_orig.clone();
            locale.remove_aux();
            if implicit.contains(&locale) {
                return true;
            }
            if explicit.contains(&locale) {
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
            if key == icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY
                && !locale.has_unicode_ext()
            {
                return false;
            }
            let mut iter = fallbacker_with_config.fallback_for(locale);
            while !iter.get().is_und() {
                if explicit.contains(iter.get()) {
                    return true;
                }
                iter.step();
            }
            log::trace!("Filtered out: {key}/{locale_orig}"); // this will print aux keys too but it avoids a clone
            false
        })
        .collect();

    Ok(result)
}

struct DisplayDuration(pub Duration);

impl fmt::Display for DisplayDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nanos = self.0.as_nanos();
        if nanos > 100_000_000 {
            write!(f, "{:.3}s", self.0.as_secs_f64())
        } else if nanos > 1_000_000 {
            write!(f, "{:.3}ms", (nanos as f64) / 1e6)
        } else if nanos > 1_000 {
            write!(f, "{:.3}µs", (nanos as f64) / 1e3)
        } else {
            write!(f, "{}ns", nanos)
        }
    }
}

#[test]
fn test_collation_filtering() {
    use icu_locid::langid;
    use std::collections::BTreeSet;

    #[derive(Debug)]
    struct TestCase<'a> {
        include_collations: &'a [&'a str],
        language: LanguageIdentifier,
        expected: &'a [&'a str],
    }
    let cases = [
        TestCase {
            include_collations: &[],
            language: langid!("zh"),
            expected: &["zh", "zh-u-co-stroke", "zh-u-co-unihan", "zh-u-co-zhuyin"],
        },
        TestCase {
            include_collations: &["gb2312"],
            language: langid!("zh"),
            expected: &[
                "zh",
                "zh-u-co-gb2312",
                "zh-u-co-stroke",
                "zh-u-co-unihan",
                "zh-u-co-zhuyin",
            ],
        },
        TestCase {
            include_collations: &["big5han"],
            language: langid!("zh"),
            expected: &[
                "zh",
                "zh-u-co-big5han",
                "zh-u-co-stroke",
                "zh-u-co-unihan",
                "zh-u-co-zhuyin",
            ],
        },
        TestCase {
            include_collations: &["gb2312", "search*"],
            language: langid!("zh"),
            expected: &[
                "zh",
                "zh-u-co-gb2312",
                "zh-u-co-stroke",
                "zh-u-co-unihan",
                "zh-u-co-zhuyin",
            ],
        },
        TestCase {
            include_collations: &[],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["search"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-search", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["searchjl"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-searchjl", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["search", "searchjl"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-search", "ko-u-co-searchjl", "ko-u-co-unihan"],
        },
        TestCase {
            include_collations: &["search*", "big5han"],
            language: langid!("ko"),
            expected: &["ko", "ko-u-co-search", "ko-u-co-searchjl", "ko-u-co-unihan"],
        },
    ];
    for cas in cases {
        let resolved_locales = select_locales_for_key(
            &crate::DatagenProvider::new_testing(),
            icu_collator::provider::CollationDataV1Marker::KEY,
            &LocalesWithOrWithoutFallback::WithoutFallback(LocalesAndNoFallbackOptions {
                locales: [cas.language.clone()].into_iter().collect(),
                _options: Default::default(),
            }),
            &HashSet::from_iter(cas.include_collations.iter().copied().map(String::from)),
            &[],
            &once_cell::sync::Lazy::new(|| Ok(LocaleFallbacker::new_without_data())),
        )
        .unwrap()
        .into_iter()
        .map(|l| l.to_string())
        .collect::<BTreeSet<_>>();
        let expected_locales = cas
            .expected
            .iter()
            .copied()
            .map(String::from)
            .collect::<BTreeSet<_>>();
        assert_eq!(resolved_locales, expected_locales, "{cas:?}");
    }
}
