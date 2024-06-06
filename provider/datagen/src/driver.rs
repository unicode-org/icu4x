// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::rayon_prelude::*;
use displaydoc::Display;
use icu_locale::fallback::LocaleFallbackIterator;
use icu_locale::LocaleFallbacker;
use icu_locale_core::extensions::unicode::key;
use icu_locale_core::LanguageIdentifier;
use icu_locale_core::ParserError;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;
use writeable::Writeable;

/// Options bag configuring locale inclusion and behavior when runtime fallback is disabled.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct NoFallbackOptions {}

/// Choices for the code location of runtime fallback.
///
/// Some data providers support "internal" fallback in which all data requests automatically run
/// the locale fallback algorithm. If internal fallback is requested for an exporter that does
/// not support it, an error will occur.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    pub const fn with_descendants(langid: LanguageIdentifier) -> Self {
        Self {
            langid: Some(langid),
            annotations: LocaleFamilyAnnotations::with_descendants(),
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
    pub const fn without_descendants(langid: LanguageIdentifier) -> Self {
        Self {
            langid: Some(langid),
            annotations: LocaleFamilyAnnotations::without_descendants(),
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
    pub const fn without_ancestors(langid: LanguageIdentifier) -> Self {
        Self {
            langid: Some(langid),
            annotations: LocaleFamilyAnnotations::without_ancestors(),
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

    pub(crate) fn into_parts(self) -> (Option<LanguageIdentifier>, LocaleFamilyAnnotations) {
        (self.langid, self.annotations)
    }

    pub(crate) fn as_borrowed(&self) -> LocaleFamilyBorrowed {
        LocaleFamilyBorrowed {
            langid: self.langid.as_ref(),
            annotations: self.annotations,
        }
    }
}

impl Writeable for LocaleFamily {
    #[inline]
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.as_borrowed().write_to(sink)
    }

    #[inline]
    fn writeable_length_hint(&self) -> writeable::LengthHint {
        self.as_borrowed().writeable_length_hint()
    }
}

writeable::impl_display_with_writeable!(LocaleFamily);

/// A [`LocaleFamily`] that does not own its [`LanguageIdentifier`].
pub(crate) struct LocaleFamilyBorrowed<'a> {
    langid: Option<&'a LanguageIdentifier>,
    annotations: LocaleFamilyAnnotations,
}

impl<'a> LocaleFamilyBorrowed<'a> {
    pub(crate) fn from_parts(
        inner: (&'a Option<LanguageIdentifier>, &LocaleFamilyAnnotations),
    ) -> Self {
        Self {
            langid: inner.0.as_ref(),
            annotations: *inner.1,
        }
    }
}

impl Writeable for LocaleFamilyBorrowed<'_> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        match (
            &self.langid,
            self.annotations.include_ancestors,
            self.annotations.include_descendants,
        ) {
            (Some(langid), true, true) => langid.write_to(sink),
            (Some(langid), true, false) => {
                sink.write_char('^')?;
                langid.write_to(sink)
            }
            (Some(langid), false, true) => {
                sink.write_char('%')?;
                langid.write_to(sink)
            }
            (Some(langid), false, false) => {
                sink.write_char('@')?;
                langid.write_to(sink)
            }
            (None, _, _) => sink.write_str("full"),
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        match (
            &self.langid,
            self.annotations.include_ancestors,
            self.annotations.include_descendants,
        ) {
            (Some(langid), true, true) => langid.writeable_length_hint(),
            (Some(langid), true, false) => langid.writeable_length_hint() + 1,
            (Some(langid), false, true) => langid.writeable_length_hint() + 1,
            (Some(langid), false, false) => langid.writeable_length_hint() + 1,
            (None, _, _) => writeable::LengthHint::exact(4),
        }
    }
}

/// An error while parsing a [`LocaleFamily`].
#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum LocaleFamilyParseError {
    /// An error bubbled up from parsing a [`LanguageIdentifier`].
    #[displaydoc("{0}")]
    LanguageIdentifier(ParserError),
    /// Some other error specific to parsing the family, such as an invalid lead byte.
    #[displaydoc("Invalid locale family")]
    InvalidFamily,
}

