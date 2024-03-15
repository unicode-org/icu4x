// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::ffi::ICU4XError;
use core::fmt;
use icu_provider::{DataError, DataErrorKind};

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    /// A common enum for errors that ICU4X may return, organized by API
    ///
    /// The error names are stable and can be checked against as strings in the JS API
    #[diplomat::rust_link(fixed_decimal::FixedDecimalError, Enum, compact)]
    #[diplomat::rust_link(icu::calendar::CalendarError, Enum, compact)]
    #[diplomat::rust_link(icu::datetime::DateTimeError, Enum, compact)]
    #[diplomat::rust_link(icu::datetime::MismatchedCalendarError, Struct, hidden)]
    #[diplomat::rust_link(icu::locid::ParseError, Enum, compact)]
    #[diplomat::rust_link(icu::provider::DataError, Struct, compact)]
    #[diplomat::rust_link(icu::provider::DataErrorKind, Enum, compact)]
    #[diplomat::rust_link(icu::timezone::InvalidOffsetError, Struct, compact)]
    pub enum ICU4XError {
        // general errors
        /// The error is not currently categorized as ICU4XError.
        /// Please file a bug
        UnknownError = 0x00,
        /// An error arising from writing to a string
        /// Typically found when not enough space is allocated
        /// Most APIs that return a string may return this error
        WriteableError = 0x01,
        // Some input was out of bounds
        OutOfBoundsError = 0x02,

        // general data errors
        // See DataError
        DataMissingDataKeyError = 0x1_00,
        DataMissingVariantError = 0x1_01,
        DataMissingLocaleError = 0x1_02,
        DataNeedsVariantError = 0x1_03,
        DataNeedsLocaleError = 0x1_04,
        DataExtraneousLocaleError = 0x1_05,
        DataFilteredResourceError = 0x1_06,
        DataMismatchedTypeError = 0x1_07,
        DataMissingPayloadError = 0x1_08,
        DataInvalidStateError = 0x1_09,
        DataCustomError = 0x1_0A,
        DataIoError = 0x1_0B,
        DataUnavailableBufferFormatError = 0x1_0C,
        DataMismatchedAnyBufferError = 0x1_0D,

        // locale errors
        /// The subtag being requested was not set
        LocaleUndefinedSubtagError = 0x2_00,
        /// The locale or subtag string failed to parse
        LocaleParserLanguageError = 0x2_01,
        LocaleParserSubtagError = 0x2_02,
        LocaleParserExtensionError = 0x2_03,

        // data struct errors
        /// Attempted to construct an invalid data struct
        DataStructValidityError = 0x3_00,

        // property errors
        PropertyUnexpectedPropertyNameError = 0x4_02,

        // fixed_decimal errors
        FixedDecimalLimitError = 0x5_00,
        FixedDecimalSyntaxError = 0x5_01,

        // plural errors
        PluralsParseError = 0x6_00,

        // datetime errors
        CalendarParseError = 0x7_00,
        CalendarOverflowError = 0x7_01,
        CalendarUnderflowError = 0x7_02,
        CalendarOutOfRangeError = 0x7_03,
        CalendarUnknownEraError = 0x7_04,
        CalendarUnknownMonthCodeError = 0x7_05,
        CalendarMissingInputError = 0x7_06,
        CalendarUnknownKindError = 0x7_07,
        CalendarMissingError = 0x7_08,

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

        // tinystr errors
        TinyStrTooLargeError = 0x9_00,
        TinyStrContainsNullError = 0x9_01,
        TinyStrNonAsciiError = 0x9_02,

        // timezone errors
        TimeZoneInvalidOffsetError = 0xA_01,
        TimeZoneMissingInputError = 0xA_02,
        TimeZoneInvalidIdError = 0xA_03,
    }
}

impl ICU4XError {
    #[cfg(feature = "logging")]
    #[inline]
    pub(crate) fn log_original<T: core::fmt::Display + ?Sized>(self, e: &T) -> Self {
        use core::any;
        log::warn!(
            "Returning ICU4XError::{:?} based on original {}: {}",
            self,
            any::type_name::<T>(),
            e
        );
        self
    }

    #[cfg(not(feature = "logging"))]
    #[inline]
    pub(crate) fn log_original<T: core::fmt::Display + ?Sized>(self, _e: &T) -> Self {
        self
    }
}

impl From<fmt::Error> for ICU4XError {
    fn from(e: fmt::Error) -> Self {
        ICU4XError::WriteableError.log_original(&e)
    }
}

