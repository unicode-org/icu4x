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
use icu_provider::marker::ErasedMarker;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use writeable::LengthHint;
use writeable::{PartsWrite, TryWriteable, adapters::LossyWrap};
use zerovec::VarZeroCow;

/// An error returned when a display name was not found in data and has fallen back to the raw BCP-47 subtag code.
#[derive(displaydoc::Display, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[allow(clippy::exhaustive_structs)]
pub struct LanguageIdentifierNameFallbackError;

/// A data struct that is either [`MenuNameParts`] or a string
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[allow(clippy::exhaustive_enums)] // provider-unstable
enum MenuNamePartsOrString<'a> {
    /// A data struct that is [`MenuNameParts`]
    MenuNameParts(VarZeroCow<'a, MenuNamePartsULE>),
    /// A data struct that is a string
    String(VarZeroCow<'a, str>),
}

size_test!(
    LanguageIdentifierDisplayNameOwned,
    language_identifier_display_name_owned_size,
    192
);

/// A localized display name for a language identifier, owned version.
///
/// The formatter falls back to the BCP-47 subtag when localized display names are missing
/// from the data provider. Fallback can be detected using [`TryWriteable`].
///
/// # Examples
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
    language_payload: DataPayloadOr<ErasedMarker<MenuNamePartsOrString<'static>>, Language>,
    /// All other fields (shared between Standard and Menu)
    qualifiers: QualifiersOwned,
}

#[derive(Debug)]
struct QualifiersOwned {
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
        // Only try dialect if requested (or default)
        let language_payload =
            if options.language_display.unwrap_or_default() == LanguageDisplay::Dialect {
                Self::load_language_dialect_name(provider, &formatting_locale, &mut subject)?
            } else {
                None
            };

        // If the language name is not loaded yet, try loading it from the language subtag alone.
        let language_payload = match language_payload {
            Some(response) => DataPayloadOr::from_payload(
                response
                    .payload
                    .map_project(|payload, _| MenuNamePartsOrString::String(payload)),
            ),
            None => {
                match Self::load_language_subtag_name(
                    provider,
                    &formatting_locale,
                    subject.language,
                )? {
                    Some(response) => DataPayloadOr::from_payload(
                        response
                            .payload
                            .map_project(|payload, _| MenuNamePartsOrString::String(payload)),
                    ),
                    None => DataPayloadOr::from_other(subject.language),
                }
            }
        };