impl From<ParserError> for LocaleFamilyParseError {
    fn from(err: ParserError) -> Self {
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
                langid: Some(LanguageIdentifier::try_from_bytes(remainder)?),
                annotations: LocaleFamilyAnnotations::without_descendants(),
            }),
            b'%' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_bytes(remainder)?),
                annotations: LocaleFamilyAnnotations::without_ancestors(),
            }),
            b'@' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_bytes(remainder)?),
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
enum LocalesWithOrWithoutFallback {
    WithFallback {
        families: HashMap<Option<LanguageIdentifier>, LocaleFamilyAnnotations>,
        options: FallbackOptions,
    },
    WithoutFallback {
        langids: HashSet<LanguageIdentifier>,
    },
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
///     .with_markers([icu::list::provider::AndListV1Marker::INFO])
///     .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
///     .export(
///         &DatagenProvider::new_latest_tested(),
///         BlobExporter::new_with_sink(Box::new(&mut Vec::new())),
///     )
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct DatagenDriver {
    markers: Option<HashSet<DataMarkerInfo>>,
    locales_fallback: Option<LocalesWithOrWithoutFallback>,
    additional_collations: HashSet<String>,
    segmenter_models: Vec<String>,
}

impl DatagenDriver {
    /// Creates an empty [`DatagenDriver`].
    ///
    /// Note that markers and locales need to be set before calling [`export`](Self::export).
    #[allow(clippy::new_without_default)] // this is not directly usable
    pub fn new() -> Self {
        Self {
            markers: None,
            locales_fallback: None,
            additional_collations: HashSet::new(),
            segmenter_models: Vec::new(),
        }
        .with_recommended_segmenter_models()
    }

