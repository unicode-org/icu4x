// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use std::{ffi::FromVecWithNulError, str::Utf8Error};

use crate::parse::posix::PosixParseError;

/// An error encountered while retrieving the system locale
#[derive(Debug, PartialEq)]
pub enum RetrievalError {
    /// Error converting into `&CStr` to `&str`
    ConversionError(Utf8Error),

    /// Error creating a `CString` from a buffer with a null terminator
    FromVecWithNulError(FromVecWithNulError),

    /// Unable to retrieve the calendar
    NullCalendar,

    /// Unable to retrieve the locale
    NullLocale,

    /// Unable to retrieve TimeZone
    NullTimeZone,

    /// UnknownCategory when retrieving locale for linux
    #[cfg(any(doc, target_os = "linux"))]
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

impl From<FromVecWithNulError> for RetrievalError {
    fn from(input: FromVecWithNulError) -> Self {
        Self::FromVecWithNulError(input)
    }
}

/// An error encountered while either retrieving or parsing a system locale
#[derive(Display, Debug, PartialEq)]
pub enum ParseError {
    #[displaydoc("Locale failed native parsing logic: {0}")]
    Posix(PosixParseError),
    #[displaydoc("Unable to parse ICU4X locale: {0}")]
    Icu(icu_locale_core::ParseError),
}

impl From<PosixParseError> for ParseError {
    fn from(value: PosixParseError) -> Self {
        Self::Posix(value)
    }
}

impl From<icu_locale_core::ParseError> for ParseError {
    fn from(value: icu_locale_core::ParseError) -> Self {
        Self::Icu(value)
    }
}

/// An error encountered while either retrieving or parsing a system locale
#[derive(Display, Debug)]
pub enum LocaleError {
    #[displaydoc("Unable to retrieve locales: {0:?}")]
    Retrieval(RetrievalError),
    #[displaydoc("Unable to parse locale: {0}")]
    Parse(ParseError),
}

impl From<RetrievalError> for LocaleError {
    fn from(value: RetrievalError) -> Self {
        Self::Retrieval(value)
    }
}

impl From<ParseError> for LocaleError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}
