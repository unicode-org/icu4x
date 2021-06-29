// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::manifest::SyntaxOption;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}: {1:?}")]
    Io(#[source] std::io::Error, Option<PathBuf>),
    #[error(transparent)]
    DataProvider(#[from] icu_provider::DataError),
    #[error("Deserializer error: {0}: {1:?}")]
    Deserializer(
        #[source] Box<dyn std::error::Error + Send + Sync>,
        Option<PathBuf>,
    ),
    #[cfg(feature = "export")]
    #[error("Serializer error: {0}: {1:?}")]
    Serializer(#[source] erased_serde::Error, Option<PathBuf>),
    #[error("Unknown syntax {0:?}. Do you need to enable a feature?")]
    UnknownSyntax(SyntaxOption),
}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`Error::Io`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

/// To help with debugging, JSON errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`Error::Deserializer`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(serde_json::error::Error, P)> for Error {
    fn from(pieces: (serde_json::error::Error, P)) -> Self {
        Self::Deserializer(Box::new(pieces.0), Some(pieces.1.as_ref().to_path_buf()))
    }
}

#[cfg(feature = "export")]
impl<P: AsRef<Path>> From<(crate::export::serializers::Error, P)> for Error {
    fn from(pieces: (crate::export::serializers::Error, P)) -> Self {
        use crate::export::serializers::Error;
        let path: Option<PathBuf> = Some(pieces.1.as_ref().to_path_buf());
        match pieces.0 {
            Error::Io(err) => Self::Io(err, path),
            Error::Serializer(err) => Self::Serializer(err, path),
        }
    }
}

impl Error {
    /// Conversion from [`serializers::Error`](crate::export::serializers::Error) when the path is unavailable
    #[cfg(feature = "export")]
    pub fn from_serializers_error(err: crate::export::serializers::Error) -> Self {
        use crate::export::serializers::Error;
        match err {
            Error::Io(err) => Self::Io(err, None),
            Error::Serializer(err) => Self::Serializer(err, None),
        }
    }
}

impl From<Error> for icu_provider::DataError {
    fn from(err: Error) -> Self {
        Self::Resource(Box::new(err))
    }
}