    /// Sets this driver to generate the given data markers.
    ///
    /// See [`icu_datagen::markers`], [`icu_datagen::all_markers`], [`icu_datagen::marker`] and [`icu_datagen::markers_from_bin`].
    ///
    /// [`icu_datagen::markers`]: crate::markers
    /// [`icu_datagen::all_markers`]: crate::all_markers
    /// [`icu_datagen::marker`]: crate::marker
    /// [`icu_datagen::markers_from_bin`]: crate::markers_from_bin
    pub fn with_markers(self, markers: impl IntoIterator<Item = DataMarkerInfo>) -> Self {
        Self {
            markers: Some(markers.into_iter().collect()),
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
        _options: NoFallbackOptions,
    ) -> Self {
        Self {
            locales_fallback: Some(LocalesWithOrWithoutFallback::WithoutFallback {
                langids: locales.into_iter().collect(),
            }),
            ..self
        }
    }

    /// Sets this driver to generate the given locales assuming runtime fallback.
    ///
    /// Use the [`langid!`] macro from the prelude to create an
    /// explicit list, or [`DatagenProvider::locales_for_coverage_levels`] for CLDR coverage levels.
    ///
    /// If there are multiple [`LocaleFamily`]s for the same [`LanguageIdentifier`], the last entry
    /// in the iterator takes precedence.
    ///
    /// [`langid!`]: crate::prelude::langid
    /// [`DatagenProvider::locales_for_coverage_levels`]: crate::DatagenProvider::locales_for_coverage_levels
    pub fn with_locales_and_fallback(
        self,
        locales: impl IntoIterator<Item = LocaleFamily>,
        options: FallbackOptions,
    ) -> Self {
        Self {
            locales_fallback: Some(LocalesWithOrWithoutFallback::WithFallback {
                families: locales.into_iter().map(LocaleFamily::into_parts).collect(),
                options,
            }),
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
            markers,
            locales_fallback,
            additional_collations,
            segmenter_models,
        } = self;

        let Some(markers) = markers else {
            return Err(DataError::custom(
                "`DatagenDriver::with_markers` needs to be called",
            ));
        };

        let locales_fallback = locales_fallback.ok_or_else(|| DataError::custom(
            "`DatagenDriver::with_locales_and_fallback` or `with_locales_no_fallback` needs to be called",
        ))?;

        if markers.is_empty() {
            log::warn!("No markers selected");
        }

        let (uses_internal_fallback, deduplication_strategy) = match &locales_fallback {
            LocalesWithOrWithoutFallback::WithoutFallback { langids } => {
                let mut sorted_locale_strs = langids
                    .iter()
                    .map(|x| x.write_to_string())
                    .collect::<Vec<_>>();
                sorted_locale_strs.sort_unstable();
                log::info!(
                    "Datagen configured without fallback with these locales: {:?}",
                    sorted_locale_strs
                );
                (false, DeduplicationStrategy::None)
            }
            LocalesWithOrWithoutFallback::WithFallback { options, families } => {
                let uses_internal_fallback = match options.runtime_fallback_location {
                    None => sink.supports_built_in_fallback(),
                    Some(RuntimeFallbackLocation::Internal) => true,
                    Some(RuntimeFallbackLocation::External) => false,
                };
                let deduplication_strategy = match options.deduplication_strategy {
                    // TODO(2.0): Default to RetainBaseLanguages here
                    None => {
                        if sink.supports_built_in_fallback() {
                            DeduplicationStrategy::Maximal
                        } else {
                            DeduplicationStrategy::None
                        }
                    }
                    Some(x) => x,
                };
                let mut sorted_locale_strs = families
                    .iter()
                    .map(LocaleFamilyBorrowed::from_parts)
                    .map(|family| family.write_to_string().into_owned())
                    .collect::<Vec<_>>();
                sorted_locale_strs.sort_unstable();
                log::info!(
                    "Datagen configured with {}, {}, and these locales: {:?}",
                    if uses_internal_fallback {
                        "internal fallback"
                    } else {
                        "external fallback"
                    },
                    match deduplication_strategy {
                        DeduplicationStrategy::Maximal => "maximal deduplication",
                        DeduplicationStrategy::RetainBaseLanguages =>
                            "deduplication retaining base languages",
                        DeduplicationStrategy::None => "no deduplication",
                    },
                    sorted_locale_strs
                );
                (uses_internal_fallback, deduplication_strategy)
            }
        };

        let fallbacker = OnceLock::new();
        let fallbacker = || {
            fallbacker
                .get_or_init(|| {
                    LocaleFallbacker::try_new_with_any_provider(&provider.as_any_provider())
                })
                .as_ref()
                .map_err(|&e| e)
        };

        let load_with_fallback = |marker, locale: &_, marker_attributes: &_| {
            log::trace!("Generating marker/locale: {marker}/{locale:}");
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            // Lazy-compute the fallback iterator so that we don't always require CLDR data
            let mut locale_iter: Option<LocaleFallbackIterator> = None;
            loop {
                let req = DataRequest {
                    locale: locale_iter.as_ref().map(|i| i.get()).unwrap_or(locale),
                    marker_attributes,
                    metadata,
                };
                match provider.load_data(marker, req) {
                    Ok(data_response) => {
                        if let Some(iter) = locale_iter.as_ref() {
                            if iter.get().is_und() && !locale.is_und() {
                                log::debug!("Falling back to und: {marker}/{locale}");
                            }
                        }
                        return Some(
                            data_response
                                .take_payload()
                                .map_err(|e| e.with_req(marker, req)),
                        );
                    }
                    Err(DataError {
                        kind: DataErrorKind::MissingLocale,
                        ..
                    }) => {
                        if let Some(iter) = locale_iter.as_mut() {
                            if iter.get().is_und() {
                                log::debug!("Could not find data for: {marker}/{locale}");
                                return None;
                            }
                            iter.step();
                        } else {
                            match fallbacker() {
                                Ok(fallbacker) => {
                                    locale_iter = Some(
                                        fallbacker
                                            .for_config(marker.fallback_config)
                                            .fallback_for(locale.clone()),
                                    )
                                }
                                Err(e) => return Some(Err(e)),
                            }
                        }
                    }
                    Err(e) => return Some(Err(e.with_req(marker, req))),
                }
            }
        };

        markers.clone().into_par_iter().try_for_each(|marker| {
            log::trace!("Generating marker {marker}");
            let instant1 = Instant::now();

            if marker.is_singleton {
                if provider.supported_requests_for_marker(marker)?
                    != HashSet::from_iter([Default::default()])
                {
                    return Err(DataError::custom(
                        "Invalid supported locales for singleton marker",
                    )
                    .with_marker(marker));
                }

                let payload = provider
                    .load_data(marker, Default::default())
                    .and_then(DataResponse::take_payload)
                    .map_err(|e| e.with_req(marker, Default::default()))?;

                let transform_duration = instant1.elapsed();

                sink.flush_singleton(marker, &payload)
                    .map_err(|e| e.with_req(marker, Default::default()))?;

                let final_duration = instant1.elapsed();
                let flush_duration = final_duration - transform_duration;

                if final_duration > Duration::new(0, 500_000_000) {
                    // Print durations if the marker took longer than 500 ms
                    log::info!(
                        "Generated marker {marker} ({}, flushed in {})",
                        DisplayDuration(final_duration),
                        DisplayDuration(flush_duration)
                    );
                } else {
                    log::info!("Generated marker {marker}");
                }
                return Ok(());
            }

            let locales_to_export = select_locales_for_marker(
                provider,
                marker,
                &locales_fallback,
                &additional_collations,
                &segmenter_models,
                fallbacker,
            )?;

            let (slowest_duration, slowest_locale) = match deduplication_strategy {
                DeduplicationStrategy::Maximal => {
                    let payloads = locales_to_export
                        .into_par_iter()
                        .filter_map(|(locale, marker_attributes)| {
                            let instant2 = Instant::now();
                            load_with_fallback(marker, &locale, &marker_attributes).map(|r| {
                                r.map(|payload| {
                                    ((locale, marker_attributes), (payload, instant2.elapsed()))
                                })
                            })
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?;
                    deduplicate_payloads::<true>(marker, &payloads, fallbacker()?, sink)?
                }
                DeduplicationStrategy::RetainBaseLanguages => {
                    let payloads = locales_to_export
                        .into_par_iter()
                        .filter_map(|(locale, marker_attributes)| {
                            let instant2 = Instant::now();
                            load_with_fallback(marker, &locale, &marker_attributes).map(|r| {
                                r.map(|payload| {
                                    ((locale, marker_attributes), (payload, instant2.elapsed()))
                                })
                            })
                        })
                        .collect::<Result<HashMap<_, _>, _>>()?;
                    deduplicate_payloads::<false>(marker, &payloads, fallbacker()?, sink)?
                }
                DeduplicationStrategy::None => locales_to_export
                    .into_par_iter()
                    .filter_map(|(locale, marker_attributes)| {
                        let instant2 = Instant::now();
                        let result = load_with_fallback(marker, &locale, &marker_attributes)?;
                        let result = result
                            .and_then(|payload| {
                                sink.put_payload(marker, &locale, &marker_attributes, &payload)
                            })
                            // Note: in Hybrid mode the elapsed time includes sink.put_payload.
                            // In Runtime mode the elapsed time is only load_with_fallback.
                            .map(|_| (instant2.elapsed(), locale.write_to_string().into_owned()))
                            .map_err(|e| {
                                e.with_req(
                                    marker,
                                    DataRequest {
                                        locale: &locale,
                                        marker_attributes: &marker_attributes,
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

            // segmenter uses hardcoded locales internally, so fallback is not necessary.
            // TODO(#4511): Use auxiliary keys for segmenter
            if uses_internal_fallback && !marker.path.get().starts_with("segmenter") {
                sink.flush_with_built_in_fallback(marker, BuiltInFallbackMode::Standard)
            } else {
                sink.flush(marker)
            }
            .map_err(|e| e.with_marker(marker))?;

            let final_duration = instant1.elapsed();
            let flush_duration = final_duration - transform_duration;

            if final_duration > Duration::new(0, 500_000_000) {
                // Print durations if the marker took longer than 500 ms
                log::info!(
                    "Generated marker {marker} ({}, '{slowest_locale}' in {}, flushed in {})",
                    DisplayDuration(final_duration),
                    DisplayDuration(slowest_duration),
                    DisplayDuration(flush_duration)
                );
            } else {
                log::info!("Generated marker {marker}");
            }
            Ok(())
        })?;

        sink.close()
    }
}

/// Selects the maximal set of locales to export based on a [`DataMarkerInfo`] and this datagen
/// provider's options bag. The locales may be later optionally deduplicated for fallback.
fn select_locales_for_marker<'a>(
    provider: &dyn ExportableProvider,
    marker: DataMarkerInfo,
    locales_fallback: &LocalesWithOrWithoutFallback,
    additional_collations: &HashSet<String>,
    segmenter_models: &[String],
    fallbacker: impl Fn() -> Result<&'a LocaleFallbacker, DataError>,
) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
    // Map from all supported LanguageIdentifiers to their
    // corresponding supported DataLocales.
    let mut supported_map =
        HashMap::<LanguageIdentifier, HashSet<(DataLocale, DataMarkerAttributes)>>::new();
    for (locale, marker_attributes) in provider
        .supported_requests_for_marker(marker)
        .map_err(|e| e.with_marker(marker))?
    {
        supported_map
            .entry(locale.get_langid())
            .or_default()
            .insert((locale, marker_attributes));
    }

    if marker.path.get().starts_with("segmenter/dictionary/") {
        supported_map.retain(|_, locales| {
            locales.retain(|(_, attrs)| segmenter_models.iter().any(|m| **m == **attrs));
            !locales.is_empty()
        });
        // Don't perform additional locale filtering
        return Ok(supported_map.into_values().flatten().collect());
    } else if marker.path.get().starts_with("segmenter/lstm/") {
        supported_map.retain(|_, locales| {
            locales.retain(|(_, attrs)| segmenter_models.iter().any(|m| **m == **attrs));
            !locales.is_empty()
        });
        // Don't perform additional locale filtering
        return Ok(supported_map.into_values().flatten().collect());
    } else if marker.path.get().starts_with("collator/") {
        supported_map.retain(|_, locales| {
            locales.retain(|(locale, _)| {
                let Some(collation) = locale
                    .get_unicode_ext(&key!("co"))
                    .and_then(|co| co.as_single_subtag().copied())
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
            !locales.is_empty()
        });
    }

    // The explicitly requested families, except for the `full` family.
    let mut include_full = false;
    let requested_families: HashMap<LanguageIdentifier, LocaleFamilyAnnotations> =
        match locales_fallback {
            LocalesWithOrWithoutFallback::WithFallback { families, .. } if families.is_empty() => {
                // If no locales are selected but fallback is enabled, select the root locale
                [(LanguageIdentifier::UND, LocaleFamilyAnnotations::single())]
                    .into_iter()
                    .collect()
            }
            LocalesWithOrWithoutFallback::WithFallback { families, .. } => families
                .iter()
                .filter_map(|(langid, annotations)| {
                    if let Some(langid) = langid.as_ref() {
                        if *langid == LanguageIdentifier::UND {
                            // Root locale: do not include descendants (use `full` for that)
                            Some((LanguageIdentifier::UND, LocaleFamilyAnnotations::single()))
                        } else {
                            // All other locales: copy the requested annotations
                            Some((langid.clone(), *annotations))
                        }
                    } else {
                        // Full locale family: set the bit instead of adding to the set
                        debug_assert_eq!(annotations, &LocaleFamily::FULL.annotations);
                        include_full = true;
                        None
                    }
                })
                .collect(),
            LocalesWithOrWithoutFallback::WithoutFallback { langids } => langids
                .iter()
                // Map langids without fallback to the `single` family
                .map(|langid| (langid.clone(), LocaleFamilyAnnotations::single()))
                .collect(),
        };

    if include_full && requested_families.is_empty() {
        // Special case: return now so we don't need the fallbacker (and its requisite CLDR data)
        let selected_locales = supported_map.into_values().flatten().collect();
        return Ok(selected_locales);
    }

    // Need the fallbacker now.
    let fallbacker_with_config = fallbacker()?.for_config(marker.fallback_config);

    // The "candidate" langids that could be exported is the union of requested and supported.
    let all_candidate_langids = supported_map
        .keys()
        .chain(requested_families.keys())
        .collect::<HashSet<_>>();

    // Compute a map from LanguageIdentifiers to DataLocales, including inherited auxiliary keys
    // and extensions. Also resolve the ancestors and descendants while building this map.
    let mut selected_langids = requested_families.keys().cloned().collect::<HashSet<_>>();
    let expansion_map: HashMap<&LanguageIdentifier, HashSet<(DataLocale, DataMarkerAttributes)>> =
        all_candidate_langids
            .into_iter()
            .map(|current_langid| {
                let mut expansion = supported_map
                    .get(current_langid)
                    .cloned()
                    .unwrap_or_default();
                if include_full && !selected_langids.contains(current_langid) {
                    log::trace!("Including {current_langid}: full locale family: {marker}");
                    selected_langids.insert(current_langid.clone());
                }
                if current_langid.language.is_empty() && current_langid != &LanguageIdentifier::UND
                {
                    log::trace!("Including {current_langid}: und variant: {marker}");
                    selected_langids.insert(current_langid.clone());
                }
                let include_ancestors = requested_families
                    .get(current_langid)
                    .map(|family| family.include_ancestors)
                    // default to `false` if the langid was not requested
                    .unwrap_or(false);
                let mut iter = fallbacker_with_config.fallback_for(current_langid.into());
                loop {
                    // Inherit aux keys and extension keywords from parent locales
                    let parent_langid: LanguageIdentifier = iter.get().get_langid();
                    let maybe_parent_locales = supported_map.get(&parent_langid);
                    let include_descendants = requested_families
                        .get(&parent_langid)
                        .map(|family| family.include_descendants)
                        // default to `false` if the langid was not requested
                        .unwrap_or(false);
                    if include_descendants && !selected_langids.contains(current_langid) {
                        log::trace!(
                            "Including {current_langid}: descendant of {parent_langid}: {marker}"
                        );
                        selected_langids.insert(current_langid.clone());
                    }
                    if include_ancestors && !selected_langids.contains(&parent_langid) {
                        log::trace!(
                            "Including {parent_langid}: ancestor of {current_langid}: {marker}"
                        );
                        selected_langids.insert(parent_langid);
                    }
                    if let Some(parent_locales) = maybe_parent_locales {
                        for morphed_req in parent_locales.iter() {
                            // Special case: don't pull extensions or aux keys up from the root.
                            if morphed_req.0.is_langid_und()
                                && !(morphed_req.0.is_empty() && morphed_req.1.is_empty())
                            {
                                continue;
                            }
                            let mut morphed_req = morphed_req.clone();
                            morphed_req.0.set_langid(current_langid.clone());
                            expansion.insert(morphed_req);
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
    payloads: &HashMap<(DataLocale, DataMarkerAttributes), (DataPayload<ExportMarker>, Duration)>,
    fallbacker: &LocaleFallbacker,
    sink: &dyn DataExporter,
) -> Result<Option<(Duration, String)>, DataError> {
    let fallbacker_with_config = fallbacker.for_config(marker.fallback_config);
    payloads
        .iter()
        .try_for_each(|((locale, marker_attributes), (payload, _duration))| {
            // Always export `und`. This prevents calling `step` on an empty locale.
            if locale.is_und() {
                return sink
                    .put_payload(marker, locale, marker_attributes, payload)
                    .map_err(|e| {
                        e.with_req(
                            marker,
                            DataRequest {
                                locale,
                                ..Default::default()
                            },
                        )
                    });
            }
            let mut iter = fallbacker_with_config.fallback_for(locale.clone());
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

                if let Some((inherited_payload, _duration)) =
                    payloads.get(&(iter.get().clone(), marker_attributes.clone()))
                {
                    if inherited_payload == payload {
                        // Found a match: don't need to write anything
                        log::trace!(
                            "Deduplicating {marker}/{locale} (inherits from {})",
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
            sink.put_payload(marker, locale, marker_attributes, payload)
                .map_err(|e| {
                    e.with_req(
                        marker,
                        DataRequest {
                            locale,
                            ..Default::default()
                        },
                    )
                })
        })?;

    // Slowest locale calculation:
    Ok(payloads
        .iter()
        .map(|((locale, marker_attributes), (_payload, duration))| {
            (
                *duration,
                locale.write_to_string().into_owned() + "/" + marker_attributes,
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
    use icu_locale_core::langid;
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
        TestCase {
            include_collations: &[],
            language: langid!("und"),
            expected: &["und", "und-u-co-emoji", "und-u-co-eor"],
        },
    ];
    let fallbacker = LocaleFallbacker::new_without_data();
    for cas in cases {
        let resolved_locales = select_locales_for_marker(
            &crate::provider::DatagenProvider::new_testing(),
            icu_collator::provider::CollationDataV1Marker::INFO,
            &LocalesWithOrWithoutFallback::WithoutFallback {
                langids: [cas.language.clone()].into_iter().collect(),
            },
            &HashSet::from_iter(cas.include_collations.iter().copied().map(String::from)),
            &[],
            || Ok(&fallbacker),
        )
        .unwrap()
        .into_iter()
        .map(|(l, _)| l.to_string())
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
    let driver = DatagenDriver::new().with_locales_and_fallback(
        [
            "en".parse().unwrap(),
            "%en".parse().unwrap(),
            "@en".parse().unwrap(),
            "%zh-TW".parse().unwrap(),
            "^zh-TW".parse().unwrap(),
        ],
        Default::default(),
    );

    let Some(LocalesWithOrWithoutFallback::WithFallback { families, .. }) = driver.locales_fallback
    else {
        panic!("expected locales with fallback")
    };

    assert_eq!(
        families,
        [
            "@en".parse::<LocaleFamily>().unwrap().into_parts(),
            "^zh-TW".parse::<LocaleFamily>().unwrap().into_parts()
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
    );
}
