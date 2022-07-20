// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::extensions::unicode::Key;
use icu_locid::extensions_unicode_key as key;
use icu_locid::subtags::Language;
use icu_locid::LanguageIdentifier;
use icu_provider::FallbackPriority;

use super::*;

const SUBDIVISION_KEY: Key = key!("sd");

impl<'a> LocaleFallbackerWithConfig<'a> {
    pub(crate) fn normalize(&self, ro: &mut DataOptions) {
        let language = ro.language();
        // 1. Populate the region (required for region fallback only)
        if self.config.priority == FallbackPriority::Region && ro.region().is_none() {
            // 1a. First look for region based on language+script
            if let Some(script) = ro.script() {
                ro.set_region(
                    self.likely_subtags
                        .ls2r
                        .get(&language.into(), &script.into())
                        .ok()
                        .copied(),
                );
            }
            // 1b. If that fails, try language only
            if ro.region().is_none() {
                ro.set_region(self.likely_subtags.l2r.get(&language.into()).copied());
            }
        }
        // 2. Remove the script if it is implied by the other subtags
        if let Some(script) = ro.script() {
            let default_script = self
                .likely_subtags
                .l2s
                .get_copied(&language.into())
                .unwrap_or(DEFAULT_SCRIPT);
            if let Some(region) = ro.region() {
                if script
                    == self
                        .likely_subtags
                        .lr2s
                        .get_copied(&language.into(), &region.into())
                        .unwrap_or(default_script)
                {
                    ro.set_script(None);
                }
            } else if script == default_script {
                ro.set_script(None);
            }
        }
        // 3. Remove irrelevant extension subtags
        ro.retain_unicode_ext(|key| {
            match *key {
                // Always retain -u-sd
                SUBDIVISION_KEY => true,
                // Retain the query-specific keyword
                _ if Some(*key) == self.config.extension_key => true,
                // Drop all others
                _ => false,
            }
        });
        // 4. If there is an invalid "sd" subtag, drop it
        // For now, ignore it, and let fallback do it for us
    }
}

impl<'a, 'b> LocaleFallbackIteratorInner<'a, 'b> {
    pub fn step(&mut self, ro: &mut DataOptions) {
        match self.config.priority {
            FallbackPriority::Language => self.step_language(ro),
            FallbackPriority::Region => self.step_region(ro),
            // This case should not normally happen, but `FallbackPriority` is non_exhaustive.
            // Make it go directly to `und`.
            _ => *ro = Default::default(),
        }
    }

    fn step_language(&mut self, ro: &mut DataOptions) {
        // 1. Remove the extension fallback keyword
        if let Some(extension_key) = self.config.extension_key {
            if let Some(value) = ro.remove_unicode_ext(&extension_key) {
                self.backup_extension = Some(value);
                return;
            }
        }
        // 2. Remove the subdivision keyword
        if let Some(value) = ro.remove_unicode_ext(&SUBDIVISION_KEY) {
            self.backup_subdivision = Some(value);
            return;
        }
        // 3. Assert that the locale is a language identifier
        debug_assert!(!ro.has_unicode_ext());
        // 4. Remove variants
        if ro.has_variants() {
            self.backup_variants = Some(ro.clear_variants());
            return;
        }
        // 5. Check for parent override
        if let Some(parent) = self
            .parents
            .parents
            .get_copied_by(|bytes| ro.strict_cmp(bytes).reverse())
        {
            let lid = LanguageIdentifier::from(parent);
            ro.set_langid(lid);
            self.restore_extensions_variants(ro);
            return;
        }
        // 6. Add the script subtag if necessary
        if ro.script().is_none() {
            if let Some(region) = ro.region() {
                let language = ro.language();
                if let Ok(script) = self
                    .likely_subtags
                    .lr2s
                    .get_copied(&language.into(), &region.into())
                {
                    ro.set_script(Some(script));
                    self.restore_extensions_variants(ro);
                    return;
                }
            }
        }
        // 7. Remove region
        if ro.region().is_some() {
            ro.set_region(None);
            return;
        }
        // 8. Remove language+script
        debug_assert!(!ro.language().is_empty()); // don't call .step() on und
        ro.set_script(None);
        ro.set_language(Language::UND);
    }

    fn step_region(&mut self, ro: &mut DataOptions) {
        // 1. Remove the extension fallback keyword
        if let Some(extension_key) = self.config.extension_key {
            if let Some(value) = ro.remove_unicode_ext(&extension_key) {
                self.backup_extension = Some(value);
                return;
            }
        }
        // 2. Remove the subdivision keyword
        if let Some(value) = ro.remove_unicode_ext(&SUBDIVISION_KEY) {
            self.backup_subdivision = Some(value);
            return;
        }
        // 3. Assert that the locale is a language identifier
        debug_assert!(!ro.has_unicode_ext());
        // 4. Remove variants
        if ro.has_variants() {
            self.backup_variants = Some(ro.clear_variants());
            return;
        }
        // 5. Remove language+script
        if !ro.language().is_empty() || ro.script().is_some() {
            ro.set_script(None);
            ro.set_language(Language::UND);
            self.restore_extensions_variants(ro);
            return;
        }
        // 6. Remove region
        debug_assert!(ro.region().is_some()); // don't call .step() on und
        ro.set_region(None);
    }

