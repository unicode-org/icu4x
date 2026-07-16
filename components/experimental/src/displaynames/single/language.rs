// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::displaynames::provider::{
    LocaleNamesEssentialsV1, LocaleNamesLanguageCoreLongV1, LocaleNamesLanguageCoreMediumV1,
    LocaleNamesLanguageCoreShortV1, LocaleNamesLanguageExtendedLongV1,
    LocaleNamesLanguageExtendedMediumV1, LocaleNamesLanguageExtendedShortV1,
    LocaleNamesLanguageMenuCoreMediumV1, LocaleNamesLanguageMenuExtendedMediumV1,
    LocaleNamesLanguageMinimalMediumV1, LocaleNamesRegionCoreMediumV1,
    LocaleNamesRegionCoreShortV1, LocaleNamesRegionExtendedShortV1,
    LocaleNamesRegionMinimalMediumV1, LocaleNamesRegionMinimalShortV1,
    LocaleNamesScriptCoreMediumV1, LocaleNamesScriptExtendedMediumV1,
    LocaleNamesScriptExtendedShortV1, LocaleNamesScriptMinimalMediumV1,
    LocaleNamesVariantExtendedMediumV1, MenuNamePartsULE,
};
use crate::displaynames::single::{
    RegionDisplayNameOwned, ScriptDisplayNameOwned, VariantDisplayNameOwned, try_load_markers,
};
use crate::displaynames::{DisplayNamesPreferences, LanguageIdentifierDisplayNameOptions};
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
    script_payload: DataPayloadOr<ErasedMarker<VarZeroCow<'static, str>>, Option<Script>>,
    /// Either the region display name, the subtag as fallback, or None if absent
    region_payload: DataPayloadOr<ErasedMarker<VarZeroCow<'static, str>>, Option<Region>>,
    /// Either a single variant display name, the subtag as fallback, or
    /// a vector of variant display names or subtags as fallback.
    /// The vector may be empty.
    variant_payloads: DataPayloadOr<
        ErasedMarker<VarZeroCow<'static, str>>,
        Result<Vec<DataPayloadOr<ErasedMarker<VarZeroCow<'static, str>>, Variant>>, Variant>,
    >,
    essentials_payload: DataPayload<LocaleNamesEssentialsV1>,
}

/// Loads the name for a language dialect, which includes script and region subtags.
///
/// We try to load names for combinations of subtags:
///
/// - Language + Script + Region (e.g., "zh-Hant-HK")
/// - Language + Script (e.g., "zh-Hant")
/// - Language + Region (e.g., "en-GB")
///
/// We then "consume"  the corresponding subtags from the input `LanguageIdentifier`
/// so they are not repeated in the qualifiers.
macro_rules! try_load_dialect_name {
    ($provider:expr, $locale:expr, $subject:expr, [ $($marker:ident),+ $(,)? ]) => {{
        let mut result = None;
        for (language, script, region) in [
            ($subject.language, Some($subject.script), Some($subject.region)),
            ($subject.language, Some($subject.script), None),
            ($subject.language, None, Some($subject.region)),
        ] {
            let script_val = match script {
                Some(Some(s)) => Some(s),
                Some(None) => continue,
                None => None,
            };
            let region_val = match region {
                Some(Some(r)) => Some(r),
                Some(None) => continue,
                None => None,
            };
            let mut buffer = TinyAsciiStr::EMPTY;
            let attrs = LocaleNamesLanguageCoreMediumV1::make_attributes(
                language,
                script_val,
                region_val,
                &mut buffer,
            );
            if let Ok(payload) = try_load_markers!($provider, $locale, attrs, [ $($marker),+ ]) {
                if script_val.is_some() {
                    $subject.script = None;
                }
                if region_val.is_some() {
                    $subject.region = None;
                }
                let payload_cast: DataPayload<LocaleNamesLanguageCoreMediumV1> = payload.cast();
                result = Some(payload_cast.map_project(|p, _| MenuNamePartsOrString::String(p)));
                break;
            }
        }
        result
    }};
}

