// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{DeduplicationStrategy, ExportDriver, LocaleFamilyAnnotations};
use icu_locale::extensions::unicode::key;
use icu_locale::fallback::LocaleFallbackIterator;
use icu_locale::LanguageIdentifier;
use icu_locale::LocaleFallbacker;
use icu_provider::export::*;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::time::Duration;
use std::time::Instant;
use writeable::Writeable;

#[cfg(not(feature = "rayon"))]
trait IntoParallelIterator: IntoIterator + Sized {
    fn into_par_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
#[cfg(not(feature = "rayon"))]
impl<T: IntoIterator> IntoParallelIterator for T {}
#[cfg(feature = "rayon")]
use rayon::prelude::*;

impl ExportDriver {
    /// Exports data from the given provider to the given exporter.
    ///
    /// See
    /// [`make_exportable_provider!`](icu_provider::export::make_exportable_provider),
    /// [`BlobExporter`](icu_provider_blob::export),
    /// [`FileSystemExporter`](icu_provider_fs::export),
    /// and [`BakedExporter`](icu_provider_baked::export).
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
            markers,
            requested_families,
            include_full,
            fallbacker,
            deduplication_strategy,
            additional_collations,
            segmenter_models,
        } = self;

        let markers = markers.unwrap_or_else(|| provider.supported_markers());

        if markers.is_empty() {
            log::warn!("No markers selected");
        }

        log::info!(
            "Datagen configured with {}, and these locales: {:?}",
            match deduplication_strategy {
                DeduplicationStrategy::Maximal => "maximal deduplication",
                DeduplicationStrategy::RetainBaseLanguages =>
                    "deduplication retaining base languages",
                DeduplicationStrategy::None => "no deduplication",
            },
            if include_full {
                vec!["<all>".to_string()]
            } else {
                let mut sorted_locale_strs = requested_families
                    .iter()
                    .map(|(l, a)| {
                        let mut s = String::new();
                        let _infallible = a.write_to(&mut s);
                        let _infallible = l.write_to(&mut s);
                        s
                    })
                    .collect::<Vec<_>>();
                sorted_locale_strs.sort_unstable();
                sorted_locale_strs
            }
        );

