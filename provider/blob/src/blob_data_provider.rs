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
        enum LocalError {
            MissingResourceKey,
            Postcard(postcard::Error),
        }
        let path = path_util::resource_path_to_string(&req.resource_path);
        let raw_payload = self
            .blob
            .try_project_cloned_with_capture::<M::Yokeable, String, LocalError>(
                path,
                move |blob, path, _| {
                    let BlobSchema::V001(blob) = blob;
                    let file = blob
                        .resources
                        .get(&*path)
                        .ok_or(LocalError::MissingResourceKey)
                        .map(|v| *v)?;
                    let mut d = postcard::Deserializer::from_bytes(file);
                    let data =
                        YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)
                            .map_err(LocalError::Postcard)?;
                    Ok(data.0)
                },
            )
            .map_err(|local_error| match local_error {
                LocalError::MissingResourceKey => {
                    DataError::MissingResourceKey(req.resource_path.key)
                }
                LocalError::Postcard(err) => DataError::new_resc_error(err),
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_rc_buffer_yoke(raw_payload)),
        })
    }
}
