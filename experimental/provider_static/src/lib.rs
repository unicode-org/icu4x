// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

const STATIC_STR_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/static_data.bincode"));

/// A data provider loading data statically baked in to the binary. Useful for testing in situations
/// where setting up a filesystem is tricky (e.g. WASM).
///
/// This should probably not be used in production code since it bloats the binary.
pub struct StaticDataProvider {
    json: HashMap<&'static str, &'static str>,
}

impl StaticDataProvider {
    pub fn new() -> Self {
        StaticDataProvider {
            json: bincode::deserialize(&STATIC_STR_DATA).unwrap(),
        }
    }
}

impl Default for StaticDataProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl<'d, 's, M> DataProvider<'d, 's, M> for StaticDataProvider
where
    M: DataMarker<'s>,
    // TODO(#667): Use zero-copy Deserialize instead of DeserializeOwned
    M::Yokeable: serde::de::DeserializeOwned,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 's, M>, DataError> {
        let key_components = req.resource_path.key.get_components();
        let opt_components = req.resource_path.options.get_components();
        let key: Vec<&str> = key_components.iter().chain(opt_components.iter()).collect();
        let key = "/".to_string() + &key.join("/");
        let file = self
            .json
            .get(&*key)
            .ok_or(DataError::UnsupportedResourceKey(req.resource_path.key))?;
        let data: M::Yokeable =
            M::Yokeable::deserialize(&mut serde_json::Deserializer::from_reader(file.as_bytes()))
                .map_err(|e| DataError::Resource(Box::new(e)))?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}
