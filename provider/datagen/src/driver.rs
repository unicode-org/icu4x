// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rayon_prelude::*;
use crate::FallbackMode;
use icu_locid::extensions::unicode::key;
use icu_locid::LanguageIdentifier;
use icu_locid_transform::fallback::LocaleFallbackIterator;
use icu_locid_transform::LocaleFallbacker;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::time::Duration;
use std::time::Instant;
use writeable::Writeable;

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
    // `None` means not set, `Some(None)` means all
    locales: Option<Option<HashSet<LanguageIdentifier>>>,
    fallback: FallbackMode,
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
            locales: None,
            fallback: FallbackMode::default(),
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
    pub fn with_locales(self, locales: impl IntoIterator<Item = LanguageIdentifier>) -> Self {
        Self {
            locales: Some(Some(locales.into_iter().collect())),
            ..self
        }
    }

    /// Sets this driver to generate all available locales.
    pub fn with_all_locales(self) -> Self {
        Self {
            locales: Some(None),
            ..self
        }
    }

    /// Sets the fallback type that the data should be generated for.
    ///
    /// If locale fallback is used at runtime, smaller data can be generated.
    pub fn with_fallback_mode(self, fallback: FallbackMode) -> Self {
        Self { fallback, ..self }
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
            locales,
            fallback,
            additional_collations,
            segmenter_models,
        } = self;

        let Some(keys) = keys else {
            return Err(DataError::custom(
                "`DatagenDriver::with_keys` needs to be called",
            ));
        };

        let Some(locales) = locales else {
            return Err(DataError::custom(
                "`DatagenDriver::with_locales` or `with_all_locales` needs to be called",
            ));
        };

        if keys.is_empty() {
            log::warn!("No keys selected");
        }

        if matches!(fallback, FallbackMode::Preresolved) && locales.is_none() {
            return Err(DataError::custom(
                "FallbackMode::Preresolved requires an explicit locale set",
            ));
        }

        let fallback = match fallback {
            FallbackMode::PreferredForExporter => {
                if sink.supports_built_in_fallback() {
                    FallbackMode::Runtime
                } else {
                    FallbackMode::Hybrid
                }
            }
            f => f,
        };

        log::info!(
            "Datagen configured with fallback mode {:?} and these locales: {}",
            fallback,
            match locales {
                None => "ALL".to_string(),
                Some(ref set) => {
                    let mut list: Vec<Cow<str>> =
                        set.iter().map(Writeable::write_to_string).collect();
                    list.sort();
                    format!("{:?}", list)
                }
            }
        );

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
                fallback,
                locales.as_ref(),
                &additional_collations,
                &segmenter_models,
                &fallbacker,
            )?;

            let (slowest_duration, slowest_locale) = match fallback {
                FallbackMode::Runtime | FallbackMode::RuntimeManual => {
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
                FallbackMode::Hybrid | FallbackMode::Preresolved => locales_to_export
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
                FallbackMode::PreferredForExporter => unreachable!("resolved"),
            }
            .unwrap_or_default();

            let transform_duration = instant1.elapsed();

            // segmenter uses hardcoded locales internally, so fallback is not necessary.
            // TODO(#4511): Use auxiliary keys for segmenter
            if fallback == FallbackMode::Runtime && !key.path().get().starts_with("segmenter") {
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
/// - `explicit` contains the explicit langids but with aux keys and extension keywords included
/// - `implcit` contains any locale reachable by fallback from an `explicit` locale
fn make_explicit_implicit_sets(
    key: DataKey,
    explicit_langids: &HashSet<LanguageIdentifier>,
    supported_map: &HashMap<LanguageIdentifier, HashSet<DataLocale>>,
    fallbacker: &Lazy<
        Result<LocaleFallbacker, DataError>,
        impl FnOnce() -> Result<LocaleFallbacker, DataError>,
    >,
) -> Result<ExplicitImplicitLocaleSets, DataError> {
    let mut implicit = HashSet::new();
    // TODO: Make including the default locale configurable
    implicit.insert(DataLocale::default());

    let mut explicit: HashSet<DataLocale> = Default::default();
    for explicit_langid in explicit_langids.iter() {
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
        while !iter.get().is_und() {
            implicit.insert(iter.get().clone());
            // Inherit aux keys and extension keywords from parent locales
            let iter_langid = iter.get().get_langid();
            if let Some(locales) = supported_map.get(&iter_langid) {
                implicit.extend(locales.iter().cloned()); // adds ar-u-nu-latn
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
    fallback: FallbackMode,
    explicit_langids: Option<&HashSet<LanguageIdentifier>>,
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

    let result = match (explicit_langids, fallback) {
        // Case 1: `None` simply exports all supported locales for this key.
        (None, _) => supported_map.into_values().flatten().collect(),
        // Case 2: `FallbackMode::Preresolved` exports all supported locales whose langid matches
        // one of the explicit locales. This ensures extensions are included. In addition, any
        // explicit locales are added to the list, even if they themselves don't contain data;
        // fallback should be performed upon exporting.
        (Some(explicit_langids), FallbackMode::Preresolved) => {
            let ExplicitImplicitLocaleSets { explicit, .. } =
                make_explicit_implicit_sets(key, explicit_langids, &supported_map, fallbacker)?;
            explicit
        }
        // Case 3: All other modes resolve to the "ancestors and descendants" strategy.
        (Some(explicit_langids), _) => {
            let include_und = explicit_langids.contains(&LanguageIdentifier::UND);

            let ExplicitImplicitLocaleSets { explicit, implicit } =
                make_explicit_implicit_sets(key, explicit_langids, &supported_map, fallbacker)?;

            let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
            let fallbacker_with_config = fallbacker.for_config(key.fallback_config());

            supported_map
                .into_values()
                .flatten()
                .chain(explicit.iter().cloned())
                .filter(|locale_orig| {
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
                .collect()
        }
    };

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
            FallbackMode::Preresolved,
            Some(&HashSet::from_iter([cas.language.clone()])),
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
