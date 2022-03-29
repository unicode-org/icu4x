// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Helpers for switching between multiple providers.

use icu_provider::prelude::*;

#[cfg(feature = "datagen")]
use alloc::boxed::Box;
#[cfg(feature = "datagen")]
use icu_provider::datagen;

/// A provider that is one of two types determined at runtime.
///
/// Data provider traits implemented by both `P0` and `P1` are implemented on
/// `EitherProvider<P0, P1>`.
pub enum EitherProvider<P0, P1> {
    A(P0),
    B(P1),
}

impl<P0: AnyProvider, P1: AnyProvider> AnyProvider for EitherProvider<P0, P1> {
    #[inline]
    fn load_any(&self, key: ResourceKey, req: &DataRequest) -> Result<AnyResponse, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.load_any(key, req),
            B(p) => p.load_any(key, req),
        }
    }
}

impl<P0: BufferProvider, P1: BufferProvider> BufferProvider for EitherProvider<P0, P1> {
    #[inline]
    fn load_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.load_buffer(key, req),
            B(p) => p.load_buffer(key, req),
        }
    }
}

impl<M: DataMarker, P0: DynProvider<M>, P1: DynProvider<M>> DynProvider<M>
    for EitherProvider<P0, P1>
{
    #[inline]
    fn load_payload(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.load_payload(key, req),
            B(p) => p.load_payload(key, req),
        }
    }
}

impl<M: ResourceMarker, P0: ResourceProvider<M>, P1: ResourceProvider<M>> ResourceProvider<M>
    for EitherProvider<P0, P1>
{
    #[inline]
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.load_resource(req),
            B(p) => p.load_resource(req),
        }
    }
}

#[cfg(feature = "datagen")]
impl<M: DataMarker, P0: datagen::IterableDynProvider<M>, P1: datagen::IterableDynProvider<M>>
    datagen::IterableDynProvider<M> for EitherProvider<P0, P1>
{
    #[inline]
    fn supported_options_for_key(
        &self,
        key: ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.supported_options_for_key(key),
            B(p) => p.supported_options_for_key(key),
        }
    }
}

#[cfg(feature = "datagen")]
impl<
        M: ResourceMarker,
        P0: datagen::IterableResourceProvider<M>,
        P1: datagen::IterableResourceProvider<M>,
    > datagen::IterableResourceProvider<M> for EitherProvider<P0, P1>
{
    #[inline]
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        use EitherProvider::*;
        match self {
            A(p) => p.supported_options(),
            B(p) => p.supported_options(),
        }
    }
}
