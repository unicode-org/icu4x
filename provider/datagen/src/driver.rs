// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rayon_prelude::*;
use displaydoc::Display;
use icu_locale::extensions::unicode::key;
use icu_locale::fallback::LocaleFallbackIterator;
use icu_locale::LanguageIdentifier;
use icu_locale::LocaleFallbacker;
use icu_locale::ParseError;
use icu_provider::export::*;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;
use writeable::Writeable;

/// Options bag configuring locale inclusion and behavior when runtime fallback is disabled.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct NoFallbackOptions {}

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
/// | `RetainBaseLanguages` (TODO(#58): coming soon) | Small | Yes | Yes |
/// | [`None`] | Medium/Small | Yes | No |
///
/// [`Maximal`]: DeduplicationStrategy::Maximal
/// [`None`]: DeduplicationStrategy::None
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeduplicationStrategy {
    /// Removes from the lookup table any locale whose parent maps to the same data.
    Maximal,
    /// Removes from the lookup table any locale whose parent maps to the same data, except if
    /// the parent is `und`.
    RetainBaseLanguages,
    /// Keeps all selected locales in the lookup table.
    None,
}

/// Inner fields of a [`LocaleFamily`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LocaleFamilyAnnotations {
    include_ancestors: bool,
    include_descendants: bool,
}

impl LocaleFamilyAnnotations {
    #[inline]
    pub(crate) const fn with_descendants() -> Self {
        Self {
            include_ancestors: true,
            include_descendants: true,
        }
    }

    #[inline]
    pub(crate) const fn without_descendants() -> Self {
        Self {
            include_ancestors: true,
            include_descendants: false,
        }
    }

    #[inline]
    pub(crate) const fn without_ancestors() -> Self {
        Self {
            include_ancestors: false,
            include_descendants: true,
        }
    }

    #[inline]
    pub(crate) const fn single() -> Self {
        Self {
            include_ancestors: false,
            include_descendants: false,
        }
    }
}

impl Writeable for LocaleFamilyAnnotations {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        match (self.include_ancestors, self.include_descendants) {
            (true, true) => Ok(()),
            (true, false) => sink.write_char('^'),
            (false, true) => sink.write_char('%'),
            (false, false) => sink.write_char('@'),
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        writeable::LengthHint::exact(match (self.include_ancestors, self.include_descendants) {
            (true, true) => 0,
            _ => 1,
        })
    }
}

