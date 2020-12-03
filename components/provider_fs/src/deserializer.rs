// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::manifest::SyntaxOption;
use icu_provider::DataError;
use icu_provider::v2::DataReceiver;
use std::io;
use std::path::Path;

/// An Error type specifically for the Deserializer that doesn't carry filenames
pub enum Error {
    Json(serde_json::error::Error),
    #[cfg(feature = "bincode")]
    Bincode(bincode::Error),
    Serde(erased_serde::Error),
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

impl From<erased_serde::Error> for Error {
    fn from(err: erased_serde::Error) -> Self {
        Self::Serde(err)
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
            Self::Serde(err) => {
                CrateError::Deserializer(Box::new(err), Some(path.as_ref().to_path_buf()))
            }
            Self::UnknownSyntax(v) => CrateError::UnknownSyntax(v),
        };
        DataError::Resource(Box::new(crate_error))
    }
}

pub fn deserialize_from_reader<R, T>(rdr: R, syntax_option: &SyntaxOption) -> Result<T, Error>
where
    R: io::Read,
    T: serde::de::DeserializeOwned,
{
    match syntax_option {
        SyntaxOption::Json => serde_json::from_reader(rdr).map_err(|e| e.into()),
        #[cfg(feature = "bincode")]
        SyntaxOption::Bincode => bincode::deserialize_from(rdr).map_err(|e| e.into()),
        #[cfg(not(feature = "bincode"))]
        SyntaxOption::Bincode => Err(Error::UnknownSyntax(syntax_option.clone())),
    }
}

pub fn deserialize_into_receiver<R>(rdr: R, syntax_option: &SyntaxOption, receiver: &mut dyn DataReceiver) -> Result<(), Error>
where
    R: io::Read,
{
    match syntax_option {
        SyntaxOption::Json => {
            let mut d = serde_json::Deserializer::from_reader(rdr);
            receiver.set_to(&mut erased_serde::Deserializer::erase(&mut d))?;
            Ok(())
        },
        #[cfg(feature = "bincode")]
        SyntaxOption::Bincode => {
            use bincode::Options;
            let options = bincode::DefaultOptions::new()
                .with_fixint_encoding()
                .allow_trailing_bytes();
            let mut d = bincode::de::Deserializer::with_reader(rdr, options);
            receiver.set_to(&mut erased_serde::Deserializer::erase(&mut d))?;
            Ok(())
        },
        #[cfg(not(feature = "bincode"))]
        SyntaxOption::Bincode => Err(Error::UnknownSyntax(syntax_option.clone())),
    }
}
