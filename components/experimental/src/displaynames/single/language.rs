// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::displaynames::provider::*;
use crate::displaynames::{DisplayNamesOptions, DisplayNamesPreferences, LanguageDisplay};
use alloc::vec::Vec;
use icu_pattern::DoublePlaceholderPattern;
use icu_provider::DataPayloadOr;
use icu_provider::prelude::*;

/// A localized display name for a language, owned version.
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::{
///     DisplayNamesPreferences, DisplayNamesOptions, single::LanguageDisplayNameOwned,
/// };
/// use icu::locale::{locale, langid};
/// use writeable::assert_writeable_eq;
///
/// let prefs = DisplayNamesPreferences::from(locale!("en"));
/// let options = DisplayNamesOptions::default();
/// let display_name = LanguageDisplayNameOwned::try_new(
///     prefs,
///     langid!("fr-CA"),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_writeable_eq!(display_name, "Canadian French");
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct LanguageDisplayNameOwned {
    locale: DataLocale,
    options: DisplayNamesOptions,
    language_payload: DataPayload<LocaleNamesLanguageMediumV1>,
    script_payload: DataPayloadOr<LocaleNamesScriptMediumV1, ()>,
    region_payload: DataPayloadOr<LocaleNamesRegionMediumV1, ()>,
    variant_payloads: Vec<DataPayload<LocaleNamesVariantMediumV1>>,
    essentials_payload: DataPayload<LocaleNamesEssentialsV1>,
}

impl LanguageDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, locale_id: icu_locale::LanguageIdentifier, options: DisplayNamesOptions) -> result: Result<Self, DataError>,
        /// Loads the language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        locale_id: icu_locale::LanguageIdentifier,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<LocaleNamesLanguageMediumV1>
            + DataProvider<LocaleNamesScriptMediumV1>
            + DataProvider<LocaleNamesRegionMediumV1>
            + DataProvider<LocaleNamesVariantMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>
            + ?Sized,
    {
        let locale = LocaleNamesLanguageMediumV1::make_locale(prefs.locale_preferences);

        // Step 1: Load/Resolve Language Name (with Dialect resolution)
        // We want to find the best display name for the given locale_id.
        // In Dialect mode (default), we try to load names for combinations of subtags:
        // - Language + Script + Region (e.g., "zh-Hant-HK")
        // - Language + Script (e.g., "zh-Hant")
        // - Language + Region (e.g., "en-GB")
        // If any of these are found in the CLDR language names, we use it as the base name,
        // and we "consume" the corresponding subtags so they are not repeated in the qualifiers.
        // If none are found, we fall back to the base language name (e.g., "zh") and all
        // present subtags (script, region, variants) will be formatted as qualifiers.

        let mut language_payload = None;
        let mut script_consumed = false;
        let mut region_consumed = false;

        // Only try dialect if requested (which is the default)
        if options.language_display == LanguageDisplay::Dialect {
            // 1a. Try 3-subtag combination (if both script and region are present)
            if let (Some(script), Some(region)) = (locale_id.script, locale_id.region) {
                let temp_id = icu_locale::LanguageIdentifier {
                    language: locale_id.language,
                    script: Some(script),
                    region: Some(region),
                    variants: Default::default(),
                };
                let attr = temp_id.to_string();
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(&attr)
                        .map_err(|_| DataError::custom("Invalid dialect attr"))?,
                    &locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                if let Ok(response) = provider.load(DataRequest { id, metadata }) {
                    language_payload = Some(response.payload);
                    script_consumed = true;
                    region_consumed = true;
                }
            }

            // 1b. Try language + script (if script is present and not consumed)
            if language_payload.is_none()
                && let Some(script) = locale_id.script
            {
                let temp_id = icu_locale::LanguageIdentifier {
                    language: locale_id.language,
                    script: Some(script),
                    region: None,
                    variants: Default::default(),
                };
                let attr = temp_id.to_string();
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(&attr)
                        .map_err(|_| DataError::custom("Invalid dialect attr"))?,
                    &locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                if let Ok(response) = provider.load(DataRequest { id, metadata }) {
                    language_payload = Some(response.payload);
                    script_consumed = true;
                }
            }

            // 1c. Try language + region (if region is present and not consumed)
            if language_payload.is_none()
                && let Some(region) = locale_id.region
            {
                let temp_id = icu_locale::LanguageIdentifier {
                    language: locale_id.language,
                    script: None,
                    region: Some(region),
                    variants: Default::default(),
                };
                let attr = temp_id.to_string();
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(&attr)
                        .map_err(|_| DataError::custom("Invalid dialect attr"))?,
                    &locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                if let Ok(response) = provider.load(DataRequest { id, metadata }) {
                    language_payload = Some(response.payload);
                    region_consumed = true;
                }
            }
        }

        // 1d. Fallback to base language
        let language_payload = match language_payload {
            Some(payload) => payload,
            None => {
                provider
                    .load(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::try_from_str(locale_id.language.as_str())
                                .map_err(|_| DataError::custom("Invalid language"))?,
                            &locale,
                        ),
                        ..Default::default()
                    })?
                    .payload
            }
        };

        // Step 2: Load script name (if present in locale_id and not consumed)
        let script_payload = if let Some(script) = locale_id.script {
            if !script_consumed {
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(script.as_str())
                        .map_err(|_| DataError::custom("Invalid script"))?,
                    &locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                match provider.load(DataRequest { id, metadata }) {
                    Ok(response) => DataPayloadOr::from_payload(response.payload),
                    Err(DataError {
                        kind: DataErrorKind::IdentifierNotFound,
                        ..
                    }) => DataPayloadOr::none(),
                    Err(e) => return Err(e),
                }
            } else {
                DataPayloadOr::none()
            }
        } else {
            DataPayloadOr::none()
        };

        // Step 3: Load region name (if present in locale_id and not consumed)
        let region_payload = if let Some(region) = locale_id.region {
            if !region_consumed {
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(region.as_str())
                        .map_err(|_| DataError::custom("Invalid region"))?,
                    &locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                match provider.load(DataRequest { id, metadata }) {
                    Ok(response) => DataPayloadOr::from_payload(response.payload),
                    Err(DataError {
                        kind: DataErrorKind::IdentifierNotFound,
                        ..
                    }) => DataPayloadOr::none(),
                    Err(e) => return Err(e),
                }
            } else {
                DataPayloadOr::none()
            }
        } else {
            DataPayloadOr::none()
        };

        // Step 4: Load variant names (if present in locale_id)
        let mut variant_payloads = Vec::new();
        for variant in locale_id.variants.iter() {
            let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::try_from_str(variant.as_str())
                    .map_err(|_| DataError::custom("Invalid variant"))?,
                &locale,
            );
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            match provider.load(DataRequest { id, metadata }) {
                Ok(response) => variant_payloads.push(response.payload),
                Err(DataError {
                    kind: DataErrorKind::IdentifierNotFound,
                    ..
                }) => {}
                Err(e) => return Err(e),
            }
        }

        // Step 5: Load essentials
        let essentials_payload = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            locale,
            options,
            language_payload,
            script_payload,
            region_payload,
            variant_payloads,
            essentials_payload,
        })
    }

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> LanguageDisplayName<'_> {
        LanguageDisplayName {
            base_name: self.language_payload.get(),
            script_name: self.script_payload.get_option().map(|p| &**p),
            region_name: self.region_payload.get_option().map(|p| &**p),
            variants: &self.variant_payloads,
            locale_pattern: &self.essentials_payload.get().locale_pattern,
            locale_separator: &self.essentials_payload.get().locale_separator,
        }
    }
}

