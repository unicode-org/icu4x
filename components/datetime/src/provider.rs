// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::date;
use crate::error::DateTimeFormatError;
use crate::fields;
use crate::options::{style, DateTimeFormatOptions};
use crate::pattern::Pattern;
use icu_data_provider::structs;
use std::borrow::Cow;

type Result<T> = std::result::Result<T, DateTimeFormatError>;

pub trait DateTimeDates {
    fn get_pattern_for_options(&self, options: &DateTimeFormatOptions) -> Result<Option<Pattern>>;
    fn get_pattern_for_style_bag(&self, style: &style::Bag) -> Result<Option<Pattern>>;
    fn get_pattern_for_date_style(&self, style: style::Date) -> Result<Pattern>;
    fn get_pattern_for_time_style(&self, style: style::Time) -> Result<Pattern>;
    fn get_pattern_for_date_time_style(
        &self,
        style: style::Date,
        date: Pattern,
        time: Pattern,
    ) -> Result<Pattern>;
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: date::Month,
    ) -> &Cow<str>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::WeekDay,
    ) -> &Cow<str>;
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::Hour,
    ) -> &Cow<str>;
}

impl DateTimeDates for structs::dates::gregory::DatesV1 {
    fn get_pattern_for_options(&self, options: &DateTimeFormatOptions) -> Result<Option<Pattern>> {
        match options {
            DateTimeFormatOptions::Style(bag) => self.get_pattern_for_style_bag(bag),
            DateTimeFormatOptions::Components(_) => unimplemented!(),
        }
    }

    fn get_pattern_for_style_bag(&self, style: &style::Bag) -> Result<Option<Pattern>> {
        match (style.date, style.time) {
            (None, None) => Ok(None),
            (None, Some(time_style)) => self.get_pattern_for_time_style(time_style).map(Some),
            (Some(date_style), None) => self.get_pattern_for_date_style(date_style).map(Some),
            (Some(date_style), Some(time_style)) => {
                let time = self.get_pattern_for_time_style(time_style)?;
                let date = self.get_pattern_for_date_style(date_style)?;

                self.get_pattern_for_date_time_style(date_style, date, time)
                    .map(Some)
            }
        }
    }

    fn get_pattern_for_date_style(&self, style: style::Date) -> Result<Pattern> {
        let date = &self.patterns.date;
        let s = match style {
            style::Date::Full => &date.full,
            style::Date::Long => &date.long,
            style::Date::Medium => &date.medium,
            style::Date::Short => &date.short,
        };
        Ok(Pattern::from_bytes(s)?)
    }

    fn get_pattern_for_date_time_style(
        &self,
        style: style::Date,
        date: Pattern,
        time: Pattern,
    ) -> Result<Pattern> {
        let date_time = &self.patterns.date_time;
        let s = match style {
            style::Date::Full => &date_time.full,
            style::Date::Long => &date_time.long,
            style::Date::Medium => &date_time.medium,
            style::Date::Short => &date_time.short,
        };
        Ok(Pattern::from_bytes_combination(s, date, time)?)
    }

    fn get_pattern_for_time_style(&self, style: style::Time) -> Result<Pattern> {
        let time = &self.patterns.time;
        let s = match style {
            style::Time::Full => &time.full,
            style::Time::Long => &time.long,
            style::Time::Medium => &time.medium,
            style::Time::Short => &time.short,
        };
        Ok(Pattern::from_bytes(s)?)
    }

    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::WeekDay,
    ) -> &Cow<str> {
        let widths = match weekday {
            fields::Weekday::Format => &self.symbols.weekdays.format,
            fields::Weekday::StandAlone => {
                if let Some(ref widths) = self.symbols.weekdays.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        fields::FieldLength::Six => widths
                            .short
                            .as_ref()
                            .or_else(|| widths.abbreviated.as_ref()),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return &symbols.0[usize::from(day)];
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
        &symbols.0[usize::from(day)]
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: date::Month,
    ) -> &Cow<str> {
        let widths = match month {
            fields::Month::Format => &self.symbols.months.format,
            fields::Month::StandAlone => {
                if let Some(ref widths) = self.symbols.months.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return &symbols.0[usize::from(num)];
                    } else {
                        return self.get_symbol_for_month(fields::Month::Format, length, num);
                    }
                } else {
                    return self.get_symbol_for_month(fields::Month::Format, length, num);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        &symbols.0[usize::from(num)]
    }

    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::Hour,
    ) -> &Cow<str> {
        let widths = match day_period {
            fields::DayPeriod::AmPm => &self.symbols.day_periods.format,
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };

        //TODO: Once we have more dayperiod types, we'll need to handle
        //      this logic in the right location.
        if u8::from(hour) < 12 {
            &symbols.am
        } else {
            &symbols.pm
        }
    }
}
