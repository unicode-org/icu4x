// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Displaynames component.

use crate::options::*;
use crate::provider::*;
use alloc::borrow::Cow;
use alloc::string::String;
use icu_locid::{
    subtags::Language, subtags::Region, subtags::Script, subtags::Variant, LanguageIdentifier,
    Locale,
};
use icu_provider::prelude::*;
use zerovec::ule::UnvalidatedStr;

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
        // Step - 1: Construct a locale display name string (LDN) for the longest matching subtag.
        let mut ldn = None;

        // These qualifying strings (QS) will later be set to None if they are part of the LDN.
        // Using references as we might borrow from these later
        let mut script_qs = locale.id.script.as_ref();
        let mut region_qs = locale.id.region.as_ref();
        let variants_qs = Cow::Borrowed(&*locale.id.variants);

        // TODO: handle the other possible longest subtag cases

        if let Some(script) = locale.id.script {
            let data = self.locale_data.get();
            let id = LanguageIdentifier::from((locale.id.language, Some(script), None));
            let cmp = |uvstr: &UnvalidatedStr| id.strict_cmp(uvstr).reverse();
            if let Some(x) = match self.options.style {
                Some(Style::Short) => data.short_names.get_by(cmp),
                Some(Style::Long) => data.long_names.get_by(cmp),
                Some(Style::Menu) => data.menu_names.get_by(cmp),
                _ => None,
            }
            .or_else(|| data.names.get_by(cmp))
            {
                ldn = Some(x);
                script_qs = None;
            }
        }

        if ldn.is_none() {
            if let Some(region) = locale.id.region {
                let data = self.locale_data.get();
                let id = LanguageIdentifier::from((locale.id.language, None, Some(region)));
                let cmp = |uvstr: &UnvalidatedStr| id.strict_cmp(uvstr).reverse();
                if let Some(x) = match self.options.style {
                    Some(Style::Short) => data.short_names.get_by(cmp),
                    Some(Style::Long) => data.long_names.get_by(cmp),
                    Some(Style::Menu) => data.menu_names.get_by(cmp),
                    _ => None,
                }
                .or_else(|| data.names.get_by(cmp))
                {
                    ldn = Some(x);
                    region_qs = None;
                }
            }
        }

        let ldn = ldn
            .or_else(|| {
                let data = self.language_data.get();
                let key = locale.id.language.into_tinystr().to_unvalidated();
                match self.options.style {
                    Some(Style::Short) => data.short_names.get(&key),
                    Some(Style::Long) => data.long_names.get(&key),
                    Some(Style::Menu) => data.menu_names.get(&key),
                    _ => None,
                }
                .or_else(|| data.names.get(&key))
            })
            .unwrap_or(locale.id.language.as_str());

        if script_qs.is_none() && region_qs.is_none() && variants_qs.is_empty() {
            // The LDN fully represents the locale
            return Cow::Borrowed(ldn);
        }

        // Step - 2: Construct the list of qualifying substrings (LQS).

        let script_qs = script_qs.map(|script| {
            let data = self.script_data.get();
            let key = script.into_tinystr().to_unvalidated();
            match self.options.style {
                Some(Style::Short) => data.short_names.get(&key),
                _ => None,
            }
            .or_else(|| data.names.get(&key))
            .unwrap_or(script.as_str())
        });

        let region_qs = region_qs.map(|region| {
            let data = self.region_data.get();
            let key = region.into_tinystr().to_unvalidated();
            match self.options.style {
                Some(Style::Short) => data.short_names.get(&key),
                _ => None,
            }
            .or_else(|| data.names.get(&key))
            .unwrap_or(region.as_str())
        });

        let variants_qs = variants_qs.iter().map(|variant_key| {
            self.variant_data
                .get()
                .names
                .get(&variant_key.into_tinystr().to_unvalidated())
                .unwrap_or(variant_key.as_str())
        });

        let lqs = script_qs.into_iter().chain(region_qs).chain(variants_qs);

        // Step - 3: Write LDN and LQS to output
        // TODO: Move to an `impl Writeable`

        // TODO: load from data
        let (before, middle, after) = (" (", ", ", ")");

        let mut output = String::with_capacity(
            ldn.len() + before.len() + lqs.clone().map(|x| x.len() + middle.len()).sum::<usize>()
                - middle.len()
                + after.len(),
        );
        output.push_str(ldn);
        output.push_str(before);

        let mut first = true;
        for lqs in lqs {
            if !first {
                output.push_str(middle);
            } else {
                first = false;
            }
            output.push_str(lqs);
        }
        output.push_str(after);
        Cow::Owned(output)
    }
}
