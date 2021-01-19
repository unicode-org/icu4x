// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::fmt;

#[derive(Debug)]
pub enum Error {
    PpucdParse(PpucdParseError),
    UnisetConversion(icu_uniset::UnicodeSetError),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PpucdParse(err) => err.fmt(f),
            Self::UnisetConversion(err) => err.fmt(f),
        }
    }
}
