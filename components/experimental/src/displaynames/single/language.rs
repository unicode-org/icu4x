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
use icu_pattern::{DoublePlaceholderPattern, DoublePlaceholderValueProviderTry};
use icu_provider::DataPayloadOr;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use writeable::{PartsWrite, TryWriteable, adapters::LossyWrap};

/// Display name fallback occurred
#[derive(displaydoc::Display, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[allow(clippy::exhaustive_structs)]
pub struct LanguageIdentifierNameFallbackError;

/// Represents a subtag that is either absent or has fallen back to its code.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
enum AbsentOrFallback {
    /// The subtag was not present in the subject.
    Absent,
    /// The subtag was present, but its display name was not found, so we fall back to the code.
    Fallback(TinyAsciiStr<4>),
}

/// Represents a payload that was either successfully loaded or has fallen back to its code.
type PayloadOrFallback<M> = DataPayloadOr<M, TinyAsciiStr<8>>;

/// A localized display name for a language identifier, owned version.
///
/// # Example
///
/// ```
/// use icu::experimental::displaynames::{
///     DisplayNamesPreferences, LanguageIdentifierDisplayNameOptions, single::LanguageIdentifierDisplayNameOwned,
/// };
/// use icu::locale::{locale, langid};
/// use writeable::assert_writeable_eq;
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
/// assert_writeable_eq!(display_name.as_borrowed_with_fallback(), "Canadian French");
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct LanguageIdentifierDisplayNameOwned {
    formatting_locale: DataLocale,
    options: LanguageIdentifierDisplayNameOptions,
    language_payload: DataPayloadOr<LocaleNamesLanguageMediumV1, TinyAsciiStr<4>>,
    script_payload: DataPayloadOr<LocaleNamesScriptMediumV1, AbsentOrFallback>,
    region_payload: DataPayloadOr<LocaleNamesRegionMediumV1, AbsentOrFallback>,
    variant_payloads: DataPayloadOr<
        LocaleNamesVariantMediumV1,
        Vec<PayloadOrFallback<LocaleNamesVariantMediumV1>>,
    >,
    essentials_payload: DataPayload<LocaleNamesEssentialsV1>,
}

impl LanguageIdentifierDisplayNameOwned {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: icu_locale::LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
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
                // - Some(Some(subtag)) in the first position means "this should be present"
                // - Some(None) in the first position means "this must be absent"
                // - None in the first position means "skip this field"
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
                let result = provider.load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        attrs,
                        &formatting_locale,
                    ),
                    ..Default::default()
                });
                match result {
                    Ok(response) => DataPayloadOr::from_payload(response.payload),
                    Err(e) if e.kind == DataErrorKind::IdentifierNotFound => {
                        DataPayloadOr::from_other(subject.language.to_tinystr().resize())
                    }
                    Err(e) => return Err(e),
                }
            }
        };

        // Step 2: Load script name (if present in subject)
        let script_payload = if let Some(script) = subject.script {
            let res = ScriptDisplayNameOwned::try_new_unstable(provider, prefs, script);
            match res {
                Ok(obj) => DataPayloadOr::from_payload(obj.payload),
                Err(e) if e.kind == DataErrorKind::IdentifierNotFound => DataPayloadOr::from_other(
                    AbsentOrFallback::Fallback(script.to_tinystr().resize()),
                ),
                Err(e) => return Err(e),
            }
        } else {
            DataPayloadOr::from_other(AbsentOrFallback::Absent)
        };

        // Step 3: Load region name (if present in subject)
        let region_payload = if let Some(region) = subject.region {
            let res = RegionDisplayNameOwned::try_new_unstable(provider, prefs, region);
            match res {
                Ok(obj) => DataPayloadOr::from_payload(obj.payload),
                Err(e) if e.kind == DataErrorKind::IdentifierNotFound => DataPayloadOr::from_other(
                    AbsentOrFallback::Fallback(region.to_tinystr().resize()),
                ),
                Err(e) => return Err(e),
            }
        } else {
            DataPayloadOr::from_other(AbsentOrFallback::Absent)
        };

        // Step 4: Load variant names (if present in subject)
        let load_variant = |variant: icu_locale::subtags::Variant| -> Result<PayloadOrFallback<LocaleNamesVariantMediumV1>, DataError> {
            let res = VariantDisplayNameOwned::try_new_unstable(provider, prefs, variant);
            match res {
                Ok(obj) => Ok(DataPayloadOr::from_payload(obj.payload)),
                Err(e) if e.kind == DataErrorKind::IdentifierNotFound => {
                    Ok(DataPayloadOr::from_other(variant.to_tinystr().resize()))
                }
                Err(e) => Err(e),
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

    /// Returns a borrowed version of this display name.
    pub fn as_borrowed(&self) -> LanguageIdentifierDisplayName<'_> {
        let base_name = match self.language_payload.get() {
            Ok(p) => Ok(p.as_ref()),
            Err(lang) => Err(lang.as_str()),
        };

        let script_name = match self.script_payload.get() {
            Ok(p) => Some(Ok(p.as_ref())),
            Err(AbsentOrFallback::Fallback(script)) => Some(Err(script.as_str())),
            Err(AbsentOrFallback::Absent) => None,
        };

        let region_name = match self.region_payload.get() {
            Ok(p) => Some(Ok(p.as_ref())),
            Err(AbsentOrFallback::Fallback(region)) => Some(Err(region.as_str())),
            Err(AbsentOrFallback::Absent) => None,
        };

        let variants = match self.variant_payloads.get() {
            Ok(variant_name) => BorrowedVariants::One(variant_name),
            Err(vec) => BorrowedVariants::Slice(vec.as_slice()),
        };

        LanguageIdentifierDisplayName {
            base_name,
            script_name,
            region_name,
            variants,
            locale_pattern: &self.essentials_payload.get().locale_pattern,
            locale_separator: &self.essentials_payload.get().locale_separator,
        }
    }

    /// Returns a writeable that formats the display name, ignoring fallback errors.
    #[inline]
    pub fn as_borrowed_with_fallback(&self) -> LossyWrap<LanguageIdentifierDisplayName<'_>> {
        self.as_borrowed().with_fallback()
    }
}

/// Borrowed variants representation to avoid heap allocation.
///
/// Note: if a compiled-data-only constructor is added in the future,
/// this will need a new variant for a vec of borrowed variant names.
#[derive(Debug, Clone, Copy)]
pub(crate) enum BorrowedVariants<'a> {
    One(&'a str),
    Slice(&'a [PayloadOrFallback<LocaleNamesVariantMediumV1>]),
}

impl BorrowedVariants<'_> {
    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            Self::One(_) => false,
            Self::Slice(slice) => slice.is_empty(),
        }
    }
}

/// A localized display name for a language identifier.
#[derive(Debug, Clone, Copy)]
pub struct LanguageIdentifierDisplayName<'a> {
    base_name: Result<&'a str, &'a str>,
    script_name: Option<Result<&'a str, &'a str>>,
    region_name: Option<Result<&'a str, &'a str>>,
    variants: BorrowedVariants<'a>,
    locale_pattern: &'a DoublePlaceholderPattern,
    locale_separator: &'a DoublePlaceholderPattern,
}

impl<'a> LanguageIdentifierDisplayName<'a> {
    /// Returns a writeable that formats the display name, ignoring fallback errors.
    #[inline]
    pub fn with_fallback(&self) -> LossyWrap<Self> {
        LossyWrap(*self)
    }
}

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
            if let icu_pattern::PatternItem::Literal(s) = item {
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

impl<'a> TryWriteable for LanguageIdentifierDisplayName<'a> {
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