/// Loads the name for an individual language subtag.
macro_rules! try_load_subtag_name {
    ($provider:expr, $locale:expr, $language:expr, [ $($marker:ident),+ $(,)? ]) => {{
        let mut buffer = TinyAsciiStr::EMPTY;
        let attrs = LocaleNamesLanguageCoreMediumV1::make_attributes(
            $language,
            None,
            None,
            &mut buffer,
        );
        match try_load_markers!($provider, $locale, attrs, [ $($marker),+ ]) {
            Ok(payload) => {
                let payload_cast: DataPayload<LocaleNamesLanguageCoreMediumV1> = payload.cast();
                Some(payload_cast.map_project(|p, _| MenuNamePartsOrString::String(p)))
            }
            Err(_) => None,
        }
    }};
}

/// Loads the name for a language with menu core and extension parts.
macro_rules! try_load_menu_name {
    ($provider:expr, $locale:expr, $language:expr, [ $($marker:ident),+ $(,)? ]) => {{
        let mut buffer = TinyAsciiStr::EMPTY;
        let attrs = LocaleNamesLanguageCoreMediumV1::make_attributes(
            $language,
            None,
            None,
            &mut buffer,
        );
        match try_load_markers!($provider, $locale, attrs, [ $($marker),+ ]) {
            Ok(payload) => {
                let payload_cast: DataPayload<LocaleNamesLanguageMenuCoreMediumV1> = payload.cast();
                Some(payload_cast.map_project(|p, _| MenuNamePartsOrString::MenuNameParts(p)))
            }
            Err(_) => None,
        }
    }};
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
        D: ?Sized
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        let qualifiers = QualifiersOwned::try_new_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the minimal language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new_minimal,
            try_new_minimal_with_buffer_provider,
            try_new_minimal_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_minimal)]
    pub fn try_new_minimal_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        // Step 1: Load language name
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        // Only try dialect if requested (or default)
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [LocaleNamesLanguageMinimalMediumV1]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [LocaleNamesLanguageMinimalMediumV1]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        // Load the remaining data
        let qualifiers = QualifiersOwned::try_new_minimal_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the minimal short language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new_minimal_short,
            try_new_minimal_short_with_buffer_provider,
            try_new_minimal_short_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_minimal_short)]
    pub fn try_new_minimal_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        // Step 1: Load language name
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        // Only try dialect if requested (or default)
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [LocaleNamesLanguageMinimalMediumV1]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [LocaleNamesLanguageMinimalMediumV1]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        // Load the remaining data
        let qualifiers = QualifiersOwned::try_new_minimal_short_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the minimal long language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new_minimal_long,
            try_new_minimal_long_with_buffer_provider,
            try_new_minimal_long_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_minimal_long)]
    pub fn try_new_minimal_long_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        // Step 1: Load language name
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        // Only try dialect if requested (or default)
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [LocaleNamesLanguageMinimalMediumV1]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [LocaleNamesLanguageMinimalMediumV1]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        // Load the remaining data
        let qualifiers = QualifiersOwned::try_new_minimal_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the minimal menu-style language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new_minimal_menu,
            try_new_minimal_menu_with_buffer_provider,
            try_new_minimal_menu_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_minimal_menu)]
    pub fn try_new_minimal_menu_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
        _options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let mut language_payload = try_load_subtag_name!(
            provider,
            prefs,
            subject.language,
            [LocaleNamesLanguageMinimalMediumV1]
        );
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [LocaleNamesLanguageMinimalMediumV1]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        let qualifiers = QualifiersOwned::try_new_minimal_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the short language display name for a given language identifier and locale using compiled data.
        ///
        /// Falls back to the medium name if the short name is not available.
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
        ///
        /// // Default (Medium) length format:
        /// let display_name_medium = LanguageIdentifierDisplayNameOwned::try_new(
        ///     prefs,
        ///     langid!("en-US"),
        ///     options,
        /// )
        /// .expect("Data should load successfully");
        /// assert_try_writeable_eq!(
        ///     display_name_medium.as_borrowed(),
        ///     "American English",
        ///     Ok(())
        /// );
        ///
        /// // Short length format uses shorter subtag/qualifier names when available:
        /// let display_name_short = LanguageIdentifierDisplayNameOwned::try_new_short(
        ///     prefs,
        ///     langid!("en-US"),
        ///     options,
        /// )
        /// .expect("Data should load successfully");
        /// assert_try_writeable_eq!(
        ///     display_name_short.as_borrowed(),
        ///     "US English",
        ///     Ok(())
        /// );
        /// ```
        functions: [
            try_new_short,
            try_new_short_with_buffer_provider,
            try_new_short_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short)]
    pub fn try_new_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageCoreShortV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [
                    LocaleNamesLanguageCoreShortV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageCoreShortV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        let qualifiers = QualifiersOwned::try_new_short_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the long language display name for a given language identifier and locale using compiled data.
        ///
        /// Falls back to the medium name if the long name is not available.
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
        ///
        /// // Default (Medium) length format:
        /// let display_name_medium = LanguageIdentifierDisplayNameOwned::try_new(
        ///     prefs,
        ///     langid!("zh"),
        ///     options,
        /// )
        /// .expect("Data should load successfully");
        /// assert_try_writeable_eq!(
        ///     display_name_medium.as_borrowed(),
        ///     "Chinese",
        ///     Ok(())
        /// );
        ///
        /// // Long length format uses longer subtag names when available:
        /// let display_name_long = LanguageIdentifierDisplayNameOwned::try_new_extended_long(
        ///     prefs,
        ///     langid!("zh"),
        ///     options,
        /// )
        /// .expect("Data should load successfully");
        /// assert_try_writeable_eq!(
        ///     display_name_long.as_borrowed(),
        ///     "Mandarin Chinese",
        ///     Ok(())
        /// );
        /// ```
        functions: [
            try_new_long,
            try_new_long_with_buffer_provider,
            try_new_long_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_long)]
    pub fn try_new_long_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageCoreLongV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [
                    LocaleNamesLanguageCoreLongV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageCoreLongV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

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
            + DataProvider<LocaleNamesLanguageMenuCoreMediumV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let mut language_payload = try_load_menu_name!(
            provider,
            prefs,
            subject.language,
            [LocaleNamesLanguageMenuCoreMediumV1]
        );
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        let qualifiers = QualifiersOwned::try_new_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the short menu-style language display name for a given language identifier and locale using compiled data.
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
        /// let display_name = LanguageIdentifierDisplayNameOwned::try_new_short_menu(
        ///     prefs,
        ///     langid!("en-US"),
        ///     options,
        /// )
        /// .expect("Data should load successfully");
        ///
        /// assert_try_writeable_eq!(display_name.as_borrowed(), "English (US)", Ok(()));
        /// ```
        functions: [
            try_new_short_menu,
            try_new_short_menu_with_buffer_provider,
            try_new_short_menu_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short_menu)]
    pub fn try_new_short_menu_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
        _options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageMenuCoreMediumV1>
            + DataProvider<LocaleNamesLanguageCoreShortV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let mut language_payload = try_load_menu_name!(
            provider,
            prefs,
            subject.language,
            [LocaleNamesLanguageMenuCoreMediumV1]
        );
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageCoreShortV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        let qualifiers = QualifiersOwned::try_new_short_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the extended language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new_extended,
            try_new_extended_with_buffer_provider,
            try_new_extended_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_extended)]
    pub fn try_new_extended_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageExtendedMediumV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        // Step 1: Load language name
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        // Only try dialect if requested (or default)
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [
                    LocaleNamesLanguageExtendedMediumV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageExtendedMediumV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        // Load the remaining data
        let qualifiers = QualifiersOwned::try_new_extended_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the extended short language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new_extended_short,
            try_new_extended_short_with_buffer_provider,
            try_new_extended_short_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_extended_short)]
    pub fn try_new_extended_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageExtendedShortV1>
            + DataProvider<LocaleNamesLanguageCoreShortV1>
            + DataProvider<LocaleNamesLanguageExtendedMediumV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptExtendedShortV1>
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionExtendedShortV1>
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        // Step 1: Load language name
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        // Only try dialect if requested (or default)
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [
                    LocaleNamesLanguageExtendedShortV1,
                    LocaleNamesLanguageCoreShortV1,
                    LocaleNamesLanguageExtendedMediumV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageExtendedShortV1,
                    LocaleNamesLanguageCoreShortV1,
                    LocaleNamesLanguageExtendedMediumV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        // Load the remaining data
        let qualifiers =
            QualifiersOwned::try_new_extended_short_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the extended long language display name for a given language identifier and locale using compiled data.
        functions: [
            try_new_extended_long,
            try_new_extended_long_with_buffer_provider,
            try_new_extended_long_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_extended_long)]
    pub fn try_new_extended_long_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        mut subject: LanguageIdentifier,
        options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageExtendedLongV1>
            + DataProvider<LocaleNamesLanguageCoreLongV1>
            + DataProvider<LocaleNamesLanguageExtendedMediumV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let load_dialect = options.should_load_dialect();
        let mut language_payload = None;
        if load_dialect {
            language_payload = try_load_dialect_name!(
                provider,
                prefs,
                subject,
                [
                    LocaleNamesLanguageExtendedLongV1,
                    LocaleNamesLanguageCoreLongV1,
                    LocaleNamesLanguageExtendedMediumV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageExtendedLongV1,
                    LocaleNamesLanguageCoreLongV1,
                    LocaleNamesLanguageExtendedMediumV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        let qualifiers = QualifiersOwned::try_new_extended_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }

    icu_provider::gen_buffer_data_constructors!(
        (prefs: DisplayNamesPreferences, subject: LanguageIdentifier, options: LanguageIdentifierDisplayNameOptions) -> result: Result<Self, DataError>,
        /// Loads the menu-style language display name for a given language identifier and locale using compiled data with extended coverage.
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
        /// let display_name = LanguageIdentifierDisplayNameOwned::try_new_extended_menu(
        ///     prefs,
        ///     langid!("en-US"),
        ///     options,
        /// )
        /// .expect("Data should load successfully");
        ///
        /// assert_try_writeable_eq!(display_name.as_borrowed(), "English (United States)", Ok(()));
        /// ```
        functions: [
            try_new_extended_menu,
            try_new_extended_menu_with_buffer_provider,
            try_new_extended_menu_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_extended_menu)]
    pub fn try_new_extended_menu_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
        _options: LanguageIdentifierDisplayNameOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesLanguageMenuExtendedMediumV1>
            + DataProvider<LocaleNamesLanguageMenuCoreMediumV1>
            + DataProvider<LocaleNamesLanguageExtendedMediumV1>
            + DataProvider<LocaleNamesLanguageCoreMediumV1>
            + DataProvider<LocaleNamesLanguageMinimalMediumV1>
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        let mut language_payload = try_load_menu_name!(
            provider,
            prefs,
            subject.language,
            [
                LocaleNamesLanguageMenuExtendedMediumV1,
                LocaleNamesLanguageMenuCoreMediumV1
            ]
        );
        if language_payload.is_none() {
            language_payload = try_load_subtag_name!(
                provider,
                prefs,
                subject.language,
                [
                    LocaleNamesLanguageExtendedMediumV1,
                    LocaleNamesLanguageCoreMediumV1,
                    LocaleNamesLanguageMinimalMediumV1
                ]
            );
        }
        let language_payload = match language_payload {
            Some(payload) => DataPayloadOr::from_payload(payload),
            None => DataPayloadOr::from_other(subject.language),
        };

        let qualifiers = QualifiersOwned::try_new_extended_unstable(provider, prefs, subject)?;
        Ok(Self {
            language_payload,
            qualifiers,
        })
    }
}

