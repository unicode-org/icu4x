// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Displaynames component.

use crate::alloc::string::ToString;
use crate::options::*;
use crate::provider::*;
use alloc::borrow::Cow;
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
/// use icu_locid::{locale, subtags_region as region};
///
/// let locale = locale!("en-001");
/// let options: DisplayNamesOptions = Default::default();
/// let display_name = RegionDisplayNames::try_new_unstable(
///     &icu_testdata::unstable(),
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
    /// Creates a new [`RegionDisplayNames`] from locale data and an options bag.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
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

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_unstable,
            try_new_with_any_provider,
            try_new_with_buffer_provider
        ]
    );

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
/// let display_name = ScriptDisplayNames::try_new_unstable(
///     &icu_testdata::unstable(),
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
    /// Creates a new [`ScriptDisplayNames`] from locale data and an options bag.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
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

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_unstable,
            try_new_with_any_provider,
            try_new_with_buffer_provider
        ]
    );

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
/// let display_name = VariantDisplayNames::try_new_unstable(
///     &icu_testdata::unstable(),
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
    /// Creates a new [`VariantDisplayNames`] from locale data and an options bag.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
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

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_unstable,
            try_new_with_any_provider,
            try_new_with_buffer_provider
        ]
    );

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
/// let display_name = LanguageDisplayNames::try_new_unstable(
///     &icu_testdata::unstable(),
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
    /// Creates a new [`LanguageDisplayNames`] from locale data and an options bag.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
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

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_unstable,
            try_new_with_any_provider,
            try_new_with_buffer_provider
        ]
    );

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
/// let display_name = LocaleDisplayNamesFormatter::try_new_unstable(
///     &icu_testdata::unstable(),
///     &locale.into(),
///     options,
/// )
/// .expect("Data should load successfully");
///
/// assert_eq!(display_name.of(&locale!("en-GB")), "British English");
/// assert_eq!(display_name.of(&locale!("en")), "English");
/// assert_eq!(display_name.of(&locale!("en-MX")), "English (Mexico)");
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

// LongestMatching subtag is a longest substring of a given locale that exists as a key in the CLDR locale data.
// This is used for implementing Locale Display Name Algorithm.
#[derive(PartialEq)]
enum LongestMatchingSubtag {
    // Longest matching subtag of type ${lang}-${region}.
    // Example: "de-ET", "en-GB"
    LangRegion,
    // Longest matching subtag of type ${lang}-${script}.
    // Example: "hi-Latn", "zh-Hans"
    LangScript,
    // Longest matching subtag of type ${lang}
    // Example: "en", "hi"
    Lang,
}

impl LocaleDisplayNamesFormatter {
    /// Creates a new [`LocaleDisplayNamesFormatter`] from locale data and an options bag.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
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

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DisplayNamesOptions,
        error: DataError,
        functions: [
            Self::try_new_unstable,
            try_new_with_any_provider,
            try_new_with_buffer_provider
        ]
    );

    /// Returns the display name of a locale.
    /// This implementation is based on the algorithm described in
    /// https://www.unicode.org/reports/tr35/tr35-general.html#locale_display_name_algorithm
    ///
    // TODO: Make this return a writeable instead of using alloc
    pub fn of<'a, 'b: 'a, 'c: 'a>(&'b self, locale: &'c Locale) -> Cow<'a, str> {
        let longest_matching_subtag = find_longest_matching_subtag(&locale, &self);

        // Step - 1: Construct a locale display name string (LDN).
        // Find the displayname for the longest_matching_subtag which was derived above.
        let ldn = get_locale_display_name(&locale, &longest_matching_subtag, &self);

        // Step - 2: Construct a vector of longest qualifying substrings (LQS).
        // Find the displayname for the remaining locale if exists.
        let lqs = get_longest_qualifying_substrings(&locale, &longest_matching_subtag, &self);

        // Step - 3: Return the displayname based on the size of LQS.
        if lqs.len() == 0 {
            return ldn.to_string().into();
        } else {
            return Cow::Owned(alloc::format!("{} ({})", ldn, lqs.join(", ")));
        }
    }
}

