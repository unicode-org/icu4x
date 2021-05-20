// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}: {1:?}")]
    Io(#[source] io::Error, Option<PathBuf>),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("HTTP request failed: {0}: {1}")]
    HttpStatus(reqwest::StatusCode, String),
    #[error("dirs::cache_dir() returned None")]
    NoCacheDir,
}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: [`Error::Io`]`(err, `[`None`]`)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}
