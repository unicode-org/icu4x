// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale fallbacking in data provider.

use icu_provider::prelude::*;
use icu_locid::extensions::unicode::Key;
use icu_locid::subtags::{Region, Script};
use icu_locid::{LanguageIdentifier, Locale};
use yoke::Yokeable;
use zerofrom::ZeroFrom;
use zerovec::ZeroMap;
use zerovec::ZeroMap2d;

#[derive(Yokeable, ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct LocaleFallbackDataV1<'data> {
    pub l2s: ZeroMap<'data, [u8; 3], Script>,
    pub lr2s: ZeroMap2d<'data, [u8; 3], [u8; 3], Script>,
    pub l2r: ZeroMap<'data, [u8; 3], Region>,
    pub ls2r: ZeroMap2d<'data, [u8; 3], [u8; 4], Region>,
    pub parents: ZeroMap<'data, [u8], [u8]>,
}

pub struct LocaleFallbackDataV1Marker;

impl DataMarker for LocaleFallbackDataV1Marker {
    type Yokeable = LocaleFallbackDataV1<'static>;
}

pub enum LocaleFallbackStrategy {
    LanguagePriority,
    RegionPriority,
}

pub struct LocaleFallbacker {
    pub metadata: DataPayload<LocaleFallbackDataV1Marker>,
    pub strategy: LocaleFallbackStrategy,
    pub extension_kw: Key,
}

impl LocaleFallbacker {
    pub fn normalize(&self, loc: &mut Locale) -> Result<(), DataError> {
        let lang_raw = loc.id.language.into_raw();
        // 1. Populate the region
        if loc.id.region.is_none() {
            // 1a. First look for region based on language+script
            if let Some(script) = loc.id.script {
                loc.id.region = self
                    .metadata
                    .get()
                    .ls2r
                    .get(&lang_raw, &script.into_raw())
                    .ok()
                    .copied();
            }
            // 1b. If that fails, try language only
            if loc.id.region.is_none() {
                loc.id.region = self.metadata.get().l2r.get(&lang_raw).copied();
            }
        }
        // 2. Check if this is a multi-script language
        let maybe_default_script = self.metadata.get().l2s.get(&lang_raw).copied();
        // 3. Populate the script iff multi-script language
        if let Some(default_script) = maybe_default_script {
            if loc.id.script.is_none() {
                // 3a. First look for the script based on language+region
                if let Some(region) = loc.id.region {
                    loc.id.script = self
                        .metadata
                        .get()
                        .lr2s
                        .get(&lang_raw, &region.into_raw())
                        .ok()
                        .copied();
                }
                // 3b. If that fails, use the default script loaded above
                if loc.id.script.is_none() {
                    loc.id.script = Some(default_script);
                }
            }
            debug_assert!(loc.id.script.is_some());
        } else {
            // 3c. If single-script language, clear out the script
            loc.id.script = None;
        }
        // TODO: Handle extension keywords.
        Ok(())
    }

    pub fn step(&self, loc: &mut Locale, original: &LanguageIdentifier) {
        match self.strategy {
            LocaleFallbackStrategy::LanguagePriority => self.step_language(loc),
            LocaleFallbackStrategy::RegionPriority => self.step_region(loc, original),
        }
    }

    fn step_language(&self, loc: &mut Locale) {
        // 1. TODO: Handle -u-sd
        // 2. Check for parent locale
        if let Some(parent) = self
            .metadata
            .get()
            .parents
            .get_by(|bytes| loc.id.cmp_bytes(bytes).reverse())
        {
            // TODO: Avoid the parsing step
            loc.id = LanguageIdentifier::from_bytes(parent).unwrap();
            return;
        }
        // 3. Remove variants
        if !loc.id.variants.is_empty() {
            loc.id.variants.clear();
            return;
        }
        // 4. Remove region
        if loc.id.region.is_some() {
            loc.id.region = None;
            return;
        }
        // 5. Remove language+script
        debug_assert!(!loc.id.language.is_empty());
        loc.id.script = None;
        loc.id.language.clear();
    }

    fn step_region(&self, _loc: &mut Locale, _original: &LanguageIdentifier) {
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
    fn load_fallback_for_key(&self, key: ResourceKey) -> Result<LocaleFallbacker, DataError>;
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
