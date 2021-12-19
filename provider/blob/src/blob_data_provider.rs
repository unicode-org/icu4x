// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::BlobSchema;
use crate::path_util;
use alloc::rc::Rc;
use alloc::string::String;
use icu_provider::buffer_provider::BufferFormat;
use icu_provider::prelude::*;
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::*;
use zerovec::map::ZeroMapBorrowed;

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
/// use icu_locid_macros::langid;
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_provider_blob::BlobDataProvider;
/// use std::fs::File;
/// use std::io::Read;
/// use std::rc::Rc;
///
/// // Read an ICU4X data blob dynamically:
/// let mut blob: Vec<u8> = Vec::new();
/// let filename = concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/data/hello_world.postcard",
/// );
/// File::open(filename)
///     .expect("File should exist")
///     .read_to_end(&mut blob)
///     .expect("Reading pre-computed postcard buffer");
///
/// // Create a DataProvider from it:
/// let provider = BlobDataProvider::new_from_rc_blob(Rc::from(blob))
///     .expect("Deserialization should succeed");
///
/// // Check that it works:
/// let response: DataPayload<HelloWorldV1Marker> = provider.load_payload(
///     &DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: langid!("la").into(),
///         }
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
    data: Yoke<ZeroMapBorrowed<'static, str, [u8]>, Rc<[u8]>>,
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
            })
            .map_err(DataError::new_resc_error)?,
        })
    }

    /// Gets the buffer for the given DataRequest out of the BlobSchema and returns it yoked
    /// to the buffer backing the BlobSchema.
    fn get_file(&self, req: &DataRequest) -> Result<Yoke<&'static [u8], Rc<[u8]>>, DataError> {
        let path = path_util::resource_path_to_string(&req.resource_path);
        self.data
            .try_project_cloned_with_capture::<&'static [u8], String, ()>(path, |zm, path, _| {
                zm.get(&path).ok_or(())
            })
            .map_err(|_| DataError::MissingResourceKey(req.resource_path.key))
    }
}

impl<M> DataProvider<M> for BlobDataProvider
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as yoke::Yokeable<'de>>::Output>:
        serde::de::Deserialize<'de>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        self.as_deserializing().load_payload(req)
    }
}

impl BufferProvider for BlobDataProvider {
    fn load_buffer(&self, req: &DataRequest) -> Result<DataResponse<BufferMarker>, DataError> {
        let yoked_buffer = self.get_file(req)?;
        let mut metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        metadata.buffer_format = Some(BufferFormat::Postcard07);
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_yoked_buffer(yoked_buffer)),
        })
    }
}
