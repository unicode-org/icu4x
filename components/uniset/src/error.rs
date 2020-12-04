// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error, Option<PathBuf>),
    PpucdParse(PpucdParseError),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PpucdParseError {
    pub src: &'static str,
}

impl From<PpucdParseError> for Error {
    fn from(err: PpucdParseError) -> Self {
        Self::PpucdParse(err)
    }
}

impl fmt::Display for PpucdParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse PPUCD file: {}", self.src)
    }
}

/// To help with debugging, I/O errors should be paired with a file path.
/// If a path is unavailable, create the error directly: `Error::Io(err, None)`
impl<P: AsRef<Path>> From<(std::io::Error, P)> for Error {
    fn from(pieces: (std::io::Error, P)) -> Self {
        Self::Io(pieces.0, Some(pieces.1.as_ref().to_path_buf()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err, Some(path)) => write!(f, "{}: {:?}", err, path),
            Self::Io(err, None) => err.fmt(f),
            Self::PpucdParse(err) => err.fmt(f),
        }
    }
}
