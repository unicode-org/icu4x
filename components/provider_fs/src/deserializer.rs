// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use crate::manifest::SyntaxOption;
use icu_provider::prelude::*;
use icu_provider::serde::SerdeDeDataReceiver;
use std::io::Read;
use std::path::Path;

/// An Error type specifically for the Deserializer that doesn't carry filenames
pub enum Error {
    Json(serde_json::error::Error),
    #[cfg(feature = "bincode")]
    Bincode(bincode::Error),
    DataProvider(DataError),
    #[allow(dead_code)]
    UnknownSyntax(SyntaxOption),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

#[cfg(feature = "bincode")]
impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Self::Bincode(err)
    }
}

impl From<DataError> for Error {
    fn from(err: DataError) -> Self {
        Self::DataProvider(err)
    }
}

impl Error {
    pub fn into_resource_error<P: AsRef<Path>>(self, path: P) -> DataError {
        use crate::error::Error as CrateError;
        let crate_error = match self {
            Self::Json(err) => {
                CrateError::Deserializer(Box::new(err), Some(path.as_ref().to_path_buf()))
            }
            #[cfg(feature = "bincode")]
            Self::Bincode(err) => {
                CrateError::Deserializer(Box::new(err), Some(path.as_ref().to_path_buf()))
            }
            Self::DataProvider(err) => {
                CrateError::Deserializer(Box::new(err), Some(path.as_ref().to_path_buf()))
            }
            Self::UnknownSyntax(v) => CrateError::UnknownSyntax(v),
        };
        DataError::Resource(Box::new(crate_error))
    }
}

/// Get a JSON Deserializer. Implemeted as a macro because the return type is complex/private.
macro_rules! get_json_deserializer {
    ($rdr:tt) => {
        serde_json::Deserializer::from_reader($rdr)
    };
}

/// Get a Bincode Deserializer. Implemeted as a macro because the return type is complex/private.
#[cfg(feature = "bincode")]
macro_rules! get_bincode_deserializer {
    ($rdr:tt) => {{
        use bincode::Options;
        let options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();
        bincode::de::Deserializer::with_reader($rdr, options)
    }};
}

/// Deserialize into a generic type (DataProvider). Covers all supported data formats.
pub fn deserialize_into_type<'de, T>(
    rdr: impl Read,
    syntax_option: &SyntaxOption,
) -> Result<T, Error>
where
    T: serde::Deserialize<'de>,
{
    match syntax_option {
        SyntaxOption::Json => {
            let mut d = get_json_deserializer!(rdr);
            let data = T::deserialize(&mut d)?;
            Ok(data)
        }
        #[cfg(feature = "bincode")]
        SyntaxOption::Bincode => {
            let mut d = get_bincode_deserializer!(rdr);
            let data = T::deserialize(&mut d)?;
            Ok(data)
        }
        #[cfg(not(feature = "bincode"))]
        SyntaxOption::Bincode => Err(Error::UnknownSyntax(syntax_option.clone())),
    }
}

/// Deserialize into a receiver (SerdeDataProvider). Covers all supported data formats.
pub fn deserialize_into_receiver(
    rdr: impl Read,
    syntax_option: &SyntaxOption,
    receiver: &mut dyn SerdeDeDataReceiver,
) -> Result<(), Error> {
    match syntax_option {
        SyntaxOption::Json => {
            let mut d = get_json_deserializer!(rdr);
            receiver.receive_deserializer(&mut erased_serde::Deserializer::erase(&mut d))?;
            Ok(())
        }
        #[cfg(feature = "bincode")]
        SyntaxOption::Bincode => {
            let mut d = get_bincode_deserializer!(rdr);
            receiver.receive_deserializer(&mut erased_serde::Deserializer::erase(&mut d))?;
            Ok(())
        }
        #[cfg(not(feature = "bincode"))]
        SyntaxOption::Bincode => Err(Error::UnknownSyntax(syntax_option.clone())),
    }
}