/// For a given locale and the data, find the longest prefix of the string that exists as a key in the CLDR locale data.
fn find_longest_matching_subtag<'a>(
    locale: &Locale,
    locale_dn_formatter: &'a LocaleDisplayNamesFormatter,
) -> LongestMatchingSubtag {
    let LocaleDisplayNamesFormatter { locale_data, .. } = locale_dn_formatter;

    // NOTE: The subtag ordering of the canonical locale is `languageᵣ_scriptᵣ_regionᵣ + variants + extensions`.
    // The logic to find the longest matching subtag is based on this ordering.
    if let Some(script) = locale.id.script {
        let lang_script_identifier: LanguageIdentifier = (locale.id.language, script).into();
        if locale_data
            .get()
            .names
            .get_by(|uvstr| lang_script_identifier.strict_cmp(uvstr).reverse())
            .is_some()
        {
            return LongestMatchingSubtag::LangScript;
        }
    }
    if let Some(region) = locale.id.region {
        if locale.id.script.is_none() {
            let lang_region_identifier: LanguageIdentifier = (locale.id.language, region).into();
            if locale_data
                .get()
                .names
                .get_by(|uvstr| lang_region_identifier.strict_cmp(uvstr).reverse())
                .is_some()
            {
                return LongestMatchingSubtag::LangRegion;
            }
        }
    }
    return LongestMatchingSubtag::Lang;
}

fn get_locale_display_name<'a>(
    locale: &Locale,
    longest_matching_subtag: &LongestMatchingSubtag,
    locale_dn_formatter: &'a LocaleDisplayNamesFormatter,
) -> &'a str {
    let LocaleDisplayNamesFormatter {
        options,
        locale_data,
        language_data,
        ..
    } = locale_dn_formatter;

    let lang_id: LanguageIdentifier = match *longest_matching_subtag {
        LongestMatchingSubtag::LangRegion => (locale.id.language, locale.id.region.unwrap()).into(),
        LongestMatchingSubtag::LangScript => (locale.id.language, locale.id.script.unwrap()).into(),
        LongestMatchingSubtag::Lang => locale.id.language.into(),
    };

    // Check if the key exists in the locale_data first.
    // Example: "en_GB", "nl_BE".
    let mut ldn = match options.style {
        Some(Style::Short) => locale_data
            .get()
            .short_names
            .get_by(|uvstr| lang_id.strict_cmp(uvstr).reverse()),
        Some(Style::Long) => locale_data
            .get()
            .long_names
            .get_by(|uvstr| lang_id.strict_cmp(uvstr).reverse()),
        Some(Style::Menu) => locale_data
            .get()
            .menu_names
            .get_by(|uvstr| lang_id.strict_cmp(uvstr).reverse()),
        _ => None,
    }
    .or_else(|| {
        locale_data
            .get()
            .names
            .get_by(|uvstr| lang_id.strict_cmp(uvstr).reverse())
    });

    // At this point the key should exist in the language_data.
    // Example: "en", "nl", "zh".
    if ldn.is_none() {
        ldn = match options.style {
            Some(Style::Short) => language_data
                .get()
                .short_names
                .get(&lang_id.language.into_tinystr().to_unvalidated()),
            Some(Style::Long) => language_data
                .get()
                .long_names
                .get(&lang_id.language.into_tinystr().to_unvalidated()),
            Some(Style::Menu) => language_data
                .get()
                .menu_names
                .get(&lang_id.language.into_tinystr().to_unvalidated()),
            _ => None,
        }
        .or_else(|| {
            language_data
                .get()
                .names
                .get(&lang_id.language.into_tinystr().to_unvalidated())
        });
    }

    // Throw an error if the LDN is none as it is not possible to have a locale string without the language.
    return ldn.expect("cannot parse locale displayname.");
}

fn get_longest_qualifying_substrings<'a>(
    locale: &Locale,
    longest_matching_subtag: &LongestMatchingSubtag,
    locale_dn_formatter: &'a LocaleDisplayNamesFormatter,
) -> Vec<&'a str> {
    let LocaleDisplayNamesFormatter {
        options,
        region_data,
        script_data,
        variant_data,
        ..
    } = locale_dn_formatter;

    let mut lqs: Vec<&str> = vec![];

    if let Some(script) = locale.id.script {
        // Ignore if the script was used to derive LDN.
        if *longest_matching_subtag != LongestMatchingSubtag::LangScript {
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
            if let Some(scriptdn) = scriptdisplay {
                lqs.push(scriptdn);
            }
        }
    }

    if let Some(region) = locale.id.region {
        // Ignore if the region was used to derive LDN.
        if *longest_matching_subtag != LongestMatchingSubtag::LangRegion {
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

            if let Some(regiondn) = regiondisplay {
                lqs.push(regiondn);
            }
        }
    }

    for &variant_key in locale.id.variants.iter() {
        if let Some(variant_dn) = variant_data
            .get()
            .names
            .get(&variant_key.into_tinystr().to_unvalidated())
        {
            lqs.push(variant_dn);
        }
    }

    return lqs;
}
