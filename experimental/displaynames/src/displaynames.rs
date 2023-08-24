// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Displaynames component.

use crate::options::*;
use crate::provider::*;
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use icu_locid::{
    subtags::Language, subtags::Region, subtags::Script, subtags::Variant, LanguageIdentifier,
    Locale,
};
use icu_provider::prelude::*;
/// Lookup of the locale-specific display names by region code.
///
/// # Example
///
/// ```
/// use icu_displaynames::{DisplayNamesOptions, RegionDisplayNames};
/// use icu_locid::{locale, subtags::region};
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
        /// Creates a new [`RegionDisplayNames`] from locale data and an options bag using compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
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
        provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let region_data = provider
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
/// use icu_locid::{locale, subtags::script};
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

impl ScriptDisplayNames {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`ScriptDisplayNames`] from locale data and an options bag using compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
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
        provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let script_data = provider
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
/// use icu_locid::{locale, subtags::variant};
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

impl VariantDisplayNames {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`VariantDisplayNames`] from locale data and an options bag using compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
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
        provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let variant_data = provider
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
/// use icu_locid::{locale, subtags::language};
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
        /// Creates a new [`LanguageDisplayNames`] from locale data and an options bag using compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
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
        provider: &D,
        locale: &DataLocale,
        options: DisplayNamesOptions,
    ) -> Result<Self, DataError> {
        let language_data = provider
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
/// use icu_locid::{locale, subtags::language};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = LocaleDisplayNamesFormatter::try_new(
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(&locale!("en-GB")), "British English");
/// assert_eq!(display_name.of(&locale!("en")), "English");
/// assert_eq!(display_name.of(&locale!("en-MX")), "English (Mexico)");
/// assert_eq!(display_name.of(&locale!("xx-YY")), "xx (YY)");
/// assert_eq!(display_name.of(&locale!("xx")), "xx");
/// ```
pub struct LocaleDisplayNamesFormatter {
    options: DisplayNamesOptions,
    // patterns: DataPayload<LocaleDisplayNamesPatternsV1Marker>,
    locale_data: DataPayload<LocaleDisplayNamesV1Marker>,

    language_data: DataPayload<LanguageDisplayNamesV1Marker>,
    script_data: DataPayload<ScriptDisplayNamesV1Marker>,
    region_data: DataPayload<RegionDisplayNamesV1Marker>,
    variant_data: DataPayload<VariantDisplayNamesV1Marker>,
    // key_data: DataPayload<KeyDisplayNamesV1Marker>,
    // measurement_data: DataPayload<MeasurementSystemsDisplayNamesV1Marker>,
    // subdivisions_data: DataPayload<SubdivisionsDisplayNamesV1Marker>,
    // transforms_data: DataPayload<TransformsDisplayNamesV1Marker>,
}

