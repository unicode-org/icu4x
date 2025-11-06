// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{DataLocaleFamilyAnnotations, DeduplicationStrategy, ExportDriver, ExportMetadata};
use icu_locale::fallback::LocaleFallbackIterator;
use icu_locale::LocaleFallbacker;
use icu_provider::export::*;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;
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
    pub(crate) fn export_dyn(
        self,
        provider: &dyn ExportableProvider,
        sink: &mut dyn DataExporter,
    ) -> Result<ExportMetadata, DataError> {
        let Self {
            markers,
            requested_families,
            include_full,
            fallbacker,
            deduplication_strategy,
            attributes_filters,
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
            log::trace!("Generating marker/locale: {marker:?}/{id}");
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
                match provider.load_data(marker, req).allow_identifier_not_found() {
                    Ok(Some(data_response)) => {
                        if let Some(iter) = locale_iter.as_ref() {
                            if iter.get().is_unknown() && !id.locale.is_unknown() {
                                log::debug!("Falling back to und: {marker:?}/{id}");
                            }
                        }
                        return Some(Ok(data_response));
                    }
                    Ok(None) => {
                        if let Some(iter) = locale_iter.as_mut() {
                            if iter.get().is_unknown() {
                                log::debug!("Could not find data for: {marker:?}/{id}");
                                return None;
                            }
                            iter.step();
                        } else {
                            locale_iter = Some(
                                fallbacker
                                    .for_config(marker.fallback_config)
                                    .fallback_for(*id.locale),
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

            let mut flush_metadata = FlushMetadata::default();
            flush_metadata.supports_dry_provider = matches!(
                deduplication_strategy,
                DeduplicationStrategy::RetainBaseLanguages | DeduplicationStrategy::None
            );

            if marker.is_singleton {
                let supported = provider.iter_ids_for_marker(marker)?;
                if supported.len() != 1 || !supported.first().unwrap().is_unknown() {
                    return Err(DataError::custom(
                        "Invalid supported locales for singleton marker",
                    )
                    .with_marker(marker));
                }

                let response = provider
                    .load_data(marker, Default::default())
                    .map_err(|e| e.with_req(marker, Default::default()))?;

                let transform_duration = instant1.elapsed();

                if marker.has_checksum {
                    flush_metadata.checksum = response.metadata.checksum;
                } else if response.metadata.checksum.is_some() {
                    log::warn!("{marker:?} returns a checksum, but it's not configured to");
                }

                sink.flush_singleton(marker, &response.payload, flush_metadata)
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
                &attributes_filters,
                include_full,
                &fallbacker,
            )?;

            let responses = locales_to_export
                .into_par_iter()
                .filter_map(|id| {
                    let instant2 = Instant::now();
                    load_with_fallback(marker, id.as_borrowed())
                        .map(|r| r.map(move |payload| (id, (payload, instant2.elapsed()))))
                })
                .collect::<Result<HashMap<_, _>, _>>()?;

            if marker.has_checksum {
                flush_metadata.checksum =
                    responses
                        .iter()
                        .try_fold(None, |acc, (id, (response, _))| {
                            match (acc, response.metadata.checksum) {
                                (Some(a), Some(b)) if a != b => {
                                    Err(DataError::custom("Mismatched checksums").with_req(
                                        marker,
                                        DataRequest {
                                            id: id.as_borrowed(),
                                            ..Default::default()
                                        },
                                    ))
                                }
                                (a, b) => Ok(a.or(b)),
                            }
                        })?;
            } else if responses.iter().any(|r| r.1 .0.metadata.checksum.is_some()) {
                log::warn!("{marker:?} returns a checksum, but it's not configured to");
            }

            let (slowest_duration, slowest_id) = match deduplication_strategy {
                DeduplicationStrategy::Maximal | DeduplicationStrategy::RetainBaseLanguages => {
                    deduplicate_responses(
                        deduplication_strategy == DeduplicationStrategy::Maximal,
                        marker,
                        responses,
                        &fallbacker,
                        sink,
                    )?
                }
                DeduplicationStrategy::None => responses
                    .into_iter()
                    .map(|(id, (response, time))| {
                        sink.put_payload(marker, id.as_borrowed(), &response.payload)
                            .map_err(|e| {
                                e.with_req(
                                    marker,
                                    DataRequest {
                                        id: id.as_borrowed(),
                                        ..Default::default()
                                    },
                                )
                            })
                            .map(|()| (time, id))
                    })
                    .collect::<Result<Vec<_>, DataError>>()?
                    .into_iter()
                    .max(),
            }
            .unwrap_or_default();

            let transform_duration = instant1.elapsed();

            sink.flush(marker, flush_metadata)
                .map_err(|e| e.with_marker(marker))?;

            let final_duration = instant1.elapsed();
            let flush_duration = final_duration - transform_duration;

            if final_duration > Duration::new(0, 500_000_000) {
                // Print durations if the marker took longer than 500 ms
                log::info!(
                    "Generated marker {marker:?} ({}, '{}/{}' in {}, flushed in {})",
                    DisplayDuration(final_duration),
                    slowest_id.locale,
                    slowest_id.marker_attributes.as_str(),
                    DisplayDuration(slowest_duration),
                    DisplayDuration(flush_duration)
                );
            } else {
                log::info!("Generated marker {marker:?}");
            }
            Ok(())
        })?;

        let exporter = sink.close()?;

        Ok(ExportMetadata { exporter })
    }
}

/// Selects the maximal set of locales to export based on a [`DataMarkerInfo`] and this datagen
/// provider's options bag. The locales may be later optionally deduplicated for fallback.
#[expect(clippy::type_complexity)] // sigh
fn select_locales_for_marker<'a>(
    provider: &'a dyn ExportableProvider,
    marker: DataMarkerInfo,
    requested_families: &HashMap<DataLocale, DataLocaleFamilyAnnotations>,
    attributes_filters: &HashMap<
        String,
        Arc<Box<dyn Fn(&DataMarkerAttributes) -> bool + Send + Sync + 'static>>,
    >,
    include_full: bool,
    fallbacker: &LocaleFallbacker,
) -> Result<HashSet<DataIdentifierCow<'a>>, DataError> {
    // Map from all supported DataLocales to their corresponding supported DataIdentifiers.
    let mut supported_map = HashMap::<DataLocale, HashSet<DataIdentifierCow<'a>>>::new();
    for id in provider
        .iter_ids_for_marker(marker)
        .map_err(|e| e.with_marker(marker))?
    {
        supported_map.entry(id.locale).or_default().insert(id);
    }

    if !marker.attributes_domain.is_empty() {
        if let Some(filter) = attributes_filters.get(marker.attributes_domain) {
            supported_map.retain(|_, ids| {
                ids.retain(|id| filter(&id.marker_attributes));
                !ids.is_empty()
            });
        }
    }

    if include_full && requested_families.is_empty() {
        // Special case: return now so we don't need the fallbacker (and its requisite CLDR data)
        let selected_locales = supported_map.into_values().flatten().collect();
        return Ok(selected_locales);
    }

    // The "candidate" locales that could be exported is the union of requested and supported.
    let all_candidate_locales = supported_map
        .keys()
        .chain(requested_families.keys())
        .collect::<HashSet<_>>();

    // Compute a map from LanguageIdentifiers to DataLocales, including inherited auxiliary keys
    // and extensions. Also resolve the ancestors and descendants while building this map.
    let mut selected_locales = requested_families.keys().cloned().collect::<HashSet<_>>();
    let expansion_map: HashMap<&DataLocale, HashSet<DataIdentifierCow>> = all_candidate_locales
        .into_iter()
        .map(|current_locale| {
            let mut expansion = supported_map
                .get(current_locale)
                .cloned()
                .unwrap_or_default();
            if include_full && !selected_locales.contains(current_locale) {
                log::trace!("Including {current_locale}: full locale family: {marker:?}");
                selected_locales.insert(*current_locale);
            }
            if current_locale.language.is_unknown() && !current_locale.is_unknown() {
                log::trace!("Including {current_locale}: und variant: {marker:?}");
                selected_locales.insert(*current_locale);
            }
            let include_ancestors = requested_families
                .get(current_locale)
                .map(|family| family.include_ancestors)
                // default to `false` if the locale was not requested
                .unwrap_or(false);
            let mut iter = fallbacker
                .for_config(marker.fallback_config)
                .fallback_for(*current_locale);
            loop {
                // Inherit aux keys and extension keywords from parent locales
                let parent_locale = iter.get();
                let maybe_parent_ids = supported_map.get(parent_locale);
                let include_descendants = requested_families
                    .get(parent_locale)
                    .map(|family| family.include_descendants)
                    // default to `false` if the locale was not requested
                    .unwrap_or(false);
                if include_descendants && !selected_locales.contains(current_locale) {
                    log::trace!(
                        "Including {current_locale}: descendant of {parent_locale}: {marker:?}"
                    );
                    selected_locales.insert(*current_locale);
                }
                if include_ancestors && !selected_locales.contains(parent_locale) {
                    log::trace!(
                        "Including {parent_locale}: ancestor of {current_locale}: {marker:?}"
                    );
                    selected_locales.insert(*parent_locale);
                }
                if let Some(parent_ids) = maybe_parent_ids {
                    for morphed_id in parent_ids.iter() {
                        // Special case: don't pull extensions or aux keys up from the root.
                        if morphed_id.locale.is_unknown() && !morphed_id.is_unknown() {
                            continue;
                        }
                        let mut morphed_id = morphed_id.clone();
                        morphed_id.locale = *current_locale;
                        expansion.insert(morphed_id);
                    }
                }
                if iter.get().is_unknown() {
                    break;
                }
                iter.step();
            }
            (current_locale, expansion)
        })
        .collect();

    let selected_locales = expansion_map
        .into_iter()
        .filter(|(locale, _)| selected_locales.contains(locale))
        .flat_map(|(_, data_locales)| data_locales)
        .collect();
    Ok(selected_locales)
}

