// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale fallbacking in data provider.

use icu_locid::extensions::unicode::{Key, Value};
use icu_provider::prelude::*;

mod adapter;
mod algorithms;
mod provider;

pub use adapter::LocaleFallbackAdapter;
pub use adapter::LocaleFallbackProvider;
pub use provider::LocaleFallbackDataV1;
pub use provider::LocaleFallbackDataV1Marker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocaleFallbackStrategy {
    LanguagePriority,
    RegionPriority,
}

#[non_exhaustive]
pub struct LocaleFallbackKeyMetadata {
    pub strategy: LocaleFallbackStrategy,
    pub extension_kw: Key,
}

pub struct LocaleFallbacker {
    fallback_data: DataPayload<LocaleFallbackDataV1Marker>,
}

pub struct LocaleFallbackerForKey<'a> {
    fallback_data: &'a LocaleFallbackDataV1<'a>,
    key_metadata: LocaleFallbackKeyMetadata,
}

pub struct LocaleFallbackIterator<'a, 'b> {
    fallback_data: &'b LocaleFallbackDataV1<'a>,
    key_metadata: &'b LocaleFallbackKeyMetadata,
    current: ResourceOptions,
    backup_extension: Option<Value>,
    backup_subdivision: Option<Value>,
}

impl LocaleFallbacker {
    pub fn try_new<P>(provider: &P) -> Result<Self, DataError>
    where
        P: ResourceProvider<LocaleFallbackDataV1Marker> + ?Sized,
    {
        let fallback_data = provider
            .load_resource(&Default::default())?
            .take_payload()?;
        Ok(LocaleFallbacker { fallback_data })
    }

    pub fn for_key_metadata(
        &self,
        key_metadata: LocaleFallbackKeyMetadata,
    ) -> LocaleFallbackerForKey {
        LocaleFallbackerForKey {
            fallback_data: self.fallback_data.get(),
            key_metadata,
        }
    }
}

impl<'a> LocaleFallbackerForKey<'a> {
    pub fn fallback_for<'b>(&'b self, mut ro: ResourceOptions) -> LocaleFallbackIterator<'a, 'b> {
        self.normalize(&mut ro);
        LocaleFallbackIterator {
            fallback_data: &self.fallback_data,
            key_metadata: &self.key_metadata,
            current: ro,
            backup_extension: None,
            backup_subdivision: None,
        }
    }
}

impl<'a, 'b> LocaleFallbackIterator<'a, 'b> {
    pub fn get(&self) -> &ResourceOptions {
        &self.current
    }
}
