// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CldrPaths;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use std::convert::TryFrom;
use std::sync::RwLock;

/// A [`DataProvider`] whose supported keys are known statically at compile time.
///
/// Implementing this trait means that a [`DataProvider`] is built to support a specific set of
/// keys; for example, by transforming those keys from an external data source.
///
/// TODO(#442): Think about a better way to do this. This is not fully supported.
/// TODO: When const_trait_impl is stable, most implementations of this trait should be const.
pub trait KeyedDataProvider {
    fn supported_keys() -> Vec<ResourceKey>;
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
    T: DynProvider<SerializeMarker>
        + IterableProvider
        + KeyedDataProvider
        + TryFrom<&'b dyn CldrPaths, Error = crate::error::Error>,
{
    /// Call [`DataProvider::load_payload()`], initializing `T` if necessary.
    pub fn try_load_serde(
        &self,
        key: ResourceKey,
        req: &DataRequest,
        cldr_paths: &'b dyn CldrPaths,
    ) -> Result<Option<DataResponse<SerializeMarker>>, DataError> {
        if !T::supported_keys().contains(&key) {
            return Ok(None);
        }
        if self.src.read()?.is_none() {
            let mut src = self.src.write()?;
            if src.is_none() {
                src.replace(T::try_from(cldr_paths)?);
            }
        }
        DynProvider::load_payload(
            self.src
                .read()?
                .as_ref()
                .expect("The RwLock must be populated at this point."),
            key,
            req,
        )
        .map(Some)
    }

    /// Call [`IterableProvider::supported_options_for_key()`], initializing `T` if necessary.
    pub fn try_supported_options(
        &self,
        key: &ResourceKey,
        cldr_paths: &'b dyn CldrPaths,
    ) -> Result<Option<Vec<ResourceOptions>>, DataError> {
        if !T::supported_keys().contains(key) {
            return Ok(None);
        }
        if self.src.read()?.is_none() {
            let mut src = self.src.write()?;
            if src.is_none() {
                src.replace(T::try_from(cldr_paths)?);
            }
        }
        self.src
            .read()?
            .as_ref()
            .expect("The RwLock must be populated at this point.")
            .supported_options_for_key(key)
            .map(|i| i.collect())
            .map(Some)
    }
}
