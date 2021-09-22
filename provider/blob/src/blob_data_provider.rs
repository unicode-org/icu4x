// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::blob_schema::BlobSchema;
use crate::path_util;
use alloc::rc::Rc;
use icu_provider::prelude::*;
use icu_provider::serde::{SerdeDeDataProvider, SerdeDeDataReceiver};
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::*;

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
        let path = path_util::resource_path_to_string(&req.resource_path);
        let raw_payload: Result<Yoke<M::Yokeable, Rc<[u8]>>, DataError> =
            self.blob.try_project_cloned_with_capture(
                (path, req.resource_path.key),
                move |blob, (path, key), _| {
                    let BlobSchema::V001(blob) = blob;
                    let file = blob
                        .resources
                        .get(&*path)
                        .ok_or(DataError::MissingResourceKey(key))
                        .map(|v| *v)?;
                    let mut d = postcard::Deserializer::from_bytes(file);
                    let data =
                        YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)
                            .map_err(DataError::new_resc_error)?;
                    Ok(data.0)
                },
            );
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_rc_buffer_yoke(raw_payload?)),
        })
    }
}