fn deduplicate_responses<'a>(
    maximal: bool,
    marker: DataMarkerInfo,
    responses: HashMap<DataIdentifierCow<'a>, (DataResponse<ExportMarker>, Duration)>,
    fallbacker: &LocaleFallbacker,
    sink: &dyn DataExporter,
) -> Result<Option<(Duration, DataIdentifierCow<'a>)>, DataError> {
    let fallbacker_with_config = fallbacker.for_config(marker.fallback_config);
    responses
        .iter()
        .try_for_each(|(id, (response, _duration))| {
            // Always export `und`. This prevents calling `step` on an empty locale.
            if id.locale.is_unknown() {
                return sink
                    .put_payload(marker, id.as_borrowed(), &response.payload)
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
            let mut iter = fallbacker_with_config.fallback_for(id.locale);
            loop {
                if !maximal {
                    // To retain base languages, preemptively step to the
                    // parent locale. This should retain the locale if
                    // the next parent is `und`.
                    iter.step();
                }
                if iter.get().is_unknown() {
                    break;
                }
                if maximal {
                    iter.step();
                }

                if let Some((inherited_response, _duration)) = responses.get(
                    &DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        &id.marker_attributes,
                        iter.get(),
                    )
                    .as_cow(),
                ) {
                    if inherited_response.payload == response.payload {
                        // Found a match: don't need to write anything
                        log::trace!("Deduplicating {id} (inherits from {})", iter.get());
                        return Ok(());
                    } else {
                        // Not a match: we must include this
                        break;
                    }
                }
            }
            // Did not find a match: export this payload
            sink.put_payload(marker, id.as_borrowed(), &response.payload)
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
    Ok(responses
        .into_iter()
        .map(|(id, (_response, duration))| (duration, id))
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
            write!(f, "{nanos}ns")
        }
    }
}

#[test]
fn test_collation_filtering() {
    use crate::DataLocaleFamily;
    use icu::locale::locale;
    use std::collections::BTreeSet;

    struct Provider;

    impl DataProvider<icu::collator::provider::CollationTailoringV1> for Provider {
        fn load(
            &self,
            _req: DataRequest,
        ) -> Result<DataResponse<icu::collator::provider::CollationTailoringV1>, DataError>
        {
            unreachable!()
        }
    }

    impl IterableDataProvider<icu::collator::provider::CollationTailoringV1> for Provider {
        fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow<'_>>, DataError> {
            Ok(BTreeSet::from_iter(
                [
                    (locale!("ko"), "search"),
                    (locale!("ko"), "searchjl"),
                    (locale!("ko"), "unihan"),
                    (locale!("ko"), ""),
                    (locale!("und"), "emoji"),
                    (locale!("und"), "eor"),
                    (locale!("und"), "search"),
                    (locale!("und"), ""),
                    (locale!("zh"), "stroke"),
                    (locale!("zh"), "unihan"),
                    (locale!("zh"), "zhuyin"),
                    (locale!("zh"), ""),
                ]
                .into_iter()
                .map(|(l, a)| {
                    DataIdentifierCow::from_borrowed_and_owned(
                        DataMarkerAttributes::from_str_or_panic(a),
                        l.into(),
                    )
                }),
            ))
        }
    }

    extern crate alloc;
    icu_provider::export::make_exportable_provider!(
        Provider,
        [icu::collator::provider::CollationTailoringV1,]
    );

    #[derive(Debug)]
    struct TestCase<'a> {
        include_collations: &'a [&'a str],
        language: DataLocale,
        expected: &'a [&'a str],
    }
    let cases = [
        TestCase {
            include_collations: &[],
            language: locale!("zh").into(),
            expected: &["", "stroke", "unihan", "zhuyin"],
        },
        TestCase {
            include_collations: &["search*"],
            language: locale!("zh").into(),
            expected: &["", "stroke", "unihan", "zhuyin"],
        },
        TestCase {
            include_collations: &[],
            language: locale!("ko").into(),
            expected: &["", "unihan"],
        },
        TestCase {
            include_collations: &["search"],
            language: locale!("ko").into(),
            expected: &["", "search", "unihan"],
        },
        TestCase {
            include_collations: &["searchjl"],
            language: locale!("ko").into(),
            expected: &["", "searchjl", "unihan"],
        },
        TestCase {
            include_collations: &["search", "searchjl"],
            language: locale!("ko").into(),
            expected: &["", "search", "searchjl", "unihan"],
        },
        TestCase {
            include_collations: &["search*"],
            language: locale!("ko").into(),
            expected: &["", "search", "searchjl", "unihan"],
        },
        TestCase {
            include_collations: &[],
            language: locale!("und").into(),
            expected: &["", "emoji", "eor"],
        },
    ];
    for cas in cases {
        let driver = ExportDriver::new(
            [DataLocaleFamily::single(cas.language)],
            DeduplicationStrategy::None.into(),
            LocaleFallbacker::new_without_data(),
        )
        .with_additional_collations(cas.include_collations.iter().copied().map(String::from));
        let resolved_locales = select_locales_for_marker(
            &Provider,
            icu::collator::provider::CollationTailoringV1::INFO,
            &driver.requested_families,
            &driver.attributes_filters,
            false,
            &driver.fallbacker,
        )
        .unwrap()
        .into_iter()
        .map(|id| id.marker_attributes.to_string())
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
