// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of iteration APIs for data providers.

use crate::prelude::*;
use alloc::boxed::Box;

/// A [`DynProvider`] that can iterate over all supported [`ResourceOptions`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableDynProvider<M: DataMarker>: DynProvider<M> {
    /// Given a [`ResourceKey`], returns a boxed iterator over [`ResourceOptions`].
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError>;
}

/// A [`ResourceProvider`] that can iterate over all supported [`ResourceOptions`] for a certain key.
///
/// Implementing this trait means that a data provider knows all of the data it can successfully
/// return from a load request.
pub trait IterableResourceProvider<M: ResourceMarker>: ResourceProvider<M> {
    /// Returns a boxed iterator over [`ResourceOptions`].
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError>;
}
