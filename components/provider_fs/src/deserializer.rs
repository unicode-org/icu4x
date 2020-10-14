use crate::manifest::SyntaxOption;
use icu_provider::DataError;
use std::io;
use std::path::Path;

/// An Error type specifically for the Deserializer that doesn't carry filenames
pub enum Error {
    Json(serde_json::error::Error),
    #[cfg(feature = "bincode")]
    Bincode(bincode::Error),
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
