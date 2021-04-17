// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;
use std::error;
use std::fmt;
use std::path::{Path, PathBuf};

#[cfg(feature = "download")]
use crate::download;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error, Option<PathBuf>),
    Json(serde_json::error::Error, Option<PathBuf>),
    Custom(String, Option<LanguageIdentifier>),
    MissingSource(MissingSourceError),
    #[cfg(feature = "download")]
    Download(download::Error),
    Poison,
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

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`Error::Io`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

/// To help with debugging, JSON errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`Error::Json`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(serde_json::error::Error, P)> for Error {
    fn from(pieces: (serde_json::error::Error, P)) -> Self {
        Self::Json(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

/// To help with debugging, string errors should be paired with a locale.
/// If a locale is unavailable, create the error directly: [`Error::Custom`]`(err, `[`None`]`)`
impl<L: AsRef<LanguageIdentifier>> From<(String, L)> for Error {
    fn from(pieces: (String, L)) -> Self {
        Self::Custom(pieces.0, Some(pieces.1.as_ref().clone()))
    }
}

/// To help with debugging, string errors should be paired with a locale.
/// If a locale is unavailable, create the error directly: [`Error::Custom`]`(err, `[`None`]`)`
impl<L: AsRef<LanguageIdentifier>> From<(&'static str, L)> for Error {
    fn from(pieces: (&'static str, L)) -> Self {
        Self::Custom(pieces.0.to_string(), Some(pieces.1.as_ref().clone()))
    }
}

impl From<MissingSourceError> for Error {
    fn from(err: MissingSourceError) -> Self {
        Self::MissingSource(err)
    }
}

#[cfg(feature = "download")]
impl From<download::Error> for Error {
    fn from(err: download::Error) -> Error {
        match err {
            download::Error::Io(err, path) => Error::Io(err, path),
            _ => Error::Download(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err, Some(path)) => write!(f, "{}: {:?}", err, path),
            Self::Io(err, None) => err.fmt(f),
            Self::Json(err, Some(path)) => write!(f, "JSON error: {}: {:?}", err, path),
            Self::Json(err, None) => write!(f, "JSON error: {}", err),
            Self::Custom(s, Some(langid)) => write!(f, "{}: {:?}", s, langid),
            Self::Custom(s, None) => write!(f, "{}", s),
            Self::MissingSource(err) => err.fmt(f),
            #[cfg(feature = "download")]
            Self::Download(err) => err.fmt(f),
            Self::Poison => write!(f, "poisoned lock on CLDR provider"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err, _) => Some(err),
            Self::Json(err, _) => Some(err),
            #[cfg(feature = "download")]
            Self::Download(err) => Some(err),
            _ => None,
        }
    }
}
