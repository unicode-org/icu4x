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
#[allow(clippy::module_inception)] // the file pattern.rs should contain DateTimePattern
mod pattern;

pub use crate::error::ErrorField;
pub use formatter::DateTimePatternFormatter;
pub use formatter::FormattedDateTimePattern;
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
