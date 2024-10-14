// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for managing data needed by [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter).

use crate::fields;
use crate::fields::Field;
use crate::input;
use crate::provider::neo::SimpleSubstitutionPattern;
use crate::time_zone::TimeZoneDataPayloadsBorrowed;
use icu_calendar::types::Era;
use icu_calendar::types::MonthCode;

pub(crate) enum GetNameForMonthError {
    Missing,
    MissingNames(Field),
}
pub(crate) enum GetNameForWeekdayError {
    Missing,
    MissingNames(Field),
}

pub(crate) enum GetSymbolForEraError {
    Missing,
    MissingNames(Field),
}

pub(crate) enum GetNameForDayPeriodError {
    MissingNames(Field),
}

/// Internal enum to represent the kinds of month symbols for interpolation
pub(crate) enum MonthPlaceholderValue<'a> {
    PlainString(&'a str),
    Numeric,
    NumericPattern(&'a SimpleSubstitutionPattern<'a>),
}

pub(crate) trait DateSymbols<'data> {
    fn get_name_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, GetNameForMonthError>;

    fn get_name_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, GetNameForWeekdayError>;

    /// Gets the era symbol, or `None` if data is loaded but symbol isn't found.
    ///
    /// `None` should fall back to the era code directly, if, for example,
    /// a japanext datetime is formatted with a `DateTimeFormat<Japanese>`
    fn get_name_for_era(
        &self,
        length: fields::FieldLength,
        era_code: Era,
    ) -> Result<&str, GetSymbolForEraError>;
}

pub(crate) trait TimeSymbols {
    /// Gets the day period symbol.
    ///
    /// Internally, 'noon' and 'midnight' should fall back to 'am' and 'pm'.
    fn get_name_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, GetNameForDayPeriodError>;
}

pub(crate) trait ZoneSymbols<'data> {
    fn get_payloads(&self) -> TimeZoneDataPayloadsBorrowed<'data>;
}

impl<'data> ZoneSymbols<'data> for () {
    fn get_payloads(&self) -> TimeZoneDataPayloadsBorrowed<'data> {
        TimeZoneDataPayloadsBorrowed::default()
    }
}
