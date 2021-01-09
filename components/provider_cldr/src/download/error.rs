// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Io(io::Error, Option<PathBuf>),
    Reqwest(reqwest::Error),
    HttpStatus(reqwest::StatusCode, String),
    NoCacheDir,
}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: Error::Io(err, None)
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err, Some(path)) => write!(f, "{}: {:?}", err, path),
            Self::Io(err, None) => err.fmt(f),
            Self::Reqwest(err) => err.fmt(f),
            Self::HttpStatus(status, url) => write!(f, "HTTP request failed: {}: {}", status, url),
            Self::NoCacheDir => write!(f, "dirs::cache_dir() returned None"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err, _) => Some(err),
            Self::Reqwest(err) => Some(err),
            _ => None,
        }
    }
}
