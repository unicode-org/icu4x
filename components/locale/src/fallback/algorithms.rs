// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::LocaleFallbackPriority;
use icu_locale_core::subtags::{Language, Region, Script};

use super::*;

impl<'a> LocaleFallbackerWithConfig<'a> {
    pub(crate) fn normalize(&self, locale: &mut DataLocale) {
        let language = locale.language;
        // 1. Populate the region (required for region fallback only)
        if self.config.priority == LocaleFallbackPriority::Region && locale.region.is_none() {
            // 1a. First look for region based on language+script
            if let Some(script) = locale.script {
                locale.region = self
                    .likely_subtags
                    .language_script
                    .get(&(
                        language.into_tinystr().to_unvalidated(),
                        script.into_tinystr().to_unvalidated(),
                    ))
                    .copied();
            }
            // 1b. If that fails, try language only
            if locale.region.is_none() {
                locale.region = self
                    .likely_subtags
                    .language
                    .get_copied(&language.into_tinystr().to_unvalidated())
                    .map(|(_s, r)| r);
            }
        }
        // 2. Remove the script if it is implied by the other subtags
        if let Some(script) = locale.script {
            let default_script = locale
                .region
                .and_then(|region| {
                    self.likely_subtags.language_region.get_copied(&(
                        language.into_tinystr().to_unvalidated(),
                        region.into_tinystr().to_unvalidated(),
                    ))
                })
                .or_else(|| {
                    self.likely_subtags
                        .language
                        .get_copied(&language.into_tinystr().to_unvalidated())
                        .map(|(s, _r)| s)
                });

            if Some(script) == default_script {
                locale.script = None;
            }
        }
        // 4. If there is an invalid "sd" subtag, drop it
        // For now, ignore it, and let fallback do it for us
    }
}

impl<'a> LocaleFallbackIteratorInner<'a> {
    pub fn step(&mut self, locale: &mut DataLocale) {
        match self.config.priority {
            LocaleFallbackPriority::Language => self.step_language(locale),
            LocaleFallbackPriority::Region => self.step_region(locale),
            // This case should not normally happen, but `LocaleFallbackPriority` is non_exhaustive.
            // Make it go directly to `und`.
            _ => {
                debug_assert!(
                    false,
                    "Unknown LocaleFallbackPriority: {:?}",
                    self.config.priority
                );
                *locale = Default::default()
            }
        }
    }

    fn step_language(&mut self, locale: &mut DataLocale) {
        // 2. Remove the subdivision keyword
        if let Some(value) = locale.subdivision.take() {
            self.backup_subdivision = Some(value);
            return;
        }
        // 3. Assert that the locale is a language identifier
        debug_assert!(locale.subdivision.is_none());
        // 4. Remove variants
        if let Some(single_variant) = locale.variant.take() {
            self.backup_variant = Some(single_variant);
            return;
        }
        // 5. Check for parent override
        if let Some((language, script, region)) = self.get_explicit_parent(locale) {
            locale.language = language;
            locale.script = script;
            locale.region = region;
            self.restore_subdivision_variants(locale);
            return;
        }
        // 6. Add the script subtag if necessary
        if locale.script.is_none() {
            if let Some(region) = locale.region {
                let language = locale.language;
                if let Some(script) = self.likely_subtags.language_region.get_copied(&(
                    language.into_tinystr().to_unvalidated(),
                    region.into_tinystr().to_unvalidated(),
                )) {
                    locale.script = Some(script);
                    self.restore_subdivision_variants(locale);
                    return;
                }
            }
        }
        // 7. Remove region
        if locale.region.is_some() {
            locale.region = None;
            self.restore_subdivision_variants(locale);
            return;
        }
        // 8. Remove language+script
        debug_assert!(!locale.language.is_empty()); // don't call .step() on und
        locale.script = None;
        locale.language = Language::UND;
    }

