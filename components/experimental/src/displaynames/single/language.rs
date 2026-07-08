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
use crate::size_test_macro::size_test;
use alloc::vec::Vec;
use icu_locale_core::LanguageIdentifier;
use icu_locale_core::subtags::{Language, Region, Script, Variant};
use icu_pattern::{DoublePlaceholderPattern, DoublePlaceholderValueProviderTry, PatternItem};
use icu_provider::DataPayloadOr;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use writeable::LengthHint;
use writeable::{PartsWrite, TryWriteable, adapters::LossyWrap};

/// An error returned when a display name was not found in data and has fallen back to the raw BCP-47 subtag code.
#[derive(displaydoc::Display, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[allow(clippy::exhaustive_structs)]
pub struct LanguageIdentifierNameFallbackError;

size_test!(
    LanguageIdentifierDisplayNameOwned,
    language_identifier_display_name_owned_size,
    184
);

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
/// // To format in lossy mode (ignoring fallback errors), use [`Writeable`] or [`Display`]:
/// use writeable::assert_writeable_eq;
/// assert_writeable_eq!(borrowed, "Italian (Qabc, Europe)");
/// ```
#[doc = language_identifier_display_name_owned_size!()]
#[derive(Debug)]
pub struct LanguageIdentifierDisplayNameOwned {
    /// Either the language display name or the subtag as fallback
    language_payload: DataPayloadOr<LocaleNamesLanguageMediumV1, Language>,
    /// Either the script display name, the subtag as fallback, or None if absent
    script_payload: DataPayloadOr<LocaleNamesScriptMediumV1, Option<Script>>,
    /// Either the region display name, the subtag as fallback, or None if absent
    region_payload: DataPayloadOr<LocaleNamesRegionMediumV1, Option<Region>>,
    /// Either a single variant display name, the subtag as fallback, or
    /// a vector of variant display names or subtags as fallback.
    /// The vector may be empty.
    variant_payloads: DataPayloadOr<
        LocaleNamesVariantMediumV1,
        Result<Vec<DataPayloadOr<LocaleNamesVariantMediumV1, Variant>>, Variant>,
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
        let mut language_payload = None;

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
                    language_payload = Some(DataPayloadOr::from_payload(response.payload));
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
            DataPayloadOr<LocaleNamesVariantMediumV1, Variant>,
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
            .map(|variant| load_variant(*variant));

        let variant_payloads = if let Some(first) = variant_results.next() {
            if let Some(second) = variant_results.next() {
                // 2 or more variants
                let payload_vec = [first, second]
                    .into_iter()
                    .chain(variant_results)
                    .collect::<Result<Vec<_>, _>>()?;
                DataPayloadOr::from_other(Ok(payload_vec))
            } else {
                // 1 variant
                match first?.into_inner() {
                    Ok(payload) => DataPayloadOr::from_payload(payload),
                    Err(fallback_code) => DataPayloadOr::from_other(Err(fallback_code)),
                }
            }
        } else {
            // 0 variants
            DataPayloadOr::from_other(Ok(Vec::new()))
        };

