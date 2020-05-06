#![no_std]

extern crate no_std_compat as std;

use std::prelude::v1::*;
// use async_trait::async_trait;

use icu_util::datap;

mod schema;

#[cfg(feature = "std")]
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::error::Error)
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::JsonError(err)
    }
}

pub struct JsonDataProvider {
    data: schema::JsonSchema,
}

impl JsonDataProvider {
    #[cfg(feature = "std")]
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let result: schema::JsonSchema = serde_json::from_reader(reader)?;
        Ok(JsonDataProvider{ data: result })
    }

    pub fn from_str(data: &str) -> Result<Self, Error> {
        let result: schema::JsonSchema = serde_json::from_str(data)?;
        Ok(JsonDataProvider{ data: result })
    }

    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        let result: schema::JsonSchema = serde_json::from_slice(data)?;
        Ok(JsonDataProvider{ data: result })
    }
}

impl datap::DataProvider for JsonDataProvider {
    fn load(&self, _request: datap::Request) -> Result<datap::Response, datap::ResponseError> {
        // TODO: Use the request variable
        let mut response = datap::Response::with_owned_payload(self.data.decimal.symbols_v1_a.clone());
        response.locale = "en".to_string();
        Ok(response)
    }
}