    fn step_region(&mut self, locale: &mut DataLocale) {
        // TODO(#4413): -u-rg is not yet supported
        // 2. Remove the subdivision keyword
        if let Some(value) = locale.subdivision.take() {
            self.backup_subdivision = Some(value);
            return;
        }
        // 3. Assert that the locale is a language identifier
        debug_assert!(locale.subdivision.is_none());
        // 4. Remove variants
        if let Some(variant) = locale.variant.take() {
            self.backup_variant = Some(variant);
            return;
        }
        // 5. Remove language+script
        if !locale.language.is_empty() || locale.script.is_some() {
            locale.script = None;
            locale.language = Language::UND;
            self.restore_subdivision_variants(locale);
            return;
        }
        // 6. Remove region
        debug_assert!(locale.region.is_some()); // don't call .step() on und
        locale.region = None;
    }

    fn restore_subdivision_variants(&mut self, locale: &mut DataLocale) {
        if let Some(subdivision) = self.backup_subdivision.take() {
            locale.subdivision = Some(subdivision);
        }
        if let Some(variant) = self.backup_variant.take() {
            locale.variant = Some(variant);
        }
    }

    fn get_explicit_parent(
        &self,
        locale: &DataLocale,
    ) -> Option<(Language, Option<Script>, Option<Region>)> {
        self.parents
            .parents
            .get_copied_by(|uvstr| locale.strict_cmp(uvstr).reverse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use writeable::Writeable;

    struct TestCase {
        input: &'static str,
        requires_data: bool,
        // Note: The first entry in the chain is the normalized locale
        expected_language_chain: &'static [&'static str],
        expected_region_chain: &'static [&'static str],
    }