    fn restore_extensions_variants(&mut self, ro: &mut DataOptions) {
        if let Some(value) = self.backup_extension.take() {
            #[allow(clippy::unwrap_used)] // not reachable unless extension_key is present
            ro.set_unicode_ext(self.config.extension_key.unwrap(), value);
        }
        if let Some(value) = self.backup_subdivision.take() {
            ro.set_unicode_ext(SUBDIVISION_KEY, value);
        }
        if let Some(variants) = self.backup_variants.take() {
            ro.set_variants(variants);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::Locale;
    use std::str::FromStr;

    struct TestCase {
        input: &'static str,
        requires_data: bool,
        extension_key: Option<Key>,
        // Note: The first entry in the chain is the normalized locale
        expected_language_chain: &'static [&'static str],
        expected_region_chain: &'static [&'static str],
    }

    // TODO: Consider loading these from a JSON file
    const TEST_CASES: &[TestCase] = &[
        TestCase {
            input: "en-u-hc-h12-sd-usca",
            requires_data: false,
            extension_key: None,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-u-sd-usca", "en", "und-u-sd-usca"],
        },
        TestCase {
            input: "en-US-u-hc-h12-sd-usca",
            requires_data: false,
            extension_key: None,
            expected_language_chain: &["en-US-u-sd-usca", "en-US", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-US-fonipa-u-hc-h12-sd-usca",
            requires_data: false,
            extension_key: Some(key!("hc")),
            expected_language_chain: &[
                "en-US-fonipa-u-hc-h12-sd-usca",
                "en-US-fonipa-u-sd-usca",
                "en-US-fonipa",
                "en-US",
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
            extension_key: None,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-Latn-u-sd-usca",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["en-u-sd-usca", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            input: "en-Latn-US-u-sd-usca",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["en-US-u-sd-usca", "en-US", "en"],
            expected_region_chain: &["en-US-u-sd-usca", "en-US", "und-US-u-sd-usca", "und-US"],
        },
        TestCase {
            // NOTE: -u-rg is not yet supported; when it is, this test should be updated
            input: "en-u-rg-gbxxxx",
            requires_data: false,
            extension_key: None,
            expected_language_chain: &["en"],
            expected_region_chain: &["en"],
        },
        TestCase {
            input: "sr-ME",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["sr-ME", "sr-Latn-ME", "sr-Latn"],
            expected_region_chain: &["sr-ME", "und-ME"],
        },
        TestCase {
            input: "sr-ME-fonipa",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &[
                "sr-ME-fonipa",
                "sr-ME",
                "sr-Latn-ME-fonipa",
                "sr-Latn-ME",
                "sr-Latn",
            ],
            expected_region_chain: &["sr-ME-fonipa", "sr-ME", "und-ME-fonipa", "und-ME"],
        },
        TestCase {
            input: "de-Latn-LI",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["de-LI", "de"],
            expected_region_chain: &["de-LI", "und-LI"],
        },
        TestCase {
            input: "ca-ES-valencia",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["ca-ES-valencia", "ca-ES", "ca"],
            expected_region_chain: &["ca-ES-valencia", "ca-ES", "und-ES-valencia", "und-ES"],
        },
        TestCase {
            input: "es-AR",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["es-AR", "es-419", "es"],
            expected_region_chain: &["es-AR", "und-AR"],
        },
        TestCase {
            input: "hi-IN",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["hi-IN", "hi"],
            expected_region_chain: &["hi-IN", "und-IN"],
        },
        TestCase {
            input: "hi-Latn-IN",
            requires_data: true,
            extension_key: None,
            expected_language_chain: &["hi-Latn-IN", "hi-Latn", "en-IN", "en-001", "en"],
            expected_region_chain: &["hi-Latn-IN", "und-IN"],
        },
    ];

    #[test]
    fn test_fallback() {
        let fallbacker_no_data = LocaleFallbacker::new_without_data();
        let provider = icu_testdata::get_postcard_provider();
        let fallbacker_with_data = LocaleFallbacker::try_new(&provider).unwrap();
        for cas in TEST_CASES {
            for (priority, expected_chain) in [
                (FallbackPriority::Language, cas.expected_language_chain),
                (FallbackPriority::Region, cas.expected_region_chain),
            ] {
                let config = LocaleFallbackConfig {
                    priority,
                    extension_key: cas.extension_key,
                };
                let key_fallbacker = if cas.requires_data {
                    fallbacker_with_data.for_config(config)
                } else {
                    fallbacker_no_data.for_config(config)
                };
                let loc = Locale::from_str(cas.input).unwrap();
                let mut ro = DataOptions::from(loc);
                let mut it = key_fallbacker.fallback_for(&mut ro);
                for expected in expected_chain {
                    assert_eq!(
                        expected,
                        &it.get().to_string(),
                        "{:?} ({:?})",
                        cas.input,
                        priority
                    );
                    it.step();
                }
                assert_eq!(
                    "und",
                    it.get().to_string(),
                    "{:?} ({:?})",
                    cas.input,
                    priority
                );
            }
        }
    }
}
