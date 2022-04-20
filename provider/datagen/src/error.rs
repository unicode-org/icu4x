// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locid::LanguageIdentifier;
use icu_provider::{DataError, DataErrorKind};
use std::path::{Path, PathBuf};

#[non_exhaustive]
#[derive(Display, Debug)]
pub enum DatagenError {
    #[displaydoc("{0}: {1:?}")]
    Io(std::io::Error, Option<PathBuf>),
    #[displaydoc("JSON error: {0}: {1:?}")]
    Json(serde_json::error::Error, Option<PathBuf>),
    #[displaydoc("TOML error: {0}: {1:?}")]
    Toml(toml::de::Error, Option<PathBuf>),
    #[displaydoc("{0}: {1:?}")]
    Custom(String, Option<LanguageIdentifier>),
    #[displaydoc("Missing CLDR data")]
    MissingCldrPaths,
    #[displaydoc("Missing Unicode properties data")]
    MissingUpropsPath,
}

impl std::error::Error for DatagenError {}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`DatagenError::Io`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for DatagenError {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

/// To help with debugging, JSON errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`DatagenError::Json`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(serde_json::error::Error, P)> for DatagenError {
    fn from(pieces: (serde_json::error::Error, P)) -> Self {
        Self::Json(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

/// To help with debugging, TOML errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`DatagenError::Toml`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(toml::de::Error, P)> for DatagenError {
    fn from(pieces: (toml::de::Error, P)) -> Self {
        Self::Toml(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

/// To help with debugging, string errors should be paired with a locale.
/// If a locale is unavailable, create the error directly: [`DatagenError::Custom`]`(err, `[`None`]`)`
impl<L: AsRef<LanguageIdentifier>> From<(String, L)> for DatagenError {
    fn from(pieces: (String, L)) -> Self {
        Self::Custom(pieces.0, Some(pieces.1.as_ref().clone()))
    }
}

/// To help with debugging, string errors should be paired with a locale.
/// If a locale is unavailable, create the error directly: [`DatagenError::Custom`]`(err, `[`None`]`)`
impl<L: AsRef<LanguageIdentifier>> From<(&'static str, L)> for DatagenError {
    fn from(pieces: (&'static str, L)) -> Self {
        Self::Custom(pieces.0.to_string(), Some(pieces.1.as_ref().clone()))
    }
}

impl From<DatagenError> for DataError {
    fn from(err: DatagenError) -> Self {
        use DatagenError::*;
        match err {
            Io(e, Some(path_buf)) => DataError::from(e).with_path(&path_buf),
            Io(e, None) => DataError::from(e),
            Json(e, Some(path_buf)) => DataError::custom("JSON Parse Error")
                .with_error_context(&e)
                .with_path(&path_buf),
            Json(e, None) => DataError::custom("JSON Parse Error").with_error_context(&e),
            Toml(e, Some(path_buf)) => DataError::custom("TOML Parse Error")
                .with_error_context(&e)
                .with_path(&path_buf),
            Toml(e, None) => DataError::custom("TOML Parse Error").with_error_context(&e),
            Custom(s, Some(langid)) => DataError::custom("")
                .with_display_context(&s)
                .with_display_context(&langid),
            Custom(s, None) => DataError::custom("").with_display_context(&s),
            MissingCldrPaths => DataError::custom("CLDR data not specified."),
            MissingUpropsPath => DataError::custom("Uprops data not specified."),
        }
    }
}

pub fn is_missing_cldr_error(e: DataError) -> bool {
    matches!(
        e,
        DataError {
            kind: DataErrorKind::Custom,
            str_context: Some("CLDR data not specified."),
            ..
        }
    )
}

pub fn is_missing_uprops_error(e: DataError) -> bool {
    matches!(
        e,
        DataError {
            kind: DataErrorKind::Custom,
            str_context: Some("Uprops data not specified."),
            ..
        }
    )
}
