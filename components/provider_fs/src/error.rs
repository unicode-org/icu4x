// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use crate::manifest::SyntaxOption;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error, Option<PathBuf>),
    DataProvider(icu_provider::DataError),
    Deserializer(Box<dyn std::error::Error>, Option<PathBuf>),
    #[cfg(feature = "export")]
    Serializer(erased_serde::Error, Option<PathBuf>),
    UnknownSyntax(SyntaxOption),
}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: `Error::Io(err, None)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

/// To help with debugging, JSON errors should be paired with a file path.
/// If a path is unavailable, create the error directly: `Error::Json(err, None)`
impl<P: AsRef<Path>> From<(serde_json::error::Error, P)> for Error {
    fn from(pieces: (serde_json::error::Error, P)) -> Self {
        Self::Deserializer(Box::new(pieces.0), Some(pieces.1.as_ref().to_path_buf()))
    }
}

impl From<icu_provider::DataError> for Error {
    fn from(err: icu_provider::DataError) -> Self {
        Self::DataProvider(err)
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
    /// Conversion from serializers::Error when the path is unavailable
    #[cfg(feature = "export")]
    pub fn from_serializers_error(err: crate::export::serializers::Error) -> Self {
        use crate::export::serializers::Error;
        match err {
            Error::Io(err) => Self::Io(err, None),
            Error::Serializer(err) => Self::Serializer(err, None),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err, Some(path)) => write!(f, "{}: {:?}", err, path),
            Self::Io(err, None) => err.fmt(f),
            Self::DataProvider(err) => err.fmt(f),
            Self::Deserializer(err, Some(filename)) => {
                write!(f, "Deserializer error: {}: {:?}", err, filename)
            }
            Self::Deserializer(err, None) => write!(f, "Deserializer error: {}", err),
            #[cfg(feature = "export")]
            Self::Serializer(err, Some(filename)) => {
                write!(f, "Serializer error: {}: {:?}", err, filename)
            }
            #[cfg(feature = "export")]
            Self::Serializer(err, None) => write!(f, "Serializer error: {}", err),
            Self::UnknownSyntax(syntax_option) => write!(
                f,
                "Unknown syntax {:?}. Do you need to enable a feature?",
                syntax_option
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err, _) => Some(err),
            Self::DataProvider(err) => Some(err),
            Self::Deserializer(err, _) => Some(err.as_ref()),
            #[cfg(feature = "export")]
            Self::Serializer(err, _) => Some(err),
            _ => None,
        }
    }
}
