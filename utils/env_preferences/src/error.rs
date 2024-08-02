// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::str::Utf8Error;

#[derive(Debug)]
pub enum RetrievalError {
    /// Unable to retrieve the locale
    NullLocale,

    /// Unable to retrieve the calendar
    NullCalendar,

    /// Received NULL Pointer
    NullPointer,

    /// Error converting into `&CStr` to `&str`
    ConversionError(Utf8Error),

    /// UnknownCategory when retrieving locale for linux
    UnknownCategory,

    /// Error handling for windows system
    #[cfg(target_os = "windows")]
    Windows(windows::core::Error),

    Other(String),
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for RetrievalError {
    fn from(input: windows::core::Error) -> Self {
        Self::Windows(input)
    }
}

impl From<Utf8Error> for RetrievalError {
    fn from(input: Utf8Error) -> Self {
        Self::ConversionError(input)
    }
}
