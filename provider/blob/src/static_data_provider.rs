// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::*;
use crate::path_util;
use icu_provider::prelude::*;
use icu_provider::serde::{SerdeDeDataProvider, SerdeDeDataReceiver};
use litemap::LiteMap;
use serde::de::Deserialize;

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
/// use icu_locid_macros::langid;
///
/// const HELLO_WORLD_BLOB: &[u8] = include_bytes!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/data/hello_world.postcard"
/// ));
///
/// let provider = StaticDataProvider::new_from_static_blob(&HELLO_WORLD_BLOB)
///     .expect("Deserialization should succeed");
///
/// let response: DataPayload<HelloWorldV1Marker> = provider.load_payload(
///     &DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: langid!("la").into(),
///         }
///     }
/// )
/// .expect("Data should be valid")
/// .take_payload()
/// .expect("Data should be present");
///
/// assert_eq!(response.get().message, "Ave, munde");
/// ```
///
/// [`BlobDataProvider`]: crate::BlobDataProvider
pub struct StaticDataProvider {
    blob: BlobSchema<'static>,
}

impl StaticDataProvider {
    /// Create a [`StaticDataProvider`] from a `'static` blob of ICU4X data.
    pub fn new_from_static_blob(blob: &'static [u8]) -> Result<Self, DataError> {
        Ok(StaticDataProvider {
            blob: BlobSchema::deserialize(&mut postcard::Deserializer::from_bytes(blob))
                .map_err(DataError::new_resc_error)?,
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
    /// use icu_locid_macros::langid;
    ///
    /// let stub_provider = StaticDataProvider::new_empty();
    ///
    /// DataProvider::<HelloWorldV1Marker>::load_payload(
    ///     &stub_provider,
    ///     &DataRequest {
    ///         resource_path: ResourcePath {
    ///             key: key::HELLO_WORLD_V1,
    ///             options: langid!("la").into(),
    ///         }
    ///     }
    /// )
    /// .expect_err("Stub provider returns no data");
    /// ```
    pub fn new_empty() -> Self {
        StaticDataProvider {
            blob: BlobSchema::V001(BlobSchemaV1 {
                resources: LiteMap::new(),
            }),
        }
    }

    fn get_file(&self, req: &DataRequest) -> Result<&'static [u8], DataError> {
        let path = path_util::resource_path_to_string(&req.resource_path);
        let BlobSchema::V001(blob) = &self.blob;
        blob.resources
            .get(&*path)
            .ok_or(DataError::MissingResourceKey(req.resource_path.key))
            .map(|v| *v)
    }
}

impl<M> DataProvider<M> for StaticDataProvider
where
    M: DataMarker,
    // 'static is what we want here, because we are deserializing from a static buffer.
    M::Yokeable: Deserialize<'static>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let file = self.get_file(req)?;
        let data = M::Yokeable::deserialize(&mut postcard::Deserializer::from_bytes(file))
            .map_err(DataError::new_resc_error)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

impl SerdeDeDataProvider for StaticDataProvider {
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDeDataReceiver,
    ) -> Result<DataResponseMetadata, DataError> {
        let file = self.get_file(req)?;
        receiver.receive_static(&mut <dyn erased_serde::Deserializer>::erase(
            &mut postcard::Deserializer::from_bytes(file),
        ))?;

        Ok(DataResponseMetadata {
            data_langid: req.resource_path.options.langid.clone(),
        })
    }
}
