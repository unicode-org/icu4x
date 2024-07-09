// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::ffi::ICU4XError;
use ffi::*;
use icu_provider::{DataError, DataErrorKind};

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    #[diplomat::rust_link(icu::provider::DataError, Struct, compact)]
    #[diplomat::rust_link(icu::provider::DataErrorKind, Enum, compact)]
    pub enum ICU4XDataError {
        Unknown = 0x00,
        MarkerNotFound = 0x01,
        IdentifierNotFound = 0x02,
        InvalidRequest = 0x03,
        InconsistentData = 0x04,
        Downcast = 0x05,
        Deserialize = 0x06,
        Custom = 0x07,
        Io = 0x08,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    #[diplomat::rust_link(icu::locale::ParseError, Enum, compact)]
    pub enum ICU4XLocaleParseError {
        Unknown = 0x00,
        Language = 0x01,
        Subtag = 0x02,
        Extension = 0x03,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    #[diplomat::rust_link(fixed_decimal::ParseError, Enum, compact)]
    #[cfg(any(feature = "icu_decimal", feature = "icu_plurals"))]
    pub enum ICU4XFixedDecimalParseError {
        Unknown = 0x00,
        Limit = 0x01,
        Syntax = 0x02,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    #[diplomat::rust_link(fixed_decimal::LimitError, Struct, compact)]
    #[cfg(feature = "icu_decimal")]
    pub enum ICU4XFixedDecimalLimitError {
        TodoZst,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    #[diplomat::rust_link(icu::calendar::RangeError, Struct, compact)]
    #[diplomat::rust_link(icu::calendar::DateError, Enum, compact)]
    #[cfg(any(
        feature = "icu_datetime",
        feature = "icu_timezone",
        feature = "icu_calendar"
    ))]
    pub enum ICU4XCalendarError {
        Unknown = 0x00,
        OutOfRange = 0x01,
        UnknownEra = 0x02,
        UnknownMonthCode = 0x03,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    #[diplomat::rust_link(icu::timezone::InvalidOffsetError, Struct, compact)]
    #[cfg(any(feature = "icu_datetime", feature = "icu_timezone"))]
    pub enum ICU4XTimeZoneInvalidOffsetError {
        TodoZst,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    #[cfg(any(feature = "icu_datetime", feature = "icu_timezone"))]
    pub enum ICU4XTimeZoneInvalidIdError {
        TodoZst,
    }

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    /// Legacy error
    // TODO(2.0): remove
    #[diplomat::rust_link(icu::datetime::DateTimeError, Enum, compact)]
    #[diplomat::rust_link(icu::datetime::MismatchedCalendarError, Struct, hidden)]
    #[diplomat::rust_link(icu::provider::DataError, Struct, compact)]
    #[diplomat::rust_link(icu::provider::DataErrorKind, Enum, compact)]
    pub enum ICU4XError {
        /// The error is not currently categorized as ICU4XError.
        /// Please file a bug
        UnknownError = 0x00,

        // general data errors
        // See DataError
        DataMissingDataMarkerError = 0x1_00,
        DataMissingLocaleError = 0x1_02,
        DataNeedsLocaleError = 0x1_04,
        DataExtraneousLocaleError = 0x1_05,
        DataFilteredResourceError = 0x1_06,
        DataMismatchedTypeError = 0x1_07,
        DataCustomError = 0x1_0A,
        DataIoError = 0x1_0B,
        DataUnavailableBufferFormatError = 0x1_0C,

        // property errors
        PropertyUnexpectedPropertyNameError = 0x4_02,

        // datetime format errors
        DateTimePatternError = 0x8_00,
        DateTimeMissingInputFieldError = 0x8_01,
        DateTimeSkeletonError = 0x8_02,
        DateTimeUnsupportedFieldError = 0x8_03,
        DateTimeUnsupportedOptionsError = 0x8_04,
        DateTimeMissingWeekdaySymbolError = 0x8_05,
        DateTimeMissingMonthSymbolError = 0x8_06,
        DateTimeFixedDecimalError = 0x8_07,
        DateTimeMismatchedCalendarError = 0x8_08,
    }
}

impl From<DataError> for ICU4XError {
    fn from(e: DataError) -> Self {
        match e.kind {
            DataErrorKind::MarkerNotFound => ICU4XError::DataMissingDataMarkerError,
            DataErrorKind::IdentifierNotFound => ICU4XError::DataMissingLocaleError,
            DataErrorKind::InvalidRequest => ICU4XError::DataExtraneousLocaleError,
            DataErrorKind::Downcast(..) => ICU4XError::DataMismatchedTypeError,
            DataErrorKind::Custom => ICU4XError::DataCustomError,
            #[cfg(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            ))]
            DataErrorKind::Io(..) => ICU4XError::DataIoError,
            _ => ICU4XError::UnknownError,
        }
    }
}

impl From<DataError> for ICU4XDataError {
    fn from(e: DataError) -> Self {
        match e.kind {
            DataErrorKind::MarkerNotFound => Self::MarkerNotFound,
            DataErrorKind::IdentifierNotFound => Self::IdentifierNotFound,
            DataErrorKind::InvalidRequest => Self::InvalidRequest,
            DataErrorKind::InconsistentData(..) => Self::InconsistentData,
            DataErrorKind::Downcast(..) => Self::Downcast,
            DataErrorKind::Deserialize => Self::Deserialize,
            DataErrorKind::Custom => Self::Custom,
            #[cfg(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            ))]
            DataErrorKind::Io(..) => Self::Io,
            _ => Self::Unknown,
        }
    }
}

#[cfg(feature = "icu_properties")]
impl From<icu_properties::UnexpectedPropertyNameOrDataError> for ICU4XError {
    fn from(e: icu_properties::UnexpectedPropertyNameOrDataError) -> Self {
        match e {
            icu_properties::UnexpectedPropertyNameOrDataError::Data(e) => e.into(),
            icu_properties::UnexpectedPropertyNameOrDataError::UnexpectedPropertyName => {
                ICU4XError::PropertyUnexpectedPropertyNameError
            }
        }
    }
}

#[cfg(any(
    feature = "icu_datetime",
    feature = "icu_timezone",
    feature = "icu_calendar"
))]
impl From<icu_calendar::RangeError> for ICU4XCalendarError {
    fn from(_: icu_calendar::RangeError) -> Self {
        Self::OutOfRange
    }
}