        // Load the remaining data
        let qualifiers = QualifiersOwned::try_new_unstable(provider, prefs, subject)?;

        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the menu-style language display name for a given language identifier and locale using compiled data.
        ///
        /// # Examples
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
        /// let display_name = LanguageIdentifierDisplayNameOwned::try_new_menu(
        ///     prefs,
        ///     langid!("fr-CA"),
        ///     options,
        /// )
        /// .expect("Data should load successfully");
        ///
        /// assert_try_writeable_eq!(display_name.as_borrowed(), "French (Canada)", Ok(()));
        /// ```
        functions: [
            try_new_menu,
            try_new_menu_with_buffer_provider,
            try_new_menu_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_menu)]
    pub fn try_new_menu_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
        _options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageMenuMediumV1>
            + DataProvider<LocaleNamesLanguageMediumV1>
            + DataProvider<LocaleNamesScriptMediumV1>
            + DataProvider<LocaleNamesRegionMediumV1>
            + DataProvider<LocaleNamesVariantMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let formatting_locale = LocaleNamesLanguageMediumV1::make_locale(prefs.locale_preferences);

        // Step 1: Load language name
        // Try the menu name
        let language_payload =
            Self::load_language_menu_name(provider, &formatting_locale, subject.language)?;

        // If the language name is not loaded yet, try loading it from the language subtag alone.
        let language_payload = match language_payload {
            Some(response) => {
                DataPayloadOr::from_payload(response.payload.map_project(|menu_ule, _phantom| {
                    MenuNamePartsOrString::MenuNameParts(menu_ule)
                }))
            }
            None => {
                match Self::load_language_subtag_name(
                    provider,
                    &formatting_locale,
                    subject.language,
                )? {
                    Some(response) => DataPayloadOr::from_payload(response.payload.map_project(
                        |subtag_name, _phantom| MenuNamePartsOrString::String(subtag_name),
                    )),
                    None => DataPayloadOr::from_other(subject.language),
                }
            }
        };

        // Load the remaining data
        let qualifiers = QualifiersOwned::try_new_unstable(provider, prefs, subject)?;

        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    /// Loads the name for a langauge dialect, which includes script and region subtags.
    ///
    /// We try to load names for combinations of subtags:
    ///
    /// - Language + Script + Region (e.g., "zh-Hant-HK")
    /// - Language + Script (e.g., "zh-Hant")
    /// - Language + Region (e.g., "en-GB")
    ///
    /// We then "consume"  the corresponding subtags from the input `LanguageIdentifier`
    /// so they are not repeated in the qualifiers.
    fn load_language_dialect_name<P>(
        provider: &P,
        formatting_locale: &DataLocale,
        subject: &mut LanguageIdentifier,
    ) -> Result<Option<DataResponse<LocaleNamesLanguageMediumV1>>, DataError>
    where
        P: ?Sized + DataProvider<LocaleNamesLanguageMediumV1>,
    {
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
            let attrs =
                LocaleNamesLanguageMediumV1::make_attributes(language, script, region, &mut buffer);
            let id =
                DataIdentifierBorrowed::for_marker_attributes_and_locale(attrs, formatting_locale);
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            if let Some(response) = provider
                .load(DataRequest { id, metadata })
                .allow_identifier_not_found()?
            {
                if script.is_some() {
                    subject.script = None;
                }
                if region.is_some() {
                    subject.region = None;
                }
                return Ok(Some(response));
            }
        }
        Ok(None)
    }

    /// Loads the name for an individual language subtag.
    fn load_language_subtag_name<P>(
        provider: &P,
        formatting_locale: &DataLocale,
        language: Language,
    ) -> Result<Option<DataResponse<LocaleNamesLanguageMediumV1>>, DataError>
    where
        P: ?Sized + DataProvider<LocaleNamesLanguageMediumV1>,
    {
        let mut buffer = TinyAsciiStr::EMPTY;
        let attrs = LocaleNamesLanguageMediumV1::make_attributes(language, None, None, &mut buffer);
        provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    attrs,
                    formatting_locale,
                ),
                ..Default::default()
            })
            .allow_identifier_not_found()
    }

    /// Loads the name for a language with menu core and extension parts.
    fn load_language_menu_name<P>(
        provider: &P,
        formatting_locale: &DataLocale,
        language: Language,
    ) -> Result<Option<DataResponse<LocaleNamesLanguageMenuMediumV1>>, DataError>
    where
        P: ?Sized + DataProvider<LocaleNamesLanguageMenuMediumV1>,
    {
        let mut buffer = TinyAsciiStr::EMPTY;
        // NOTE: Menu and non-Menu use the same attributes
        let attrs = LocaleNamesLanguageMediumV1::make_attributes(language, None, None, &mut buffer);
        provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    attrs,
                    formatting_locale,
                ),
                ..Default::default()
            })
            .allow_identifier_not_found()
    }
}

impl QualifiersOwned {
    fn try_new_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptMediumV1>
            + DataProvider<LocaleNamesRegionMediumV1>
            + DataProvider<LocaleNamesVariantMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
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
                id: DataIdentifierBorrowed::for_locale(&LocaleNamesEssentialsV1::make_locale(
                    prefs.locale_preferences,
                )),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            script_payload,
            region_payload,
            variant_payloads,
            essentials_payload,
        })
    }
}

impl LanguageIdentifierDisplayNameOwned {
    /// Returns a borrowed version of this display name
    /// suitable for writing out to a string.
    pub fn as_borrowed(&self) -> LanguageIdentifierDisplayName<'_> {
        let mut qualifiers = self.qualifiers.as_borrowed();
        let base_name = match self.language_payload.get() {
            Ok(MenuNamePartsOrString::String(string)) => NameOrFallback(Ok(string.as_ref())),
            Ok(MenuNamePartsOrString::MenuNameParts(parts)) => {
                if !parts.extension().is_empty() {
                    qualifiers.menu_extension = Some(parts.extension());
                }
                NameOrFallback(Ok(parts.core()))
            }
            Err(lang) => NameOrFallback(Err(lang.as_str())),
        };

