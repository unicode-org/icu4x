// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::displaynames::provider::*;
use crate::displaynames::single::{
    RegionDisplayNameOwned, ScriptDisplayNameOwned, VariantDisplayNameOwned,
};
use crate::displaynames::{
    DisplayNamesPreferences, LanguageDisplay, LanguageIdentifierDisplayNameOptions,
};
use alloc::{vec, vec::Vec};
use icu_locale_core::LanguageIdentifier;
use icu_locale_core::subtags::{Language, Region, Script, Variant};
use icu_pattern::{DoublePlaceholderPattern, DoublePlaceholderValueProviderTry, PatternItem};
use icu_provider::DataPayloadOr;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use writeable::{PartsWrite, TryWriteable, adapters::LossyWrap};

/// An error returned when a display name was not found in data and has fallen back to the raw BCP-47 subtag code.
#[derive(displaydoc::Display, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[allow(clippy::exhaustive_structs)]
pub struct LanguageIdentifierNameFallbackError;

/// Represents a payload that was either successfully loaded or has fallen back to its code.
type PayloadOrFallback<M, S> = DataPayloadOr<M, S>;

/// A localized display name for a language identifier, owned version.
///
/// The formatter falls back to the BCP-47 subtag when localized display names are missing
/// from the data provider. Fallback can be detected using [`TryWriteable`].
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::{
///     DisplayNamesPreferences, LanguageIdentifierDisplayNameOptions, single::LanguageIdentifierDisplayNameOwned,
/// };
/// use icu::locale::{locale, langid};
/// use writeable::assert_try_writeable_eq;
///
/// let prefs = DisplayNamesPreferences::from(locale!("en"));
/// let options = LanguageIdentifierDisplayNameOptions::default();
/// let display_name = LanguageIdentifierDisplayNameOwned::try_new(
///     prefs,
///     langid!("fr-CA"),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_try_writeable_eq!(display_name.as_borrowed(), "Canadian French", Ok(()));
/// ```
///
/// When a subtag is unknown:
///
/// ```
/// use icu::experimental::displaynames::{
///     DisplayNamesPreferences, LanguageIdentifierDisplayNameOptions,
///     single::LanguageIdentifierDisplayNameOwned, single::LanguageIdentifierNameFallbackError,
/// };
/// use icu::locale::{locale, langid};
/// use writeable::{Part, TryWriteable, assert_try_writeable_parts_eq};
///
/// let prefs = DisplayNamesPreferences::from(locale!("en"));
/// let options = LanguageIdentifierDisplayNameOptions::default();
///
/// // "it-Qabc-150" has known language "it" ("Italian") and known region "150" ("Europe"),
/// // but unknown script "Qabc".
/// let lang_id = langid!("it-Qabc-150");
/// let display_name = LanguageIdentifierDisplayNameOwned::try_new(
///     prefs,
///     lang_id,
///     options,
/// )
/// .expect("Data should load successfully");
///
/// let borrowed = display_name.as_borrowed();
///
/// // The fallback string is identified with a [`Part::ERROR`](writeable::Part::Error):
/// assert_try_writeable_parts_eq!(
///     borrowed,
///     "Italian (Qabc, Europe)",
///     Err(LanguageIdentifierNameFallbackError),
///     [(9, 13, Part::ERROR)] // the span of Qabc
/// );
///
/// // To format in lossy mode (ignoring fallback errors), use standard `Writeable` or `Display`:
/// use writeable::assert_writeable_eq;
/// assert_writeable_eq!(borrowed, "Italian (Qabc, Europe)");
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct LanguageIdentifierDisplayNameOwned {
    formatting_locale: DataLocale,
    options: LanguageIdentifierDisplayNameOptions,
    language_payload: DataPayloadOr<LocaleNamesLanguageMediumV1, Language>,
    /// In the fallback (`Option<Script>`) state:
    /// - `None` indicates the script subtag was absent from the subject.
    /// - `Some(script)` indicates the script subtag was present, but display name data was not found, so it falls back to the raw code.
    script_payload: DataPayloadOr<LocaleNamesScriptMediumV1, Option<Script>>,
    /// In the fallback (`Option<Region>`) state:
    /// - `None` indicates the region subtag was absent from the subject.
    /// - `Some(region)` indicates the region subtag was present, but display name data was not found, so it falls back to the raw code.
    region_payload: DataPayloadOr<LocaleNamesRegionMediumV1, Option<Region>>,
    variant_payloads: DataPayloadOr<
        LocaleNamesVariantMediumV1,
        Vec<PayloadOrFallback<LocaleNamesVariantMediumV1, Variant>>,
    >,
    essentials_payload: DataPayload<LocaleNamesEssentialsV1>,
}

impl LanguageIdentifierDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
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
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
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

        // Step 1: Load language name
        // We want to find the best display name for the given subject.
        // In Dialect mode (default), we try to load names for combinations of subtags:
        // - Language + Script + Region (e.g., "zh-Hant-HK")
        // - Language + Script (e.g., "zh-Hant")
        // - Language + Region (e.g., "en-GB")
        // If any of these are found in the CLDR language names, we use it as the base name,
        // and we "consume" the corresponding subtags so they are not repeated in the qualifiers.
        // If none are found, we fall back to the base language name (e.g., "zh") and all
        // present subtags (script, region, variants) will be formatted as qualifiers.
        //
        // Prefer dialect handling if requested and available.
        let mut language_payload_or = None;

        // Only try dialect if requested (which is the default)
        if options.language_display.unwrap_or_default() == LanguageDisplay::Dialect {
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
                    language_payload_or = Some(DataPayloadOr::from_payload(response.payload));
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
        let language_payload = match language_payload_or {
            Some(payload) => payload,
            None => {
                let mut buffer = TinyAsciiStr::EMPTY;
                let attrs = LocaleNamesLanguageMediumV1::make_attributes(
                    subject.language,
                    None,
                    None,
                    &mut buffer,
                );
                let response = provider
                    .load(DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            attrs,
                            &formatting_locale,
                        ),
                        ..Default::default()
                    })
                    .allow_identifier_not_found()?;
                match response {
                    Some(obj) => DataPayloadOr::from_payload(obj.payload),
                    None => DataPayloadOr::from_other(subject.language),
                }
            }
        };

        // Step 2: Load script name (if present in subject)
        let script_payload = if let Some(script) = subject.script {
            match ScriptDisplayNameOwned::try_new_unstable(provider, prefs, script)
                .allow_identifier_not_found()?
            {
                Some(obj) => DataPayloadOr::from_payload(obj.payload),
                None => DataPayloadOr::from_other(Some(script)),
            }
        } else {
            DataPayloadOr::from_other(None)
        };

        // Step 3: Load region name (if present in subject)
        let region_payload = if let Some(region) = subject.region {
            match RegionDisplayNameOwned::try_new_unstable(provider, prefs, region)
                .allow_identifier_not_found()?
            {
                Some(obj) => DataPayloadOr::from_payload(obj.payload),
                None => DataPayloadOr::from_other(Some(region)),
            }
        } else {
            DataPayloadOr::from_other(None)
        };

        // Step 4: Load variant names (if present in subject)
        let load_variant = |variant: Variant| -> Result<
            PayloadOrFallback<LocaleNamesVariantMediumV1, Variant>,
            DataError,
        > {
            match VariantDisplayNameOwned::try_new_unstable(provider, prefs, variant)
                .allow_identifier_not_found()?
            {
                Some(obj) => Ok(DataPayloadOr::from_payload(obj.payload)),
                None => Ok(DataPayloadOr::from_other(variant)),
            }
        };

        let mut variant_results = subject
            .variants
            .iter()
            .map(|variant| load_variant(*variant))
            .peekable();

        let variant_payloads = if let Some(result) = variant_results.next() {
            let first_val = result?;
            if variant_results.peek().is_some() {
                // 2 or more variants
                let payload_vec = core::iter::once(Ok(first_val))
                    .chain(variant_results)
                    .collect::<Result<Vec<_>, _>>()?;
                DataPayloadOr::from_other(payload_vec)
            } else {
                // 1 variant
                match first_val.into_inner() {
                    Ok(payload) => DataPayloadOr::from_payload(payload),
                    Err(fallback_code) => {
                        DataPayloadOr::from_other(vec![DataPayloadOr::from_other(fallback_code)])
                    }
                }
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

    /// Returns a borrowed version of this display name
    /// suitable for writing out to a string.
    pub fn as_borrowed(&self) -> LanguageIdentifierDisplayName<'_> {
        let base_name = match self.language_payload.get() {
            Ok(p) => Ok(p.as_ref()),
            Err(lang) => Err(lang.as_str()),
        };

        let script_name = match self.script_payload.get() {
            Ok(p) => Some(Ok(p.as_ref())),
            Err(Some(script)) => Some(Err(script.as_str())),
            Err(None) => None,
        };

        let region_name = match self.region_payload.get() {
            Ok(p) => Some(Ok(p.as_ref())),
            Err(Some(region)) => Some(Err(region.as_str())),
            Err(None) => None,
        };

        let variants = match self.variant_payloads.get() {
            Ok(variant_name) => BorrowedVariants::One(variant_name),
            Err(vec) => BorrowedVariants::Slice(vec.as_slice()),
        };

        LanguageIdentifierDisplayName(LossyWrap(LanguageIdentifierDisplayNameInner {
            base_name,
            script_name,
            region_name,
            variants,
            locale_pattern: &self.essentials_payload.get().locale_pattern,
            locale_separator: &self.essentials_payload.get().locale_separator,
        }))
    }
}

