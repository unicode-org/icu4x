// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Debug)]
pub enum RetrievalError {
    /// Unable to retrieve the locale
    NullLocale,

    /// Unable to retrieve the calendar
    NullCalendar,

    /// Received NULL Pointer
    NullPointer,

    /// Error converting into `&CStr` to `&str`
    ConversionError,

    /// UnknownCategory when retrieving locale for linux
    UnknownCategory,

    Other(String),
}
