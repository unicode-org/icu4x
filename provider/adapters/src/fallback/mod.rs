// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale fallbacking in data provider.

use icu_locid::extensions::unicode::{Key, Value};
use icu_locid::subtags::Variants;
use icu_provider::prelude::*;

mod algorithms;

pub mod provider;

use provider::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocaleFallbackStrategy {
    LanguagePriority,
    RegionPriority,
}

#[non_exhaustive]
pub struct LocaleFallbackKeyMetadata {
    pub strategy: LocaleFallbackStrategy,
    pub extension_kw: Option<Key>,
}

pub struct LocaleFallbacker {
    likely_subtags: DataPayload<LocaleFallbackLikelySubtagsV1Marker>,
    parents: DataPayload<LocaleFallbackParentsV1Marker>,
}

pub struct LocaleFallbackerForKey<'a> {
    likely_subtags: &'a LocaleFallbackLikelySubtagsV1<'a>,
    parents: &'a LocaleFallbackParentsV1<'a>,
    key_metadata: LocaleFallbackKeyMetadata,
}

pub struct LocaleFallbackIterator<'a, 'b> {
    likely_subtags: &'a LocaleFallbackLikelySubtagsV1<'a>,
    parents: &'a LocaleFallbackParentsV1<'a>,
    key_metadata: &'b LocaleFallbackKeyMetadata,
    current: ResourceOptions,
    backup_extension: Option<Value>,
    backup_subdivision: Option<Value>,
    backup_variants: Option<Variants>,
}

impl LocaleFallbacker {
    pub fn try_new<P>(provider: &P) -> Result<Self, DataError>
    where
        P: ResourceProvider<LocaleFallbackLikelySubtagsV1Marker>
            + ResourceProvider<LocaleFallbackParentsV1Marker>
            + ?Sized,
    {
        let likely_subtags = provider
            .load_resource(&Default::default())?
            .take_payload()?;
        let parents = provider
            .load_resource(&Default::default())?
            .take_payload()?;
        Ok(LocaleFallbacker {
            likely_subtags,
            parents,
        })
    }

    pub fn new_without_data() -> Self {
        LocaleFallbacker {
            likely_subtags: DataPayload::from_owned(Default::default()),
            parents: DataPayload::from_owned(Default::default()),
        }
    }

    pub fn for_key_metadata(
        &self,
        key_metadata: LocaleFallbackKeyMetadata,
    ) -> LocaleFallbackerForKey {
        LocaleFallbackerForKey {
            likely_subtags: self.likely_subtags.get(),
            parents: self.parents.get(),
            key_metadata,
        }
    }
}

impl<'a> LocaleFallbackerForKey<'a> {
    pub fn fallback_for<'b>(&'b self, mut ro: ResourceOptions) -> LocaleFallbackIterator<'a, 'b> {
        self.normalize(&mut ro);
        LocaleFallbackIterator {
            likely_subtags: self.likely_subtags,
            parents: self.parents,
            key_metadata: &self.key_metadata,
            current: ro,
            backup_extension: None,
            backup_subdivision: None,
            backup_variants: None,
        }
    }
}

impl<'a, 'b> LocaleFallbackIterator<'a, 'b> {
    pub fn get(&self) -> &ResourceOptions {
        &self.current
    }
}
