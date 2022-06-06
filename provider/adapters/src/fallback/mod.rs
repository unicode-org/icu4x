// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale fallbacking in data provider.

use icu_locid::extensions::unicode::{Key, Value};
use icu_locid::subtags::{Language, Region, Script};
use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;
use yoke::Yokeable;
use zerofrom::ZeroFrom;
use zerovec::ZeroMap;
use zerovec::ZeroMap2d;

const REGION_KEY: Key = icu_locid::unicode_ext_key!("rg");
const SUBDIVISION_KEY: Key = icu_locid::unicode_ext_key!("sd");

#[derive(Yokeable, ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct LocaleFallbackDataV1<'data> {
    pub l2s: ZeroMap<'data, [u8; 3], Script>,
    pub lr2s: ZeroMap2d<'data, [u8; 3], [u8; 3], Script>,
    pub l2r: ZeroMap<'data, [u8; 3], Region>,
    pub ls2r: ZeroMap2d<'data, [u8; 3], [u8; 4], Region>,
    pub parents: ZeroMap<'data, [u8], (Language, Option<Script>, Option<Region>)>,
}

pub struct LocaleFallbackDataV1Marker;

impl DataMarker for LocaleFallbackDataV1Marker {
    type Yokeable = LocaleFallbackDataV1<'static>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocaleFallbackStrategy {
    LanguagePriority,
    RegionPriority,
}

pub struct LocaleFallbackerForKey {
    pub metadata: DataPayload<LocaleFallbackDataV1Marker>,
    pub strategy: LocaleFallbackStrategy,
    pub extension_kw: Key,
}

impl LocaleFallbackerForKey {
    pub fn fallback_for(&self, mut ro: ResourceOptions) -> LocaleFallbackIterator {
        self.normalize_options(&mut ro);
        LocaleFallbackIterator {
            metadata: self.metadata.get(),
            strategy: self.strategy,
            extension_kw: self.extension_kw,
            current: ro,
            backup_extension: None,
            backup_subdivision: None,
        }
    }

    pub fn normalize_options(&self, ro: &mut ResourceOptions) {
        let lang_raw = ro.language().into_raw();
        // 1. Populate the region
        if ro.region().is_none() {
            // 1a. First look for region based on language+script
            if let Some(script) = ro.script() {
                ro.set_region(
                    self.metadata
                        .get()
                        .ls2r
                        .get(&lang_raw, &script.into_raw())
                        .ok()
                        .copied(),
                );
            }
            // 1b. If that fails, try language only
            if ro.region().is_none() {
                ro.set_region(self.metadata.get().l2r.get(&lang_raw).copied());
            }
        }
        // 2. Check if this is a multi-script language
        let maybe_default_script = self.metadata.get().l2s.get(&lang_raw).copied();
        // 3. Populate the script iff multi-script language
        if let Some(default_script) = maybe_default_script {
            if ro.script().is_none() {
                // 3a. First look for the script based on language+region
                if let Some(region) = ro.region() {
                    ro.set_script(
                        self.metadata
                            .get()
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
                REGION_KEY => self.strategy == LocaleFallbackStrategy::RegionPriority,
                // Retain the query-specific keyword
                _ if *key == self.extension_kw => true,
                // Drop all others
                _ => false,
            }
        });
        // 5. If there is an invalid "sd" subtag, drop it
        // For now, ignore it, and let fallback do it for us
    }
}

pub struct LocaleFallbackIterator<'a> {
    metadata: &'a LocaleFallbackDataV1<'a>,
    strategy: LocaleFallbackStrategy,
    extension_kw: Key,
    current: ResourceOptions,
    backup_extension: Option<Value>,
    backup_subdivision: Option<Value>,
}

impl<'a> LocaleFallbackIterator<'a> {
    pub fn get(&self) -> &ResourceOptions {
        &self.current
    }

    pub fn step(&mut self) {
        match self.strategy {
            LocaleFallbackStrategy::LanguagePriority => self.step_language(),
            LocaleFallbackStrategy::RegionPriority => self.step_region(),
        }
    }

    fn step_language(&mut self) {
        let ro = &mut self.current;
        // 1. Remove the extension fallback keyword
        if let Some(value) = ro.remove_unicode_ext(&self.extension_kw) {
            self.backup_extension = Some(value);
            return;
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
            .metadata
            .parents
            .get_copied_by(|bytes| ro.cmp_bytes(bytes).reverse())
        {
            let lid = LanguageIdentifier::from(parent);
            ro.set_langid(lid);
            if let Some(value) = self.backup_extension.take() {
                ro.set_unicode_ext(self.extension_kw, value);
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

#[test]
fn test_normalize() {
    fn check(input: &str, expected: &str) {
        let mut loc: Locale = input.parse().expect("Valid input locale");
        // TODO: Finish this test
    }
}

pub trait LocaleFallbackProvider {
    fn load_fallback_for_key(&self, key: ResourceKey) -> Result<LocaleFallbackerForKey, DataError>;
}

pub struct LocaleFallbackAdapter<P>(pub P);

impl<P> BufferProvider for LocaleFallbackAdapter<P>
where
    P: LocaleFallbackProvider + BufferProvider,
{
    fn load_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let data = self.0.load_fallback_for_key(key)?;
        todo!()
    }
}