#[cfg(any(
    feature = "icu_datetime",
    feature = "icu_timezone",
    feature = "icu_calendar"
))]
impl From<icu_calendar::DateError> for ICU4XCalendarError {
    fn from(e: icu_calendar::DateError) -> Self {
        match e {
            icu_calendar::DateError::Range { .. } => Self::OutOfRange,
            icu_calendar::DateError::UnknownEra(..) => Self::UnknownEra,
            icu_calendar::DateError::UnknownMonthCode(..) => Self::UnknownMonthCode,
            _ => Self::Unknown,
        }
    }
}

#[cfg(feature = "icu_datetime")]
impl From<icu_datetime::DateTimeError> for ICU4XError {
    fn from(e: icu_datetime::DateTimeError) -> Self {
        match e {
            icu_datetime::DateTimeError::Pattern(_) => ICU4XError::DateTimePatternError,
            icu_datetime::DateTimeError::Data(err) => err.into(),
            icu_datetime::DateTimeError::MissingInputField(_) => {
                ICU4XError::DateTimeMissingInputFieldError
            }
            // TODO(#1324): Add back skeleton errors
            // DateTimeFormatterError::Skeleton(_) => ICU4XError::DateTimeFormatSkeletonError,
            icu_datetime::DateTimeError::UnsupportedField(_) => {
                ICU4XError::DateTimeUnsupportedFieldError
            }
            icu_datetime::DateTimeError::UnsupportedOptions => {
                ICU4XError::DateTimeUnsupportedOptionsError
            }
            icu_datetime::DateTimeError::MissingWeekdaySymbol(_) => {
                ICU4XError::DateTimeMissingWeekdaySymbolError
            }
            icu_datetime::DateTimeError::MissingMonthSymbol(_) => {
                ICU4XError::DateTimeMissingMonthSymbolError
            }
            icu_datetime::DateTimeError::FixedDecimal => ICU4XError::DateTimeFixedDecimalError,
            icu_datetime::DateTimeError::MismatchedAnyCalendar(_, _) => {
                ICU4XError::DateTimeMismatchedCalendarError
            }
            _ => ICU4XError::UnknownError,
        }
    }
}

#[cfg(any(feature = "icu_decimal", feature = "icu_plurals"))]
impl From<fixed_decimal::ParseError> for ICU4XFixedDecimalParseError {
    fn from(e: fixed_decimal::ParseError) -> Self {
        match e {
            fixed_decimal::ParseError::Limit => Self::Limit,
            fixed_decimal::ParseError::Syntax => Self::Syntax,
            _ => Self::Unknown,
        }
    }
}

#[cfg(feature = "icu_decimal")]
impl From<fixed_decimal::LimitError> for ICU4XFixedDecimalLimitError {
    fn from(_: fixed_decimal::LimitError) -> Self {
        Self::TodoZst
    }
}

impl From<icu_locale_core::ParseError> for ICU4XLocaleParseError {
    fn from(e: icu_locale_core::ParseError) -> Self {
        match e {
            icu_locale_core::ParseError::InvalidLanguage => Self::Language,
            icu_locale_core::ParseError::InvalidSubtag => Self::Subtag,
            icu_locale_core::ParseError::InvalidExtension => Self::Extension,
            icu_locale_core::ParseError::DuplicatedExtension => Self::Extension,
            _ => Self::Unknown,
        }
    }
}

#[cfg(any(feature = "icu_timezone", feature = "icu_datetime"))]
impl From<icu_timezone::InvalidOffsetError> for ICU4XTimeZoneInvalidOffsetError {
    fn from(_: icu_timezone::InvalidOffsetError) -> Self {
        Self::TodoZst
    }
}
