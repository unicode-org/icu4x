// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Lower-level, power-user APIs for formatting datetimes with pattern strings.
//!
//! ❗ This module forgoes most internationalization functionality of the datetime crate.
//! It assumes that the pattern is already localized for the customer's locale. Most clients
//! should use [`DateTimeFormatter`] instead of directly formatting with patterns.
//!
//! [`DateTimeFormatter`]: crate::DateTimeFormatter

mod formatter;
mod names;
#[expect(clippy::module_inception)] // the file pattern.rs should contain DateTimePattern
mod pattern;

pub use crate::error::ErrorField;
use crate::unchecked::MissingInputFieldKind;
pub use formatter::DateTimePatternFormatter;
pub use formatter::FormattedDateTimePattern;
use icu_calendar::types::MonthCode;
use icu_calendar::AnyCalendarKind;
use icu_pattern::SinglePlaceholderPattern;
pub use names::DateTimeNames;
pub(crate) use names::DateTimeNamesMetadata;
pub use names::DayPeriodNameLength;
pub use names::FixedCalendarDateTimeNames;
pub use names::MonthNameLength;
pub(crate) use names::RawDateTimeNames;
pub(crate) use names::RawDateTimeNamesBorrowed;
pub(crate) use names::TimeZoneDataPayloadsBorrowed;
pub use names::WeekdayNameLength;
pub use names::YearNameLength;
pub use pattern::DateTimePattern;
use tinystr::TinyStr16;

#[cfg(doc)]
use icu_calendar::types::CyclicYear;
#[cfg(doc)]
use icu_decimal::DecimalFormatter;
#[cfg(doc)]
use writeable::TryWriteable;

pub(crate) enum GetNameForMonthError {
    InvalidMonthCode,
    InvalidFieldLength,
    NotLoaded,
}
pub(crate) enum GetNameForWeekdayError {
    InvalidFieldLength,
    NotLoaded,
}

pub(crate) enum GetNameForEraError {
    InvalidEraCode,
    InvalidFieldLength,
    NotLoaded,
}

pub(crate) enum GetNameForCyclicYearError {
    InvalidYearNumber { max: u8 },
    InvalidFieldLength,
    NotLoaded,
}

pub(crate) enum GetNameForDayPeriodError {
    InvalidFieldLength,
    NotLoaded,
}