impl writeable::Writeable for LanguageDisplayNameOwned {
    #[inline]
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.as_borrowed().write_to(sink)
    }

    #[inline]
    fn writeable_length_hint(&self) -> writeable::LengthHint {
        self.as_borrowed().writeable_length_hint()
    }
}

writeable::impl_display_with_writeable!(LanguageDisplayNameOwned);

/// A localized display name for a language.
#[derive(Debug, Clone, Copy)]
pub struct LanguageDisplayName<'a> {
    base_name: &'a str,
    script_name: Option<&'a str>,
    region_name: Option<&'a str>,
    variants: &'a [DataPayload<LocaleNamesVariantMediumV1>],
    locale_pattern: &'a DoublePlaceholderPattern,
    locale_separator: &'a DoublePlaceholderPattern,
}

struct QualifiersWriteable<'a> {
    script: Option<&'a str>,
    region: Option<&'a str>,
    variants: &'a [DataPayload<LocaleNamesVariantMediumV1>],
    separator: &'a DoublePlaceholderPattern,
}

impl<'a> writeable::Writeable for QualifiersWriteable<'a> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        // Collect all non-consumed qualifiers (script, region, variants) into a stack array.
        // We have at most 4 qualifiers (1 script, 1 region, and up to 2 variants in practice,
        // though BCP-47 allows more, CLDR data rarely has more than 2 variants for display).
        let mut qs: [&'a str; 4] = [""; 4];
        let mut len = 0;
        if let Some(script) = self.script
            && let Some(q) = qs.get_mut(len)
        {
            *q = script;
            len += 1;
        }
        if let Some(region) = self.region
            && let Some(q) = qs.get_mut(len)
        {
            *q = region;
            len += 1;
        }
        for variant in self.variants.iter() {
            if let Some(q) = qs.get_mut(len) {
                *q = &**variant.get();
                len += 1;
            }
        }

        // Format the collected qualifiers.
        // If we have 1 qualifier, we just write it.
        // If we have multiple, we join them pair-wise using the locale separator pattern.
        // For example, if we have [Q0, Q1, Q2], we format it as:
        //   interpolate(interpolate(Q0, Q1), Q2)
        // This avoids any allocation and matches the TR35 spec for list formatting of qualifiers.
        match len {
            0 => Ok(()),
            1 => sink.write_str(qs[0]),
            2 => self.separator.interpolate((qs[0], qs[1])).write_to(sink),
            3 => self
                .separator
                .interpolate((self.separator.interpolate((qs[0], qs[1])), qs[2]))
                .write_to(sink),
            4 => self
                .separator
                .interpolate((
                    self.separator
                        .interpolate((self.separator.interpolate((qs[0], qs[1])), qs[2])),
                    qs[3],
                ))
                .write_to(sink),
            _ => unreachable!(),
        }
    }
}

impl<'a> writeable::Writeable for LanguageDisplayName<'a> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let has_qualifiers =
            self.script_name.is_some() || self.region_name.is_some() || !self.variants.is_empty();

        if !has_qualifiers {
            sink.write_str(self.base_name)
        } else {
            let qualifiers = QualifiersWriteable {
                script: self.script_name,
                region: self.region_name,
                variants: self.variants,
                separator: self.locale_separator,
            };
            self.locale_pattern
                .interpolate((self.base_name, qualifiers))
                .write_to(sink)
        }
    }
}

writeable::impl_display_with_writeable!(LanguageDisplayName<'_>);
