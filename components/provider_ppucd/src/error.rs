// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error, Option<PathBuf>),
    UnisetConversion(icu_uniset::UnicodeSetError),
}
