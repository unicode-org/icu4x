// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_provider::DataError;
use std::path::{Path, PathBuf};

#[derive(Display, Debug)]
#[non_exhaustive]
pub enum Error {
    #[displaydoc("{0}: {1:?}")]
    Io(std::io::Error, Option<PathBuf>),
    #[displaydoc("{0}")]
    DataProvider(DataError),
    #[displaydoc("Deserializer error: {0}: {1:?}")]
    Deserializer(String, Option<PathBuf>),
    #[cfg(feature = "export")]
    #[displaydoc("Serializer error: {0}: {1:?}")]
    Serializer(erased_serde::Error, Option<PathBuf>),
}

impl std::error::Error for Error {}

impl From<DataError> for Error {
    fn from(e: DataError) -> Self {
        Error::DataProvider(e)
    }
}

impl From<icu_provider::serde::Error> for Error {
    fn from(e: icu_provider::serde::Error) -> Self {
        Error::DataProvider(e.into())
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
/// If a path is unavailable, create the error directly: [`Error::Deserializer`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(serde_json_core::de::Error, P)> for Error {
    fn from(pieces: (serde_json_core::de::Error, P)) -> Self {
        Self::Deserializer(
            format!("{}", pieces.0),
            Some(pieces.1.as_ref().to_path_buf()),
        )
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

impl From<Error> for DataError {
    fn from(err: Error) -> Self {
        use Error::*;
        match err {
            Io(e, Some(path_buf)) => DataError::from(e).with_path(&path_buf),
            Io(e, None) => DataError::from(e),
            DataProvider(e) => e,
            Deserializer(s, Some(path_buf)) => DataError::custom("FS: Deserializer")
                .with_display_context(&s)
                .with_path(&path_buf),
            Deserializer(s, None) => DataError::custom("FS: Deserializer").with_display_context(&s),
            #[cfg(feature = "export")]
            Serializer(e, Some(path_buf)) => DataError::custom("FS: Serializer")
                .with_error_context(&e)
                .with_path(&path_buf),
            #[cfg(feature = "export")]
            Serializer(e, None) => DataError::custom("FS: Serializer").with_display_context(&e),
        }
    }
}