/// Borrowed variants representation to avoid heap allocation.
///
/// Note: if a compiled-data-only constructor is added in the future,
/// this will need a new variant for a vec of borrowed variant names.
#[derive(Debug, Clone, Copy)]
enum BorrowedVariants<'a> {
    One(&'a str),
    Slice(&'a [PayloadOrFallback<LocaleNamesVariantMediumV1, Variant>]),
}

impl BorrowedVariants<'_> {
    #[inline]
    fn is_empty(&self) -> bool {
        matches!(self, Self::Slice([]))
    }
}

/// A localized display name for a language identifier.
///
/// See [`LanguageIdentifierDisplayNameOwned`].
#[derive(Debug, Clone, Copy)]
pub struct LanguageIdentifierDisplayName<'a>(
    pub(crate) LossyWrap<LanguageIdentifierDisplayNameInner<'a>>,
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct LanguageIdentifierDisplayNameInner<'a> {
    base_name: Result<&'a str, &'a str>,
    script_name: Option<Result<&'a str, &'a str>>,
    region_name: Option<Result<&'a str, &'a str>>,
    variants: BorrowedVariants<'a>,
    locale_pattern: &'a DoublePlaceholderPattern,
    locale_separator: &'a DoublePlaceholderPattern,
}

writeable::impl_try_writeable_delegate!(
    LanguageIdentifierDisplayName<'_>,
    |&self| &self.0.0,
    Error = LanguageIdentifierNameFallbackError
);

