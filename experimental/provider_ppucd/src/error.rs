// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    PpucdParse(#[from] PpucdParseError),
}

#[derive(Error, Debug, PartialEq, Copy, Clone)]
#[error("Could not parse PPUCD file: {src}")]
pub struct PpucdParseError {
    pub src: &'static str,
}
