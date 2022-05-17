// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::buf::BufferProvider;
use crate::prelude::*;
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;

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
impl DataPayload<BufferMarker> {
    pub fn into_deserialized<M>(
        self,
        buffer_format: BufferFormat,
    ) -> Result<DataPayload<M>, DataError>
    where
        M: DataMarker,
        // Actual bound:
        //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
        // Necessary workaround bound (see `yoke::trait_hack` docs):
        for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
    {
        // Allow `bytes` to be unused in case all buffer formats are disabled
        #[allow(unused_variables)]
        self.try_map_project_with_capture(buffer_format, |bytes, buffer_format, _| {
            match buffer_format {
                #[cfg(feature = "deserialize_json")]
                BufferFormat::Json => {
                    let mut d = serde_json::Deserializer::from_slice(bytes);
                    let data =
                        YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
                    Ok(data.0)
                }

                #[cfg(feature = "deserialize_bincode_1")]
                BufferFormat::Bincode1 => {
                    use bincode::Options;
                    let options = bincode::DefaultOptions::new()
                        .with_fixint_encoding()
                        .allow_trailing_bytes();
                    let mut d = bincode::de::Deserializer::from_slice(bytes, options);
                    let data =
                        YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
                    Ok(data.0)
                }

                #[cfg(feature = "deserialize_postcard_07")]
                BufferFormat::Postcard07 => {
                    let mut d = postcard::Deserializer::from_bytes(bytes);
                    let data =
                        YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
                    Ok(data.0)
                }

                // Allowed for cases in which all features are enabled
                #[allow(unreachable_patterns)]
                _ => Err(DataErrorKind::UnavailableBufferFormat(buffer_format).into_error()),
            }
        })
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
    fn load_payload(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        let (buffer_response, buffer_format) = BufferProvider::load_buffer(self.0, key, req)?;
        Ok(DataResponse {
            metadata: buffer_response.metadata,
            payload: buffer_response
                .payload
                .map(|p| p.into_deserialized(buffer_format))
                .transpose()?,
        })
    }
}

/// Implements [ResourceProvider] and [DynProvider] if [BufferProvider] is implemented.
/// This allows dropping the call to `.as_deserializing()`.
#[macro_export]
macro_rules! impl_auto_deserializing {
    ($buffer_provider: ty) => {
        impl<M> ResourceProvider<M> for $buffer_provider
        where
            M: ResourceMarker,
            // Actual bound:
            //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
            // Necessary workaround bound (see `yoke::trait_hack` docs):
            for<'de> yoke::trait_hack::YokeTraitHack<<M::Yokeable as yoke::Yokeable<'de>>::Output>:
                serde::de::Deserialize<'de>,
        {
            fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
                self.as_deserializing().load_resource(req)
            }
        }

        impl<M> DynProvider<M> for $buffer_provider
        where
            M: DataMarker,
            // Actual bound:
            //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
            // Necessary workaround bound (see `yoke::trait_hack` docs):
            for<'de> yoke::trait_hack::YokeTraitHack<<M::Yokeable as yoke::Yokeable<'de>>::Output>:
                serde::de::Deserialize<'de>,
        {
            fn load_payload(
                &self,
                key: ResourceKey,
                req: &DataRequest,
            ) -> Result<DataResponse<M>, DataError> {
                self.as_deserializing().load_payload(key, req)
            }
        }
    };
}
