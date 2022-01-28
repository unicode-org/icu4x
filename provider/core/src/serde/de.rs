// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::Error;
use crate::buf::BufferFormat;
use crate::buf::BufferProvider;
use crate::prelude::*;
use core::marker::PhantomData;
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;

/// Returns an error if the buffer format is not enabled.
pub fn check_format_supported(buffer_format: BufferFormat) -> Result<(), Error> {
    match buffer_format {
        #[cfg(feature = "deserialize_json")]
        BufferFormat::Json => Ok(()),

        #[cfg(feature = "deserialize_bincode_1")]
        BufferFormat::Bincode1 => Ok(()),

        #[cfg(feature = "deserialize_postcard_07")]
        BufferFormat::Postcard07 => Ok(()),

        // Allowed for cases in which all features are enabled
        #[allow(unreachable_patterns)]
        _ => Err(Error::UnavailableFormat(buffer_format)),
    }
}

/// A [`BufferProvider`] that deserializes its data using Serde.
pub struct DeserializingBufferProvider<'a, P: ?Sized>(&'a P);

/// Auto-implemented for all [`BufferProvider`] for easy wrapping in [`DeserializingBufferProvider`].
pub trait AsDeserializingBufferProvider {
    fn as_deserializing(&self) -> DeserializingBufferProvider<Self>;
}

impl<P> AsDeserializingBufferProvider for P
where
    P: BufferProvider + ?Sized,
{
    /// Wrap this [`BufferProvider`] in a [`DeserializingBufferProvider`].
    fn as_deserializing(&self) -> DeserializingBufferProvider<Self> {
        DeserializingBufferProvider(self)
    }
}

fn deserialize_impl<'data, M>(
    // Allow `bytes` to be unused in case all buffer formats are disabled
    #[allow(unused_variables)] bytes: &'data [u8],
    buffer_format: BufferFormat,
    _: PhantomData<&'data ()>,
) -> Result<<M::Yokeable as Yokeable<'data>>::Output, Error>
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    match buffer_format {
        #[cfg(feature = "deserialize_json")]
        BufferFormat::Json => {
            let mut d = serde_json::Deserializer::from_slice(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        }

        #[cfg(feature = "deserialize_bincode_1")]
        BufferFormat::Bincode1 => {
            use bincode::Options;
            let options = bincode::DefaultOptions::new()
                .with_fixint_encoding()
                .allow_trailing_bytes();
            let mut d = bincode::de::Deserializer::from_slice(bytes, options);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        }

        #[cfg(feature = "deserialize_postcard_07")]
        BufferFormat::Postcard07 => {
            let mut d = postcard::Deserializer::from_bytes(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        }

        // Allowed for cases in which all features are enabled
        #[allow(unreachable_patterns)]
        _ => Err(Error::UnavailableFormat(buffer_format)),
    }
}

impl DataPayload<BufferMarker> {
    pub fn into_deserialized<M>(self, buffer_format: BufferFormat) -> Result<DataPayload<M>, Error>
    where
        M: DataMarker,
        // Actual bound:
        //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
        // Necessary workaround bound (see `yoke::trait_hack` docs):
        for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
    {
        self.try_map_project_with_capture(buffer_format, deserialize_impl::<M>)
    }
}

impl<P, M> ResourceProvider<M> for DeserializingBufferProvider<'_, P>
where
    M: ResourceMarker,
    P: BufferProvider + ?Sized,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    /// Converts a buffer into a concrete type by deserializing from a supported buffer format.
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        self.load_payload(M::KEY, req)
    }
}

impl<P, M> DynProvider<M> for DeserializingBufferProvider<'_, P>
where
    M: DataMarker,
    P: BufferProvider + ?Sized,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    fn load_payload(&self, key: ResourceKey, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let old_response = BufferProvider::load_buffer(self.0, key, req)?;
        if let Some(old_payload) = old_response.payload {
            let buffer_format = old_response
                .metadata
                .buffer_format
                .ok_or(Error::FormatNotSpecified)?;
            let new_payload = old_payload.into_deserialized(buffer_format)?;
            Ok(DataResponse {
                metadata: old_response.metadata,
                payload: Some(new_payload),
            })
        } else {
            Ok(DataResponse {
                metadata: old_response.metadata,
                payload: None,
            })
        }
    }
}
