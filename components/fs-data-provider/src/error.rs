use std::fmt;

#[derive(Debug)]
pub enum Error {
    DataProviderError(icu_data_provider::DataError),
    SerdeJsonError(serde_json::error::Error),
    #[cfg(feature = "export")]
    SerializerError(erased_serde::Error),
    // TODO: Consider adding the path to IoError
    IoError(std::io::Error),
}

impl From<icu_data_provider::DataError> for Error {
    fn from(err: icu_data_provider::DataError) -> Error {
        Error::DataProviderError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::SerdeJsonError(err)
    }
}

#[cfg(feature = "export")]
impl From<erased_serde::Error> for Error {
    fn from(err: erased_serde::Error) -> Error {
        Error::SerializerError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DataProviderError(error) => write!(f, "{}", error),
            Error::SerdeJsonError(error) => write!(f, "{}", error),
            #[cfg(feature = "export")]
            Error::SerializerError(error) => write!(f, "{}", error),
            Error::IoError(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DataProviderError(error) => Some(error),
            Error::SerdeJsonError(error) => Some(error),
            #[cfg(feature = "export")]
            Error::SerializerError(error) => Some(error),
            Error::IoError(error) => Some(error),
        }
    }
}
