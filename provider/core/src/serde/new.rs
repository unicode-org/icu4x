// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::de_impls;
use super::Error;
use crate::buffer_provider::{BufferProvider, SerdeBufferProvider};
use crate::prelude::*;
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;

impl<P, M> DataProvider<M> for SerdeBufferProvider<P>
where
    P: BufferProvider,
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        // TODO(#1077): Remove the `req.clone()` when we start taking `req` by value.
        let old_response = BufferProvider::load_buffer(&self.0, req.clone())?;
        if let Some(old_payload) = old_response.payload {
            let buffer_format = old_response
                .metadata
                .buffer_format
                .ok_or(Error::FormatNotSpecified)?;
            let new_payload = old_payload
                .try_map_project_with_capture(buffer_format, |bytes, buffer_format, _| {
                    de_impls::deserialize::<M>(bytes, buffer_format)
                })?;
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
