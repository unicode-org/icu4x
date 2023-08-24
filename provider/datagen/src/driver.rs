// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rayon_prelude::*;
use crate::FallbackMode;
use icu_locid::extensions::unicode::key;
use icu_locid::LanguageIdentifier;
use icu_locid_transform::fallback::LocaleFallbackIterator;
use icu_locid_transform::fallback::LocaleFallbacker;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use writeable::Writeable;

/// Configuration for a data export operation.
///
/// # Examples
///
/// ```no_run
/// use icu_datagen::prelude::*;
/// use icu_datagen::blob_exporter::*;
///
/// DatagenDriver::new()
///       .with_keys([icu::list::provider::AndListV1Marker::KEY])
///       .export(&DatagenProvider::latest_tested(), BlobExporter::new_with_sink(Box::new(&mut Vec::new())))
///       .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct DatagenDriver {
    keys: HashSet<DataKey>,
    // `None` means all
    locales: Option<HashSet<LanguageIdentifier>>,
    fallback: FallbackMode,
    collations: HashSet<String>,
    segmenter_models: Vec<String>,
}

impl DatagenDriver {
    /// Creates an empty [`DatagenDriver`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets this driver to generate the given keys. See [`icu_datagen::keys`],
    /// [`icu_datagen::all_keys`], [`icu_datagen::key`] and [`icu_datagen::keys_from_bin`].
    ///
    /// [`icu_datagen::keys`]: crate::keys
    /// [`icu_datagen::all_keys`]: crate::all_keys
    /// [`icu_datagen::key`]: crate::key
    /// [`icu_datagen::keys_from_bin`]: crate::keys_from_bin
    pub fn with_keys(self, keys: impl IntoIterator<Item = DataKey>) -> Self {
        Self {
            keys: keys.into_iter().collect(),
            ..self
        }
    }

    /// Sets the fallback type that the data should be generated for. If locale fallback is
    /// used at runtime, smaller data can be generated.
    pub fn with_fallback_mode(self, fallback: FallbackMode) -> Self {
        Self { fallback, ..self }
    }

    /// Sets the locales to generate.
    pub fn with_locales(self, locales: impl IntoIterator<Item = LanguageIdentifier>) -> Self {
        Self {
            locales: Some(locales.into_iter().collect()),
            ..self
        }
    }

    /// Sets this driver to generate all available locales.
    pub fn with_all_locales(self) -> Self {
        Self {
            locales: None,
            ..self
        }
    }