        // Step 5: Load essentials
        let essentials_payload = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&formatting_locale),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
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
            Ok(p) => NameOrFallback(Ok(p.as_ref())),
            Err(lang) => NameOrFallback(Err(lang.as_str())),
        };

        let script_name = match self.script_payload.get() {
            Ok(p) => Some(NameOrFallback(Ok(p.as_ref()))),
            Err(Some(script)) => Some(NameOrFallback(Err(script.as_str()))),
            Err(None) => None,
        };

        let region_name = match self.region_payload.get() {
            Ok(p) => Some(NameOrFallback(Ok(p.as_ref()))),
            Err(Some(region)) => Some(NameOrFallback(Err(region.as_str()))),
            Err(None) => None,
        };

        let variants = match self.variant_payloads.get() {
            Ok(variant_name) => BorrowedVariants::One(NameOrFallback(Ok(variant_name))),
            Err(Ok(vec)) => BorrowedVariants::Slice(vec.as_slice()),
            Err(Err(variant)) => BorrowedVariants::One(NameOrFallback(Err(variant.as_str()))),
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
    One(NameOrFallback<'a>),
    Slice(&'a [DataPayloadOr<LocaleNamesVariantMediumV1, Variant>]),
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

/// A struct implementing [`TryWriteable`] that returns a [`LanguageIdentifierNameFallbackError`]
#[derive(Debug, Clone, Copy)]
struct NameOrFallback<'a>(Result<&'a str, &'a str>);

writeable::impl_try_writeable_delegate!(
    NameOrFallback<'a>,
    |&self| &self.0,
    Error = LanguageIdentifierNameFallbackError,
    |_fallback_str| LanguageIdentifierNameFallbackError,
    where 'a
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct LanguageIdentifierDisplayNameInner<'a> {
    base_name: NameOrFallback<'a>,
    script_name: Option<NameOrFallback<'a>>,
    region_name: Option<NameOrFallback<'a>>,
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
    script: Option<NameOrFallback<'a>>,
    region: Option<NameOrFallback<'a>>,
    variants: BorrowedVariants<'a>,
    separator: &'a DoublePlaceholderPattern,
}

impl<'a> QualifiersWriteable<'a> {
    fn separator_str(&self) -> &'a str {
        let mut separator_str = ", ";
        for item in self.separator.iter() {
            if let PatternItem::Literal(s) = item {
                separator_str = s;
                break;
            }
        }
        separator_str
    }
}

impl<'a> TryWriteable for QualifiersWriteable<'a> {
    type Error = LanguageIdentifierNameFallbackError;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, core::fmt::Error> {
        // TODO: See whether we can share this code with the list component.
        let mut first = true;
        let separator_str = self.separator_str();

        let mut write_item = |sink: &mut S,
                              res: NameOrFallback|
         -> Result<
            Result<(), LanguageIdentifierNameFallbackError>,
            core::fmt::Error,
        > {
            if !first {
                sink.write_str(separator_str)?;
            }
            first = false;
            res.try_write_to_parts(sink)
        };

        let mut result = Ok(());
        if let Some(script) = self.script {
            result = result.and(write_item(sink, script)?);
        }
        if let Some(region) = self.region {
            result = result.and(write_item(sink, region)?);
        }
        match self.variants {
            BorrowedVariants::One(variant) => {
                result = result.and(write_item(sink, variant)?);
            }
            BorrowedVariants::Slice(slice) => {
                for item in slice.iter() {
                    let res = match item.get() {
                        Ok(p) => NameOrFallback(Ok(p.as_ref())),
                        Err(var) => NameOrFallback(Err(var.as_str())),
                    };
                    result = result.and(write_item(sink, res)?);
                }
            }
        }

        Ok(result)
    }

    fn writeable_length_hint(&self) -> LengthHint {
        let mut length_hint = LengthHint::exact(0);
        let mut num_items = 0;
        if let Some(script) = self.script {
            length_hint += script.writeable_length_hint();
            num_items += 1;
        }
        if let Some(region) = self.region {
            length_hint += region.writeable_length_hint();
            num_items += 1;
        }
        match self.variants {
            BorrowedVariants::One(variant) => {
                length_hint += variant.writeable_length_hint();
                num_items += 1;
            }
            BorrowedVariants::Slice(slice) => {
                for item in slice.iter() {
                    length_hint += match item.get() {
                        Ok(p) => writeable::Writeable::writeable_length_hint(&**p),
                        Err(var) => writeable::Writeable::writeable_length_hint(var.as_str()),
                    };
                    num_items += 1;
                }
            }
        }
        length_hint += LengthHint::exact(self.separator_str().len() * (num_items - 1));
        length_hint
    }
}

impl LanguageIdentifierDisplayNameInner<'_> {
    fn has_qualifiers(&self) -> bool {
        self.script_name.is_some() || self.region_name.is_some() || !self.variants.is_empty()
    }
}

impl<'a> TryWriteable for LanguageIdentifierDisplayNameInner<'a> {
    type Error = LanguageIdentifierNameFallbackError;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, core::fmt::Error> {
        if !self.has_qualifiers() {
            self.base_name.try_write_to_parts(sink)
        } else {
            let result = self
                .locale_pattern
                .try_interpolate(DoublePlaceholderValueProviderTry(
                    self.base_name,
                    QualifiersWriteable {
                        script: self.script_name,
                        region: self.region_name,
                        variants: self.variants,
                        separator: self.locale_separator,
                    },
                ))
                .try_write_to_parts(sink)?;
            Ok(result.map_err(either::Either::into_inner))
        }
    }

    fn writeable_length_hint(&self) -> LengthHint {
        if !self.has_qualifiers() {
            self.base_name.writeable_length_hint()
        } else {
            self.locale_pattern
                .try_interpolate(DoublePlaceholderValueProviderTry(
                    self.base_name,
                    QualifiersWriteable {
                        script: self.script_name,
                        region: self.region_name,
                        variants: self.variants,
                        separator: self.locale_separator,
                    },
                ))
                .writeable_length_hint()
        }
    }

    fn try_writeable_borrow(&self) -> Option<Result<&str, (Self::Error, &str)>> {
        if !self.has_qualifiers() {
            self.base_name.try_writeable_borrow()
        } else {
            None
        }
    }
}
