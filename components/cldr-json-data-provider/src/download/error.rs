use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Io(io::Error, PathBuf),
    Reqwest(reqwest::Error),
    HttpStatus(reqwest::StatusCode, String),
}

impl From<io::Error> for Error {
    /// Note: Prefer adding the path to Error::Io instead of using this conversion.
    fn from(err: io::Error) -> Error {
        Error::Io(err, PathBuf::new())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err, path) => write!(f, "{}: {}", err, path.to_string_lossy()),
            Error::Reqwest(err) => err.fmt(f),
            Error::HttpStatus(status, url) => write!(f, "HTTP request failed: {}: {}", status, url),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err, _) => Some(err),
            Error::Reqwest(err) => Some(err),
            _ => None,
        }
    }
}
