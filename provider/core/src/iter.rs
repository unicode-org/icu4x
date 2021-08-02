// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for [`DataProvider`].

use crate::error::Error;
use crate::prelude::*;
use alloc::boxed::Box;

/// A provider that can iterate over all supported [`ResourceOptions`] for a certain key.
///
/// Implementing this trait means that a [`DataProvider`] knows all of the data it can successfully
/// return from a load request.
pub trait IterableDataProviderCore {
    /// Given a [`ResourceKey`], returns a boxed iterator over [`ResourceOptions`].
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, Error>;
}

/// A super-trait combining [`DataProvider`] and [`IterableDataProviderCore`], auto-implemented
/// for all types implementing both of those traits.
pub trait IterableDataProvider<'data, M>:
    IterableDataProviderCore + DataProvider<'data, M>
where
    M: DataMarker<'data>,
{
}

impl<'data, S, M> IterableDataProvider<'data, M> for S
where
    S: IterableDataProviderCore + DataProvider<'data, M>,
    M: DataMarker<'data>,
{
}

/// A [`DataProvider`] whose supported keys are known statically at compile time.
///
/// Implementing this trait means that a [`DataProvider`] is built to support a specific set of
/// keys; for example, by transforming those keys from an external data source.
///
/// TODO: When const_trait_impl is stable, most implementations of this trait should be const.
pub trait KeyedDataProvider {
    /// Given a [`ResourceKey`], checks whether this type of [`DataProvider`] supports it.
    ///
    /// Returns Ok if the key is supported, or an Error with more information if not. The Error
    /// should be [`MissingResourceKey`].
    ///
    /// [`MissingResourceKey`]: crate::error::Error::MissingResourceKey
    fn supports_key(resc_key: &ResourceKey) -> Result<(), Error>;

    /// Auto-implemented function that enables chaining of [`KeyedDataProviders`] while preserving
    /// [`MissingResourceKey`].
    ///
    /// [`KeyedDataProviders`]: KeyedDataProvider
    /// [`MissingResourceKey`]: crate::error::Error::MissingResourceKey
    ///
    /// # Examples
    ///
    /// ```ignore
    /// DataProviderA::supports_key(resc_key)
    ///     .or_else(|err| DataProviderB::or_else_supports_key(err, resc_key))
    ///     .or_else(|err| DataProviderC::or_else_supports_key(err, resc_key))
    /// ```
    fn or_else_supports_key(err: Error, resc_key: &ResourceKey) -> Result<(), Error> {
        match Self::supports_key(resc_key) {
            Ok(()) => Ok(()),
            Err(new_err) => {
                if let Error::MissingResourceKey(_) = err {
                    Err(err)
                } else {
                    Err(new_err)
                }
            }
        }
    }
}
