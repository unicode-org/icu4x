// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Display, Debug)]
pub enum Error {
    #[displaydoc("{0}: {1:?}")]
    Io(io::Error, Option<PathBuf>),
    #[displaydoc("{0}")]
    Reqwest(reqwest::Error),
    #[displaydoc("HTTP request failed: {0}: {1}")]
    HttpStatus(reqwest::StatusCode, String),
    #[displaydoc("dirs::cache_dir() returned None")]
    NoCacheDir,
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`Error::Io`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}
