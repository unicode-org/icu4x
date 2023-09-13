// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use displaydoc::Display;

extern crate alloc;

pub mod helpers;
pub mod provider;

#[derive(Display, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    #[displaydoc("Magnitude or number of digits exceeded")]
    Limit,

    #[displaydoc("The input is not valid")]
    InvalidInput,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
