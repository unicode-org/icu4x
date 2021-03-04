// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for `DataProvider`.

use crate::error::Error;
use crate::prelude::*;
use std::fmt::Debug;

/// A provider that can iterate over all supported `ResourceOptions` for a certain key.
///
/// Implementing this trait means that a DataProvider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDataProvider<'d> {
    /// Given a `ResourceKey`, returns a boxed iterator over `ResourceOptions`.
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, Error>;
}

/// Super-trait combining DataProvider and IterableDataProvider, auto-implemented
/// for all types implementing both of those traits.
pub trait IterableTypedDataProvider<'d, T>: IterableDataProvider<'d> + DataProvider<'d, T>
where
    T: Clone + Debug + 'd,
{
}

impl<'d, S, T> IterableTypedDataProvider<'d, T> for S
where
    S: IterableDataProvider<'d> + DataProvider<'d, T>,
    T: Clone + Debug + 'd,
{
}

/// Super-trait combining ErasedDataProvider and IterableDataProvider, auto-implemented
/// for all types implementing both of those traits.
#[cfg(feature = "erased")]
pub trait IterableErasedDataProvider<'d>:
    IterableDataProvider<'d> + crate::erased::ErasedDataProvider<'d>
{
}

#[cfg(feature = "erased")]
impl<'d, S> IterableErasedDataProvider<'d> for S where
    S: IterableDataProvider<'d> + crate::erased::ErasedDataProvider<'d>
{
}

/// A DataProvider whose supported keys are known statically at compile time.
///
/// Implementing this trait means that a DataProvider is built to support a specific set of
/// keys; for example, by transforming those keys from an external data source.
///
/// TODO: When const_trait_impl is stable, most implementations of this trait should be const.
pub trait KeyedDataProvider {
    /// Given a `ResourceKey`, checks whether this type of DataProvider supports it.
    ///
    /// Returns Ok if the key is supported, or an Error with more information if not. The Error
    /// should be either UnsupportedCategory or UnsupportedResourceKey.
    fn supports_key(resc_key: &ResourceKey) -> Result<(), Error>;

    /// Auto-implemented function that enables chaining of KeyedDataProviders while preserving
    /// UnsupportedResourceKey.
    ///
    /// # Example
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
                if let Error::UnsupportedResourceKey(_) = err {
                    Err(err)
                } else {
                    Err(new_err)
                }
            }
        }
    }
}

/// An object capable of storing/persisting data payloads to be read by a DataProvider.
///
/// A DataProvider by itself is "read-only"; this trait enables it to be "read-write".
#[cfg(feature = "erased")]
pub trait DataExporter {
    /// Save a `payload` corresponding to the given data request (resource path).
    fn put_payload(
        &mut self,
        req: &DataRequest,
        payload: &dyn crate::erased::ErasedDataStruct,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Whether to load and dump data for the given entry. This function enables the
    /// `DataExporter` to filter out certain data entries.
    fn include_resource_options(&self, resc_options: &ResourceOptions) -> bool;

    /// Auto-implemented function that loads data from an `IterableDataProvider` and dumps it
    /// into this `DataExporter`.
    fn put_key_from_provider<'a, 'd>(
        &mut self,
        resc_key: &ResourceKey,
        provider: &impl IterableErasedDataProvider<'d>,
    ) -> Result<(), Error> {
        for resc_options in provider.supported_options_for_key(resc_key)? {
            if !self.include_resource_options(&resc_options) {
                continue;
            }
            let req = DataRequest {
                resource_path: ResourcePath {
                    key: *resc_key,
                    options: resc_options,
                },
            };
            let mut receiver =
                crate::erased::DataReceiver::<dyn crate::erased::ErasedDataStruct>::new();
            provider.load_to_receiver(&req, &mut receiver)?;
            self.put_payload(&req, receiver.borrow_payload()?)?;
        }
        Ok(())
    }
}
