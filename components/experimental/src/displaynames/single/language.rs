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
///     DisplayNamesPreferences, DisplayNamesOptions, single::LanguageIdentifierDisplayNameOwned,
/// };
/// use icu::locale::{locale, langid};
/// use writeable::assert_writeable_eq;
///
/// let prefs = DisplayNamesPreferences::from(locale!("en"));
/// let options = DisplayNamesOptions::default();
/// let display_name = LanguageIdentifierDisplayNameOwned::try_new(
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
pub struct LanguageIdentifierDisplayNameOwned {
    formatting_locale: DataLocale,
    options: DisplayNamesOptions,
    language_payload: DataPayload<LocaleNamesLanguageMediumV1>,
    script_payload: DataPayloadOr<LocaleNamesScriptMediumV1, ()>,
    region_payload: DataPayloadOr<LocaleNamesRegionMediumV1, ()>,
    variant_payloads:
        DataPayloadOr<LocaleNamesVariantMediumV1, Vec<DataPayload<LocaleNamesVariantMediumV1>>>,
    essentials_payload: DataPayload<LocaleNamesEssentialsV1>,
}

impl LanguageIdentifierDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: icu_locale::LanguageIdentifier, options: DisplayNamesOptions) -> result: Result<Self, DataError>,
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
        mut subject: icu_locale::LanguageIdentifier,
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
        let formatting_locale = LocaleNamesLanguageMediumV1::make_locale(prefs.locale_preferences);

        // Step 1: Load/Resolve Language Name (with Dialect resolution)
        // We want to find the best display name for the given subject.
        // In Dialect mode (default), we try to load names for combinations of subtags:
        // - Language + Script + Region (e.g., "zh-Hant-HK")
        // - Language + Script (e.g., "zh-Hant")
        // - Language + Region (e.g., "en-GB")
        // If any of these are found in the CLDR language names, we use it as the base name,
        // and we "consume" the corresponding subtags so they are not repeated in the qualifiers.
        // If none are found, we fall back to the base language name (e.g., "zh") and all
        // present subtags (script, region, variants) will be formatted as qualifiers.

        let mut language_payload = None;

        // Only try dialect if requested (which is the default)
        if options.language_display == LanguageDisplay::Dialect {
            // 1a. Try 3-subtag combination (if both script and region are present)
            if let (Some(script), Some(region)) = (subject.script, subject.region) {
                let attr = LocaleNamesLanguageMediumV1::make_attributes(
                    subject.language,
                    Some(script),
                    Some(region),
                );
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(attr.as_str())
                        .map_err(|_| DataError::custom("Invalid dialect attr"))?,
                    &formatting_locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                if let Some(response) = provider
                    .load(DataRequest { id, metadata })
                    .allow_identifier_not_found()?
                {
                    language_payload = Some(response.payload);
                    subject.script = None;
                    subject.region = None;
                }
            }

            // 1b. Try language + script (if script is present)
            if language_payload.is_none()
                && let Some(script) = subject.script
            {
                let attr = LocaleNamesLanguageMediumV1::make_attributes(
                    subject.language,
                    Some(script),
                    None,
                );
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(attr.as_str())
                        .map_err(|_| DataError::custom("Invalid dialect attr"))?,
                    &formatting_locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                if let Some(response) = provider
                    .load(DataRequest { id, metadata })
                    .allow_identifier_not_found()?
                {
                    language_payload = Some(response.payload);
                    subject.script = None;
                }
            }

            // 1c. Try language + region (if region is present)
            if language_payload.is_none()
                && let Some(region) = subject.region
            {
                let attr = LocaleNamesLanguageMediumV1::make_attributes(
                    subject.language,
                    None,
                    Some(region),
                );
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(attr.as_str())
                        .map_err(|_| DataError::custom("Invalid dialect attr"))?,
                    &formatting_locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                if let Some(response) = provider
                    .load(DataRequest { id, metadata })
                    .allow_identifier_not_found()?
                {
                    language_payload = Some(response.payload);
                    subject.region = None;
                }
            }
        }

        // 1d. Fallback to base language
        let language_payload = match language_payload {
            Some(payload) => Some(payload),
            None => {
                let attr =
                    LocaleNamesLanguageMediumV1::make_attributes(subject.language, None, None);
                provider
                    .load(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::try_from_str(attr.as_str())
                                .map_err(|_| DataError::custom("Invalid language"))?,
                            &formatting_locale,
                        ),
                        ..Default::default()
                    })
                    .allow_identifier_not_found()?
                    .map(|response| response.payload)
            }
        };

        let language_payload = language_payload.ok_or_else(|| {
            // TODO(#8100): Fall back to the code instead of failing with DataError if the language name is not found
            DataError::custom("Language not found")
        })?;

        // Step 2: Load script name (if present in subject)
        let script_payload = if let Some(script) = subject.script {
            let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::try_from_str(script.as_str())
                    .map_err(|_| DataError::custom("Invalid script"))?,
                &formatting_locale,
            );
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            if let Some(response) = provider
                .load(DataRequest { id, metadata })
                .allow_identifier_not_found()?
            {
                DataPayloadOr::from_payload(response.payload)
            } else {
                DataPayloadOr::none()
            }
        } else {
            DataPayloadOr::none()
        };

        // Step 3: Load region name (if present in subject)
        let region_payload = if let Some(region) = subject.region {
            let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::try_from_str(region.as_str())
                    .map_err(|_| DataError::custom("Invalid region"))?,
                &formatting_locale,
            );
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            if let Some(response) = provider
                .load(DataRequest { id, metadata })
                .allow_identifier_not_found()?
            {
                DataPayloadOr::from_payload(response.payload)
            } else {
                DataPayloadOr::none()
            }
        } else {
            DataPayloadOr::none()
        };

        // Step 4: Load variant names (if present in subject)
        let variant_payloads = match subject.variants.len() {
            0 => DataPayloadOr::from_other(Vec::new()),
            1 => {
                if let Some(variant) = subject.variants.iter().next() {
                    let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::try_from_str(variant.as_str())
                            .map_err(|_| DataError::custom("Invalid variant"))?,
                        &formatting_locale,
                    );
                    let mut metadata = DataRequestMetadata::default();
                    metadata.silent = true;
                    if let Some(response) = provider
                        .load(DataRequest { id, metadata })
                        .allow_identifier_not_found()?
                    {
                        DataPayloadOr::from_payload(response.payload)
                    } else {
                        // TODO(#8100): Fall back to the code instead of dropping it if the variant name is not found
                        DataPayloadOr::from_other(Vec::new())
                    }
                } else {
                    DataPayloadOr::from_other(Vec::new())
                }
            }
            _ => {
                let mut loaded_variants = Vec::new();
                for variant in subject.variants.iter() {
                    let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        DataMarkerAttributes::try_from_str(variant.as_str())
                            .map_err(|_| DataError::custom("Invalid variant"))?,
                        &formatting_locale,
                    );
                    let mut metadata = DataRequestMetadata::default();
                    metadata.silent = true;
                    if let Some(response) = provider
                        .load(DataRequest { id, metadata })
                        .allow_identifier_not_found()?
                    {
                        loaded_variants.push(response.payload);
                    } else {
                        // TODO(#8100): Fall back to the code instead of dropping it if the variant name is not found
                    }
                }
                DataPayloadOr::from_other(loaded_variants)
            }
        };

        // Step 5: Load essentials
        let essentials_payload = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&formatting_locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            formatting_locale,
            options,
            language_payload,
            script_payload,
            region_payload,
            variant_payloads,
            essentials_payload,
        })
    }

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> LanguageIdentifierDisplayName<'_, '_> {
        let variants = match self.variant_payloads.get() {
            Ok(payload) => BorrowedVariants::One(payload),
            Err(vec) => {
                if vec.is_empty() {
                    BorrowedVariants::None
                } else {
                    BorrowedVariants::Slice(vec.as_slice())
                }
            }
        };

        LanguageIdentifierDisplayName {
            base_name: self.language_payload.get(),
            script_name: self.script_payload.get_option().map(|p| &**p),
            region_name: self.region_payload.get_option().map(|p| &**p),
            variants,
            locale_pattern: &self.essentials_payload.get().locale_pattern,
            locale_separator: &self.essentials_payload.get().locale_separator,
        }
    }
}

