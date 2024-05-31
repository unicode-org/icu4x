// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::ffi::ICU4XError;
use icu_provider::{DataError, DataErrorKind};

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[derive(Debug, PartialEq, Eq)]
    #[repr(C)]
    /// A common enum for errors that ICU4X may return, organized by API
    ///
    /// The error names are stable and can be checked against as strings in the JS API
    #[diplomat::rust_link(fixed_decimal::ParseError, Enum, compact)]
    #[diplomat::rust_link(fixed_decimal::LimitError, Struct, compact)]
    #[diplomat::rust_link(icu::calendar::RangeError, Struct, compact)]
    #[diplomat::rust_link(icu::calendar::DateError, Enum, compact)]
    #[diplomat::rust_link(icu::datetime::DateTimeError, Enum, compact)]
    #[diplomat::rust_link(icu::datetime::MismatchedCalendarError, Struct, hidden)]
    #[diplomat::rust_link(icu::locale::ParseError, Enum, compact)]
    #[diplomat::rust_link(icu::provider::DataError, Struct, compact)]
    #[diplomat::rust_link(icu::provider::DataErrorKind, Enum, compact)]
    #[diplomat::rust_link(icu::timezone::InvalidOffsetError, Struct, compact)]
    #[diplomat::rust_link(icu_experimental::units::InvalidUnitError, Struct, compact)]
    pub enum ICU4XError {
        /// The error is not currently categorized as ICU4XError.
        /// Please file a bug
        UnknownError = 0x00,

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
        CalendarOutOfRangeError = 0x7_03,
        CalendarUnknownEraError = 0x7_04,
        CalendarUnknownMonthCodeError = 0x7_05,

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

        // timezone errors
        TimeZoneInvalidOffsetError = 0xA_01,
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
impl From<icu_calendar::RangeError> for ICU4XError {
    fn from(e: icu_calendar::RangeError) -> Self {
        ICU4XError::CalendarOutOfRangeError.log_original(&e)
    }
}

#[cfg(any(
    feature = "icu_datetime",
    feature = "icu_timezone",
    feature = "icu_calendar"
))]
impl From<icu_calendar::DateError> for ICU4XError {
    fn from(e: icu_calendar::DateError) -> Self {
        match e {
            icu_calendar::DateError::Range { .. } => ICU4XError::CalendarOutOfRangeError,
            icu_calendar::DateError::UnknownEra(..) => ICU4XError::CalendarUnknownEraError,
            icu_calendar::DateError::UnknownMonthCode(..) => {
                ICU4XError::CalendarUnknownMonthCodeError
            }
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
        .log_original(&e)
    }
}

#[cfg(feature = "icu_decimal")]
impl From<fixed_decimal::ParseError> for ICU4XError {
    fn from(e: fixed_decimal::ParseError) -> Self {
        match e {
            fixed_decimal::ParseError::Limit => ICU4XError::FixedDecimalLimitError,
            fixed_decimal::ParseError::Syntax => ICU4XError::FixedDecimalSyntaxError,
            _ => ICU4XError::UnknownError,
        }
        .log_original(&e)
    }
}

#[cfg(feature = "icu_decimal")]
impl From<fixed_decimal::LimitError> for ICU4XError {
    fn from(e: fixed_decimal::LimitError) -> Self {
        ICU4XError::FixedDecimalLimitError.log_original(&e)
    }
}

impl From<icu_locale_core::ParseError> for ICU4XError {
    fn from(e: icu_locale_core::ParseError) -> Self {
        match e {
            icu_locale_core::ParseError::InvalidLanguage => ICU4XError::LocaleParserLanguageError,
            icu_locale_core::ParseError::InvalidSubtag => ICU4XError::LocaleParserSubtagError,
            icu_locale_core::ParseError::InvalidExtension => ICU4XError::LocaleParserExtensionError,
            icu_locale_core::ParseError::DuplicatedExtension => {
                ICU4XError::LocaleParserExtensionError
            }
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
