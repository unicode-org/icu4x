// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use crate::ResourceKey;
use alloc::boxed::Box;
use core::fmt;

/// A trait that allows for converting between data payloads of different types.
///
/// These payloads will typically be some kind of erased payload, either with
/// AnyMarker, BufferMarker, or SerializeMarker, where converting requires reifying the type.
/// A type implementing [`DataConverter`] will essentially have a "registry" mapping keys to
/// concrete marker types M, and reifying the input to a `DataPayload<M>`, performing some conversion
/// or computation, and erasing the result to `DataPayload<MTo>`.
///
/// It will typically be implemented on data providers used in datagen.
///
/// The `impl_dyn_provider` is able to automatically implement this trait.
pub trait DataConverter<MFrom: DataMarker, MTo: DataMarker> {
    fn convert(
        &self,
        key: ResourceKey,
        from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, ReturnedPayloadError<MFrom>>;
}

impl<MFrom, MTo, P> DataConverter<MFrom, MTo> for Box<P>
where
    MFrom: DataMarker,
    MTo: DataMarker,
    P: DataConverter<MFrom, MTo> + ?Sized,
{
    fn convert(
        &self,
        key: ResourceKey,
        from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, ReturnedPayloadError<MFrom>> {
        (**self).convert(key, from)
    }
}

pub struct ReturnedPayloadError<M: DataMarker>(pub DataPayload<M>, pub DataError);

impl<M: DataMarker> From<ReturnedPayloadError<M>> for DataError {
    fn from(other: ReturnedPayloadError<M>) -> Self {
        other.1
    }
}

impl<M: DataMarker> fmt::Debug for ReturnedPayloadError<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.1.fmt(f)
    }
}

impl<M: DataMarker> fmt::Display for ReturnedPayloadError<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.1.fmt(f)
    }
}

#[cfg(feature = "std")]
impl<M: DataMarker> std::error::Error for ReturnedPayloadError<M> {}
