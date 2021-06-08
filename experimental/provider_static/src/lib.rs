// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{prelude::*, serde::{SerdeDeDataProvider, SerdeDeDataReceiver}};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

const STATIC_STR_DATA: &str = include_str!(concat!(env!("OUT_DIR"), "/static_data.json"));

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "ty", content = "contents")]
enum StaticFileOrDir {
    File(String),
    Dir(Directory),
}

#[derive(Serialize, Deserialize, Debug)]
struct Directory(Box<HashMap<String, StaticFileOrDir>>);

/// A data provider loading data statically baked in to the binary. Useful for testing in situations
/// where setting up a filesystem is tricky (e.g. WASM).
///
/// This should probably not be used in production code since it bloats the binary.
pub struct StaticDataProvider {
    json: Directory,
}

impl StaticDataProvider {
    pub fn new() -> Self {
        StaticDataProvider {
            json: serde_json::from_str(&STATIC_STR_DATA).unwrap(),
        }
    }

    fn get_file(&self, req: &DataRequest) -> Result<&str, DataError> {
        let components = req.resource_path.key.get_components();
        let mut dir = &self.json;
        let mut file: Option<&str> = None;
        for component in components
            .iter()
            .chain(req.resource_path.options.get_components().iter())
        {
            if file.is_some() {
                // We should eventually distinguish between UnsupportedResourceKey
                // and UnsupportedResourceOptions here
                return Err(DataError::UnsupportedResourceKey(req.resource_path.key));
            }
            let fod = dir
                .0
                .get(component)
                .ok_or(DataError::UnsupportedResourceKey(req.resource_path.key))?;
            match fod {
                StaticFileOrDir::Dir(ref d) => dir = d,
                StaticFileOrDir::File(ref f) => file = Some(f),
            }
        }
        Ok(file.ok_or(DataError::UnsupportedResourceKey(req.resource_path.key))?)
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
        let file = self.get_file(req)?;
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

impl<'de> SerdeDeDataProvider<'de> for StaticDataProvider {
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn SerdeDeDataReceiver<'de>,
    ) -> Result<DataResponseMetadata, DataError> {
        let file = self.get_file(req)?;
        let mut d = serde_json::Deserializer::from_reader(file.as_bytes());
        receiver.receive_deserializer(&mut <dyn erased_serde::Deserializer>::erase(&mut d))?;

        Ok(DataResponseMetadata {
            data_langid: req.resource_path.options.langid.clone(),
        })
    }
}