        LanguageIdentifierDisplayName(LossyWrap(LanguageIdentifierDisplayNameInner {
            base_name,
            qualifiers,
        }))
    }
}

impl QualifiersOwned {
    pub fn as_borrowed(&self) -> QualifiersBorrowed<'_> {
        let script = match self.script_payload.get() {
            Ok(p) => Some(NameOrFallback(Ok(p.as_ref()))),
            Err(Some(script)) => Some(NameOrFallback(Err(script.as_str()))),
            Err(None) => None,
        };

        let region = match self.region_payload.get() {
            Ok(p) => Some(NameOrFallback(Ok(p.as_ref()))),
            Err(Some(region)) => Some(NameOrFallback(Err(region.as_str()))),
            Err(None) => None,
        };

        let variants = match self.variant_payloads.get() {
            Ok(variant_name) => BorrowedVariants::One(NameOrFallback(Ok(variant_name))),
            Err(Ok(vec)) => BorrowedVariants::Slice(vec.as_slice()),
            Err(Err(variant)) => BorrowedVariants::One(NameOrFallback(Err(variant.as_str()))),
        };

        QualifiersBorrowed {
            menu_extension: None,
            script,
            region,
            variants,
            glue: &self.essentials_payload.get().locale_pattern,
            separator: &self.essentials_payload.get().locale_separator,
        }
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
    qualifiers: QualifiersBorrowed<'a>,
}

writeable::impl_try_writeable_delegate!(
    LanguageIdentifierDisplayName<'_>,
    |&self| &self.0.0,
    Error = LanguageIdentifierNameFallbackError
);

writeable::impl_writeable_delegate!(LanguageIdentifierDisplayName<'_>, |&self| &self.0);

writeable::impl_display_with_writeable!(LanguageIdentifierDisplayName<'_>);

#[derive(Debug, Copy, Clone)]
struct QualifiersBorrowed<'a> {
    menu_extension: Option<&'a str>,
    script: Option<NameOrFallback<'a>>,
    region: Option<NameOrFallback<'a>>,
    variants: BorrowedVariants<'a>,
    glue: &'a DoublePlaceholderPattern,
    separator: &'a DoublePlaceholderPattern,
}

impl<'a> QualifiersBorrowed<'a> {
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

    fn is_empty(&self) -> bool {
        self.menu_extension.is_none()
            && self.script.is_none()
            && self.region.is_none()
            && self.variants.is_empty()
    }
}

impl<'a> TryWriteable for QualifiersBorrowed<'a> {
    type Error = LanguageIdentifierNameFallbackError;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, core::fmt::Error> {
        // TODO: See whether we can share this code with the list component.
        let mut first = true;
        let separator_str = self.separator_str();

        if let Some(menu_extension) = self.menu_extension {
            sink.write_str(menu_extension)?;
            first = false;
        }

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
        if let Some(menu_extension) = self.menu_extension {
            length_hint += writeable::Writeable::writeable_length_hint(&menu_extension);
            num_items += 1;
        }
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

impl<'a> TryWriteable for LanguageIdentifierDisplayNameInner<'a> {
    type Error = LanguageIdentifierNameFallbackError;

    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, core::fmt::Error> {
        if self.qualifiers.is_empty() {
            self.base_name.try_write_to_parts(sink)
        } else {
            let result = self
                .qualifiers
                .glue
                .try_interpolate(DoublePlaceholderValueProviderTry(
                    self.base_name,
                    self.qualifiers,
                ))
                .try_write_to_parts(sink)?;
            Ok(result.map_err(either::Either::into_inner))
        }
    }

    fn writeable_length_hint(&self) -> LengthHint {
        if self.qualifiers.is_empty() {
            self.base_name.writeable_length_hint()
        } else {
            self.qualifiers
                .glue
                .try_interpolate(DoublePlaceholderValueProviderTry(
                    self.base_name,
                    self.qualifiers,
                ))
                .writeable_length_hint()
        }
    }

    fn try_writeable_borrow(&self) -> Option<Result<&str, (Self::Error, &str)>> {
        if self.qualifiers.is_empty() {
            self.base_name.try_writeable_borrow()
        } else {
            None
        }
    }
}
