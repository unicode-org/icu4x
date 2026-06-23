// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::displaynames::provider::*;
use crate::displaynames::single::{
    RegionDisplayNameOwned, ScriptDisplayNameOwned, VariantDisplayNameOwned,
};
use crate::displaynames::{DisplayNamesOptions, DisplayNamesPreferences, LanguageDisplay};
use alloc::vec::Vec;
use icu_pattern::DoublePlaceholderPattern;
use icu_provider::DataPayloadOr;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;

/// A localized display name for a language identifier, owned version.
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
            for (language, script, region) in [
                (subject.language, Some(subject.script), Some(subject.region)),
                (subject.language, Some(subject.script), None),
                (subject.language, None, Some(subject.region)),
            ] {
                // For Script and Region:
                // - Some in the first position means "this should be present"
                // - None in the first position means "skip this field"
                // We skip Some(None) because that case will be handled in a subsequent iteration
                let script = match script {
                    Some(Some(script)) => Some(script),
                    Some(None) => continue,
                    None => None,
                };
                let region = match region {
                    Some(Some(region)) => Some(region),
                    Some(None) => continue,
                    None => None,
                };
                let mut buffer = TinyAsciiStr::EMPTY;
                let attrs = LocaleNamesLanguageMediumV1::make_attributes(
                    language,
                    script,
                    region,
                    &mut buffer,
                );
                let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    attrs,
                    &formatting_locale,
                );
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                if let Some(response) = provider
                    .load(DataRequest { id, metadata })
                    .allow_identifier_not_found()?
                {
                    language_payload = Some(response.payload);
                    if script.is_some() {
                        subject.script = None;
                    }
                    if region.is_some() {
                        subject.region = None;
                    }
                    break;
                }
            }
        }

        // If the language name is not loaded yet, try loading it from the language subtag alone.
        // TODO(#8100): Fall back to the code instead of failing with DataError if the language name is not found
        let language_payload = match language_payload {
            Some(payload) => payload,
            None => {
                let mut buffer = TinyAsciiStr::EMPTY;
                let attrs = LocaleNamesLanguageMediumV1::make_attributes(
                    subject.language,
                    None,
                    None,
                    &mut buffer,
                );
                provider
                    .load(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            attrs,
                            &formatting_locale,
                        ),
                        ..Default::default()
                    })?
                    .payload
            }
        };

        // Step 2: Load script name (if present in subject)
        // TODO(#8100): Fall back to the code instead of failing with DataError if the script name is not found
        let script_payload = if let Some(script) = subject.script {
            DataPayloadOr::from_payload(
                ScriptDisplayNameOwned::try_new_unstable(provider, prefs, script)?.payload,
            )
        } else {
            DataPayloadOr::none()
        };

        // Step 3: Load region name (if present in subject)
        // TODO(#8100): Fall back to the code instead of failing with DataError if the region name is not found
        let region_payload = if let Some(region) = subject.region {
            DataPayloadOr::from_payload(
                RegionDisplayNameOwned::try_new_unstable(provider, prefs, region)?.payload,
            )
        } else {
            DataPayloadOr::none()
        };

        // Step 4: Load variant names (if present in subject)
        let mut variant_results = subject
            .variants
            .iter()
            .map(|variant| VariantDisplayNameOwned::try_new_unstable(provider, prefs, *variant))
            .peekable();
        let variant_payloads = if let Some(result) = variant_results.next() {
            if variant_results.peek().is_some() {
                // 2 or more variants
                // TODO(#8100): Fall back to the code instead of dropping it if the variant name is not found
                let payload_vec = core::iter::once(result)
                    .chain(variant_results)
                    .map(|result| result.map(|obj| obj.payload))
                    .collect::<Result<Vec<_>, _>>()?;
                DataPayloadOr::from_other(payload_vec)
            } else {
                // 1 variant
                // TODO(#8100): Fall back to the code instead of dropping it if the variant name is not found
                DataPayloadOr::from_payload(result?.payload)
            }
        } else {
            // 0 variants
            DataPayloadOr::from_other(Vec::new())
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
    pub fn as_borrowed(&self) -> LanguageIdentifierDisplayName<'_> {
        let variants = match self.variant_payloads.get() {
            Ok(variant_name) => BorrowedVariants::One(variant_name),
            Err(vec) => BorrowedVariants::Slice(vec.as_slice()),
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
///
/// Note: if a compiled-data-only constructor is added in the future,
/// this will need a new variant for a vec of borrowed variant names.
#[derive(Debug, Clone, Copy)]
enum BorrowedVariants<'a> {
    One(&'a str),
    Slice(&'a [DataPayload<LocaleNamesVariantMediumV1>]),
}

impl BorrowedVariants<'_> {
    #[inline]
    fn is_empty(&self) -> bool {
        matches!(self, Self::Slice([]))
    }
}

/// A localized display name for a language identifier.
#[derive(Debug, Clone, Copy)]
pub struct LanguageIdentifierDisplayName<'a> {
    base_name: &'a str,
    script_name: Option<&'a str>,
    region_name: Option<&'a str>,
    variants: BorrowedVariants<'a>,
    locale_pattern: &'a DoublePlaceholderPattern,
    locale_separator: &'a DoublePlaceholderPattern,
}

struct QualifiersWriteable<'a> {
    script: Option<&'a str>,
    region: Option<&'a str>,
    variants: BorrowedVariants<'a>,
    separator: &'a DoublePlaceholderPattern,
}

impl<'a> writeable::Writeable for QualifiersWriteable<'a> {
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

impl<'a> writeable::Writeable for LanguageIdentifierDisplayName<'a> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let has_variants = !self.variants.is_empty();
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

writeable::impl_display_with_writeable!(LanguageIdentifierDisplayName<'_>);
