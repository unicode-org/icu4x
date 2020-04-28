#![no_std]

extern crate no_std_compat as std;

use std::prelude::v1::*;


mod schema;

#[cfg(feature = "std")]
use std::io::Read;

pub enum Error {
    JsonError(serde_json::error::Error)
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::JsonError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct JsonDataProvider {
    data: schema::JsonSchema,
}

impl JsonDataProvider {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let result: schema::JsonSchema = serde_json::from_reader(reader)?;
        Ok(JsonDataProvider{ data: result })
    }
}
