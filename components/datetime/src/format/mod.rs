// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod datetime;
pub(crate) mod neo;
pub(crate) mod time_zone;

use crate::provider::neo::SimpleSubstitutionPattern;

pub(crate) enum GetNameForMonthError {
    Invalid,
    NotLoaded,
}
pub(crate) enum GetNameForWeekdayError {
    NotLoaded,
}

pub(crate) enum GetSymbolForEraError {
    Invalid,
    NotLoaded,
}

pub(crate) enum GetSymbolForCyclicYearError {
    Invalid {
        max: usize,
    },
    #[allow(dead_code)] // TODO(#3761)
    NotLoaded,
}

pub(crate) enum GetNameForDayPeriodError {
    NotLoaded,
}

/// Internal enum to represent the kinds of month symbols for interpolation
pub(crate) enum MonthPlaceholderValue<'a> {
    PlainString(&'a str),
    Numeric,
    NumericPattern(&'a SimpleSubstitutionPattern<'a>),
}
