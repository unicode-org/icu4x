// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use self::ffi::ICU4XError;
use core::fmt;
use fixed_decimal::Error as DecimalError;
use icu_calendar::DateTimeError;
use icu_collator::CollatorError;
use icu_datetime::DateTimeFormatterError;
use icu_decimal::FixedDecimalFormatterError;
use icu_locid::ParserError;
use icu_normalizer::NormalizerError;
use icu_plurals::PluralRulesError;
use icu_properties::PropertiesError;
use icu_provider::{DataError, DataErrorKind};
use icu_timezone::TimeZoneError;
use tinystr::TinyStrError;

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    /// A common enum for errors that ICU4X may return, organized by API
    ///
    /// The error names are stable and can be checked against as strings in the JS API
    #[diplomat::rust_link(fixed_decimal::Error, Enum, compact)]
    #[diplomat::rust_link(icu::calendar::DateTimeError, Enum, compact)]
    #[diplomat::rust_link(icu::datetime::DateTimeFormatterError, Enum, compact)]
    #[diplomat::rust_link(icu::locid::ParserError, Enum, compact)]
    #[diplomat::rust_link(icu::properties::PropertiesError, Enum, compact)]
    #[diplomat::rust_link(icu::plurals::PluralRulesError, Enum, compact)]
    #[diplomat::rust_link(icu::provider::DataError, Struct, compact)]
    #[diplomat::rust_link(icu::provider::DataErrorKind, Enum, compact)]
    #[diplomat::rust_link(icu::normalizer::NormalizerError, Enum, compact)]
    #[diplomat::rust_link(icu::timezone::TimeZoneError, Enum, compact)]
    #[diplomat::rust_link(icu::collator::CollatorError, Enum, compact)]
    #[diplomat::rust_link(icu::decimal::FixedDecimalFormatterError, Enum, compact)]
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
        PropertyUnknownScriptIdError = 0x4_00,
        PropertyUnknownGeneralCategoryGroupError = 0x4_01,

        // fixed_decimal errors
        FixedDecimalLimitError = 0x5_00,
        FixedDecimalSyntaxError = 0x5_01,

        // plural errors
        PluralParserError = 0x6_00,

        // datetime errors
        DateTimeParseError = 0x7_00,
        DateTimeOverflowError = 0x7_01,
        DateTimeUnderflowError = 0x7_02,
        DateTimeOutOfRangeError = 0x7_03,
        DateTimeUnknownEraError = 0x7_04,
        DateTimeUnknownMonthCodeError = 0x7_05,
        DateTimeMissingInputError = 0x7_06,
        DateTimeUnknownAnyCalendarKindError = 0x7_07,

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

        // tinystr errors
        TinyStrTooLargeError = 0x9_00,
        TinyStrContainsNullError = 0x9_01,
        TinyStrNonAsciiError = 0x9_02,

        // timezone errors
        TimeZoneOffsetOutOfBoundsError = 0xA_00,
        TimeZoneInvalidOffsetError = 0xA_01,
        TimeZoneMissingInputError = 0xA_02,

        // normalizer errors
        NormalizerFutureExtensionError = 0xB_00,
        NormalizerValidationError = 0xB_01,
    }
}

#[cfg(feature = "logging")]
#[inline]
pub(crate) fn log_conversion<T: core::fmt::Display>(e: &T, ffi_error: ICU4XError) {
    use core::any;
    log::warn!(
        "Returning ICU4XError::{:?} based on original {}: {}",
        ffi_error,
        any::type_name::<T>(),
        e
    );
}

#[cfg(not(feature = "logging"))]
#[inline]
pub(crate) fn log_conversion<T: core::fmt::Display>(_e: &T, _ffi_error: ICU4XError) {}

impl From<fmt::Error> for ICU4XError {
    fn from(e: fmt::Error) -> Self {
        log_conversion(&e, ICU4XError::WriteableError);
        ICU4XError::WriteableError
    }
}

