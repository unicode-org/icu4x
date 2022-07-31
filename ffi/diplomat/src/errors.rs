// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::ffi::ICU4XError;
use core::fmt;
use fixed_decimal::Error as DecimalError;
use icu_calendar::DateTimeError;
use icu_datetime::TypedDateTimeFormatterError;
use icu_decimal::FixedDecimalFormatterError;
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

        // locale errors
        /// The subtag being requested was not set
        LocaleUndefinedSubtagError = 0x2_00,
        /// The locale or subtag string failed to parse
        LocaleParserError = 0x2_01,

        // data struct errors
        /// Attempted to construct an invalid data struct
        DataStructValidityError = 0x3_00,

        // property errors
        PropertyUnknownScriptIdError = 0x4_00,
        PropertyUnknownGeneralCategoryGroupError = 0x4_01,

        // decimal errors
        DecimalLimitError = 0x5_00,
        DecimalSyntaxError = 0x5_01,

        // plural errors
        PluralParserError = 0x6_00,

        // datetime errors
        DateTimeParseError = 0x7_00,
        DateTimeOverflowError = 0x7_01,
        DateTimeUnderflowError = 0x7_02,
        DateTimeOutOfRangeError = 0x7_03,
        DateTimeMissingInputError = 0x7_04,

        // datetime format errors
        DateTimeFormatPatternError = 0x8_00,
        DateTimeFormatMissingInputFieldError = 0x8_01,
        DateTimeFormatSkeletonError = 0x8_02,
        DateTimeFormatUnsupportedFieldError = 0x8_03,
        DateTimeFormatUnsupportedOptionsError = 0x8_04,
        DateTimeFormatMissingWeekdaySymbolError = 0x8_05,
        DateTimeFormatMissingMonthSymbolError = 0x8_06,
        DateTimeFormatFixedDecimalError = 0x8_07,
        DateTimeFormatMismatchedAnyCalendarError = 0x8_08,
        DateTimeFormatMismatchedCalendarLocaleError = 0x8_09,
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

impl From<DateTimeError> for ICU4XError {
    fn from(e: DateTimeError) -> Self {
        match e {
            DateTimeError::Parse => ICU4XError::DateTimeParseError,
            DateTimeError::Overflow { field: _, max: _ } => ICU4XError::DateTimeOverflowError,
            DateTimeError::Underflow { field: _, min: _ } => ICU4XError::DateTimeUnderflowError,
            DateTimeError::OutOfRange => ICU4XError::DateTimeOutOfRangeError,
            DateTimeError::MissingInput(_) => ICU4XError::DateTimeMissingInputError,
            _ => ICU4XError::UnknownError,
        }
    }
}

impl From<TypedDateTimeFormatterError> for ICU4XError {
    fn from(e: TypedDateTimeFormatterError) -> Self {
        match e {
            TypedDateTimeFormatterError::Pattern(_) => ICU4XError::DateTimeFormatPatternError,
            TypedDateTimeFormatterError::Format(err) => err.into(),
            TypedDateTimeFormatterError::DataProvider(err) => err.into(),
            TypedDateTimeFormatterError::MissingInputField(_) => {
                ICU4XError::DateTimeFormatMissingInputFieldError
            }
            TypedDateTimeFormatterError::Skeleton(_) => ICU4XError::DateTimeFormatSkeletonError,
            TypedDateTimeFormatterError::UnsupportedField(_) => {
                ICU4XError::DateTimeFormatUnsupportedFieldError
            }
            TypedDateTimeFormatterError::UnsupportedOptions => {
                ICU4XError::DateTimeFormatUnsupportedOptionsError
            }
            TypedDateTimeFormatterError::PluralRules(err) => err.into(),
            TypedDateTimeFormatterError::DateTimeInput(err) => err.into(),
            TypedDateTimeFormatterError::MissingWeekdaySymbol(_) => {
                ICU4XError::DateTimeFormatMissingWeekdaySymbolError
            }
            TypedDateTimeFormatterError::MissingMonthSymbol(_) => {
                ICU4XError::DateTimeFormatMissingMonthSymbolError
            }
            TypedDateTimeFormatterError::FixedDecimal => {
                ICU4XError::DateTimeFormatFixedDecimalError
            }
            TypedDateTimeFormatterError::FixedDecimalFormatter(err) => err.into(),
            TypedDateTimeFormatterError::MismatchedAnyCalendar(_, _) => {
                ICU4XError::DateTimeFormatMismatchedAnyCalendarError
            }
            TypedDateTimeFormatterError::MismatchedCalendarLocale(_, _) => {
                ICU4XError::DateTimeFormatMismatchedCalendarLocaleError
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

impl From<FixedDecimalFormatterError> for ICU4XError {
    fn from(e: FixedDecimalFormatterError) -> Self {
        match e {
            FixedDecimalFormatterError::Data(e) => e.into(),
            _ => ICU4XError::UnknownError,
        }
    }
}
