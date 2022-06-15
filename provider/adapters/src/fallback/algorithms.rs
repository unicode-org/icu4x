// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::extensions::unicode::Key;
use icu_locid::subtags::Language;
use icu_locid::unicode_ext_key;
use icu_locid::LanguageIdentifier;

use super::*;

const REGION_KEY: Key = unicode_ext_key!("rg");
const SUBDIVISION_KEY: Key = unicode_ext_key!("sd");

impl<'a> LocaleFallbackerForKey<'a> {
    pub(crate) fn normalize(&self, ro: &mut ResourceOptions) {
        let language = ro.language();
        // 1. Populate the region
        if ro.region().is_none() {
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
            if let Some(region) = ro.region() {
                if let Ok(region_script) = self
                    .likely_subtags
                    .lr2s
                    .get_copied(&language.into(), &region.into())
                {
                    if region_script == script {
                        ro.set_script(None);
                    }
                } else if let Some(language_script) =
                    self.likely_subtags.l2s.get_copied(&language.into())
                {
                    if language_script == script {
                        ro.set_script(None);
                    }
                }
            }
        }
        // 3. Remove irrelevant extension subtags
        ro.retain_unicode_ext(|key| {
            match *key {
                // Always retain -u-sd
                SUBDIVISION_KEY => true,
                // Retain -u-rg only in region fallback mode
                REGION_KEY => self.key_metadata.strategy == LocaleFallbackStrategy::RegionPriority,
                // Retain the query-specific keyword
                _ if Some(*key) == self.key_metadata.extension_kw => true,
                // Drop all others
                _ => false,
            }
        });
        // 4. If there is an invalid "sd" subtag, drop it
        // For now, ignore it, and let fallback do it for us
    }
}

impl<'a, 'b> LocaleFallbackIterator<'a, 'b> {
    pub fn step(&mut self) {
        match self.key_metadata.strategy {
            LocaleFallbackStrategy::LanguagePriority => self.step_language(),
            LocaleFallbackStrategy::RegionPriority => self.step_region(),
        }
    }

    fn step_language(&mut self) {
        let ro = &mut self.current;
        // 1. Remove the extension fallback keyword
        if let Some(extension_kw) = self.key_metadata.extension_kw {
            if let Some(value) = ro.remove_unicode_ext(&extension_kw) {
                self.backup_extension = Some(value);
                println!("Fallback 1");
                return;
            }
        }
        // 2. Remove the subdivision keyword
        if let Some(value) = ro.remove_unicode_ext(&SUBDIVISION_KEY) {
            self.backup_subdivision = Some(value);
            println!("Fallback 2");
            return;
        }
        // 3. Assert that the locale is a language identifier
        debug_assert!(!ro.has_unicode_ext());
        // 4. Remove variants
        if ro.has_variants() {
            self.backup_variants = Some(ro.clear_variants());
            println!("Fallback 4");
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
            self.restore_extensions_variants();
            println!("Fallback 5");
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
                    self.restore_extensions_variants();
                    println!("Fallback 6");
                    return;
                }
            }
        }
        // 7. Remove region
        if ro.region().is_some() {
            ro.set_region(None);
            println!("Fallback 7");
            return;
        }
        // 8. Remove language+script
        debug_assert!(!ro.language().is_empty());
        ro.set_script(None);
        ro.set_language(Language::UND);
        println!("Fallback 8");
    }

    fn step_region(&mut self) {
        todo!()
    }

    fn restore_extensions_variants(&mut self) {
        let ro = &mut self.current;
        if let Some(value) = self.backup_extension.take() {
            #[allow(clippy::unwrap_used)] // not reachable unless extension_kw is present
            ro.set_unicode_ext(self.key_metadata.extension_kw.unwrap(), value);
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

    #[test]
    fn test_normalize_no_data() {
        struct TestCase {
            input: &'static str,
            strategy: LocaleFallbackStrategy,
            extension_kw: Option<Key>,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                input: "en-u-hc-h12-sd-usca",
                strategy: LocaleFallbackStrategy::LanguagePriority,
                extension_kw: None,
                expected: "en-u-sd-usca",
            },
            TestCase {
                input: "en-u-hc-h12-sd-usca",
                strategy: LocaleFallbackStrategy::LanguagePriority,
                extension_kw: Some(unicode_ext_key!("hc")),
                expected: "en-u-hc-h12-sd-usca",
            },
            TestCase {
                input: "en-u-rg-gb",
                strategy: LocaleFallbackStrategy::LanguagePriority,
                extension_kw: None,
                expected: "en",
            },
            TestCase {
                input: "en-u-rg-gbxxxx",
                strategy: LocaleFallbackStrategy::RegionPriority,
                extension_kw: None,
                expected: "en-u-rg-gbxxxx",
            },
        ];
        let fallbacker = LocaleFallbacker::new_without_data();
        for cas in cases {
            let key_fallbacker = fallbacker.for_key_metadata(LocaleFallbackKeyMetadata {
                strategy: cas.strategy,
                extension_kw: cas.extension_kw,
            });
            let loc = Locale::from_str(cas.input).unwrap();
            let mut ro = ResourceOptions::from(loc);
            key_fallbacker.normalize(&mut ro);
            assert_eq!(cas.expected, ro.to_string());
        }
    }

    #[test]
    fn test_fallback() {
        struct TestCase {
            input: &'static str,
            extension_kw: Option<Key>,
            expected_language_chain: &'static [&'static str],
            expected_region_chain: &'static [&'static str],
        }
        let cases = [
            TestCase {
                input: "en-u-hc-h12-sd-usca",
                extension_kw: None,
                expected_language_chain: &["en-US-u-sd-usca", "en-US", "en"],
                expected_region_chain: &["en-US-u-sd-usca", "und-US-u-sd-usca", "und-US"]
            },
            // TestCase {
            //     input: "en-u-hc-h12-sd-usca",
            //     strategy: LocaleFallbackStrategy::LanguagePriority,
            //     extension_kw: Some(unicode_ext_key!("hc")),
            //     expected: "en-u-hc-h12-sd-usca",
            // },
            // TestCase {
            //     input: "en-u-rg-gb",
            //     strategy: LocaleFallbackStrategy::LanguagePriority,
            //     extension_kw: None,
            //     expected: "en",
            // },
            // TestCase {
            //     input: "en-u-rg-gbxxxx",
            //     strategy: LocaleFallbackStrategy::RegionPriority,
            //     extension_kw: None,
            //     expected: "en-u-rg-gbxxxx",
            // },
        ];
        let provider = icu_testdata::get_provider();
        let fallbacker = LocaleFallbacker::try_new(&provider).unwrap();
        for cas in cases {
            for (strategy, expected_list) in [(LocaleFallbackStrategy::LanguagePriority, cas.expected_language_chain), (LocaleFallbackStrategy::RegionPriority, cas.expected_region_chain)] {
                let key_fallbacker = fallbacker.for_key_metadata(LocaleFallbackKeyMetadata {
                    strategy: strategy,
                    extension_kw: cas.extension_kw,
                });
                let loc = Locale::from_str(cas.input).unwrap();
                let ro = ResourceOptions::from(loc);
                let mut it = key_fallbacker.fallback_for(ro);
                for expected in expected_list {
                    assert_eq!(expected, &it.get().to_string(), "{:?} ({:?})", cas.input, strategy);
                    it.step();
                }
                assert_eq!("und", it.get().to_string());
            }
        }
    }
}