impl From<DataError> for ICU4XError {
    fn from(e: DataError) -> Self {
        let ret = match e.kind {
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
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<CollatorError> for ICU4XError {
    fn from(e: CollatorError) -> Self {
        let ret = match e {
            CollatorError::NotFound => ICU4XError::DataMissingPayloadError,
            CollatorError::MalformedData => ICU4XError::DataInvalidStateError,
            CollatorError::DataProvider(_) => ICU4XError::DataIoError,
            _ => ICU4XError::DataIoError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<PropertiesError> for ICU4XError {
    fn from(e: PropertiesError) -> Self {
        let ret = match e {
            PropertiesError::PropDataLoad(e) => e.into(),
            PropertiesError::UnknownScriptId(..) => ICU4XError::PropertyUnknownScriptIdError,
            PropertiesError::UnknownGeneralCategoryGroup(..) => {
                ICU4XError::PropertyUnknownGeneralCategoryGroupError
            }
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<DateTimeError> for ICU4XError {
    fn from(e: DateTimeError) -> Self {
        let ret = match e {
            DateTimeError::Parse => ICU4XError::DateTimeParseError,
            DateTimeError::Overflow { field: _, max: _ } => ICU4XError::DateTimeOverflowError,
            DateTimeError::Underflow { field: _, min: _ } => ICU4XError::DateTimeUnderflowError,
            DateTimeError::OutOfRange => ICU4XError::DateTimeOutOfRangeError,
            DateTimeError::UnknownEra(..) => ICU4XError::DateTimeUnknownEraError,
            DateTimeError::UnknownMonthCode(..) => ICU4XError::DateTimeUnknownMonthCodeError,
            DateTimeError::MissingInput(_) => ICU4XError::DateTimeMissingInputError,
            DateTimeError::UnknownAnyCalendarKind(_) => {
                ICU4XError::DateTimeUnknownAnyCalendarKindError
            }
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<DateTimeFormatterError> for ICU4XError {
    fn from(e: DateTimeFormatterError) -> Self {
        let ret = match e {
            DateTimeFormatterError::Pattern(_) => ICU4XError::DateTimeFormatPatternError,
            DateTimeFormatterError::Format(err) => err.into(),
            DateTimeFormatterError::DataProvider(err) => err.into(),
            DateTimeFormatterError::MissingInputField(_) => {
                ICU4XError::DateTimeFormatMissingInputFieldError
            }
            // TODO(#1324): Add back skeleton errors
            // DateTimeFormatterError::Skeleton(_) => ICU4XError::DateTimeFormatSkeletonError,
            DateTimeFormatterError::UnsupportedField(_) => {
                ICU4XError::DateTimeFormatUnsupportedFieldError
            }
            DateTimeFormatterError::UnsupportedOptions => {
                ICU4XError::DateTimeFormatUnsupportedOptionsError
            }
            DateTimeFormatterError::PluralRules(err) => err.into(),
            DateTimeFormatterError::DateTimeInput(err) => err.into(),
            DateTimeFormatterError::MissingWeekdaySymbol(_) => {
                ICU4XError::DateTimeFormatMissingWeekdaySymbolError
            }
            DateTimeFormatterError::MissingMonthSymbol(_) => {
                ICU4XError::DateTimeFormatMissingMonthSymbolError
            }
            DateTimeFormatterError::FixedDecimal => ICU4XError::DateTimeFormatFixedDecimalError,
            DateTimeFormatterError::FixedDecimalFormatter(err) => err.into(),
            DateTimeFormatterError::MismatchedAnyCalendar(_, _) => {
                ICU4XError::DateTimeFormatMismatchedAnyCalendarError
            }
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<DecimalError> for ICU4XError {
    fn from(e: DecimalError) -> Self {
        let ret = match e {
            DecimalError::Limit => ICU4XError::FixedDecimalLimitError,
            DecimalError::Syntax => ICU4XError::FixedDecimalSyntaxError,
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<PluralRulesError> for ICU4XError {
    fn from(e: PluralRulesError) -> Self {
        let ret = match e {
            PluralRulesError::DataProvider(e) => e.into(),
            PluralRulesError::Parser(..) => ICU4XError::PluralParserError,
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<FixedDecimalFormatterError> for ICU4XError {
    fn from(e: FixedDecimalFormatterError) -> Self {
        let ret = match e {
            FixedDecimalFormatterError::Data(e) => e.into(),
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<ParserError> for ICU4XError {
    fn from(e: ParserError) -> Self {
        let ret = match e {
            ParserError::InvalidLanguage => ICU4XError::LocaleParserLanguageError,
            ParserError::InvalidSubtag => ICU4XError::LocaleParserSubtagError,
            ParserError::InvalidExtension => ICU4XError::LocaleParserExtensionError,
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<TinyStrError> for ICU4XError {
    fn from(e: TinyStrError) -> Self {
        let ret = match e {
            TinyStrError::TooLarge { .. } => ICU4XError::TinyStrTooLargeError,
            TinyStrError::ContainsNull => ICU4XError::TinyStrContainsNullError,
            TinyStrError::NonAscii => ICU4XError::TinyStrNonAsciiError,
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<TimeZoneError> for ICU4XError {
    fn from(e: TimeZoneError) -> Self {
        let ret = match e {
            TimeZoneError::OffsetOutOfBounds => ICU4XError::TimeZoneOffsetOutOfBoundsError,
            TimeZoneError::InvalidOffset => ICU4XError::TimeZoneInvalidOffsetError,
            TimeZoneError::DataProvider(err) => err.into(),
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}

impl From<NormalizerError> for ICU4XError {
    fn from(e: NormalizerError) -> Self {
        let ret = match e {
            NormalizerError::FutureExtension => ICU4XError::NormalizerFutureExtensionError,
            NormalizerError::ValidationError => ICU4XError::NormalizerValidationError,
            NormalizerError::DataProvider(err) => err.into(),
            _ => ICU4XError::UnknownError,
        };
        log_conversion(&e, ret);
        ret
    }
}
