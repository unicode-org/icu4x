// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locale_core::ParseError;
use std::{ffi::FromVecWithNulError, str::Utf8Error};

/// An error encountered while retrieving the system locale
#[derive(Debug)]
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
    UnknownCategory,

    /// Error handling for windows system
    #[cfg(target_os = "windows")]
    Windows(windows::core::Error),

    /// Errors from parsing POSIX locales
    #[cfg(any(feature = "parse_posix", target_os = "linux"))]
    Posix(crate::parse::posix::PosixParseError),

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
#[derive(Display, Debug)]
pub enum LocaleError {
    #[displaydoc("Unable to retrieve locales: {0:?}")]
    Retrieval(RetrievalError),
    #[displaydoc("Unable to parse locale: {0}")]
    Parse(ParseError),
}

#[cfg(any(feature = "parse_posix", target_os = "linux"))]
impl From<crate::parse::posix::PosixParseError> for LocaleError {
    fn from(value: crate::parse::posix::PosixParseError) -> Self {
        Self::Retrieval(RetrievalError::Posix(value))
    }
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
