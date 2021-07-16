// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

#[derive(Display, Debug)]
pub enum Error {
    #[displaydoc("{0}")]
    PpucdParse(PpucdParseError),
}

impl std::error::Error for Error {}


#[derive(Display, Debug, PartialEq, Copy, Clone)]
#[displaydoc("Could not parse PPUCD file: {src}")]
pub struct PpucdParseError {
    pub src: &'static str,
}

impl std::error::Error for Error {}


impl From<PpucdParseError> for Error {
    fn from(e: PpucdParseError) -> Self {
        Error::PpucdParse(e)
    }
}
