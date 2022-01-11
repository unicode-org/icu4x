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
    /// Given a [`ResourceKey`], checks whether this type of [`DataProvider`] supports it.
    ///
    /// Returns Ok if the key is supported, or an Error with more information if not.
    /// The Error should be [`MissingResourceKey`].
    ///
    /// [`MissingResourceKey`]: crate::error::DataErrorKind::MissingResourceKey
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError>;

    /// Auto-implemented function that enables chaining of [`KeyedDataProviders`] while preserving
    /// [`MissingResourceKey`].
    ///
    /// [`KeyedDataProviders`]: KeyedDataProvider
    /// [`MissingResourceKey`]: crate::error::DataErrorKind::MissingResourceKey
    ///
    /// # Examples
    ///
    /// ```ignore
    /// DataProviderA::supports_key(resc_key)
    ///     .or_else(|err| DataProviderB::or_else_supports_key(err, resc_key))
    ///     .or_else(|err| DataProviderC::or_else_supports_key(err, resc_key))
    /// ```
    fn or_else_supports_key(err: DataError, resc_key: &ResourceKey) -> Result<(), DataError> {
        match Self::supports_key(resc_key) {
            Ok(()) => Ok(()),
            Err(new_err) => {
                if let DataErrorKind::MissingResourceKey = err.kind {
                    Err(err)
                } else {
                    Err(new_err)
                }
            }
        }
    }
}

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
        + IterableProvider
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

    /// Call [`IterableProvider::supported_options_for_key()`], initializing `T` if necessary.
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
