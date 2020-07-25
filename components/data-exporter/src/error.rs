#[derive(Debug)]
pub enum Error {
    ResponseError(icu_data_provider::error::ResponseError),
    PayloadError(icu_data_provider::error::PayloadError),
    SerializerError(erased_serde::Error),
    IoError(std::io::Error),
}

impl From<icu_data_provider::error::ResponseError> for Error {
    fn from(err: icu_data_provider::error::ResponseError) -> Error {
        Error::ResponseError(err)
    }
}

impl From<icu_data_provider::error::PayloadError> for Error {
    fn from(err: icu_data_provider::error::PayloadError) -> Error {
        Error::PayloadError(err)
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
