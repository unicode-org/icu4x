// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use std::path::PathBuf;

#[non_exhaustive]
#[derive(Display, Debug)]
pub enum Error {
    #[displaydoc("{0}: {1:?}")]
    Io(std::io::Error, PathBuf),
    #[displaydoc("{0}: {1:?}")]
    Toml(toml::de::Error, PathBuf),
    #[displaydoc("Invalid range: {0}-{1}")]
    InvalidCharRange(u32, u32),
}

impl std::error::Error for Error {}
