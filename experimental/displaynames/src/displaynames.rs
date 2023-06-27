// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Displaynames component.

use crate::options::*;
use crate::provider::*;
use alloc::borrow::Cow;
use icu_locid::{subtags::Language, subtags::Region, subtags::Script, subtags::Variant, Locale};
use icu_provider::prelude::*;

/// Lookup of the locale-specific display names by region code.
///
/// # Example
///
/// ```
/// use icu_displaynames::{DisplayNamesOptions, RegionDisplayNames};
/// use icu_locid::{locale, subtags_region as region};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = RegionDisplayNames::try_new(
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(region!("AE")), Some("United Arab Emirates"));
/// ```
#[derive(Default)]
pub struct RegionDisplayNames {
    options: DisplayNamesOptions,
    region_data: DataPayload<RegionDisplayNamesV1Marker>,
}

impl RegionDisplayNames {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`RegionDisplayNames`] from locale data and an options bag.
        functions: [
            try_new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: DataProvider<RegionDisplayNamesV1Marker> + ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let region_data = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            region_data,
        })
    }

    /// Returns the display name of a region.
    pub fn of(&self, region: Region) -> Option<&str> {
        let data = self.region_data.get();
        match self.options.style {
            Some(Style::Short) => data
                .short_names
                .get(&region.into_tinystr().to_unvalidated()),
            _ => None,
        }
        .or_else(|| data.names.get(&region.into_tinystr().to_unvalidated()))
        // TODO: Respect options.fallback
    }
}

/// Lookup of the locale-specific display names by script code.
///
/// # Example
///
/// ```
/// use icu_displaynames::{DisplayNamesOptions, ScriptDisplayNames};
/// use icu_locid::{locale, subtags_script as script};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = ScriptDisplayNames::try_new(
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(script!("Maya")), Some("Mayan hieroglyphs"));
/// ```
#[derive(Default)]
pub struct ScriptDisplayNames {
    options: DisplayNamesOptions,
    script_data: DataPayload<ScriptDisplayNamesV1Marker>,
}

#[allow(dead_code)] // not public at the moment
impl ScriptDisplayNames {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`ScriptDisplayNames`] from locale data and an options bag.
        functions: [
            try_new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: DataProvider<ScriptDisplayNamesV1Marker> + ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let script_data = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            script_data,
        })
    }

    /// Returns the display name of a script.
    pub fn of(&self, script: Script) -> Option<&str> {
        let data = self.script_data.get();
        match self.options.style {
            Some(Style::Short) => data
                .short_names
                .get(&script.into_tinystr().to_unvalidated()),
            _ => None,
        }
        .or_else(|| data.names.get(&script.into_tinystr().to_unvalidated()))
        // TODO: Respect options.fallback
    }
}

/// Lookup of the locale-specific display names by variant.
///
/// # Example
///
/// ```
/// use icu_displaynames::{DisplayNamesOptions, VariantDisplayNames};
/// use icu_locid::{locale, subtags_variant as variant};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = VariantDisplayNames::try_new(
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(variant!("POSIX")), Some("Computer"));
/// ```
#[derive(Default)]
pub struct VariantDisplayNames {
    #[allow(dead_code)] //TODO: Add DisplayNamesOptions support for Variants.
    options: DisplayNamesOptions,
    variant_data: DataPayload<VariantDisplayNamesV1Marker>,
}

#[allow(dead_code)] // not public at the moment
impl VariantDisplayNames {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`VariantDisplayNames`] from locale data and an options bag.
        functions: [
            try_new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: DataProvider<VariantDisplayNamesV1Marker> + ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let variant_data = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            variant_data,
        })
    }

    /// Returns the display name of a variant.
    pub fn of(&self, variant: Variant) -> Option<&str> {
        let data = self.variant_data.get();
        data.names.get(&variant.into_tinystr().to_unvalidated())
        // TODO: Respect options.fallback
    }
}

/// Lookup of the locale-specific display names by language code.
///
/// # Example
///
/// ```
/// use icu_displaynames::{DisplayNamesOptions, LanguageDisplayNames};
/// use icu_locid::{locale, subtags_language as language};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = LanguageDisplayNames::try_new(
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(language!("de")), Some("German"));
/// ```
#[derive(Default)]
pub struct LanguageDisplayNames {
    options: DisplayNamesOptions,
    language_data: DataPayload<LanguageDisplayNamesV1Marker>,
}

impl LanguageDisplayNames {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`LanguageDisplayNames`] from locale data and an options bag.
        functions: [
            try_new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: DataProvider<LanguageDisplayNamesV1Marker> + ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let language_data = data_provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self {
            options,
            language_data,
        })
    }

    /// Returns the display name of a language.
    pub fn of(&self, language: Language) -> Option<&str> {
        let data = self.language_data.get();
        match self.options.style {
            Some(Style::Short) => data
                .short_names
                .get(&language.into_tinystr().to_unvalidated()),
            Some(Style::Long) => data
                .long_names
                .get(&language.into_tinystr().to_unvalidated()),
            Some(Style::Menu) => data
                .menu_names
                .get(&language.into_tinystr().to_unvalidated()),
            _ => None,
        }
        .or_else(|| data.names.get(&language.into_tinystr().to_unvalidated()))
        // TODO: Respect options.fallback
    }
}

