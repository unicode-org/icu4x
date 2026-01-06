// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::preferences::extensions::unicode::keywords::{RegionOverride, RegionalSubdivision};
#[cfg(feature = "alloc")]
use crate::subtags::Variants;
use crate::subtags::{Language, Region, Script, Variant};
use crate::DataLocale;

/// The structure storing locale subtags used in preferences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalePreferences {
    /// Preference of Language
    pub(crate) language: Language,
    /// Preference of Script
    pub(crate) script: Option<Script>,
    /// Preference of Region
    pub(crate) region: Option<Region>,
    /// Preference of Variant
    pub(crate) variant: Option<Variant>,
    /// Preference of Regional Subdivision
    pub(crate) subdivision: Option<RegionalSubdivision>,
    /// Preference of Unicode Extension Region
    pub(crate) region_override: Option<RegionOverride>,
}

impl LocalePreferences {
    /// Convert to a DataLocale, with region-based fallback priority
    ///
    /// Most users should use `icu_provider::marker::make_locale()` instead.
    pub fn to_data_locale_region_priority(self) -> DataLocale {
        if let Some(ro) = self.region_override {
            return DataLocale::from_parts(self.language, self.script, Some(*ro), self.variant);
        }

        self.to_data_locale_language_priority()
    }

    /// Convert to a DataLocale, with language-based fallback priority
    ///
    /// Most users should use `icu_provider::marker::make_locale()` instead.
    pub fn to_data_locale_language_priority(self) -> DataLocale {
        use crate::extensions::unicode::{SubdivisionId, SubdivisionSuffix};

        let region = if let Some(sd) = self.subdivision {
            if let Some(region) = self.region {
                // Discard the subdivison if it doesn't match the region
                Some(SubdivisionId {
                    region,
                    suffix: if sd.region == region {
                        sd.suffix
                    } else {
                        SubdivisionSuffix::UNKNOWN
                    },
                })
            } else {
                // Use the subdivision's region if there's no region
                Some(*sd)
            }
        } else {
            self.region.map(|region| SubdivisionId {
                region,
                suffix: SubdivisionSuffix::UNKNOWN,
            })
        };

        DataLocale::from_parts(self.language, self.script, region, self.variant)
    }
}
impl Default for LocalePreferences {
    fn default() -> Self {
        Self::default()
    }
}

impl From<&crate::Locale> for LocalePreferences {
    fn from(loc: &crate::Locale) -> Self {
        Self {
            language: loc.id.language,
            script: loc.id.script,
            region: loc.id.region,
            variant: loc.id.variants.iter().copied().next(),
            subdivision: loc
                .extensions
                .unicode
                .keywords
                .get(&RegionalSubdivision::UNICODE_EXTENSION_KEY)
                .and_then(|v| RegionalSubdivision::try_from(v).ok()),
            region_override: loc
                .extensions
                .unicode
                .keywords
                .get(&RegionOverride::UNICODE_EXTENSION_KEY)
                .and_then(|v| RegionOverride::try_from(v).ok()),
        }
    }
}

impl From<&crate::LanguageIdentifier> for LocalePreferences {
    fn from(lid: &crate::LanguageIdentifier) -> Self {
        Self {
            language: lid.language,
            script: lid.script,
            region: lid.region,
            variant: lid.variants.iter().copied().next(),
            subdivision: None,
            region_override: None,
        }
    }
}

/// ✨ *Enabled with the `alloc` Cargo feature.*
#[cfg(feature = "alloc")]
impl From<LocalePreferences> for crate::Locale {
    fn from(prefs: LocalePreferences) -> Self {
        Self {
            id: crate::LanguageIdentifier {
                language: prefs.language,
                script: prefs.script,
                region: prefs.region,
                variants: prefs
                    .variant
                    .map(Variants::from_variant)
                    .unwrap_or_default(),
            },
            extensions: {
                let mut extensions = crate::extensions::Extensions::default();
                if let Some(sd) = prefs.subdivision {
                    extensions
                        .unicode
                        .keywords
                        .set(RegionalSubdivision::UNICODE_EXTENSION_KEY, sd.into());
                }
                if let Some(rg) = prefs.region_override {
                    extensions
                        .unicode
                        .keywords
                        .set(RegionOverride::UNICODE_EXTENSION_KEY, rg.into());
                }
                extensions
            },
        }
    }
}