impl From<DataError> for ICU4XError {
    fn from(e: DataError) -> Self {
        match e.kind {
            DataErrorKind::MissingDataKey => ICU4XError::DataMissingDataKeyError,
            DataErrorKind::MissingLocale => ICU4XError::DataMissingLocaleError,
            DataErrorKind::NeedsLocale => ICU4XError::DataNeedsLocaleError,
            DataErrorKind::ExtraneousLocale => ICU4XError::DataExtraneousLocaleError,
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
        .log_original(&e)
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
        .log_original(&e)
    }
}

#[cfg(any(
    feature = "icu_datetime",
    feature = "icu_timezone",
    feature = "icu_calendar"
))]
impl From<icu_calendar::CalendarError> for ICU4XError {
    fn from(e: icu_calendar::CalendarError) -> Self {
        match e {
            icu_calendar::CalendarError::Parse => ICU4XError::CalendarParseError,
            icu_calendar::CalendarError::Overflow { field: _, max: _ } => {
                ICU4XError::CalendarOverflowError
            }
            icu_calendar::CalendarError::Underflow { field: _, min: _ } => {
                ICU4XError::CalendarUnderflowError
            }
            icu_calendar::CalendarError::OutOfRange => ICU4XError::CalendarOutOfRangeError,
            icu_calendar::CalendarError::UnknownEra(..) => ICU4XError::CalendarUnknownEraError,
            icu_calendar::CalendarError::UnknownMonthCode(..) => {
                ICU4XError::CalendarUnknownMonthCodeError
            }
            icu_calendar::CalendarError::MissingInput(_) => ICU4XError::CalendarMissingInputError,
            icu_calendar::CalendarError::UnknownAnyCalendarKind(_) => {
                ICU4XError::CalendarUnknownKindError
            }
            icu_calendar::CalendarError::MissingCalendar => ICU4XError::CalendarMissingError,
            _ => ICU4XError::UnknownError,
        }
        .log_original(&e)
    }
}

#[cfg(feature = "icu_datetime")]
impl From<icu_datetime::DateTimeError> for ICU4XError {
    fn from(e: icu_datetime::DateTimeError) -> Self {
        match e {
            icu_datetime::DateTimeError::Pattern(_) => ICU4XError::DateTimePatternError,
            icu_datetime::DateTimeError::Format(err) => err.into(),
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
            icu_datetime::DateTimeError::DateTimeInput(err) => err.into(),
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
        .log_original(&e)
    }
}

#[cfg(feature = "icu_decimal")]
impl From<fixed_decimal::FixedDecimalError> for ICU4XError {
    fn from(e: fixed_decimal::FixedDecimalError) -> Self {
        match e {
            fixed_decimal::FixedDecimalError::Limit => ICU4XError::FixedDecimalLimitError,
            fixed_decimal::FixedDecimalError::Syntax => ICU4XError::FixedDecimalSyntaxError,
            _ => ICU4XError::UnknownError,
        }
        .log_original(&e)
    }
}

impl From<icu_locid::ParseError> for ICU4XError {
    fn from(e: icu_locid::ParseError) -> Self {
        match e {
            icu_locid::ParseError::InvalidLanguage => ICU4XError::LocaleParserLanguageError,
            icu_locid::ParseError::InvalidSubtag => ICU4XError::LocaleParserSubtagError,
            icu_locid::ParseError::InvalidExtension => ICU4XError::LocaleParserExtensionError,
            icu_locid::ParseError::DuplicatedExtension => ICU4XError::LocaleParserExtensionError,
            _ => ICU4XError::UnknownError,
        }
        .log_original(&e)
    }
}

impl From<tinystr::TinyStrError> for ICU4XError {
    fn from(e: tinystr::TinyStrError) -> Self {
        match e {
            tinystr::TinyStrError::TooLarge { .. } => ICU4XError::TinyStrTooLargeError,
            tinystr::TinyStrError::ContainsNull => ICU4XError::TinyStrContainsNullError,
            tinystr::TinyStrError::NonAscii => ICU4XError::TinyStrNonAsciiError,
            _ => ICU4XError::UnknownError,
        }
        .log_original(&e)
    }
}

#[cfg(any(feature = "icu_timezone", feature = "icu_datetime"))]
impl From<icu_timezone::InvalidOffsetError> for ICU4XError {
    fn from(e: icu_timezone::InvalidOffsetError) -> Self {
        ICU4XError::TimeZoneInvalidOffsetError.log_original(&e)
    }
}
