// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Datetime formatting without static checking of invariants.

use crate::format::datetime::try_write_pattern_items;
pub use crate::format::DateTimeInputUnchecked;
use crate::pattern::*;
use crate::raw::neo::*;
use crate::scaffold::*;
use crate::DateTimeFormatter;
use crate::FixedCalendarDateTimeFormatter;
use core::fmt;
use icu_calendar::types::MonthCode;
use tinystr::TinyStr16;
use writeable::TryWriteable;

#[cfg(doc)]
use crate::fieldsets::enums::CompositeFieldSet;
#[cfg(doc)]
use crate::FormattedDateTime;
#[cfg(doc)]
use icu_calendar::types::CyclicYear;
#[cfg(doc)]
use icu_decimal::DecimalFormatter;

impl<C: CldrCalendar, FSet: DateTimeNamesMarker> FixedCalendarDateTimeFormatter<C, FSet> {
    /// Formats a datetime without enforcing either the field set or the calendar.
    ///
    /// This function is useful when the caller knows something about the field set that the
    /// type system is unaware of. For example, if the formatter is represented with a
    /// [dynamic field set](crate::fieldsets::enums), the caller may be able to provide a
    /// narrower type for formatting.
    ///
    /// ❗ The caller must ensure that:
    ///
    /// 1. The calendar of the input matches the calendar of the formatter
    /// 2. The fields of the input are a superset of the fields of the formatter
    ///
    /// Returns a [`FormattedDateTimeUnchecked`] to surface errors when they occur,
    /// but not every invariant will result in an error. Use with caution!
    ///
    /// # Examples
    ///
    /// In the following example, we know that the formatter's field set is [`YMD`], but the
    /// type system thinks we are a [`CompositeFieldSet`], which requires a [`ZonedDateTime`]
    /// as input. However, since [`Date`] contains all the fields required by [`YMD`], we can
    /// successfully pass it into [`format_unchecked`].
    ///
    /// ```
    /// use icu::calendar::cal::Buddhist;
    /// use icu::datetime::fieldsets::enums::CompositeFieldSet;
    /// use icu::datetime::fieldsets::{T, YMD};
    /// use icu::datetime::input::{Date, Time};
    /// use icu::datetime::unchecked::DateTimeInputUnchecked;
    /// use icu::datetime::unchecked::FormattedDateTimeUncheckedError;
    /// use icu::datetime::unchecked::MissingInputFieldKind;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::<Buddhist, _>::try_new(
    ///     locale!("th").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .cast_into_fset::<CompositeFieldSet>();
    ///
    /// // Create a date and convert it to the correct calendar:
    /// let mut input = DateTimeInputUnchecked::default();
    /// let date = Date::try_new_iso(2025, 3, 7).unwrap().to_calendar(Buddhist);
    ///
    /// // Safe because the calendar matches the formatter:
    /// input.set_date_fields_unchecked(date);
    ///
    /// // Safe because YMD needs only date fields, which are in the input:
    /// let result = formatter.format_unchecked(input);
    ///
    /// assert_try_writeable_eq!(result, "7 มีนาคม 2568");
    ///
    /// // If we don't give all needed fields, we will get an error!
    /// let mut input = DateTimeInputUnchecked::default();
    /// let result = formatter.format_unchecked(input);
    /// assert_try_writeable_eq!(
    ///     result,
    ///     "{d} {M} {G} {y}",
    ///     Err(FormattedDateTimeUncheckedError::MissingInputField(
    ///         MissingInputFieldKind::DayOfMonth
    ///     ))
    /// );
    /// ```
    ///
    /// [`Date`]: crate::input::Date
    /// [`ZonedDateTime`]: crate::input::ZonedDateTime
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`format_unchecked`]: Self::format_unchecked
    pub fn format_unchecked(
        &self,
        datetime: DateTimeInputUnchecked,
    ) -> FormattedDateTimeUnchecked<'_> {
        FormattedDateTimeUnchecked {
            pattern: self.selection.select(&datetime),
            input: datetime,
            names: self.names.as_borrowed(),
        }
    }
}