/// A family of locales to export.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocaleFamily {
    langid: Option<LanguageIdentifier>,
    annotations: LocaleFamilyAnnotations,
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
    ///
    /// The `und` locale is treated specially and behaves like `::single("und")`.
    pub const fn with_descendants(langid: LanguageIdentifier) -> Self {
        let annotations = if langid.is_empty() {
            LocaleFamilyAnnotations::single()
        } else {
            LocaleFamilyAnnotations::with_descendants()
        };

        Self {
            langid: Some(langid),
            annotations,
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
    ///
    /// The `und` locale is treated specially and behaves like `::single("und")`.
    pub const fn without_descendants(langid: LanguageIdentifier) -> Self {
        let annotations = if langid.is_empty() {
            LocaleFamilyAnnotations::single()
        } else {
            LocaleFamilyAnnotations::without_descendants()
        };
        Self {
            langid: Some(langid),
            annotations,
        }
    }

    /// The family containing all descendants of the selected locale.
    ///
    /// This family may be useful if the root locale is not desired.
    ///
    /// For example, the family `::without_ancestors("en-001")` contains:
    ///
    /// - Self: "en-001"
    /// - Descendants: "en-GB", "en-ZA", ...
    ///
    /// but it does _not_ contain the ancestors "en" and "und".
    ///
    /// Stylized on the CLI as: "%en-US"
    ///
    /// The `und` locale is treated specially and behaves like `::single("und")`.
    pub const fn without_ancestors(langid: LanguageIdentifier) -> Self {
        let annotations = if langid.is_empty() {
            LocaleFamilyAnnotations::single()
        } else {
            LocaleFamilyAnnotations::without_ancestors()
        };
        Self {
            langid: Some(langid),
            annotations,
        }
    }

    /// The family containing only the selected locale.
    ///
    /// For example, the family `::single("en-001")` contains only "en-001".
    ///
    /// Stylized on the CLI as: "@en-US"
    pub const fn single(langid: LanguageIdentifier) -> Self {
        Self {
            langid: Some(langid),
            annotations: LocaleFamilyAnnotations::single(),
        }
    }

    /// The family containing all locales.
    ///
    /// Stylized on the CLI as: "full"
    pub const FULL: Self = Self {
        langid: None,
        annotations: LocaleFamilyAnnotations {
            include_ancestors: false,
            include_descendants: true,
        },
    };
}

impl Writeable for LocaleFamily {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        if let Some(langid) = self.langid.as_ref() {
            self.annotations.write_to(sink)?;
            langid.write_to(sink)
        } else {
            sink.write_str("full")
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        if let Some(langid) = self.langid.as_ref() {
            self.annotations.writeable_length_hint() + langid.writeable_length_hint()
        } else {
            writeable::LengthHint::exact(4)
        }
    }
}

writeable::impl_display_with_writeable!(LocaleFamily);

/// An error while parsing a [`LocaleFamily`].
#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum LocaleFamilyParseError {
    /// An error bubbled up from parsing a [`LanguageIdentifier`].
    #[displaydoc("{0}")]
    LanguageIdentifier(ParseError),
    /// Some other error specific to parsing the family, such as an invalid lead byte.
    #[displaydoc("Invalid locale family")]
    InvalidFamily,
}

impl From<ParseError> for LocaleFamilyParseError {
    fn from(err: ParseError) -> Self {
        Self::LanguageIdentifier(err)
    }
}

impl std::error::Error for LocaleFamilyParseError {}

impl FromStr for LocaleFamily {
    type Err = LocaleFamilyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "full" {
            return Ok(Self::FULL);
        }
        let (first, remainder) = s
            .as_bytes()
            .split_first()
            .ok_or(LocaleFamilyParseError::InvalidFamily)?;
        match first {
            b'^' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_utf8(remainder)?),
                annotations: LocaleFamilyAnnotations::without_descendants(),
            }),
            b'%' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_utf8(remainder)?),
                annotations: LocaleFamilyAnnotations::without_ancestors(),
            }),
            b'@' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_utf8(remainder)?),
                annotations: LocaleFamilyAnnotations::single(),
            }),
            b if b.is_ascii_alphanumeric() => Ok(Self {
                langid: Some(s.parse()?),
                annotations: LocaleFamilyAnnotations::with_descendants(),
            }),
            _ => Err(LocaleFamilyParseError::InvalidFamily),
        }
    }
}

#[test]
fn test_locale_family_parsing() {
    let valid_families = ["und", "de-CH", "^es", "@pt-BR", "%en-001", "full"];
    let invalid_families = ["invalid", "@invalid", "-foo", "@full", "full-001"];
    for family_str in valid_families {
        let family = family_str.parse::<LocaleFamily>().unwrap();
        let family_to_str = family.to_string();
        assert_eq!(family_str, family_to_str);
    }
    for family_str in invalid_families {
        assert!(family_str.parse::<LocaleFamily>().is_err());
    }
}

/// Options bag configuring locale inclusion and behavior when runtime fallback is enabled.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct FallbackOptions {
    /// The aggressiveness of deduplication of data payloads.
    pub deduplication_strategy: DeduplicationStrategy,
}

impl FallbackOptions {
    /// Creates a [`FallbackOptions`] with [`DeduplicationStrategy::None`]
    pub fn no_deduplication() -> Self {
        Self {
            deduplication_strategy: DeduplicationStrategy::None,
        }
    }

    /// Creates a [`FallbackOptions`] with [`DeduplicationStrategy::Maximal`]
    pub fn maximal_deduplication() -> Self {
        Self {
            deduplication_strategy: DeduplicationStrategy::Maximal,
        }
    }