impl LocaleDisplayNamesFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        /// Creates a new [`LocaleDisplayNamesFormatter`] from locale data and an options bag using compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
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
        provider: &D,
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
            language_data: provider.load(req)?.take_payload()?,
            locale_data: provider.load(req)?.take_payload()?,
            script_data: provider.load(req)?.take_payload()?,
            region_data: provider.load(req)?.take_payload()?,
            variant_data: provider.load(req)?.take_payload()?,
        })
    }

    /// Returns the display name of a locale.
    /// This implementation is based on the algorithm described in
    /// <https://www.unicode.org/reports/tr35/tr35-general.html#locale_display_name_algorithm>
    ///
    // TODO: Make this return a writeable instead of using alloc
    pub fn of<'a, 'b: 'a, 'c: 'a>(&'b self, locale: &'c Locale) -> Cow<'a, str> {
        let longest_matching_identifier = self.find_longest_matching_subtag(locale);

        // Step - 1: Construct a locale display name string (LDN).
        // Find the displayname for the longest_matching_subtag which was derived above.
        let ldn = self.get_locale_display_name(locale, &longest_matching_identifier);

        // Step - 2: Construct a vector of longest qualifying substrings (LQS).
        // Find the displayname for the remaining locale if exists.
        let lqs = self.get_longest_qualifying_substrings(locale, &longest_matching_identifier);

        // Step - 3: Return the displayname based on the size of LQS.
        let mut result = Cow::Borrowed(ldn);
        #[allow(clippy::indexing_slicing)] // indexes in range
        if !lqs.is_empty() {
            let mut output = String::with_capacity(
                result.len() + " (".len() + lqs.iter().map(|s| ", ".len() + s.len()).sum::<usize>()
                    - ", ".len()
                    + ")".len(),
            );
            output.push_str(&result);
            output.push_str(" (");
            output.push_str(lqs[0]);
            for lqs in &lqs[1..] {
                output.push_str(", ");
                output.push_str(lqs);
            }
            output.push(')');
            result = Cow::Owned(output);
        }
        result
    }

    /// For a given locale and the data, find the longest prefix of the string that exists as a key in the CLDR locale data.
    pub fn find_longest_matching_subtag(&self, locale: &Locale) -> LanguageIdentifier {
        // NOTE: The subtag ordering of the canonical locale is `language_script_region + variants + extensions`.
        // The logic to find the longest matching subtag is based on this ordering.
        if let Some(script) = locale.id.script {
            let lang_script_identifier: LanguageIdentifier =
                (locale.id.language, Some(script), None).into();
            if self
                .locale_data
                .get()
                .names
                .get_by(|uvstr| lang_script_identifier.strict_cmp(uvstr).reverse())
                .is_some()
            {
                return lang_script_identifier;
            }
        }
        if let Some(region) = locale.id.region {
            if locale.id.script.is_none() {
                let lang_region_identifier: LanguageIdentifier =
                    (locale.id.language, None, Some(region)).into();
                if self
                    .locale_data
                    .get()
                    .names
                    .get_by(|uvstr| lang_region_identifier.strict_cmp(uvstr).reverse())
                    .is_some()
                {
                    return lang_region_identifier;
                }
            }
        }
        (locale.id.language, None, None).into()
    }

    fn get_locale_display_name<'a>(
        &'a self,
        locale: &'a Locale,
        longest_matching_identifier: &LanguageIdentifier,
    ) -> &'a str {
        let LocaleDisplayNamesFormatter {
            options,
            locale_data,
            language_data,
            ..
        } = self;

        // Check if the key exists in the locale_data first.
        // Example: "en_GB", "nl_BE".
        let mut ldn = match options.style {
            Some(Style::Short) => locale_data
                .get()
                .short_names
                .get_by(|uvstr| longest_matching_identifier.strict_cmp(uvstr).reverse()),
            Some(Style::Long) => locale_data
                .get()
                .long_names
                .get_by(|uvstr| longest_matching_identifier.strict_cmp(uvstr).reverse()),
            Some(Style::Menu) => locale_data
                .get()
                .menu_names
                .get_by(|uvstr| longest_matching_identifier.strict_cmp(uvstr).reverse()),
            _ => None,
        }
        .or_else(|| {
            locale_data
                .get()
                .names
                .get_by(|uvstr| longest_matching_identifier.strict_cmp(uvstr).reverse())
        });

        // At this point the key should exist in the language_data.
        // Example: "en", "nl", "zh".
        if ldn.is_none() {
            ldn = match options.style {
                Some(Style::Short) => language_data.get().short_names.get(
                    &longest_matching_identifier
                        .language
                        .into_tinystr()
                        .to_unvalidated(),
                ),
                Some(Style::Long) => language_data.get().long_names.get(
                    &longest_matching_identifier
                        .language
                        .into_tinystr()
                        .to_unvalidated(),
                ),
                Some(Style::Menu) => language_data.get().menu_names.get(
                    &longest_matching_identifier
                        .language
                        .into_tinystr()
                        .to_unvalidated(),
                ),
                _ => None,
            }
            .or_else(|| {
                language_data.get().names.get(
                    &longest_matching_identifier
                        .language
                        .into_tinystr()
                        .to_unvalidated(),
                )
            });
        }
        // Fallback on language subtag in LanguageIdentifier id the key is not found in CLDR data.
        return ldn.unwrap_or(locale.id.language.as_str());
    }

    fn get_longest_qualifying_substrings<'a>(
        &'a self,
        locale: &'a Locale,
        longest_matching_identifier: &'a LanguageIdentifier,
    ) -> Vec<&'a str> {
        let LocaleDisplayNamesFormatter {
            options,
            region_data,
            script_data,
            variant_data,
            ..
        } = self;

        let mut lqs: Vec<&'a str> = vec![];

        if let Some(script) = &locale.id.script {
            // Ignore if the script was used to derive LDN.
            if longest_matching_identifier.script.is_none() {
                let scriptdisplay = match options.style {
                    Some(Style::Short) => script_data
                        .get()
                        .short_names
                        .get(&script.into_tinystr().to_unvalidated()),
                    _ => None,
                }
                .or_else(|| {
                    script_data
                        .get()
                        .names
                        .get(&script.into_tinystr().to_unvalidated())
                });
                lqs.push(scriptdisplay.unwrap_or(script.as_str()));
            }
        }

        if let Some(region) = &locale.id.region {
            // Ignore if the region was used to derive LDN.
            if longest_matching_identifier.region.is_none() {
                let regiondisplay = match options.style {
                    Some(Style::Short) => region_data
                        .get()
                        .short_names
                        .get(&region.into_tinystr().to_unvalidated()),
                    _ => None,
                }
                .or_else(|| {
                    region_data
                        .get()
                        .names
                        .get(&region.into_tinystr().to_unvalidated())
                });

                lqs.push(regiondisplay.unwrap_or(region.as_str()));
            }
        }

        for variant_key in locale.id.variants.iter() {
            lqs.push(
                variant_data
                    .get()
                    .names
                    .get(&variant_key.into_tinystr().to_unvalidated())
                    .unwrap_or(variant_key.as_str()),
            );
        }
        lqs
    }
}
