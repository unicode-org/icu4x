#![no_std]

extern crate no_std_compat as std;

use std::prelude::v1::*;

use icu_util::datap;

mod schema;

#[cfg(feature = "std")]
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::error::Error),
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::JsonError(err)
    }
}

/// A data provider reading from a JSON file.
pub struct JsonDataProvider {
    data: schema::JsonSchema,
}

impl JsonDataProvider {
    /// Create a JsonDataProvider from a file reader.
    #[cfg(feature = "std")]
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let result: schema::JsonSchema = serde_json::from_reader(reader)?;
        Ok(JsonDataProvider { data: result })
    }

    /// Create a JsonDataProvider from a JSON string slice.
    pub fn from_str(data: &str) -> Result<Self, Error> {
        let result: schema::JsonSchema = serde_json::from_str(data)?;
        Ok(JsonDataProvider { data: result })
    }

    /// Create a JsonDataProvider from a &[u8] slice.
    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        let result: schema::JsonSchema = serde_json::from_slice(data)?;
        Ok(JsonDataProvider { data: result })
    }
}

impl datap::DataProvider for JsonDataProvider {
    fn load(&self, request: datap::Request) -> Result<datap::Response, datap::ResponseError> {
        // Clone the object, since the response could outlive the provider.
        let payload = self.data.decimal.symbols_v1_a.clone();
        let response = datap::ResponseBuilder {
            data_locale: "und".to_string(),
            request: request,
        }
        .with_owned_payload(payload);
        Ok(response)
    }
}
