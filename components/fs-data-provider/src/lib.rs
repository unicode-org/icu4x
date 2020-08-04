use icu_data_provider::prelude::*;
use icu_locale::LanguageIdentifier;
use std::io::Read;
use std::str::FromStr;

mod schema;

#[cfg(feature = "export")]
pub mod export;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::error::Error),
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::JsonError(err)
    }
}

#[derive(Debug)]
pub struct JsonDataWarehouse {
    data: schema::JsonSchema,
}

/// A data provider reading from a JSON file.
#[derive(Debug)]
pub struct JsonDataProvider<'d> {
    borrowed_data: &'d schema::JsonSchema,
}

impl JsonDataWarehouse {
    /// Create a JsonDataProvider from a file reader.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let data: schema::JsonSchema = serde_json::from_reader(reader)?;
        Ok(Self { data })
    }

    /// Create a JsonDataProvider from a &[u8] slice.
    /// Also see from_str().
    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        let data: schema::JsonSchema = serde_json::from_slice(data)?;
        Ok(Self { data })
    }

    pub fn provider(&self) -> JsonDataProvider {
        self.into()
    }
}

impl FromStr for JsonDataWarehouse {
    type Err = Error;

    /// Create a JsonDataProvider from a JSON string slice.
    fn from_str(data: &str) -> Result<Self, Error> {
        let data: schema::JsonSchema = serde_json::from_str(data)?;
        Ok(Self { data })
    }
}

impl<'d> From<&'d JsonDataWarehouse> for JsonDataProvider<'d> {
    fn from(warehouse: &'d JsonDataWarehouse) -> JsonDataProvider<'d> {
        JsonDataProvider {
            borrowed_data: &warehouse.data,
        }
    }
}

impl<'d> DataProvider<'d> for JsonDataProvider<'d> {
    /// Loads JSON data. Returns borrowed data.
    fn load(
        &self,
        _request: &data_provider::Request,
    ) -> Result<data_provider::Response<'d>, data_provider::Error> {
        let response = data_provider::ResponseBuilder {
            data_langid: LanguageIdentifier::default(),
        }
        .with_borrowed_payload(&self.borrowed_data.decimal.symbols_v1_a);
        Ok(response)
    }
}

#[test]
fn test_empty_str() {
    let result = JsonDataWarehouse::from_str("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // An unconditional let is possible here because it is a one-element enum.
    // If more cases are needed, see https://github.com/rust-lang/rfcs/pull/1303
    let Error::JsonError(json_err) = err;
    assert_eq!(json_err.classify(), serde_json::error::Category::Eof);
}
