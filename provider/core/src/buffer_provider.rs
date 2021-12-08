// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::prelude::*;

pub struct BufferMarker {}

impl DataMarker for BufferMarker {
    type Yokeable = &'static [u8];
}

pub trait BufferProvider {
    fn load_buffer(&self, req: DataRequest) -> Result<DataResponse<BufferMarker>, Error>;

    #[cfg(feature = "serde")]
    fn into_serde_provider(self) -> SerdeBufferProvider<Self>
    where
        Self: Sized,
    {
        SerdeBufferProvider(self)
    }

    #[cfg(feature = "serde")]
    fn as_serde_provider(&self) -> SerdeBufferProvider<&Self>
    where
        Self: Sized,
    {
        SerdeBufferProvider(self)
    }
}

impl dyn BufferProvider {
    #[cfg(feature = "serde")]
    pub fn as_dyn_serde_provider(&self) -> SerdeBufferProvider<&Self>
    {
        SerdeBufferProvider(self)
    }
}

impl<P> BufferProvider for &P
where
    P: BufferProvider + ?Sized,
{
    fn load_buffer(&self, req: DataRequest) -> Result<DataResponse<BufferMarker>, Error> {
        P::load_buffer(*self, req)
    }
}

#[cfg(feature = "serde")]
pub struct SerdeBufferProvider<P: BufferProvider + ?Sized>(pub P);