    /// Creates a [`FallbackOptions`] with [`DeduplicationStrategy::RetainBaseLanguages`]
    pub fn retain_base_languages_deduplication() -> Self {
        Self {
            deduplication_strategy: DeduplicationStrategy::RetainBaseLanguages,
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
/// use icu_datagen_bikeshed::DatagenProvider;
///
/// let provider = DatagenProvider::new_latest_tested();
///
/// DatagenDriver::new([LocaleFamily::FULL], FallbackOptions::no_deduplication(), LocaleFallbacker::try_new_unstable(&provider).unwrap())
///     .with_markers([icu::list::provider::AndListV2Marker::INFO])
///     .export(
///         &provider,
///         BlobExporter::new_with_sink(Box::new(&mut Vec::new())),
///     )
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct DatagenDriver {
    markers: Option<HashSet<DataMarkerInfo>>,
    requested_families: HashMap<LanguageIdentifier, LocaleFamilyAnnotations>,
    fallbacker: LocaleFallbacker,
    include_full: bool,
    deduplication_strategy: DeduplicationStrategy,
    additional_collations: HashSet<String>,
    segmenter_models: Vec<String>,
}

impl DatagenDriver {
    /// Creates a [`DatagenDriver`].
    ///
    /// The fallbacker is used to resolve locale families, and to dedpulicate data if requested.
    /// Make sure to use the same fallback data when loading from the provider at runtime.
    /// Commonly, you will export the fallback markers, in which case you should construct
    /// your fallbacker with the source provider (i.e. [`LocaleFallbacker::try_new_unstable`]).
    pub fn new(
        locales: impl IntoIterator<Item = LocaleFamily>,
        options: FallbackOptions,
        fallbacker: LocaleFallbacker,
    ) -> Self {
        let mut include_full = false;
        Self {
            markers: Default::default(),
            requested_families: locales
                .into_iter()
                .filter_map(|family| {
                    Some((
                        family.langid.or_else(|| {
                            // Full locale family: set the bit instead of adding to the set
                            debug_assert_eq!(family.annotations, LocaleFamily::FULL.annotations);
                            include_full = true;
                            None
                        })?,
                        family.annotations,
                    ))
                })
                .collect(),
            include_full,
            fallbacker,
            deduplication_strategy: options.deduplication_strategy,
            additional_collations: Default::default(),
            segmenter_models: Default::default(),
        }
        .with_recommended_segmenter_models()
    }

    /// Sets this driver to generate the given data markers.
    ///
    /// If this is not called, all markers supported by the provider will be exported.
    pub fn with_markers(self, markers: impl IntoIterator<Item = DataMarkerInfo>) -> Self {
        Self {
            markers: Some(markers.into_iter().collect()),
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
    /// chosen data markers.
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
    /// chosen data markers.
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
                if provider.iter_ids_for_marker(marker)? != HashSet::from_iter([Default::default()])
                {
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
                                && !(morphed_id.locale.is_empty()
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
        fn iter_ids(&self) -> Result<HashSet<DataIdentifierCow>, DataError> {
            Ok(HashSet::from_iter(
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

/// Test that the last option with multiple conflicting families wins.
#[test]
fn test_family_precedence() {
    let driver = DatagenDriver::new(
        [
            "en".parse().unwrap(),
            "%en".parse().unwrap(),
            "@en".parse().unwrap(),
            "%zh-TW".parse().unwrap(),
            "^zh-TW".parse().unwrap(),
        ],
        FallbackOptions::no_deduplication(),
        LocaleFallbacker::new_without_data(),
    );

    assert_eq!(
        driver.requested_families,
        [
            (
                icu::locale::langid!("en"),
                LocaleFamilyAnnotations::single()
            ),
            (
                icu::locale::langid!("zh-TW"),
                LocaleFamilyAnnotations::without_descendants()
            ),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
    );
}
