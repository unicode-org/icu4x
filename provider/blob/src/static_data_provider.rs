// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use serde::de::Deserialize;
use writeable::Writeable;
use yoke::trait_hack::YokeTraitHack;
use zerovec::maps::{KeyError, ZeroMap2dBorrowed};

/// A data provider loading data statically baked in to the binary.
///
/// Although static data is convenient and highly portable, it also increases binary size. To
/// load the data files dynamically at runtime, see [`BlobDataProvider`].
///
/// To bake blob data into your binary, use [`include_bytes!`](std::include_bytes), as shown in
/// the example below.
///
/// # Examples
///
/// Load "hello world" data from a postcard blob statically linked at compile time:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_provider_blob::StaticDataProvider;
/// use icu_locid::langid;
///
/// const HELLO_WORLD_BLOB: &[u8] = include_bytes!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/data/hello_world.postcard"
/// ));
///
/// let provider = StaticDataProvider::new_from_static_blob(&HELLO_WORLD_BLOB)
///     .expect("Deserialization should succeed");
///
/// let response: DataPayload<HelloWorldV1Marker> = provider
///     .load_resource(&DataRequest {
///         options: langid!("la").into(),
///         metadata: Default::default(),
///     })
///     .expect("Data should be valid")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!(response.get().message, "Ave, munde");
/// ```
///
/// [`BlobDataProvider`]: crate::BlobDataProvider
pub struct StaticDataProvider {
    data: ZeroMap2dBorrowed<'static, ResourceKeyHash, [u8], [u8]>,
}

impl StaticDataProvider {
    /// Create a [`StaticDataProvider`] from a `'static` blob of ICU4X data.
    pub fn new_from_static_blob(blob: &'static [u8]) -> Result<Self, DataError> {
        Ok(StaticDataProvider {
            data: BlobSchema::deserialize(&mut postcard::Deserializer::from_bytes(blob)).map(
                |blob| {
                    let BlobSchema::V001(blob) = blob;
                    blob.resources
                },
            )?,
        })
    }

    /// Creates an empty [`StaticDataProvider`] that contains no data.
    ///
    /// Can be used as a stub for when a real data provider is not available.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider_blob::StaticDataProvider;
    /// use icu_locid::langid;
    ///
    /// let stub_provider = StaticDataProvider::new_empty();
    ///
    /// ResourceProvider::<HelloWorldV1Marker>::load_resource(
    ///     &stub_provider,
    ///     &DataRequest {
    ///         options: langid!("la").into(),
    ///         metadata: Default::default(),
    ///     }
    /// )
    /// .expect_err("Stub provider returns no data");
    /// ```
    pub fn new_empty() -> Self {
        StaticDataProvider {
            data: ZeroMap2dBorrowed::new(),
        }
    }

    fn get_file(&self, key: ResourceKey, req: &DataRequest) -> Result<&'static [u8], DataError> {
        self.data
            .get(&key.get_hash(), &req.options.write_to_string().as_bytes())
            .map_err(|e| {
                match e {
                    KeyError::K0 => DataErrorKind::MissingResourceKey,
                    KeyError::K1 => DataErrorKind::MissingResourceOptions,
                }
                .with_req(key, req)
            })
    }
}

impl<M> ResourceProvider<M> for StaticDataProvider
where
    M: ResourceMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as yoke::Yokeable<'de>>::Output>:
        serde::de::Deserialize<'de>,
{
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        self.as_deserializing().load_resource(req)
    }
}

impl<M> DynProvider<M> for StaticDataProvider
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as yoke::Yokeable<'de>>::Output>:
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

impl BufferProvider for StaticDataProvider {
    fn load_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let static_buffer = self.get_file(key, req)?;
        let mut metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        metadata.buffer_format = Some(BufferFormat::Postcard07);
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_static_buffer(static_buffer)),
        })
    }
}
