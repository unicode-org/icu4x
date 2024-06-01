// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::subtags::Language;
use icu_locale_core::LanguageIdentifier;
use icu_provider::FallbackPriority;

use super::*;

impl<'a> LocaleFallbackerWithConfig<'a> {
    pub(crate) fn normalize(&self, langid: &mut LanguageIdentifier) {
        let language = langid.language;
        // 1. Populate the region (required for region fallback only)
        if self.config.priority == FallbackPriority::Region && langid.region.is_none() {
            // 1a. First look for region based on language+script
            if let Some(script) = langid.script {
                langid.region = self
                    .likely_subtags
                    .ls2r
                    .get_2d(
                        &language.into_tinystr().to_unvalidated(),
                        &script.into_tinystr().to_unvalidated(),
                    )
                    .copied();
            }
            // 1b. If that fails, try language only
            if langid.region.is_none() {
                langid.region = self
                    .likely_subtags
                    .l2r
                    .get(&language.into_tinystr().to_unvalidated())
                    .copied();
            }
        }
        // 2. Remove the script if it is implied by the other subtags
        if let Some(script) = langid.script {
            let default_script = self
                .likely_subtags
                .l2s
                .get_copied(&language.into_tinystr().to_unvalidated())
                .unwrap_or(DEFAULT_SCRIPT);
            if let Some(region) = langid.region {
                if script
                    == self
                        .likely_subtags
                        .lr2s
                        .get_copied_2d(
                            &language.into_tinystr().to_unvalidated(),
                            &region.into_tinystr().to_unvalidated(),
                        )
                        .unwrap_or(default_script)
                {
                    langid.script = None;
                }
            } else if script == default_script {
                langid.script = None;
            }
        }
    }
}

impl<'a> LocaleFallbackIteratorInner<'a> {
    pub fn step(&mut self, langid: &mut LanguageIdentifier) {
        match self.config.priority {
            FallbackPriority::Language => self.step_language(langid),
            FallbackPriority::Region => self.step_region(langid),
            // TODO(#1964): Change the collation fallback rules to be different
            // from the language fallback fules.
            FallbackPriority::Collation => self.step_language(langid),
            // This case should not normally happen, but `FallbackPriority` is non_exhaustive.
            // Make it go directly to `und`.
            _ => {
                debug_assert!(
                    false,
                    "Unknown FallbackPriority: {:?}",
                    self.config.priority
                );
                *langid = Default::default()
            }
        }
    }

    fn step_language(&mut self, langid: &mut LanguageIdentifier) {
        // 4. Remove variants
        if !langid.variants.is_empty() {
            self.backup_variants = Some(core::mem::take(&mut langid.variants));
            return;
        }
        // 5. Check for parent override
        if let Some(parent) = self.get_explicit_parent(langid) {
            *langid = parent;
            self.restore_variants(langid);
            return;
        }
        // 6. Add the script subtag if necessary
        if langid.script.is_none() {
            if let Some(region) = langid.region {
                let language = langid.language;
                if let Some(script) = self.likely_subtags.lr2s.get_copied_2d(
                    &language.into_tinystr().to_unvalidated(),
                    &region.into_tinystr().to_unvalidated(),
                ) {
                    langid.script = Some(script);
                    self.restore_variants(langid);
                    return;
                }
            }
        }
        // 7. Remove region
        if langid.region.is_some() {
            langid.region = None;
            self.restore_variants(langid);
            return;
        }
        // 8. Remove language+script
        debug_assert!(!langid.language.is_empty()); // don't call .step() on und
        langid.script = None;
        langid.language = Language::UND;
    }

    fn step_region(&mut self, langid: &mut LanguageIdentifier) {
        // 4. Remove variants
        if !langid.variants.is_empty() {
            self.backup_variants = Some(core::mem::take(&mut langid.variants));
            return;
        }
        // 5. Remove language+script
        if !langid.language.is_empty() || langid.script.is_some() {
            langid.script = None;
            langid.language = Language::UND;
            self.restore_variants(langid);
            return;
        }
        // 6. Remove region
        debug_assert!(langid.region.is_some()); // don't call .step() on und
        langid.region = None;
    }

    fn restore_variants(&mut self, langid: &mut LanguageIdentifier) {
        if let Some(variants) = self.backup_variants.take() {
            langid.variants = variants;
        }
    }

    fn get_explicit_parent(&self, langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
        self.supplement
            .and_then(|supplement| {
                supplement
                    .parents
                    .get_copied_by(|uvstr| langid.strict_cmp(uvstr).reverse())
            })
            .or_else(|| {
                self.parents
                    .parents
                    .get_copied_by(|uvstr| langid.strict_cmp(uvstr).reverse())
            })
            .map(LanguageIdentifier::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use writeable::Writeable;

    struct TestCase {
        input: &'static str,
        requires_data: bool,
        fallback_supplement: Option<LocaleFallbackSupplement>,
        // Note: The first entry in the chain is the normalized locale
        expected_language_chain: &'static [&'static str],
        expected_region_chain: &'static [&'static str],
    }

