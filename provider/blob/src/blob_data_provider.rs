// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::BlobSchema;
use alloc::rc::Rc;
use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use serde::de::Deserialize;
use writeable::Writeable;
use yoke::*;
use zerovec::maps::{KeyError, ZeroMap2dBorrowed};

/// A data provider loading data from blobs dynamically created at runtime.
///
/// This enables data blobs to be read from the filesystem or from an HTTP request dynamically
/// at runtime, so that the code and data can be shipped separately.
///
/// If you prefer to bake the data into your binary, see [`StaticDataProvider`].
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_provider_blob::BlobDataProvider;
/// use std::fs;
///
/// // Read an ICU4X data blob dynamically:
/// let blob = fs::read(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/data/hello_world.postcard",
/// )).expect("Reading pre-computed postcard buffer");
///
/// // Create a DataProvider from it:
/// let provider = BlobDataProvider::new_from_rc_blob(blob.into())
///     .expect("Deserialization should succeed");
///
/// // Check that it works:
/// let response: DataPayload<HelloWorldV1Marker> = provider
///     .load_resource(&DataRequest {
///         options: locale!("la").into(),
///         metadata: Default::default(),
///     })
///     .expect("Data should be valid")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!(response.get().message, "Ave, munde");
/// ```
///
/// [`StaticDataProvider`]: crate::StaticDataProvider
pub struct BlobDataProvider {
    #[allow(clippy::type_complexity)]
    data: Yoke<ZeroMap2dBorrowed<'static, ResourceKeyHash, [u8], [u8]>, Rc<[u8]>>,
}

impl BlobDataProvider {
    /// Create a [`BlobDataProvider`] from an `Rc` blob of ICU4X data.
    pub fn new_from_rc_blob(blob: Rc<[u8]>) -> Result<Self, DataError> {
        Ok(BlobDataProvider {
            data: Yoke::try_attach_to_cart_badly(blob, |bytes| {
                BlobSchema::deserialize(&mut postcard::Deserializer::from_bytes(bytes)).map(
                    |blob| {
                        let BlobSchema::V001(blob) = blob;
                        blob.resources
                    },
                )
            })?,
        })
    }
}

impl BufferProvider for BlobDataProvider {
    fn load_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let mut metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        metadata.buffer_format = Some(BufferFormat::Postcard07);
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_yoked_buffer(
                self.data
                    .try_project_cloned_with_capture((key, req), |zm, (key, req), _| {
                        zm.get(&key.get_hash(), req.options.write_to_string().as_bytes())
                            .map_err(|e| {
                                match e {
                                    KeyError::K0 => DataErrorKind::MissingResourceKey,
                                    KeyError::K1 => DataErrorKind::MissingResourceOptions,
                                }
                                .with_req(key, req)
                            })
                    })?,
            )),
        })
    }
}

icu_provider::impl_auto_deserializing!(BlobDataProvider);