        let load_with_fallback = |marker, id: DataIdentifierBorrowed<'_>| {
            log::trace!("Generating marker/locale: {marker:?}/{}", id.locale);
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            // Lazy-compute the fallback iterator so that we don't always require CLDR data
            let mut locale_iter: Option<LocaleFallbackIterator> = None;
            loop {
                let req = DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        id.marker_attributes,
                        locale_iter.as_ref().map(|i| i.get()).unwrap_or(id.locale),
                    ),
                    metadata,
                };
                match provider.load_data(marker, req) {
                    Ok(data_response) => {
                        if let Some(iter) = locale_iter.as_ref() {
                            if iter.get().is_und() && !id.locale.is_und() {
                                log::debug!("Falling back to und: {marker:?}/{}", id.locale);
                            }
                        }
                        return Some(Ok(data_response.payload));
                    }
                    Err(DataError {
                        kind: DataErrorKind::IdentifierNotFound,
                        ..
                    }) => {
                        if let Some(iter) = locale_iter.as_mut() {
                            if iter.get().is_und() {
                                log::debug!("Could not find data for: {marker:?}/{}", id.locale);
                                return None;
                            }
                            iter.step();
                        } else {
                            locale_iter = Some(
                                fallbacker
                                    .for_config(marker.fallback_config)
                                    .fallback_for(id.locale.clone()),
                            )
                        }
                    }
                    Err(e) => return Some(Err(e.with_req(marker, req))),
                }
            }
        };

        markers.clone().into_par_iter().try_for_each(|marker| {
            log::trace!("Generating marker {marker:?}");
            let instant1 = Instant::now();

            if marker.is_singleton {
                let supported = provider.iter_ids_for_marker(marker)?;
                if supported.len() != 1 || !supported.first().unwrap().is_default() {
                    return Err(DataError::custom(
                        "Invalid supported locales for singleton marker",
                    )
                    .with_marker(marker));
                }

                let payload = provider
                    .load_data(marker, Default::default())
                    .map_err(|e| e.with_req(marker, Default::default()))?
                    .payload;

                let transform_duration = instant1.elapsed();

                sink.flush_singleton(marker, &payload)
                    .map_err(|e| e.with_req(marker, Default::default()))?;

                let final_duration = instant1.elapsed();
                let flush_duration = final_duration - transform_duration;

                if final_duration > Duration::new(0, 500_000_000) {
                    // Print durations if the marker took longer than 500 ms
                    log::info!(
                        "Generated marker {marker:?} ({}, flushed in {})",
                        DisplayDuration(final_duration),
                        DisplayDuration(flush_duration)
                    );
                } else {
                    log::info!("Generated marker {marker:?}");
                }
                return Ok(());
            }

            let locales_to_export = select_locales_for_marker(
                provider,
                marker,
                &requested_families,
                include_full,
                &additional_collations,
                &segmenter_models,
                &fallbacker,
            )?;

            let (slowest_duration, slowest_locale) = match deduplication_strategy {
                DeduplicationStrategy::Maximal => {
                    let payloads = locales_to_export
                        .into_par_iter()
                        .filter_map(|id| {
                            let instant2 = Instant::now();
                            load_with_fallback(marker, id.as_borrowed())
                                .map(|r| r.map(|payload| (id, (payload, instant2.elapsed()))))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?;
                    deduplicate_payloads::<true>(marker, &payloads, &fallbacker, sink)?
                }
                DeduplicationStrategy::RetainBaseLanguages => {
                    let payloads = locales_to_export
                        .into_par_iter()
                        .filter_map(|id| {
                            let instant2 = Instant::now();
                            load_with_fallback(marker, id.as_borrowed())
                                .map(|r| r.map(|payload| (id, (payload, instant2.elapsed()))))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?;
                    deduplicate_payloads::<false>(marker, &payloads, &fallbacker, sink)?
                }
                DeduplicationStrategy::None => locales_to_export
                    .into_par_iter()
                    .filter_map(|id| {
                        let instant2 = Instant::now();
                        let result = load_with_fallback(marker, id.as_borrowed())?;
                        let result = result
                            .and_then(|payload| {
                                sink.put_payload(marker, id.as_borrowed(), &payload)
                            })
                            // Note: in Hybrid mode the elapsed time includes sink.put_payload.
                            // In Runtime mode the elapsed time is only load_with_fallback.
                            .map(|_| (instant2.elapsed(), id.locale.write_to_string().into_owned()))
                            .map_err(|e| {
                                e.with_req(
                                    marker,
                                    DataRequest {
                                        id: id.as_borrowed(),
                                        ..Default::default()
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

            sink.flush(marker).map_err(|e| e.with_marker(marker))?;

            let final_duration = instant1.elapsed();
            let flush_duration = final_duration - transform_duration;

            if final_duration > Duration::new(0, 500_000_000) {
                // Print durations if the marker took longer than 500 ms
                log::info!(
                    "Generated marker {marker:?} ({}, '{slowest_locale}' in {}, flushed in {})",
                    DisplayDuration(final_duration),
                    DisplayDuration(slowest_duration),
                    DisplayDuration(flush_duration)
                );
            } else {
                log::info!("Generated marker {marker:?}");
            }
            Ok(())
        })?;

        sink.close()
    }
}

/// Selects the maximal set of locales to export based on a [`DataMarkerInfo`] and this datagen
/// provider's options bag. The locales may be later optionally deduplicated for fallback.
fn select_locales_for_marker<'a>(
    provider: &'a dyn ExportableProvider,
    marker: DataMarkerInfo,
    requested_families: &HashMap<LanguageIdentifier, LocaleFamilyAnnotations>,
    include_full: bool,
    additional_collations: &HashSet<String>,
    segmenter_models: &[String],
    fallbacker: &LocaleFallbacker,
) -> Result<HashSet<DataIdentifierCow<'a>>, DataError> {
    // Map from all supported LanguageIdentifiers to their
    // corresponding supported DataLocales.
    let mut supported_map = HashMap::<LanguageIdentifier, HashSet<DataIdentifierCow<'a>>>::new();
    for id in provider
        .iter_ids_for_marker(marker)
        .map_err(|e| e.with_marker(marker))?
    {
        supported_map
            .entry(id.locale.get_langid())
            .or_default()
            .insert(id);
    }

    if marker.path.as_str().starts_with("segmenter/dictionary/") {
        supported_map.retain(|_, ids| {
            ids.retain(|id| {
                segmenter_models
                    .iter()
                    .any(|m| **m == **id.marker_attributes)
            });
            !ids.is_empty()
        });
        // Don't perform additional locale filtering
        return Ok(supported_map.into_values().flatten().collect());
    } else if marker.path.as_str().starts_with("segmenter/lstm/") {
        supported_map.retain(|_, locales| {
            locales.retain(|id| {
                segmenter_models
                    .iter()
                    .any(|m| **m == **id.marker_attributes)
            });
            !locales.is_empty()
        });
        // Don't perform additional locale filtering
        return Ok(supported_map.into_values().flatten().collect());
    } else if marker.path.as_str().starts_with("collator/") {
        supported_map.retain(|_, ids| {
            ids.retain(|id| {
                let Some(collation) = id
                    .locale
                    .get_unicode_ext(&key!("co"))
                    .and_then(|co| co.into_single_subtag())
                else {
                    return true;
                };
                additional_collations.contains(collation.as_str())
                    || if collation.as_str().starts_with("search") {
                        additional_collations.contains("search*")
                    } else {
                        !["big5han", "gb2312"].contains(&collation.as_str())
                    }
            });
            !ids.is_empty()
        });
    }

    if include_full && requested_families.is_empty() {
        // Special case: return now so we don't need the fallbacker (and its requisite CLDR data)
        let selected_locales = supported_map.into_values().flatten().collect();
        return Ok(selected_locales);
    }

    // The "candidate" langids that could be exported is the union of requested and supported.
    let all_candidate_langids = supported_map
        .keys()
        .chain(requested_families.keys())
        .collect::<HashSet<_>>();

    // Compute a map from LanguageIdentifiers to DataLocales, including inherited auxiliary keys
    // and extensions. Also resolve the ancestors and descendants while building this map.
    let mut selected_langids = requested_families.keys().cloned().collect::<HashSet<_>>();
    let expansion_map: HashMap<&LanguageIdentifier, HashSet<DataIdentifierCow>> =
        all_candidate_langids
            .into_iter()
            .map(|current_langid| {
                let mut expansion = supported_map
                    .get(current_langid)
                    .cloned()
                    .unwrap_or_default();
                if include_full && !selected_langids.contains(current_langid) {
                    log::trace!("Including {current_langid}: full locale family: {marker:?}");
                    selected_langids.insert(current_langid.clone());
                }
                if current_langid.language.is_empty() && current_langid != &LanguageIdentifier::UND
                {
                    log::trace!("Including {current_langid}: und variant: {marker:?}");
                    selected_langids.insert(current_langid.clone());
                }
                let include_ancestors = requested_families
                    .get(current_langid)
                    .map(|family| family.include_ancestors)
                    // default to `false` if the langid was not requested
                    .unwrap_or(false);
                let mut iter = fallbacker
                    .for_config(marker.fallback_config)
                    .fallback_for(current_langid.into());
                loop {
                    // Inherit aux keys and extension keywords from parent locales
                    let parent_langid: LanguageIdentifier = iter.get().get_langid();
                    let maybe_parent_ids = supported_map.get(&parent_langid);
                    let include_descendants = requested_families
                        .get(&parent_langid)
                        .map(|family| family.include_descendants)
                        // default to `false` if the langid was not requested
                        .unwrap_or(false);
                    if include_descendants && !selected_langids.contains(current_langid) {
                        log::trace!(
                            "Including {current_langid}: descendant of {parent_langid}: {marker:?}"
                        );
                        selected_langids.insert(current_langid.clone());
                    }
                    if include_ancestors && !selected_langids.contains(&parent_langid) {
                        log::trace!(
                            "Including {parent_langid}: ancestor of {current_langid}: {marker:?}"
                        );
                        selected_langids.insert(parent_langid);
                    }
                    if let Some(parent_ids) = maybe_parent_ids {
                        for morphed_id in parent_ids.iter() {
                            // Special case: don't pull extensions or aux keys up from the root.
                            if morphed_id.locale.is_langid_und()
                                && !(morphed_id.locale.is_und()
                                    && morphed_id.marker_attributes.is_empty())
                            {
                                continue;
                            }
                            let mut morphed_id = morphed_id.clone();
                            morphed_id
                                .locale
                                .to_mut()
                                .set_langid(current_langid.clone());
                            expansion.insert(morphed_id);
                        }
                    }
                    if iter.get().is_und() {
                        break;
                    }
                    iter.step();
                }
                (current_langid, expansion)
            })
            .collect();

    let selected_locales = expansion_map
        .into_iter()
        .filter(|(langid, _)| selected_langids.contains(langid))
        .flat_map(|(_, data_locales)| data_locales)
        .collect();
    Ok(selected_locales)
}

fn deduplicate_payloads<const MAXIMAL: bool>(
    marker: DataMarkerInfo,
    payloads: &HashMap<DataIdentifierCow, (DataPayload<ExportMarker>, Duration)>,
    fallbacker: &LocaleFallbacker,
    sink: &dyn DataExporter,
) -> Result<Option<(Duration, String)>, DataError> {
    let fallbacker_with_config = fallbacker.for_config(marker.fallback_config);
    payloads.iter().try_for_each(|(id, (payload, _duration))| {
        // Always export `und`. This prevents calling `step` on an empty locale.
        if id.locale.is_und() {
            return sink
                .put_payload(marker, id.as_borrowed(), payload)
                .map_err(|e| {
                    e.with_req(
                        marker,
                        DataRequest {
                            id: id.as_borrowed(),
                            ..Default::default()
                        },
                    )
                });
        }
        let mut iter = fallbacker_with_config.fallback_for(id.locale.clone().into_owned());
        loop {
            if !MAXIMAL {
                // To retain base languages, preemptively step to the
                // parent locale. This should retain the locale if
                // the next parent is `und`.
                iter.step();
            }
            if iter.get().is_und() {
                break;
            }
            if MAXIMAL {
                iter.step();
            }

            if let Some((inherited_payload, _duration)) = payloads.get(
                &DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    &id.marker_attributes,
                    iter.get(),
                )
                .as_cow(),
            ) {
                if inherited_payload == payload {
                    // Found a match: don't need to write anything
                    log::trace!(
                        "Deduplicating {:?}/{} (inherits from {})",
                        id.locale,
                        id.marker_attributes.as_str(),
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
        sink.put_payload(marker, id.as_borrowed(), payload)
            .map_err(|e| {
                e.with_req(
                    marker,
                    DataRequest {
                        id: id.as_borrowed(),
                        ..Default::default()
                    },
                )
            })
    })?;

    // Slowest locale calculation:
    Ok(payloads
        .iter()
        .map(|(id, (_payload, duration))| {
            (
                *duration,
                id.locale.write_to_string().into_owned() + "/" + id.marker_attributes.as_str(),
            )
        })
        .max())
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
    use icu::locale::{langid, locale};
    use std::collections::BTreeSet;

    struct Provider;

    impl DataProvider<icu::collator::provider::CollationDataV1Marker> for Provider {
        fn load(
            &self,
            _req: DataRequest,
        ) -> Result<DataResponse<icu::collator::provider::CollationDataV1Marker>, DataError>
        {
            unreachable!()
        }
    }

    impl IterableDataProvider<icu::collator::provider::CollationDataV1Marker> for Provider {
        fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow>, DataError> {
            Ok(BTreeSet::from_iter(
                [
                    locale!("ko-u-co-search"),
                    locale!("ko-u-co-searchjl"),
                    locale!("ko-u-co-unihan"),
                    locale!("ko"),
                    locale!("und-u-co-emoji"),
                    locale!("und-u-co-eor"),
                    locale!("und-u-co-search"),
                    locale!("und"),
                    locale!("zh-u-co-big5han"),
                    locale!("zh-u-co-gb2312"),
                    locale!("zh-u-co-stroke"),
                    locale!("zh-u-co-unihan"),
                    locale!("zh-u-co-zhuyin"),
                    locale!("zh"),
                ]
                .into_iter()
                .map(|l| DataIdentifierCow::from_locale(l.into())),
            ))
        }
    }

    icu_provider::export::make_exportable_provider!(
        Provider,
        [icu::collator::provider::CollationDataV1Marker,]
    );

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
        TestCase {
            include_collations: &[],
            language: langid!("und"),
            expected: &["und", "und-u-co-emoji", "und-u-co-eor"],
        },
    ];
    for cas in cases {
        let resolved_locales = select_locales_for_marker(
            &Provider,
            icu::collator::provider::CollationDataV1Marker::INFO,
            &[(cas.language.clone(), LocaleFamilyAnnotations::single())]
                .into_iter()
                .collect(),
            false,
            &HashSet::from_iter(cas.include_collations.iter().copied().map(String::from)),
            &[],
            &LocaleFallbacker::new_without_data(),
        )
        .unwrap()
        .into_iter()
        .map(|id| id.locale.to_string())
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
