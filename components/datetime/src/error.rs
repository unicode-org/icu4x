// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::PatternLoadError;
use displaydoc::Display;
use icu_calendar::{types::MonthCode, AnyCalendarKind};
use icu_provider::DataError;
use tinystr::TinyStr16;

#[cfg(doc)]
use crate::pattern::FixedCalendarDateTimeNames;
#[cfg(doc)]
use crate::pattern::*;
#[cfg(doc)]
use crate::scaffold::*;
#[cfg(doc)]
use crate::DateTimeInputUnchecked;
#[cfg(doc)]
use icu_calendar::types::CyclicYear;
#[cfg(doc)]
use icu_decimal::DecimalFormatter;

/// An error from constructing a formatter.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum DateTimeFormatterLoadError {
    /// An error while loading display names for a field.
    #[displaydoc("{0}")]
    Names(PatternLoadError),
    /// An error while loading some other required data,
    /// such as skeleton patterns or calendar conversions.
    #[displaydoc("{0}")]
    Data(DataError),
}

impl core::error::Error for DateTimeFormatterLoadError {}

impl From<DataError> for DateTimeFormatterLoadError {
    fn from(error: DataError) -> Self {
        Self::Data(error)
    }
}

/// The specific field for which an error occurred.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
pub struct ErrorField(pub(crate) crate::provider::fields::Field);

/// An error from mixing calendar types in a formatter.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[displaydoc("DateTimeFormatter for {this_kind} calendar was given a {date_kind:?} calendar")]
#[non_exhaustive]
pub struct MismatchedCalendarError {
    /// The calendar kind of the target object (formatter).
    pub this_kind: AnyCalendarKind,
    /// The calendar kind of the input object (date being formatted).
    /// Can be `None` if the input calendar was not specified.
    pub date_kind: Option<AnyCalendarKind>,
}

impl core::error::Error for MismatchedCalendarError {}

#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone, displaydoc::Display)]
/// Error for [`TryWriteable`] implementations in the datetime component.
///
/// There are two types that expose this error:
///
/// 1. [`FormattedDateTimePattern`](crate::pattern::FormattedDateTimePattern)
/// 2. [`FormattedDateTimeUnchecked`](crate::FormattedDateTimeUnchecked)
///
/// Most of these errors can be encountered with:
///
/// 1. Bad locale data
/// 2. Bad implementations of [scaffolding traits]
///
/// where the word "bad" means "violates invariants of the data or the trait impl". If you use
/// off-the-shelf ICU4X types and data, you should not normally hit these conditions.
///
/// Besides data and trait invariants, some errors can be hit when other code invariants
/// are not satisfied. Details are listed on each error variant.
///
/// [`TryWriteable`]: writeable::TryWriteable
/// [scaffolding traits]: crate::scaffold
pub enum DateTimeWriteError {
    /// The [`MonthCode`] of the input is not valid for the formatter's calendar.
    ///
    /// Error conditions:
    ///
    /// 1. Bad locale data for datetime names (data doesn't match calendar)
    /// 2. Bad implementation of one of the following scaffolding traits:
    ///     - [`ConvertCalendar`]
    ///     - [`GetField`]
    ///     - [`InFixedCalendar`]
    ///     - [`InSameCalendar`]
    /// 3. Wrong calendar in [`DateTimeInputUnchecked`]
    ///
    /// The output will contain the raw [`MonthCode`] as a fallback value.
    #[displaydoc("Invalid month {0:?}")]
    InvalidMonthCode(MonthCode),
    /// The era code of the input is not valid for the formatter's calendar.
    ///
    /// Same error conditions as [`DateTimeWriteError::InvalidMonthCode`].
    ///
    /// The output will contain the era code as the fallback.
    #[displaydoc("Invalid era {0:?}")]
    InvalidEra(TinyStr16),
    /// The [`CyclicYear::year`] of the input is not valid for this calendar.
    ///
    /// Same error conditions as [`DateTimeWriteError::InvalidMonthCode`].
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
    /// 1. Bad locale data for datetime patterns (inconsistent fields)
    /// 2. The [`DateTimePattern`] passed to [`FixedCalendarDateTimeNames::with_pattern_unchecked`]
    ///    contains a field for which data wasn't loaded
    ///
    /// The output will contain fallback values using field identifiers (such as `tue` for `Weekday::Tuesday`,
    /// `M02` for month 2, etc.).
    #[displaydoc("Names for {0:?} not loaded")]
    NamesNotLoaded(ErrorField),
    /// The [`DecimalFormatter`] has not been loaded.
    ///
    /// Same error conditions as [`DateTimeWriteError::NamesNotLoaded`].
    ///
    /// The output will contain fallback values using Latin numerals.
    #[displaydoc("DecimalFormatter not loaded")]
    DecimalFormatterNotLoaded,

    /// An input field (such as "hour" or "month") is missing.
    ///
    /// Error conditions:
    ///
    /// 1. Bad locale data for datetime patterns (pattern requires field not in fieldset)
    /// 2. The [`DateTimePattern`] passed to [`FixedCalendarDateTimeNames::with_pattern_unchecked`]
    ///    contains a field that isn't reflected in the fieldset
    /// 3. Required fields weren't set in [`DateTimeInputUnchecked`]
    ///
    /// The output will contain the string `{X}` instead, where `X` is the symbol for which the input is missing.
    #[displaydoc("Incomplete input, missing value for {0:?}")]
    MissingInputField(&'static str),

    /// The pattern contains a field that has a valid symbol but invalid length.
    ///
    /// Error conditions:
    ///
    /// 1. Bad locale data for datetime patterns (pattern contains a non-formattable field length)
    /// 2. The [`DateTimePattern`] passed to [`FixedCalendarDateTimeNames::with_pattern_unchecked`]
    ///    contains a non-formattable field length
    ///
    /// The output will contain fallback values similar to [`DateTimeWriteError::NamesNotLoaded`].
    #[displaydoc("Field length for {0:?} is invalid")]
    UnsupportedLength(ErrorField),
    /// Unsupported field
    ///
    /// Error conditions:
    ///
    /// 1. Bad locale data for datetime patterns (pattern contains a non-formattable field type)
    /// 2. The [`DateTimePattern`] passed to [`FixedCalendarDateTimeNames::with_pattern_unchecked`]
    ///    contains a non-formattable field type
    ///
    /// The output will contain the string `{unsupported:X}`, where `X` is the symbol of the unsupported field.
    #[displaydoc("Unsupported field {0:?}")]
    UnsupportedField(ErrorField),
}

impl core::error::Error for DateTimeWriteError {}
