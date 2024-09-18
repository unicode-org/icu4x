// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for managing data needed by [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter).

use crate::fields;
use crate::fields::Field;
use crate::input;
use crate::provider;
use crate::provider::calendar::months;
use crate::provider::neo::SimpleSubstitutionPattern;
use crate::time_zone::TimeZoneDataPayloadsBorrowed;
use icu_calendar::types::Era;
use icu_calendar::types::MonthCode;

pub(crate) enum GetSymbolForMonthError {
    Missing,
    MissingNames(Field),
}
pub(crate) enum GetSymbolForWeekdayError {
    Missing,
    MissingNames(Field),
}

pub(crate) enum GetSymbolForEraError {
    Missing,
    MissingNames(Field),
}

pub(crate) enum GetSymbolForDayPeriodError {
    MissingNames(Field),
}

/// Internal enum to represent the kinds of month symbols for interpolation
pub(crate) enum MonthPlaceholderValue<'a> {
    PlainString(&'a str),
    StringNeedingLeapPrefix(&'a str),
    Numeric,
    NumericPattern(&'a SimpleSubstitutionPattern<'a>),
}

pub(crate) trait DateSymbols<'data> {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, GetSymbolForMonthError>;

    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, GetSymbolForWeekdayError>;

    /// Gets the era symbol, or `None` if data is loaded but symbol isn't found.
    ///
    /// `None` should fall back to the era code directly, if, for example,
    /// a japanext datetime is formatted with a `DateTimeFormat<Japanese>`
    fn get_symbol_for_era(
        &self,
        length: fields::FieldLength,
        era_code: Era,
    ) -> Result<&str, GetSymbolForEraError>;
}

impl<'data> provider::calendar::DateSymbolsV1<'data> {
    fn get_symbols_map_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
    ) -> Result<&months::SymbolsV1<'data>, core::convert::Infallible> {
        let widths = match month {
            fields::Month::Format => &self.months.format,
            fields::Month::StandAlone => {
                if let Some(ref widths) = self.months.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return Ok(symbols);
                    } else {
                        return self.get_symbols_map_for_month(fields::Month::Format, length);
                    }
                } else {
                    return self.get_symbols_map_for_month(fields::Month::Format, length);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        Ok(symbols)
    }
}

impl<'data> DateSymbols<'data> for provider::calendar::DateSymbolsV1<'data> {
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, GetSymbolForWeekdayError> {
        let widths = match weekday {
            fields::Weekday::Format => &self.weekdays.format,
            fields::Weekday::StandAlone => {
                if let Some(ref widths) = self.weekdays.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        fields::FieldLength::Six => {
                            widths.short.as_ref().or(widths.abbreviated.as_ref())
                        }
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        let idx = (day as usize) % 7;
                        return symbols
                            .0
                            .get(idx)
                            .map(|x| &**x)
                            .ok_or(GetSymbolForWeekdayError::Missing);
                    } else {
                        return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                    }
                } else {
                    return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                }
            }
            fields::Weekday::Local => unimplemented!(),
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            fields::FieldLength::Six => widths.short.as_ref().unwrap_or(&widths.abbreviated),
            _ => &widths.abbreviated,
        };
        let idx = (day as usize) % 7;
        symbols
            .0
            .get(idx)
            .map(|x| &**x)
            .ok_or(GetSymbolForWeekdayError::Missing)
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, GetSymbolForMonthError> {
        let symbols_map = self.get_symbols_map_for_month(month, length).unwrap();
        let mut symbol_option = symbols_map.get(code);
        let mut fallback = false;
        if symbol_option.is_none() {
            if let Some(code) = code.get_normal_if_leap() {
                let symbols_map = self.get_symbols_map_for_month(month, length).unwrap();
                symbol_option = symbols_map.get(code);
                fallback = true;
            }
        }
        let symbol = symbol_option.ok_or(GetSymbolForMonthError::Missing)?;
        Ok(if fallback {
            MonthPlaceholderValue::StringNeedingLeapPrefix(symbol)
        } else {
            MonthPlaceholderValue::PlainString(symbol)
        })
    }

    fn get_symbol_for_era(
        &self,
        length: fields::FieldLength,
        era_code: Era,
    ) -> Result<&str, GetSymbolForEraError> {
        let symbols = match length {
            fields::FieldLength::Wide => &self.eras.names,
            fields::FieldLength::Narrow => &self.eras.narrow,
            _ => &self.eras.abbr,
        };
        symbols
            .get(era_code.0.as_str().into())
            .ok_or(GetSymbolForEraError::Missing)
    }
}

pub(crate) trait TimeSymbols {
    /// Gets the day period symbol.
    ///
    /// Internally, 'noon' and 'midnight' should fall back to 'am' and 'pm'.
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, GetSymbolForDayPeriodError>;
}

impl<'data> TimeSymbols for provider::calendar::TimeSymbolsV1<'data> {
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, GetSymbolForDayPeriodError> {
        use fields::{DayPeriod::NoonMidnight, FieldLength};
        let widths = &self.day_periods.format;
        let symbols = match length {
            FieldLength::Wide => &widths.wide,
            FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        Ok(match (day_period, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => symbols.midnight.as_ref().unwrap_or(&symbols.am),
            (NoonMidnight, 12, true) => symbols.noon.as_ref().unwrap_or(&symbols.pm),
            (_, hour, _) if hour < 12 => &symbols.am,
            _ => &symbols.pm,
        })
    }
}

pub(crate) trait ZoneSymbols<'data> {
    fn get_payloads(&self) -> TimeZoneDataPayloadsBorrowed<'data>;
}

impl<'data> ZoneSymbols<'data> for () {
    fn get_payloads(&self) -> TimeZoneDataPayloadsBorrowed<'data> {
        TimeZoneDataPayloadsBorrowed::empty()
    }
}