/// Format a locale as a display string.
///
/// # Example
///
/// ```
/// use icu_displaynames::{DisplayNamesOptions, LocaleDisplayNamesFormatter};
/// use icu_locid::{locale, subtags_language as language};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = LocaleDisplayNamesFormatter::try_new(
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(&locale!("de-CH")), "Swiss High German");
/// assert_eq!(display_name.of(&locale!("de")), "German");
/// assert_eq!(display_name.of(&locale!("de-MX")), "German (Mexico)");
/// assert_eq!(display_name.of(&locale!("xx-YY")), "xx (YY)");
/// assert_eq!(display_name.of(&locale!("xx")), "xx");
/// ```
pub struct LocaleDisplayNamesFormatter {
    options: DisplayNamesOptions,
    // patterns: DataPayload<LocaleDisplayNamesPatternsV1Marker>,
    locale_data: DataPayload<LocaleDisplayNamesV1Marker>,

    language_data: DataPayload<LanguageDisplayNamesV1Marker>,
    #[allow(dead_code)] // TODO use this
    script_data: DataPayload<ScriptDisplayNamesV1Marker>,
    region_data: DataPayload<RegionDisplayNamesV1Marker>,
    #[allow(dead_code)] // TODO add support for variants
    variant_data: DataPayload<VariantDisplayNamesV1Marker>,
    // key_data: DataPayload<KeyDisplayNamesV1Marker>,
    // measuerment_data: DataPayload<MeasurementSystemsDisplayNamesV1Marker>,
    // subdivisions_data: DataPayload<SubdivisionsDisplayNamesV1Marker>,
    // transforms_data: DataPayload<TransformsDisplayNamesV1Marker>,
}

impl LocaleDisplayNamesFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`LocaleDisplayNamesFormatter`] from locale data and an options bag.
        functions: [
            try_new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D: ?Sized>(
        data_provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<LocaleDisplayNamesV1Marker>
            + DataProvider<LanguageDisplayNamesV1Marker>
            + DataProvider<ScriptDisplayNamesV1Marker>
            + DataProvider<RegionDisplayNamesV1Marker>
            + DataProvider<VariantDisplayNamesV1Marker>,
    {
        let req = DataRequest {
            locale,
            metadata: Default::default(),
        };

        Ok(Self {
            options,
            language_data: data_provider.load(req)?.take_payload()?,
            locale_data: data_provider.load(req)?.take_payload()?,
            script_data: data_provider.load(req)?.take_payload()?,
            region_data: data_provider.load(req)?.take_payload()?,
            variant_data: data_provider.load(req)?.take_payload()?,
        })
    }

    /// Returns the display name of a locale.
    // TODO: Make this return a writeable instead of using alloc
    pub fn of<'a, 'b: 'a, 'c: 'a>(&'b self, locale: &'c Locale) -> Cow<'a, str> {
        // https://www.unicode.org/reports/tr35/tr35-general.html#Display_Name_Elements

        // TODO: This binary search needs to return the longest matching found prefix
        // instead of just perfect matches
        if let Some(displayname) = match self.options.style {
            Some(Style::Short) => self
                .locale_data
                .get()
                .short_names
                .get_by(|bytes| locale.strict_cmp(bytes).reverse()),
            Some(Style::Long) => self
                .locale_data
                .get()
                .long_names
                .get_by(|bytes| locale.strict_cmp(bytes).reverse()),
            Some(Style::Menu) => self
                .locale_data
                .get()
                .menu_names
                .get_by(|bytes| locale.strict_cmp(bytes).reverse()),
            _ => None,
        }
        .or_else(|| {
            self.locale_data
                .get()
                .names
                .get_by(|bytes| locale.strict_cmp(bytes).reverse())
        }) {
            return Cow::Borrowed(displayname);
        }

        // TODO: This is a dummy implementation which does not adhere to UTS35. It only uses
        // the language and region code, and uses a hardcoded pattern to combine them.

        let langdisplay = match self.options.style {
            Some(Style::Short) => self
                .language_data
                .get()
                .short_names
                .get(&locale.id.language.into_tinystr().to_unvalidated()),
            Some(Style::Long) => self
                .language_data
                .get()
                .long_names
                .get(&locale.id.language.into_tinystr().to_unvalidated()),
            Some(Style::Menu) => self
                .language_data
                .get()
                .menu_names
                .get(&locale.id.language.into_tinystr().to_unvalidated()),
            _ => None,
        }
        .or_else(|| {
            self.language_data
                .get()
                .names
                .get(&locale.id.language.into_tinystr().to_unvalidated())
        });

        if let Some(region) = locale.id.region {
            let regiondisplay = match self.options.style {
                Some(Style::Short) => self
                    .region_data
                    .get()
                    .short_names
                    .get(&region.into_tinystr().to_unvalidated()),
                _ => None,
            }
            .or_else(|| {
                self.region_data
                    .get()
                    .names
                    .get(&region.into_tinystr().to_unvalidated())
            });
            // TODO: Use data patterns
            Cow::Owned(alloc::format!(
                "{} ({})",
                langdisplay.unwrap_or(locale.id.language.as_str()),
                regiondisplay.unwrap_or(region.as_str())
            ))
        } else {
            Cow::Borrowed(langdisplay.unwrap_or(locale.id.language.as_str()))
        }
    }
}
