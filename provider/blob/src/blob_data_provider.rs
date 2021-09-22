// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::BlobSchema;
use crate::path_util;
use alloc::rc::Rc;
use alloc::string::String;
use icu_provider::prelude::*;
use icu_provider::serde::{SerdeDeDataProvider, SerdeDeDataReceiver};
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::*;

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
    blob: Yoke<BlobSchema<'static>, Rc<[u8]>>,
}

impl BlobDataProvider {
    pub fn new_from_rc_blob(blob: Rc<[u8]>) -> Result<Self, DataError> {
        Ok(BlobDataProvider {
            blob: Yoke::try_attach_to_cart_badly(blob, |bytes| {
                BlobSchema::deserialize(&mut postcard::Deserializer::from_bytes(bytes))
            })
            .map_err(DataError::new_resc_error)?,
        })
    }

    /// Gets the buffer for the given DataRequest out of the BlobSchema and returns it yoked
    /// to the buffer backing the BlobSchema.
    fn get_file(&self, req: &DataRequest) -> Result<Yoke<&'static [u8], Rc<[u8]>>, DataError> {
        let path = path_util::resource_path_to_string(&req.resource_path);
        self.blob
            .try_project_cloned_with_capture::<&'static [u8], String, ()>(
                path,
                move |blob, path, _| {
                    let BlobSchema::V001(blob) = blob;
                    blob.resources.get(&*path).ok_or(()).map(|v| *v)
                },
            )
            .map_err(|_| DataError::MissingResourceKey(req.resource_path.key))
    }
}

impl<'data, M> DataProvider<'data, M> for BlobDataProvider
where
    M: DataMarker<'data>,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'data, M>, DataError> {
        let file = self.get_file(req)?;
        let payload =
            DataPayload::try_from_yoked_buffer::<(), DataError>(file, (), |bytes, _, _| {
                let mut d = postcard::Deserializer::from_bytes(bytes);
                let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)
                    .map_err(DataError::new_resc_error)?;
                Ok(data.0)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(payload),
        })
    }
}

impl SerdeDeDataProvider for BlobDataProvider {
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDeDataReceiver,
    ) -> Result<DataResponseMetadata, DataError> {
        let file = self.get_file(req)?;
        receiver.receive_yoked_buffer(file, |bytes, f2| {
            let mut d = postcard::Deserializer::from_bytes(bytes);
            f2(&mut <dyn erased_serde::Deserializer>::erase(&mut d))
        })?;
        Ok(DataResponseMetadata {
            data_langid: req.resource_path.options.langid.clone(),
        })
    }
}