impl<FSet: DateTimeNamesMarker> DateTimeFormatter<FSet> {
    /// Formats a datetime without enforcing either the field set or the calendar.
    ///
    /// This function is useful when the caller knows something about the field set that the
    /// type system is unaware of. For example, if the formatter is represented with a
    /// [dynamic field set](crate::fieldsets::enums), the caller may be able to provide a
    /// narrower type for formatting.
    ///
    /// ❗ The caller must ensure that:
    ///
    /// 1. The calendar of the input matches the calendar of the formatter
    /// 2. The fields of the input are a superset of the fields of the formatter
    ///
    /// Returns a [`FormattedDateTimeUnchecked`] to surface errors when they occur,
    /// but not every invariant will result in an error. Use with caution!
    ///
    /// # Examples
    ///
    /// In the following example, we know that the formatter's field set is [`YMD`], but the
    /// type system thinks we are a [`CompositeFieldSet`], which requires a [`ZonedDateTime`]
    /// as input. However, since [`Date`] contains all the fields required by [`YMD`], we can
    /// successfully pass it into [`format_unchecked`].
    ///
    /// ```
    /// use icu::datetime::fieldsets::enums::CompositeFieldSet;
    /// use icu::datetime::fieldsets::{T, YMD};
    /// use icu::datetime::input::{Date, Time};
    /// use icu::datetime::unchecked::DateTimeInputUnchecked;
    /// use icu::datetime::unchecked::FormattedDateTimeUncheckedError;
    /// use icu::datetime::unchecked::MissingInputFieldKind;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter =
    ///     DateTimeFormatter::try_new(locale!("th-TH").into(), YMD::long())
    ///         .unwrap()
    ///         .cast_into_fset::<CompositeFieldSet>();
    ///
    /// // Create a date and convert it to the correct calendar:
    /// let mut input = DateTimeInputUnchecked::default();
    /// let date = Date::try_new_iso(2025, 3, 7)
    ///     .unwrap()
    ///     .to_calendar(formatter.calendar());
    ///
    /// // Safe because the calendar matches the formatter:
    /// input.set_date_fields_unchecked(date);
    ///
    /// // Safe because YMD needs only date fields, which are in the input:
    /// let result = formatter.format_unchecked(input);
    ///
    /// assert_try_writeable_eq!(result, "7 มีนาคม 2568");
    ///
    /// // If we don't give all needed fields, we will get an error!
    /// let mut input = DateTimeInputUnchecked::default();
    /// let result = formatter.format_unchecked(input);
    /// assert_try_writeable_eq!(
    ///     result,
    ///     "{d} {M} {G} {y}",
    ///     Err(FormattedDateTimeUncheckedError::MissingInputField(
    ///         MissingInputFieldKind::DayOfMonth
    ///     ))
    /// );
    /// ```
    ///
    /// [`Date`]: crate::input::Date
    /// [`ZonedDateTime`]: crate::input::ZonedDateTime
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`format_unchecked`]: Self::format_unchecked
    pub fn format_unchecked(
        &self,
        datetime: DateTimeInputUnchecked,
    ) -> FormattedDateTimeUnchecked<'_> {
        FormattedDateTimeUnchecked {
            pattern: self.selection.select(&datetime),
            input: datetime,
            names: self.names.as_borrowed(),
        }
    }
}

/// An intermediate type during a datetime formatting operation with dynamic input.
///
/// Unlike [`FormattedDateTime`], converting this to a string could fail.
///
/// Not intended to be stored: convert to a string first.
#[derive(Debug)]
pub struct FormattedDateTimeUnchecked<'a> {
    pattern: DateTimeZonePatternDataBorrowed<'a>,
    input: DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl TryWriteable for FormattedDateTimeUnchecked<'_> {
    type Error = FormattedDateTimeUncheckedError;
    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        let err = match try_write_pattern_items(
            self.pattern.metadata(),
            self.pattern.iter_items(),
            &self.input,
            &self.names,
            self.names.decimal_formatter,
            sink,
        ) {
            Ok(Ok(())) => return Ok(Ok(())),
            Err(fmt::Error) => return Err(fmt::Error),
            Ok(Err(err)) => err,
        };
        Ok(Err(match err {
            FormattedDateTimePatternError::InvalidMonthCode(month_code) => {
                Self::Error::InvalidMonthCode(month_code)
            }
            FormattedDateTimePatternError::InvalidEra(tiny_ascii_str) => {
                Self::Error::InvalidEra(tiny_ascii_str)
            }
            FormattedDateTimePatternError::InvalidCyclicYear { value, max } => {
                Self::Error::InvalidCyclicYear { value, max }
            }
            FormattedDateTimePatternError::DecimalFormatterNotLoaded => {
                Self::Error::DecimalFormatterNotLoaded
            }
            FormattedDateTimePatternError::NamesNotLoaded(error_field) => {
                Self::Error::NamesNotLoaded(error_field)
            }
            FormattedDateTimePatternError::MissingInputField(name) => {
                Self::Error::MissingInputField(name)
            }
            FormattedDateTimePatternError::UnsupportedLength(error_field) => {
                Self::Error::UnsupportedLength(error_field)
            }
            FormattedDateTimePatternError::UnsupportedField(error_field) => {
                Self::Error::UnsupportedField(error_field)
            }
        }))
    }

    // TODO(#489): Implement writeable_length_hint
}

