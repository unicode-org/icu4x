use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum DownloadError {
    Io(io::Error, PathBuf),
    Reqwest(reqwest::Error),
    HttpStatus(reqwest::StatusCode, String),
    NoCacheDir,
}

impl From<io::Error> for DownloadError {
    /// Note: Prefer adding the path to Error::Io instead of using this conversion.
    fn from(err: io::Error) -> Self {
        Self::Io(err, PathBuf::new())
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err, path) => write!(f, "{}: {}", err, path.to_string_lossy()),
            Self::Reqwest(err) => err.fmt(f),
            Self::HttpStatus(status, url) => write!(f, "HTTP request failed: {}: {}", status, url),
            Self::NoCacheDir => write!(f, "dirs::cache_dir() returned None"),
        }
    }
}

impl error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err, _) => Some(err),
            Self::Reqwest(err) => Some(err),
            _ => None,
        }
    }
}
