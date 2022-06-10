// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::ffi::ICU4XError;
use core::fmt;
use icu_provider::{DataError, DataErrorKind};

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

        /// Attempted to construct an invalid data struct
        DataStructValidityError = 4,

        // See DataError
        DataMissingResourceKeyError = 5,
        DataMissingVariantError = 6,
        DataMissingLocaleError = 7,
        DataMissingResourceOptionsError = 8,
        DataNeedsVariantError = 9,
        DataNeedsLocaleError = 10,
        DataExtraneousResourceOptionsError = 11,
        DataFilteredResourceError = 12,
        DataMismatchedTypeError = 13,
        DataMissingPayloadError = 14,
        DataInvalidStateError = 15,
        DataCustomError = 16,
        DataIoError = 17,
        DataUnavailableBufferFormatError = 18,
    }
}

impl From<fmt::Error> for ICU4XError {
    fn from(_: fmt::Error) -> Self {
        ICU4XError::WriteableError
    }
}

impl From<DataError> for ICU4XError {
    fn from(e: DataError) -> Self {
        match e.kind {
            DataErrorKind::MissingResourceKey => ICU4XError::DataMissingResourceKeyError,
            DataErrorKind::MissingVariant => ICU4XError::DataMissingVariantError,
            DataErrorKind::MissingLocale => ICU4XError::DataMissingLocaleError,
            DataErrorKind::MissingResourceOptions => ICU4XError::DataMissingResourceOptionsError,
            DataErrorKind::NeedsVariant => ICU4XError::DataNeedsVariantError,
            DataErrorKind::NeedsLocale => ICU4XError::DataNeedsLocaleError,
            DataErrorKind::ExtraneousResourceOptions => {
                ICU4XError::DataExtraneousResourceOptionsError
            }
            DataErrorKind::FilteredResource => ICU4XError::DataFilteredResourceError,
            DataErrorKind::MismatchedType(..) => ICU4XError::DataMismatchedTypeError,
            DataErrorKind::MissingPayload => ICU4XError::DataMissingPayloadError,
            DataErrorKind::InvalidState => ICU4XError::DataInvalidStateError,
            DataErrorKind::Custom => ICU4XError::DataCustomError,
            #[cfg(feature = "provider_fs")]
            DataErrorKind::Io(..) => ICU4XError::DataIoError,
            // datagen only
            // DataErrorKind::MissingSourceData(..) => ..,
            DataErrorKind::UnavailableBufferFormat(..) => {
                ICU4XError::DataUnavailableBufferFormatError
            }
            _ => ICU4XError::UnknownError,
        }
    }
}
