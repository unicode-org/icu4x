// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CldrPaths;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use std::convert::TryFrom;
use std::sync::RwLock;

pub trait ResourceKeySupport {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError>;
}

#[derive(Debug)]
pub struct LazyCldrProvider<T> {
    src: RwLock<Option<T>>,
}

impl<T> Default for LazyCldrProvider<T> {
    fn default() -> Self {
        Self {
            src: RwLock::new(None),
        }
    }
}

/// A lazy-initialized CLDR JSON data provider.
impl<'b, T> LazyCldrProvider<T>
where
    T: DataProvider<SerializeMarker>
        + IterableDataProviderCore
        + KeyedDataProvider
        + TryFrom<&'b dyn CldrPaths, Error = crate::error::Error>,
{
    /// Call [`DataProvider::load_payload()`], initializing `T` if necessary.
    pub fn try_load_serde(
        &self,
        req: &DataRequest,
        cldr_paths: &'b dyn CldrPaths,
    ) -> Result<Option<DataResponse<SerializeMarker>>, DataError> {
        if T::supports_key(&req.resource_path.key).is_err() {
            return Ok(None);
        }
        if let Some(data_provider) = self.src.read()?.as_ref() {
            return DataProvider::load_payload(data_provider, req).map(Some);
        }
        let mut src = self.src.write()?;
        if src.is_none() {
            src.replace(T::try_from(cldr_paths)?);
        }
        let data_provider = src
            .as_ref()
            .expect("The RwLock must be populated at this point.");
        DataProvider::load_payload(data_provider, req).map(Some)
    }

    /// Call [`IterableDataProviderCore::supported_options_for_key()`], initializing `T` if necessary.
    pub fn try_supported_options(
        &self,
        resc_key: &ResourceKey,
        cldr_paths: &'b dyn CldrPaths,
    ) -> Result<Option<Vec<ResourceOptions>>, DataError> {
        if T::supports_key(resc_key).is_err() {
            return Ok(None);
        }
        if let Some(data_provider) = self.src.read()?.as_ref() {
            return data_provider
                .supported_options_for_key(resc_key)
                .map(|i| i.collect())
                .map(Some);
        }
        let mut src = self.src.write()?;
        if src.is_none() {
            src.replace(T::try_from(cldr_paths)?);
        }
        let data_provider = src
            .as_ref()
            .expect("The RwLock must be populated at this point.");
        data_provider
            .supported_options_for_key(resc_key)
            .map(|i| i.collect())
            .map(Some)
    }
}