    // TODO: Consider loading these from a JSON file
    const TEST_CASES: &[TestCase] = &[
        TestCase {
            input: "en-u-sd-usca",
            requires_data: false,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-u-sd-usca", "en", "und-u-sd-usca"],
        },
        TestCase {
            input: "en-US-u-sd-usca",
            requires_data: false,
            expected_language_chain: &["en-US-u-sd-usca", "en-US", "en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-US-fonipa-u-sd-usca",
            requires_data: false,
            expected_language_chain: &[
                "en-US-fonipa-u-sd-usca",
                "en-US-fonipa",
                "en-US",
                "en-fonipa-u-sd-usca",
                "en-fonipa",
                "en",
            ],
            expected_region_chain: &[
                "en-US-fonipa-u-sd-usca",
                "en-US-fonipa",
                "en-US",
                "und-US-fonipa-u-sd-usca",
                "und-US-fonipa",
                "und-US",
            ],
        },
        TestCase {
            input: "en-u-sd-usca",
            requires_data: true,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-Latn-u-sd-usca",
            requires_data: true,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-Latn-US-u-sd-usca",
            requires_data: true,
            expected_language_chain: &["en-US-u-sd-usca", "en-US", "en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "sr-ME",
            requires_data: true,
            expected_language_chain: &["sr-ME", "sr-Latn-ME", "sr-Latn"],
            expected_region_chain: &["sr-ME", "und-ME"],
        },
        TestCase {
            input: "sr-Latn-ME",
            requires_data: true,
            expected_language_chain: &["sr-ME", "sr-Latn-ME", "sr-Latn"],
            expected_region_chain: &["sr-ME", "und-ME"],
        },
        TestCase {
            input: "sr-ME-fonipa",
            requires_data: true,
            expected_language_chain: &[
                "sr-ME-fonipa",
                "sr-ME",
                "sr-Latn-ME-fonipa",
                "sr-Latn-ME",
                "sr-Latn-fonipa",
                "sr-Latn",
            ],
            expected_region_chain: &["sr-ME-fonipa", "sr-ME", "und-ME-fonipa", "und-ME"],
        },
        TestCase {
            input: "sr-RS",
            requires_data: true,
            expected_language_chain: &["sr-RS", "sr"],
            expected_region_chain: &["sr-RS", "und-RS"],
        },
        TestCase {
            input: "sr-Cyrl-RS",
            requires_data: true,
            expected_language_chain: &["sr-RS", "sr"],
            expected_region_chain: &["sr-RS", "und-RS"],
        },
        TestCase {
            input: "sr-Latn-RS",
            requires_data: true,
            expected_language_chain: &["sr-Latn-RS", "sr-Latn"],
            expected_region_chain: &["sr-Latn-RS", "und-RS"],
        },
        TestCase {
            input: "de-Latn-LI",
            requires_data: true,
            expected_language_chain: &["de-LI", "de"],
            expected_region_chain: &["de-LI", "und-LI"],
        },
        TestCase {
            input: "ca-ES-valencia",
            requires_data: true,
            expected_language_chain: &["ca-ES-valencia", "ca-ES", "ca-valencia", "ca"],
            expected_region_chain: &["ca-ES-valencia", "ca-ES", "und-ES-valencia", "und-ES"],
        },
        TestCase {
            input: "es-AR",
            requires_data: true,
            expected_language_chain: &["es-AR", "es-419", "es"],
            expected_region_chain: &["es-AR", "und-AR"],
        },
        TestCase {
            input: "hi-IN",
            requires_data: true,
            expected_language_chain: &["hi-IN", "hi"],
            expected_region_chain: &["hi-IN", "und-IN"],
        },
        TestCase {
            input: "hi-Latn-IN",
            requires_data: true,
            expected_language_chain: &["hi-Latn-IN", "hi-Latn", "en-IN", "en-001", "en"],
            expected_region_chain: &["hi-Latn-IN", "und-IN"],
        },
        TestCase {
            input: "zh-CN",
            requires_data: true,
            // Note: "zh-Hans" is not reachable because it is the default script for "zh".
            // The fallback algorithm does not visit the language-script bundle when the
            // script is the default for the language
            expected_language_chain: &["zh-CN", "zh"],
            expected_region_chain: &["zh-CN", "und-CN"],
        },
        TestCase {
            input: "zh-TW",
            requires_data: true,
            expected_language_chain: &["zh-TW", "zh-Hant-TW", "zh-Hant"],
            expected_region_chain: &["zh-TW", "und-TW"],
        },
        TestCase {
            input: "yue-HK",
            requires_data: true,
            expected_language_chain: &["yue-HK", "yue"],
            expected_region_chain: &["yue-HK", "und-HK"],
        },
        TestCase {
            input: "yue-HK",
            requires_data: true,
            expected_language_chain: &["yue-HK", "yue"],
            expected_region_chain: &["yue-HK", "und-HK"],
            // TODO(#3867): script fallback should do zh-Hant or und-Hant as well
        },
    ];

    #[test]
    fn test_fallback() {
        let fallbacker_no_data = LocaleFallbacker::new_without_data();
        let fallbacker_no_data = fallbacker_no_data.as_borrowed();
        let fallbacker_with_data = LocaleFallbacker::new();
        for cas in TEST_CASES {
            for (priority, expected_chain) in [
                (
                    LocaleFallbackPriority::Language,
                    cas.expected_language_chain,
                ),
                (LocaleFallbackPriority::Region, cas.expected_region_chain),
            ] {
                let mut config = LocaleFallbackConfig::default();
                config.priority = priority;
                let fallbacker = if cas.requires_data {
                    fallbacker_with_data
                } else {
                    fallbacker_no_data
                };
                let mut it = fallbacker
                    .for_config(config)
                    .fallback_for(cas.input.parse().unwrap());
                for &expected in expected_chain {
                    assert_eq!(
                        expected,
                        &*it.get().write_to_string(),
                        "{:?} ({:?})",
                        cas.input,
                        priority
                    );
                    it.step();
                }
                assert_eq!(
                    "und",
                    &*it.get().write_to_string(),
                    "{:?} ({:?})",
                    cas.input,
                    priority
                );
            }
        }
    }
}