impl FormattedDateTimeUnchecked<'_> {
    /// Gets the pattern used in this formatted value.
    ///
    /// From the pattern, one can check the properties of the included components, such as
    /// the hour cycle being used for formatting. See [`DateTimePattern`].
    pub fn pattern(&self) -> DateTimePattern {
        self.pattern.to_pattern()
    }
}

/// The kind of a missing datetime input field.
#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone, displaydoc::Display)]
pub enum MissingInputFieldKind {
    /// Day of month
    DayOfMonth,
    /// Day of year
    DayOfYear,
    /// Rata Die
    RataDie,
    /// Hour
    Hour,
    /// Minute
    Minute,
    /// Month
    Month,
    /// Second
    Second,
    /// Subsecond
    Subsecond,
    /// Weekday
    Weekday,
    /// Year
    Year,
    /// Cyclic year
    YearCyclic,
    /// Era year
    YearEra,
    /// Time zone identifier
    TimeZoneId,
    /// Time zone name timestamp
    TimeZoneNameTimestamp,
    /// Unused as of 2.1.0
    #[deprecated(since = "2.1.0", note = "unused, never returned")]
    TimeZoneVariant,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone, displaydoc::Display)]
/// Error for the [`TryWriteable`] implementation of [`FormattedDateTimeUnchecked`].
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
pub enum FormattedDateTimeUncheckedError {
    /// The [`MonthCode`] of the input is not valid for this calendar.
    ///
    /// Error conditions:
    ///
    /// - **Unchecked functions:** for example, wrong calendar passed to [`set_date_fields_unchecked`]
    /// - **Locale data:** for example, datetime names don't match the formatter's calendar
    /// - **Trait impls:** for example, the date returns fields for the wrong calendar
    ///
    /// The output will contain the raw [`MonthCode`] as a fallback value.
    ///
    /// [`set_date_fields_unchecked`]: DateTimeInputUnchecked::set_date_fields_unchecked
    #[displaydoc("Invalid month {0:?}")]
    InvalidMonthCode(MonthCode),
    /// The era code of the input is not valid for this calendar.
    ///
    /// Same error conditions as [`FormattedDateTimeUncheckedError::InvalidMonthCode`].
    ///
    /// The output will contain the era code as the fallback.
    #[displaydoc("Invalid era {0:?}")]
    InvalidEra(TinyStr16),
    /// The [`CyclicYear::year`] of the input is not valid for this calendar.
    ///
    /// Same error conditions as [`FormattedDateTimeUncheckedError::InvalidMonthCode`].
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
    /// - **Locale data:** for example, year style patterns contain inconsistant field lengths
    /// - **Trait impls:** for example, a custom field set does not include the correct names data
    ///
    /// The output will contain fallback values using field identifiers (such as `tue` for `Weekday::Tuesday`,
    /// `M02` for month 2, etc.).
    #[displaydoc("Names for {0:?} not loaded")]
    NamesNotLoaded(ErrorField),
    /// The [`DecimalFormatter`] has not been loaded.
    ///
    /// Same error conditions as [`FormattedDateTimeUncheckedError::NamesNotLoaded`].
    ///
    /// The output will contain fallback values using Latin numerals.
    #[displaydoc("DecimalFormatter not loaded")]
    DecimalFormatterNotLoaded,

    /// An input field (such as "hour" or "month") is missing.
    ///
    /// Error conditions:
    ///
    /// - **Unchecked functions:** for example, insufficient fields in [`DateTimeInputUnchecked`]
    /// - **Locale data:** for example, the pattern contains a field not in the fieldset
    /// - **Trait impls:** for example, a custom field set does not require the correct fields
    ///
    /// The output will contain the string `{X}` instead, where `X` is the symbol for which the input is missing.
    #[displaydoc("Incomplete input, missing value for {0:?}")]
    MissingInputField(MissingInputFieldKind),

    /// The pattern contains a field symbol for which formatting is unsupported.
    ///
    /// Error conditions:
    ///
    /// - **Locale data:** the pattern contains an unsupported field
    ///
    /// The output will contain the string `{unsupported:X}`, where `X` is the symbol of the unsupported field.
    #[displaydoc("Unsupported field {0:?}")]
    UnsupportedField(ErrorField),
    /// The pattern contains a field that has a valid symbol but invalid length.
    ///
    /// Same error conditions as [`FormattedDateTimeUncheckedError::UnsupportedField`].
    ///
    /// The output will contain fallback values similar to [`FormattedDateTimeUncheckedError::NamesNotLoaded`].
    #[displaydoc("Field length for {0:?} is invalid")]
    UnsupportedLength(ErrorField),
}
