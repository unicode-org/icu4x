// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::extensions::unicode::Key;
use icu_locid::subtags::Language;
use icu_locid::LanguageIdentifier;
use icu_locid::unicode_ext_key;

use super::*;

const REGION_KEY: Key = unicode_ext_key!("rg");
const SUBDIVISION_KEY: Key = unicode_ext_key!("sd");

impl<'a> LocaleFallbackerForKey<'a> {
    pub(crate) fn normalize(&self, ro: &mut ResourceOptions) {
        let lang_raw = ro.language().into_raw();
        // 1. Populate the region
        if ro.region().is_none() {
            // 1a. First look for region based on language+script
            if let Some(script) = ro.script() {
                ro.set_region(
                    self.fallback_data
                        .ls2r
                        .get(&lang_raw, &script.into_raw())
                        .ok()
                        .copied(),
                );
            }
            // 1b. If that fails, try language only
            if ro.region().is_none() {
                ro.set_region(self.fallback_data.l2r.get(&lang_raw).copied());
            }
        }
        // 2. Check if this is a multi-script language
        let maybe_default_script = self.fallback_data.l2s.get(&lang_raw).copied();
        // 3. Populate the script iff multi-script language
        if let Some(default_script) = maybe_default_script {
            if ro.script().is_none() {
                // 3a. First look for the script based on language+region
                if let Some(region) = ro.region() {
                    ro.set_script(
                        self.fallback_data
                            .lr2s
                            .get(&lang_raw, &region.into_raw())
                            .ok()
                            .copied(),
                    );
                }
                // 3b. If that fails, use the default script loaded above
                if ro.script().is_none() {
                    ro.set_script(Some(default_script));
                }
            }
            debug_assert!(ro.script().is_some());
        } else {
            // 3c. If single-script language, clear out the script
            ro.set_script(None);
        }
        // 4. Remove irrelevant extension subtags
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
        // 5. If there is an invalid "sd" subtag, drop it
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
                return;
            }
        }
        // 2. Remove the subdivision keyword
        if let Some(value) = ro.remove_unicode_ext(&SUBDIVISION_KEY) {
            self.backup_subdivision = Some(value);
            return;
        }
        // 3. Assert that the locale is a language identifier
        debug_assert!(ro.has_unicode_ext());
        // 4. Check for parent override
        if let Some(parent) = self
            .fallback_data
            .parents
            .get_copied_by(|bytes| ro.cmp_bytes(bytes).reverse())
        {
            let lid = LanguageIdentifier::from(parent);
            ro.set_langid(lid);
            if let Some(value) = self.backup_extension.take() {
                #[allow(clippy::unwrap_used)] // not reachable unless extension_kw is present
                ro.set_unicode_ext(self.key_metadata.extension_kw.unwrap(), value);
            }
            if let Some(value) = self.backup_subdivision.take() {
                ro.set_unicode_ext(SUBDIVISION_KEY, value);
            }
            return;
        }
        // 5. Remove variants
        if ro.has_variants() {
            ro.clear_variants();
            return;
        }
        // 6. Remove region
        if ro.region().is_some() {
            ro.set_region(None);
            return;
        }
        // 7. Remove language+script
        debug_assert!(!ro.language().is_empty());
        ro.set_script(None);
        ro.set_language(Language::UND);
    }

    fn step_region(&mut self) {
        todo!()
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
}