/// Internal enum to represent the kinds of month symbols for interpolation
pub(crate) enum MonthPlaceholderValue<'a> {
    PlainString(&'a str),
    Numeric,
    NumericPattern(&'a SinglePlaceholderPattern),
}

/// Error returned from [`FixedCalendarDateTimeNames`]'s pattern load methods.
#[derive(Debug, Clone, Copy, PartialEq, displaydoc::Display)]
#[non_exhaustive]
pub enum PatternLoadError {
    /// A field conflicts with a previous field.
    ///
    /// Fields conflict if they require the same type of data, for example the
    /// `EEE` and `EEEE` fields (short vs long weekday) conflict, or the `M`
    /// and `L` (format vs standalone month) conflict.
    #[displaydoc("A field {field:?} conflicts with a previously loaded field {previous_field:?}.")]
    ConflictingField {
        /// The field that was not able to be loaded.
        field: ErrorField,
        /// The field that prevented the new field from being loaded.
        previous_field: ErrorField,
    },
    /// The field symbol is not supported in that length.
    ///
    /// Some fields, such as `O` are not defined for all lengths (e.g. `OO`).
    #[displaydoc("The field {0:?} symbol is not supported in that length.")]
    UnsupportedLength(ErrorField),
    /// The specific formatter does not support this field.
    ///
    /// This happens for example when trying to load a month field
    /// on a [`FixedCalendarDateTimeNames<Gregorian, ZoneFieldSet>`].
    #[displaydoc("The specific formatter does not support the field {0:?}.")]
    FormatterTooSpecific(ErrorField),
    /// An error arising from the [`data provider`](icu_provider) for loading names.
    #[displaydoc("Problem loading data for field {1:?}: {0}")]
    Data(icu_provider::DataError, ErrorField),
}

impl core::error::Error for PatternLoadError {}

/// Error returned from constructors that map from AnyCalendar to a formatter.
#[derive(Debug, Clone, Copy, PartialEq, displaydoc::Display)]
#[displaydoc("The calendar {kind:?} is not supported in DateTimeFormatter")]
#[non_exhaustive]
pub struct UnsupportedCalendarError {
    /// The calendar kind that is not supported.
    pub kind: AnyCalendarKind,
}

impl core::error::Error for UnsupportedCalendarError {}

#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone, displaydoc::Display)]
/// Error for the [`TryWriteable`] implementation of [`FormattedDateTimePattern`].
///
/// There are 3 general conditions for these errors to occur:
///
/// 1. Invariants of **unchecked functions** are not upheld
/// 2. Invariants of **locale data** are not upheld
/// 3. Invariants of **trait impls** are not upheld (including [scaffolding traits])
///
/// It is not always possible to distinguish the source of the errors. Each variant is documented
/// with rules of thumb for when they might occur.
///
/// [unstable scaffolding traits]: crate::scaffold
pub enum FormattedDateTimePatternError {
    /// The [`MonthCode`] of the input is not valid for this calendar.
    ///
    /// Error conditions:
    ///
    /// - **Locale data:** for example, datetime names don't match the formatter's calendar
    /// - **Trait impls:** for example, the date returns fields for the wrong calendar
    ///
    /// The output will contain the raw [`MonthCode`] as a fallback value.
    #[displaydoc("Invalid month {0:?}")]
    InvalidMonthCode(MonthCode),
    /// The era code of the input is not valid for this calendar.
    ///
    /// Same error conditions as [`FormattedDateTimePatternError::InvalidMonthCode`].
    ///
    /// The output will contain the era code as the fallback.
    #[displaydoc("Invalid era {0:?}")]
    InvalidEra(TinyStr16),
    /// The [`CyclicYear::year`] of the input is not valid for this calendar.
    ///
    /// Same error conditions as [`FormattedDateTimePatternError::InvalidMonthCode`].
    ///
    /// The output will contain [`CyclicYear::related_iso`] as a fallback value.
    #[displaydoc("Invalid cyclic year {value} (maximum {max})")]
    InvalidCyclicYear {
        /// Value
        value: u8,
        /// Max
        max: u8,
    },

    /// The localized names for a field have not been loaded.
    ///
    /// Error conditions:
    ///
    /// - **Unchecked functions:** for example, the pattern in [`with_pattern_unchecked`] contains fields that haven't been loaded
    /// - **Trait impls:** for example, a custom field set does not include the correct names data
    ///
    /// The output will contain fallback values using field identifiers (such as `tue` for `Weekday::Tuesday`,
    /// `M02` for month 2, etc.).
    ///
    /// [`with_pattern_unchecked`]: FixedCalendarDateTimeNames::with_pattern_unchecked
    #[displaydoc("Names for {0:?} not loaded")]
    NamesNotLoaded(ErrorField),
    /// The [`DecimalFormatter`] has not been loaded.
    ///
    /// Same error conditions as [`FormattedDateTimePatternError::NamesNotLoaded`].
    ///
    /// The output will contain fallback values using Latin numerals.
    #[displaydoc("DecimalFormatter not loaded")]
    DecimalFormatterNotLoaded,

    /// An input field (such as "hour" or "month") is missing.
    ///
    /// Error conditions:
    ///
    /// - **Unchecked functions:** for example, the pattern in [`with_pattern_unchecked`] contains fields that aren't in the fieldset
    /// - **Trait impls:** for example, a custom field set does not require the correct fields
    ///
    /// The output will contain the string `{X}` instead, where `X` is the symbol for which the input is missing.
    ///
    /// [`with_pattern_unchecked`]: FixedCalendarDateTimeNames::with_pattern_unchecked
    #[displaydoc("Incomplete input, missing value for {0:?}")]
    MissingInputField(MissingInputFieldKind),

    /// The pattern contains a field symbol for which formatting is unsupported.
    ///
    /// Error conditions:
    ///
    /// - **Unchecked functions:** for example, the pattern in [`with_pattern_unchecked`] contains an unsupported field
    ///
    /// The output will contain the string `{unsupported:X}`, where `X` is the symbol of the unsupported field.
    ///
    /// [`with_pattern_unchecked`]: FixedCalendarDateTimeNames::with_pattern_unchecked
    #[displaydoc("Unsupported field {0:?}")]
    UnsupportedField(ErrorField),
    /// The pattern contains a field that has a valid symbol but invalid length.
    ///
    /// Same error conditions as [`FormattedDateTimePatternError::UnsupportedField`].
    ///
    /// The output will contain fallback values similar to [`FormattedDateTimePatternError::NamesNotLoaded`].
    #[displaydoc("Field length for {0:?} is invalid")]
    UnsupportedLength(ErrorField),
}
