// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[repr(C)]
    /// A common enum for errors that ICU4X may return, organized by API
    ///
    /// The error names are stable and can be checked against as strings in the JS API
    pub enum ICU4XError {
        /// The error is not currently categorized as ICU4XError.
        /// Please file a bug
        UnknownError = 0,
        /// An error arising from writing to a string
        /// Typically found when not enough space is allocated
        WriteableError = 1,

        /// The subtag being requested was not set
        LocaleUndefinedSubtagError = 2,

        /// The locale or subtag string failed to parse
        LocaleParserError = 3,
    }
}

impl From<fmt::Error> for ICU4XError {
    fn from(_: fmt::Error) -> Self {
        ICU4XError::WriteableError
    }
}