impl writeable::Writeable for LanguageIdentifierDisplayNameOwned {
    #[inline]
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.as_borrowed().write_to(sink)
    }

    #[inline]
    fn writeable_length_hint(&self) -> writeable::LengthHint {
        self.as_borrowed().writeable_length_hint()
    }
}

writeable::impl_display_with_writeable!(LanguageIdentifierDisplayNameOwned);

/// Borrowed variants representation to avoid heap allocation.
#[derive(Debug, Clone, Copy)]
pub enum BorrowedVariants<'a, 'b> {
    None,
    One(&'a str),
    Slice(&'b [DataPayload<LocaleNamesVariantMediumV1>]),
}

/// A localized display name for a language.
#[derive(Debug, Clone, Copy)]
pub struct LanguageIdentifierDisplayName<'a, 'b> {
    base_name: &'a str,
    script_name: Option<&'a str>,
    region_name: Option<&'a str>,
    variants: BorrowedVariants<'a, 'b>,
    locale_pattern: &'a DoublePlaceholderPattern,
    locale_separator: &'a DoublePlaceholderPattern,
}

struct QualifiersWriteable<'a, 'b> {
    script: Option<&'a str>,
    region: Option<&'a str>,
    variants: BorrowedVariants<'a, 'b>,
    separator: &'a DoublePlaceholderPattern,
}

impl<'a, 'b> writeable::Writeable for QualifiersWriteable<'a, 'b> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let mut first = true;

        // TODO: See whether we can share this code with the list component.
        let mut separator_str = ", ";
        for item in self.separator.iter() {
            if let icu_pattern::PatternItem::Literal(s) = item {
                separator_str = s;
                break;
            }
        }

        let mut write_item = |sink: &mut W, item: &str| -> core::fmt::Result {
            if !first {
                sink.write_str(separator_str)?;
            }
            sink.write_str(item)?;
            first = false;
            Ok(())
        };

        if let Some(script) = self.script {
            write_item(sink, script)?;
        }
        if let Some(region) = self.region {
            write_item(sink, region)?;
        }
        match self.variants {
            BorrowedVariants::None => {}
            BorrowedVariants::One(v) => {
                write_item(sink, v)?;
            }
            BorrowedVariants::Slice(slice) => {
                for variant in slice.iter() {
                    write_item(sink, variant.get())?;
                }
            }
        }
        Ok(())
    }
}

impl<'a, 'b> writeable::Writeable for LanguageIdentifierDisplayName<'a, 'b> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let has_variants = !matches!(self.variants, BorrowedVariants::None);
        let has_qualifiers =
            self.script_name.is_some() || self.region_name.is_some() || has_variants;

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

writeable::impl_display_with_writeable!(LanguageIdentifierDisplayName<'_, '_>);
