use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::error::Error),
    IoError(std::io::Error, std::path::PathBuf),
    MissingSource(MissingSourceError),
}

#[derive(Debug, Copy, Clone)]
pub struct MissingSourceError {
    pub src: &'static str,
}

impl fmt::Display for MissingSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing CLDR data source: {}", self.src)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Self::JsonError(err)
    }
}

impl From<&MissingSourceError> for Error {
    fn from(err: &MissingSourceError) -> Self {
        Self::MissingSource(*err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::JsonError(err) => write!(f, "{}", err),
            Error::IoError(err, path) => write!(f, "{}: {}", err, path.to_string_lossy()),
            Error::MissingSource(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::JsonError(err) => Some(err),
            Error::IoError(err, _) => Some(err),
            _ => None,
        }
    }
}