    /// By default, the collations `big5han`, `gb2312`, and those starting with `search`
    /// are excluded. This method can be used to reennable them.
    ///
    /// The special string `"search*"` causes all search collation tables to be included.
    pub fn with_collations(self, collations: impl IntoIterator<Item = String>) -> Self {
        Self {
            collations: collations.into_iter().collect(),
            ..self
        }
    }

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
        mut self,
        provider: &dyn ExportableProvider,
        sink: &mut dyn DataExporter,
    ) -> Result<(), DataError> {
        if self.keys.is_empty() {
            log::warn!("No keys selected");
        }

        if matches!(self.fallback, FallbackMode::Preresolved) && self.locales.is_none() {
            return Err(DataError::custom(
                "FallbackMode::Preresolved requires an explicit locale set",
            ));
        }

        self.fallback = match self.fallback {
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
            self.fallback,
            match self.locales {
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

        self.keys.clone().into_par_iter().try_for_each(|key| {
            log::info!("Generating key {key}");

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

                return sink
                    .flush_singleton(key, &payload)
                    .map_err(|e| e.with_req(key, Default::default()));
            }

            let locales_to_export = self.select_locales_for_key(provider, key, &fallbacker)?;

            match self.fallback {
                FallbackMode::Runtime | FallbackMode::RuntimeManual => {
                    let payloads = locales_to_export
                        .into_par_iter()
                        .filter_map(|locale| {
                            load_with_fallback(key, &locale)
                                .map(|r| r.map(|payload| (locale, payload)))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?;
                    let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
                    let fallbacker_with_config = fallbacker.for_config(key.fallback_config());
                    payloads.iter().try_for_each(|(locale, payload)| {
                        let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                        while !iter.get().is_und() {
                            iter.step();
                            if payloads.get(iter.get()) == Some(payload) {
                                // Found a match: don't need to write anything
                                log::trace!(
                                    "Deduplicating {key}/{locale} (inherits from {})",
                                    iter.get()
                                );
                                return Ok(());
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
                    })?
                }
                FallbackMode::Hybrid | FallbackMode::Preresolved => {
                    locales_to_export.into_par_iter().try_for_each(|locale| {
                        if let Some(payload) = load_with_fallback(key, &locale) {
                            sink.put_payload(key, &locale, &payload?)
                        } else {
                            Ok(())
                        }
                        .map_err(|e| {
                            e.with_req(
                                key,
                                DataRequest {
                                    locale: &locale,
                                    metadata: Default::default(),
                                },
                            )
                        })
                    })?
                }
                FallbackMode::PreferredForExporter => unreachable!("resolved"),
            };

            if self.fallback == FallbackMode::Runtime {
                sink.flush_with_built_in_fallback(key, BuiltInFallbackMode::Standard)
            } else {
                sink.flush(key)
            }
            .map_err(|e| e.with_key(key))
        })?;

        sink.close()
    }

    /// Selects the maximal set of locales to export based on a [`DataKey`] and this datagen
    /// provider's options bag. The locales may be later optionally deduplicated for fallback.
    fn select_locales_for_key(
        &self,
        provider: &dyn ExportableProvider,
        key: DataKey,
        fallbacker: &Lazy<
            Result<LocaleFallbacker, DataError>,
            impl FnOnce() -> Result<LocaleFallbacker, DataError>,
        >,
    ) -> Result<HashSet<icu_provider::DataLocale>, DataError> {
        let mut locales = provider
            .supported_locales_for_key(key)
            .map_err(|e| e.with_key(key))?
            .into_iter()
            .collect::<HashSet<DataLocale>>();

        if key == icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker::KEY
            || key == icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker::KEY
        {
            locales.retain(|locale| {
                let model =
                    crate::transform::segmenter::dictionary::data_locale_to_model_name(locale);
                self.segmenter_models
                    .iter()
                    .any(|m| Some(m.as_ref()) == model)
            });
            // Don't perform additional locale filtering
            return Ok(locales);
        } else if key == icu_segmenter::provider::LstmForWordLineAutoV1Marker::KEY {
            locales.retain(|locale| {
                let model = crate::transform::segmenter::lstm::data_locale_to_model_name(locale);
                self.segmenter_models
                    .iter()
                    .any(|m| Some(m.as_ref()) == model)
            });
            // Don't perform additional locale filtering
            return Ok(locales);
        } else if key == icu_collator::provider::CollationDataV1Marker::KEY
            || key == icu_collator::provider::CollationDiacriticsV1Marker::KEY
            || key == icu_collator::provider::CollationJamoV1Marker::KEY
            || key == icu_collator::provider::CollationMetadataV1Marker::KEY
            || key == icu_collator::provider::CollationReorderingV1Marker::KEY
            || key == icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY
        {
            locales.retain(|locale| {
                let Some(collation) = locale
                    .get_unicode_ext(&key!("co"))
                    .and_then(|co| co.as_single_subtag().copied())
                else { return true };
                self.collations.contains(collation.as_str())
                    || if collation.starts_with("search") {
                        self.collations.contains("search*")
                    } else {
                        !["big5han", "gb2312"].contains(&collation.as_str())
                    }
            });
        }

        locales = match (&self.locales, self.fallback) {
            // Case 1: `None` simply exports all supported locales for this key.
            (None, _) => locales,
            // Case 2: `FallbackMode::Preresolved` exports all supported locales whose langid matches
            // one of the explicit locales. This ensures extensions are included. In addition, any
            // explicit locales are added to the list, even if they themselves don't contain data;
            // fallback should be performed upon exporting.
            (Some(explicit), FallbackMode::Preresolved) => locales
                .into_iter()
                .chain(explicit.iter().map(|langid| langid.into()))
                .filter(|locale| explicit.contains(&locale.get_langid()))
                .collect(),
            // Case 3: All other modes resolve to the "ancestors and descendants" strategy.
            (Some(explicit), _) => {
                let include_und = explicit.contains(&LanguageIdentifier::UND);
                let explicit: HashSet<DataLocale> = explicit.iter().map(DataLocale::from).collect();
                let mut implicit = HashSet::new();
                // TODO: Make including the default locale configurable
                implicit.insert(DataLocale::default());
                let fallbacker = fallbacker.as_ref().map_err(|e| *e)?;
                let fallbacker_with_config = fallbacker.for_config(key.fallback_config());

                for locale in explicit.iter() {
                    let mut iter = fallbacker_with_config.fallback_for(locale.clone());
                    while !iter.get().is_und() {
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
                        while !iter.get().is_und() {
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
        };

        Ok(locales)
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
        let resolved_locales = DatagenDriver::new()
            .with_collations(cas.include_collations.iter().copied().map(String::from))
            .with_locales([cas.language.clone()])
            .with_fallback_mode(FallbackMode::Preresolved)
            .select_locales_for_key(
                &crate::DatagenProvider::latest_tested_offline_subset(),
                icu_collator::provider::CollationDataV1Marker::KEY,
                &once_cell::sync::Lazy::new(|| unreachable!()),
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