writeable::impl_writeable_delegate!(LanguageIdentifierDisplayName<'_>, |&self| &self.0);

writeable::impl_display_with_writeable!(LanguageIdentifierDisplayName<'_>);

struct QualifiersWriteable<'a> {
    script: Option<Result<&'a str, &'a str>>,
    region: Option<Result<&'a str, &'a str>>,
    variants: BorrowedVariants<'a>,
    separator: &'a DoublePlaceholderPattern,
}

impl<'a> TryWriteable for QualifiersWriteable<'a> {
    type Error = LanguageIdentifierNameFallbackError;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, core::fmt::Error> {
        let mut fallback_occurred = false;

        // TODO: See whether we can share this code with the list component.
        let mut first = true;
        let mut separator_str = ", ";
        for item in self.separator.iter() {
            if let PatternItem::Literal(s) = item {
                separator_str = s;
                break;
            }
        }

        let mut write_item =
            |sink: &mut S, res: Result<&str, &str>| -> Result<(), core::fmt::Error> {
                if !first {
                    sink.write_str(separator_str)?;
                }
                if res.try_write_to_parts(sink)?.is_err() {
                    fallback_occurred = true;
                }
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
                write_item(sink, Ok(v))?;
            }
            BorrowedVariants::Slice(slice) => {
                for item in slice.iter() {
                    let res = match item.get() {
                        Ok(p) => Ok(p.as_ref()),
                        Err(var) => Err(var.as_str()),
                    };
                    write_item(sink, res)?;
                }
            }
        }

        if fallback_occurred {
            Ok(Err(LanguageIdentifierNameFallbackError))
        } else {
            Ok(Ok(()))
        }
    }
}

impl<'a> TryWriteable for LanguageIdentifierDisplayNameInner<'a> {
    type Error = LanguageIdentifierNameFallbackError;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, core::fmt::Error> {
        let has_variants = !self.variants.is_empty();
        let has_qualifiers =
            self.script_name.is_some() || self.region_name.is_some() || has_variants;

        let fallback_occurred = if !has_qualifiers {
            self.base_name.try_write_to_parts(sink)?.is_err()
        } else {
            let qualifiers = QualifiersWriteable {
                script: self.script_name,
                region: self.region_name,
                variants: self.variants,
                separator: self.locale_separator,
            };

            self.locale_pattern
                .try_interpolate(DoublePlaceholderValueProviderTry(
                    &self.base_name,
                    &qualifiers,
                ))
                .try_write_to_parts(sink)?
                .is_err()
        };

        if fallback_occurred {
            Ok(Err(LanguageIdentifierNameFallbackError))
        } else {
            Ok(Ok(()))
        }
    }
}