impl QualifiersOwned {
    fn try_new_internal_unstable<D, FS, FR, FV>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
        load_script: FS,
        load_region: FR,
        load_variant: FV,
    ) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<LocaleNamesEssentialsV1>,
        FS: Fn(&D, DisplayNamesPreferences, Script) -> Result<ScriptDisplayNameOwned, DataError>,
        FR: Fn(&D, DisplayNamesPreferences, Region) -> Result<RegionDisplayNameOwned, DataError>,
        FV: Fn(&D, DisplayNamesPreferences, Variant) -> Result<VariantDisplayNameOwned, DataError>,
    {
        // Step 2: Load script name (if present in subject)
        let script_payload = if let Some(script) = subject.script {
            match load_script(provider, prefs, script).allow_identifier_not_found()? {
                Some(obj) => DataPayloadOr::from_payload(obj.payload.cast()),
                None => DataPayloadOr::from_other(Some(script)),
            }
        } else {
            DataPayloadOr::from_other(None)
        };

        // Step 3: Load region name (if present in subject)
        let region_payload = if let Some(region) = subject.region {
            match load_region(provider, prefs, region).allow_identifier_not_found()? {
                Some(obj) => DataPayloadOr::from_payload(obj.payload.cast()),
                None => DataPayloadOr::from_other(Some(region)),
            }
        } else {
            DataPayloadOr::from_other(None)
        };

        let load_variant_helper = |variant: Variant| -> Result<
            DataPayloadOr<ErasedMarker<VarZeroCow<'static, str>>, Variant>,
            DataError,
        > {
            match load_variant(provider, prefs, variant).allow_identifier_not_found()? {
                Some(obj) => Ok(DataPayloadOr::from_payload(obj.payload.cast())),
                None => Ok(DataPayloadOr::from_other(variant)),
            }
        };

        // Step 4: Load variant names (if present in subject)
        let mut variant_results = subject
            .variants
            .iter()
            .map(|variant| load_variant_helper(*variant));

        let variant_payloads = if let Some(first) = variant_results.next() {
            // 2 or more variants
            if let Some(second) = variant_results.next() {
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

    fn try_new_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        Self::try_new_internal_unstable(
            provider,
            prefs,
            subject,
            ScriptDisplayNameOwned::try_new_unstable,
            RegionDisplayNameOwned::try_new_unstable,
            VariantDisplayNameOwned::try_new_unstable,
        )
    }

    fn try_new_minimal_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        Self::try_new_internal_unstable(
            provider,
            prefs,
            subject,
            ScriptDisplayNameOwned::try_new_minimal_unstable,
            RegionDisplayNameOwned::try_new_minimal_unstable,
            VariantDisplayNameOwned::try_new_minimal_unstable,
        )
    }

    fn try_new_minimal_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        Self::try_new_internal_unstable(
            provider,
            prefs,
            subject,
            ScriptDisplayNameOwned::try_new_minimal_unstable,
            RegionDisplayNameOwned::try_new_minimal_short_unstable,
            VariantDisplayNameOwned::try_new_minimal_unstable,
        )
    }

    fn try_new_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        Self::try_new_internal_unstable(
            provider,
            prefs,
            subject,
            ScriptDisplayNameOwned::try_new_short_unstable,
            RegionDisplayNameOwned::try_new_short_unstable,
            VariantDisplayNameOwned::try_new_unstable,
        )
    }

    fn try_new_extended_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        Self::try_new_internal_unstable(
            provider,
            prefs,
            subject,
            ScriptDisplayNameOwned::try_new_extended_unstable,
            RegionDisplayNameOwned::try_new_extended_unstable,
            VariantDisplayNameOwned::try_new_extended_unstable,
        )
    }

    fn try_new_extended_short_unstable<D>(
        provider: &D,
        prefs: DisplayNamesPreferences,
        subject: LanguageIdentifier,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<LocaleNamesScriptExtendedShortV1>
            + DataProvider<LocaleNamesScriptExtendedMediumV1>
            + DataProvider<LocaleNamesScriptCoreMediumV1>
            + DataProvider<LocaleNamesScriptMinimalMediumV1>
            + DataProvider<LocaleNamesRegionExtendedShortV1>
            + DataProvider<LocaleNamesRegionCoreShortV1>
            + DataProvider<LocaleNamesRegionMinimalShortV1>
            + DataProvider<LocaleNamesRegionCoreMediumV1>
            + DataProvider<LocaleNamesRegionMinimalMediumV1>
            + DataProvider<LocaleNamesVariantExtendedMediumV1>
            + DataProvider<LocaleNamesEssentialsV1>,
    {
        Self::try_new_internal_unstable(
            provider,
            prefs,
            subject,
            ScriptDisplayNameOwned::try_new_extended_short_unstable,
            RegionDisplayNameOwned::try_new_extended_short_unstable,
            VariantDisplayNameOwned::try_new_extended_unstable,
        )
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
    Slice(&'a [DataPayloadOr<ErasedMarker<VarZeroCow<'static, str>>, Variant>]),
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
