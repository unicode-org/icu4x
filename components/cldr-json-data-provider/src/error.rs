use std::error;
use std::fmt;
use std::path::PathBuf;

#[cfg(feature = "download")]
use crate::download::DownloadError;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::error::Error, Option<PathBuf>),
    IoError(std::io::Error, std::path::PathBuf),
    MissingSource(MissingSourceError),
    #[cfg(feature = "download")]
    Download(DownloadError),
    PoisonError,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MissingSourceError {
    pub src: &'static str,
}

impl fmt::Display for MissingSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing CLDR data source: {}", self.src)
    }
}

impl From<(serde_json::error::Error, Option<PathBuf>)> for Error {
    fn from(pieces: (serde_json::error::Error, Option<PathBuf>)) -> Self {
        Self::JsonError(pieces.0, pieces.1)
    }
}

impl From<MissingSourceError> for Error {
    fn from(err: MissingSourceError) -> Self {
        Self::MissingSource(err)
    }
}

#[cfg(feature = "download")]
impl From<DownloadError> for Error {
    fn from(err: DownloadError) -> Error {
        match err {
            DownloadError::Io(err, path) => Error::IoError(err, path),
            _ => Error::Download(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::JsonError(err, Some(filename)) => {
                write!(f, "JSON parse error: {}: {:?}", err, filename)
            }
            Error::JsonError(err, None) => write!(f, "JSON parse error: {}", err),
            Error::IoError(err, path) => write!(f, "{}: {}", err, path.to_string_lossy()),
            Error::MissingSource(err) => err.fmt(f),
            #[cfg(feature = "download")]
            Error::Download(err) => err.fmt(f),
            Error::PoisonError => write!(f, "poisoned lock on CLDR provider"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::JsonError(err, _) => Some(err),
            Error::IoError(err, _) => Some(err),
            #[cfg(feature = "download")]
            Error::Download(err) => Some(err),
            _ => None,
        }
    }
}
