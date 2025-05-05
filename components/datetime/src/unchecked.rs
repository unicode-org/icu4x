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
use writeable::TryWriteable;

#[cfg(doc)]
use crate::fieldsets::enums::CompositeFieldSet;
#[cfg(doc)]
use crate::FormattedDateTime;

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
    /// use icu::datetime::fieldsets::{T, YMD};
    /// use icu::datetime::fieldsets::enums::CompositeFieldSet;
    /// use icu::datetime::input::{Date, Time};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::pattern::DateTimeWriteError;
    /// use icu::datetime::unchecked::DateTimeInputUnchecked;
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
    /// let date = Date::try_new_iso(2025, 3, 7)
    ///     .unwrap()
    ///     .to_calendar(Buddhist);
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
    ///     Err(DateTimeWriteError::MissingInputField("day_of_month"))
    /// );
    /// ```
    ///
    /// [`Date`]: crate::input::Date
    /// [`ZonedDateTime`]: crate::input::ZonedDateTime
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`format_unchecked`]: Self::format_unchecked
    pub fn format_unchecked(&self, datetime: DateTimeInputUnchecked) -> FormattedDateTimeUnchecked {
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
    /// use icu::datetime::fieldsets::{T, YMD};
    /// use icu::datetime::fieldsets::enums::CompositeFieldSet;
    /// use icu::datetime::input::{Date, Time};
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::pattern::DateTimeWriteError;
    /// use icu::datetime::unchecked::DateTimeInputUnchecked;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("th-TH").into(),
    ///     YMD::long(),
    /// )
    /// .unwrap()
    /// .cast_into_fset::<CompositeFieldSet>();
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
    ///     Err(DateTimeWriteError::MissingInputField("day_of_month"))
    /// );
    /// ```
    ///
    /// [`Date`]: crate::input::Date
    /// [`ZonedDateTime`]: crate::input::ZonedDateTime
    /// [`YMD`]: crate::fieldsets::YMD
    /// [`format_unchecked`]: Self::format_unchecked
    pub fn format_unchecked(&self, datetime: DateTimeInputUnchecked) -> FormattedDateTimeUnchecked {
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
    type Error = DateTimeWriteError;
    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        try_write_pattern_items(
            self.pattern.metadata(),
            self.pattern.iter_items(),
            &self.input,
            &self.names,
            self.names.decimal_formatter,
            sink,
        )
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