impl LocalePreferences {
    /// Constructs a new [`LocalePreferences`] struct with the defaults.
    pub const fn default() -> Self {
        Self {
            language: Language::UNKNOWN,
            script: None,
            region: None,
            variant: None,
            subdivision: None,
            region_override: None,
        }
    }

    /// Preference of Language
    pub const fn language(&self) -> Language {
        self.language
    }

    /// Preference of Region
    ///
    /// This respects the [`RegionOverride`], and should only be used
    /// for [data that is region-specific](https://github.com/unicode-org/cldr/blob/main/common/supplemental/rgScope.xml).
    pub const fn region(&self) -> Option<Region> {
        if let Some(rg) = self.region_override {
            Some(rg.0.region)
        } else {
            self.region
        }
    }

    /// Extends the preferences with the values from another set of preferences.
    pub fn extend(&mut self, other: LocalePreferences) {
        if !other.language.is_unknown() {
            self.language = other.language;
        }
        if let Some(script) = other.script {
            self.script = Some(script);
        }
        if let Some(region) = other.region {
            self.region = Some(region);
        }
        if let Some(variant) = other.variant {
            self.variant = Some(variant);
        }
        if let Some(subdivision) = other.subdivision {
            self.subdivision = Some(subdivision);
        }
        if let Some(region_override) = other.region_override {
            self.region_override = Some(region_override);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Locale;

    #[test]
    fn test_data_locale_conversion() {
        #[derive(Debug)]
        struct TestCase<'a> {
            input: &'a str,
            language_priority: &'a str,
            region_priority: &'a str,
        }
        let test_cases = [
            TestCase {
                input: "en",
                language_priority: "en",
                region_priority: "en",
            },
            TestCase {
                input: "en-US",
                language_priority: "en-US",
                region_priority: "en-US",
            },
            TestCase {
                input: "en-u-sd-ustx",
                language_priority: "en-US-u-sd-ustx",
                region_priority: "en-US-u-sd-ustx",
            },
            TestCase {
                input: "en-US-u-sd-ustx",
                language_priority: "en-US-u-sd-ustx",
                region_priority: "en-US-u-sd-ustx",
            },
            TestCase {
                input: "en-u-rg-gbzzzz",
                language_priority: "en",
                region_priority: "en-GB",
            },
            TestCase {
                input: "en-US-u-rg-gbzzzz",
                language_priority: "en-US",
                region_priority: "en-GB",
            },
            TestCase {
                input: "en-US-u-sd-gbzzzz",
                language_priority: "en-US",
                region_priority: "en-US",
            },
            TestCase {
                input: "en-u-rg-gbzzzz-sd-ustx",
                language_priority: "en-US-u-sd-ustx",
                region_priority: "en-GB",
            },
            TestCase {
                input: "en-US-u-rg-gbzzzz-sd-ustx",
                language_priority: "en-US-u-sd-ustx",
                region_priority: "en-GB",
            },
            TestCase {
                input: "en-US-u-rg-gbeng-sd-ustx",
                language_priority: "en-US-u-sd-ustx",
                region_priority: "en-GB-u-sd-gbeng",
            },
        ];
        for test_case in test_cases.iter() {
            let locale = Locale::try_from_str(test_case.input).unwrap();
            let prefs = LocalePreferences::from(&locale);
            assert_eq!(
                prefs.to_data_locale_language_priority().to_string(),
                test_case.language_priority,
                "{test_case:?}"
            );
            assert_eq!(
                prefs.to_data_locale_region_priority().to_string(),
                test_case.region_priority,
                "{test_case:?}"
            );
        }
    }
}
