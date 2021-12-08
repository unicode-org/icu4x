// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::BufferFormat;
use super::Error;
use crate::buffer_provider::BufferProvider;
use crate::prelude::*;
use core::marker::PhantomData;
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;

/// A [`BufferProvider`] that deserializes its data using Serde.
pub struct SerdeBufferProvider<'a, P: ?Sized>(&'a P);

/// Auto-implemented for all [`BufferProvider`] for easy wrapping in [`SerdeBufferProvider`].
pub trait AsSerdeBufferProvider {
    fn as_serde_provider(&self) -> SerdeBufferProvider<Self>;
}

impl<P> AsSerdeBufferProvider for P
where
    P: BufferProvider + ?Sized,
{
    /// Wrap this [`BufferProvider`] in a [`SerdeBufferProvider`].
    fn as_serde_provider(&self) -> SerdeBufferProvider<Self> {
        SerdeBufferProvider(self)
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
        #[cfg(feature = "provider_json")]
        BufferFormat::Json => {
            let mut d = serde_json::Deserializer::from_slice(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        }

        #[cfg(feature = "provider_bincode1")]
        BufferFormat::Bincode1 => {
            use bincode::Options;
            let options = bincode::DefaultOptions::new()
                .with_fixint_encoding()
                .allow_trailing_bytes();
            let mut d = bincode::de::Deserializer::from_slice(bytes, options);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        }

        #[cfg(feature = "provider_postcard07")]
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

impl<P, M> DataProvider<M> for SerdeBufferProvider<'_, P>
where
    P: BufferProvider + ?Sized,
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    /// Converts a buffer into a concrete type by deserializing from a supported buffer format.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        // TODO(#1077): Remove the `req.clone()` when we start taking `req` by value.
        let old_response = BufferProvider::load_buffer(self.0, req.clone())?;
        if let Some(old_payload) = old_response.payload {
            let buffer_format = old_response
                .metadata
                .buffer_format
                .ok_or(Error::FormatNotSpecified)?;
            let new_payload =
                old_payload.try_map_project_with_capture(buffer_format, deserialize_impl::<M>)?;
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
