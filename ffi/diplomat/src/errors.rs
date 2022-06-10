// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::ffi::ICU4XError;
use core::fmt;
use fixed_decimal::Error as DecimalError;
use icu_plurals::PluralRulesError;
use icu_properties::PropertiesError;
use icu_provider::{DataError, DataErrorKind};

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[repr(C)]
    /// A common enum for errors that ICU4X may return, organized by API
    ///
    /// The error names are stable and can be checked against as strings in the JS API
    pub enum ICU4XError {
        // general errors
        /// The error is not currently categorized as ICU4XError.
        /// Please file a bug
        UnknownError = 0,
        /// An error arising from writing to a string
        /// Typically found when not enough space is allocated
        /// Most APIs that return a string may return this error
        WriteableError = 1,

        // Some input was out of bounds
        OutOfBoundsError = 2,

        // general data errors
        // See DataError
        DataMissingResourceKeyError = 10,
        DataMissingVariantError = 11,
        DataMissingLocaleError = 12,
        DataMissingResourceOptionsError = 13,
        DataNeedsVariantError = 14,
        DataNeedsLocaleError = 15,
        DataExtraneousResourceOptionsError = 16,
        DataFilteredResourceError = 17,
        DataMismatchedTypeError = 18,
        DataMissingPayloadError = 19,
        DataInvalidStateError = 20,
        DataCustomError = 21,
        DataIoError = 22,
        DataUnavailableBufferFormatError = 23,

        // locale and data struct errors
        /// The subtag being requested was not set
        LocaleUndefinedSubtagError = 31,
        /// The locale or subtag string failed to parse
        LocaleParserError = 32,
        /// Attempted to construct an invalid data struct
        DataStructValidityError = 33,

        // property and decimal errors
        PropertyUnknownScriptIdError = 40,
        PropertyUnknownGeneralCategoryGroupError = 41,
        DecimalLimitError = 42,
        DecimalSyntaxError = 43,

        // plural errors
        PluralParserError = 50,
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
            #[cfg(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            ))]
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

impl From<PropertiesError> for ICU4XError {
    fn from(e: PropertiesError) -> Self {
        match e {
            PropertiesError::PropDataLoad(e) => e.into(),
            PropertiesError::UnknownScriptId(..) => ICU4XError::PropertyUnknownScriptIdError,
            PropertiesError::UnknownGeneralCategoryGroup(..) => {
                ICU4XError::PropertyUnknownGeneralCategoryGroupError
            }
            _ => ICU4XError::UnknownError,
        }
    }
}

impl From<DecimalError> for ICU4XError {
    fn from(e: DecimalError) -> Self {
        match e {
            DecimalError::Limit => ICU4XError::DecimalLimitError,
            DecimalError::Syntax => ICU4XError::DecimalSyntaxError,
        }
    }
}

impl From<PluralRulesError> for ICU4XError {
    fn from(e: PluralRulesError) -> Self {
        match e {
            PluralRulesError::DataProvider(e) => e.into(),
            PluralRulesError::Parser(..) => ICU4XError::PluralParserError,
            _ => ICU4XError::UnknownError,
        }
    }
}
