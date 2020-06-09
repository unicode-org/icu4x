#![no_std]

extern crate no_std_compat as std;

use std::prelude::v1::*;

use icu_data_provider::DataProvider;
use icu_data_provider::Request;
use icu_data_provider::Response;
use icu_data_provider::ResponseBuilder;
use icu_data_provider::ResponseError;
use icu_locale::LanguageIdentifier;

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
#[derive(Debug)]
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

impl<'a> DataProvider<'a, 'a> for JsonDataProvider {
    /// Loads JSON data. Returns borrowed data.
    fn load(&'a self, _request: &Request) -> Result<Response<'a>, ResponseError> {
        let response = ResponseBuilder {
            data_langid: LanguageIdentifier::default(),
        }
        .with_borrowed_payload(&self.data.decimal.symbols_v1_a);
        Ok(response)
    }
}

#[test]
fn test_empty_str() {
    let result = JsonDataProvider::from_str("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Coverage for Debug trait:
    println!("{:?}", err);
    // An unconditional let is possible here because it is a one-element enum.
    // If more cases are needed, see https://github.com/rust-lang/rfcs/pull/1303
    let Error::JsonError(json_err) = err;
    assert_eq!(json_err.classify(), serde_json::error::Category::Eof);
}
