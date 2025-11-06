// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locale_core::ParseError;
use std::error::Error;
use std::{ffi::FromVecWithNulError, str::Utf8Error};

/// An error encountered while retrieving the host information
#[derive(Debug, Display)]
pub enum HostInfoError {
    #[displaydoc("Error converting into `&CStr` to `&str`")]
    Conversion(Utf8Error),

    #[displaydoc("Error creating a `CString` from a buffer with a null terminator")]
    FromVecWithNul(FromVecWithNulError),

    #[displaydoc("No matching backend has been identified")]
    UnavailableBackend,

    #[displaydoc("Unknown category when retrieving locale category for linux")]
    UnknownCategory,

    #[cfg(target_os = "windows")]
    #[displaydoc("Windows error: {0}")]
    Windows(windows::core::Error),

    #[displaydoc("Host locale parsing error")]
    HostLocaleError,

    #[displaydoc("Failed to parse region")]
    UnknownRegion,

    #[displaydoc("Failed to parse a locale: {0}")]
    LocaleParse(ParseError),
}

impl Error for HostInfoError {}

impl From<Utf8Error> for HostInfoError {
    fn from(input: Utf8Error) -> Self {
        Self::Conversion(input)
    }
}

impl From<FromVecWithNulError> for HostInfoError {
    fn from(input: FromVecWithNulError) -> Self {
        Self::FromVecWithNul(input)
    }
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for HostInfoError {
    fn from(input: windows::core::Error) -> Self {
        Self::Windows(input)
    }
}

impl From<ParseError> for HostInfoError {
    fn from(input: ParseError) -> Self {
        Self::LocaleParse(input)
    }
}
