// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod datetime;
pub(crate) mod neo;
pub(crate) mod time_zone;

use crate::fields::Field;
use icu_pattern::SinglePlaceholderPattern;

pub(crate) enum GetNameForMonthError {
    Missing,
    MissingNames(Field),
}
pub(crate) enum GetNameForWeekdayError {
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
    NumericPattern(&'a SinglePlaceholderPattern),
}
