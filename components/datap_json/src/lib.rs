#![no_std]

extern crate no_std_compat as std;

use std::prelude::v1::*;
// use async_trait::async_trait;
use std::borrow::Cow;

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
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let result: schema::JsonSchema = serde_json::from_reader(reader)?;
        Ok(JsonDataProvider{ data: result })
    }
}

impl datap::DataProvider for JsonDataProvider {
    fn load(&self, _request: datap::Request) -> Result<datap::Response, Box<dyn std::error::Error>> {
        // TODO: Use the request variable
        Ok(datap::Response {
            locale: "root".to_string(),
            payload: Cow::Owned(datap::ResponsePayload::Decimal(self.data.decimal.symbols_v1.clone()))
        })
    }
}
