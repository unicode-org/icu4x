// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CldrPaths;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use icu_provider::serde::SerdeSeDataStructMarker;
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

fn map_poison<E>(_err: E) -> DataError {
    // Can't return the Poison directly because it has lifetime parameters.
    DataError::new_resc_error(crate::error::Error::Poison)
}

/// A lazy-initialized CLDR JSON data provider.
impl<'b, 'd, 's: 'd, T> LazyCldrProvider<T>
where
    T: DataProvider<'d, 's, SerdeSeDataStructMarker>
        + IterableDataProviderCore
        + KeyedDataProvider
        + TryFrom<&'b dyn CldrPaths>,
    <T as TryFrom<&'b dyn CldrPaths>>::Error: 'static + std::error::Error + Send + Sync,
{
    /// Call [`DataProvider::load_payload()`], initializing `T` if necessary.
    pub fn try_load_serde(
        &self,
        req: &DataRequest,
        cldr_paths: &'b dyn CldrPaths,
    ) -> Result<Option<DataResponse<'d, 's, SerdeSeDataStructMarker>>, DataError> {
        if T::supports_key(&req.resource_path.key).is_err() {
            return Ok(None);
        }
        if let Some(data_provider) = self.src.read().map_err(map_poison)?.as_ref() {
            return DataProvider::load_payload(data_provider, req).map(Some);
        }
        let mut src = self.src.write().map_err(map_poison)?;
        if src.is_none() {
            src.replace(T::try_from(cldr_paths).map_err(DataError::new_resc_error)?);
        }
        let data_provider = src
            .as_ref()
            .expect("The RwLock must be populated at this point.");
        return DataProvider::load_payload(data_provider, req).map(Some);
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
        if let Some(data_provider) = self.src.read().map_err(map_poison)?.as_ref() {
            return data_provider
                .supported_options_for_key(resc_key)
                .map(|i| i.collect())
                .map(Some);
        }
        let mut src = self.src.write().map_err(map_poison)?;
        if src.is_none() {
            src.replace(T::try_from(cldr_paths).map_err(DataError::new_resc_error)?);
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
