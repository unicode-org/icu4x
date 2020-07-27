use std::fmt;

#[derive(Debug)]
pub enum Error {
    DataProviderError(icu_data_provider::error::Error),
    SerdeJsonError(serde_json::error::Error),
    SerializerError(erased_serde::Error),
    IoError(std::io::Error),
}

impl From<icu_data_provider::error::Error> for Error {
    fn from(err: icu_data_provider::error::Error) -> Error {
        Error::DataProviderError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::SerdeJsonError(err)
    }
}

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
        // TODO: should the Error Display be different from Debug?
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DataProviderError(error) => Some(error),
            Error::SerdeJsonError(error) => Some(error),
            Error::SerializerError(error) => Some(error),
            Error::IoError(error) => Some(error),
        }
    }
}