    // TODO: Consider loading these from a JSON file
    const TEST_CASES: &[TestCase] = &[
        TestCase {
            input: "en-u-hc-h12-sd-usca",
            requires_data: false,
            fallback_supplement: None,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-u-sd-usca", "en", "und-u-sd-usca"],
        },
        TestCase {
            input: "en-US-u-hc-h12-sd-usca",
            requires_data: false,
            fallback_supplement: None,
            expected_language_chain: &["en-US-u-sd-usca", "en-US", "en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-US-fonipa-u-hc-h12-sd-usca",
            requires_data: false,
            fallback_supplement: None,
            expected_language_chain: &[
                "en-US-fonipa-u-hc-h12-sd-usca",
                "en-US-fonipa-u-sd-usca",
                "en-US-fonipa",
                "en-US",
                "en-fonipa-u-hc-h12-sd-usca",
                "en-fonipa-u-sd-usca",
                "en-fonipa",
                "en",
            ],
            expected_region_chain: &[
                "en-US-fonipa-u-hc-h12-sd-usca",
                "en-US-fonipa-u-sd-usca",
                "en-US-fonipa",
                "en-US",
                "und-US-fonipa-u-hc-h12-sd-usca",
                "und-US-fonipa-u-sd-usca",
                "und-US-fonipa",
                "und-US",
            ],
        },
        TestCase {
            input: "en-u-hc-h12-sd-usca",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-Latn-u-sd-usca",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-Latn-US-u-sd-usca",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["en-US-u-sd-usca", "en-US", "en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            // TODO(#4413): -u-rg is not yet supported; when it is, this test should be updated
            input: "en-u-rg-gbxxxx",
            requires_data: false,
            fallback_supplement: None,
            expected_language_chain: &["en"],
            expected_region_chain: &["en"],
        },
        TestCase {
            input: "sr-ME",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["sr-ME", "sr-Latn-ME", "sr-Latn"],
            expected_region_chain: &["sr-ME", "und-ME"],
        },
        TestCase {
            input: "sr-Latn-ME",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["sr-ME", "sr-Latn-ME", "sr-Latn"],
            expected_region_chain: &["sr-ME", "und-ME"],
        },
        TestCase {
            input: "sr-ME-fonipa",
            requires_data: true,
            fallback_supplement: None,
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
            fallback_supplement: None,
            expected_language_chain: &["sr-RS", "sr"],
            expected_region_chain: &["sr-RS", "und-RS"],
        },
        TestCase {
            input: "sr-Cyrl-RS",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["sr-RS", "sr"],
            expected_region_chain: &["sr-RS", "und-RS"],
        },
        TestCase {
            input: "sr-Latn-RS",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["sr-Latn-RS", "sr-Latn"],
            expected_region_chain: &["sr-Latn-RS", "und-RS"],
        },
        TestCase {
            input: "de-Latn-LI",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["de-LI", "de"],
            expected_region_chain: &["de-LI", "und-LI"],
        },
        TestCase {
            input: "ca-ES-valencia",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["ca-ES-valencia", "ca-ES", "ca-valencia", "ca"],
            expected_region_chain: &["ca-ES-valencia", "ca-ES", "und-ES-valencia", "und-ES"],
        },
        TestCase {
            input: "es-AR",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["es-AR", "es-419", "es"],
            expected_region_chain: &["es-AR", "und-AR"],
        },
        TestCase {
            input: "hi-IN",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["hi-IN", "hi"],
            expected_region_chain: &["hi-IN", "und-IN"],
        },
        TestCase {
            input: "hi-Latn-IN",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["hi-Latn-IN", "hi-Latn", "en-IN", "en-001", "en"],
            expected_region_chain: &["hi-Latn-IN", "und-IN"],
        },
        TestCase {
            input: "zh-CN",
            requires_data: true,
            fallback_supplement: None,
            // Note: "zh-Hans" is not reachable because it is the default script for "zh".
            // The fallback algorithm does not visit the language-script bundle when the
            // script is the default for the language
            expected_language_chain: &["zh-CN", "zh"],
            expected_region_chain: &["zh-CN", "und-CN"],
        },
        TestCase {
            input: "zh-TW",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["zh-TW", "zh-Hant-TW", "zh-Hant"],
            expected_region_chain: &["zh-TW", "und-TW"],
        },
        TestCase {
            input: "yue-HK",
            requires_data: true,
            fallback_supplement: None,
            expected_language_chain: &["yue-HK", "yue"],
            expected_region_chain: &["yue-HK", "und-HK"],
        },
        TestCase {
            input: "yue-HK",
            requires_data: true,
            fallback_supplement: Some(LocaleFallbackSupplement::Collation),
            expected_language_chain: &["yue-HK", "yue", "zh-Hant", "zh"],
            expected_region_chain: &["yue-HK", "und-HK"],
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
                config.fallback_supplement = cas.fallback_supplement;
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
