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
        num: usize,
    ) -> Result<&Cow<str>>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: usize,
    ) -> Result<&Cow<str>>;
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::Period,
        length: fields::FieldLength,
        hour: usize,
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
        Ok(Pattern::from_bytes(s.as_bytes())?)
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
        Ok(Pattern::from_bytes_combination(s.as_bytes(), date, time)?)
    }

    fn get_pattern_for_time_style(&self, style: style::Time) -> Result<Pattern> {
        let time = &self.patterns.time;
        let s = match style {
            style::Time::Full => &time.full,
            style::Time::Long => &time.long,
            style::Time::Medium => &time.medium,
            style::Time::Short => &time.short,
        };
        Ok(Pattern::from_bytes(s.as_bytes())?)
    }

    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: usize,
    ) -> Result<&Cow<str>> {
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
                        return symbols.0.get(day).ok_or(DateTimeFormatError::MissingData);
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
        symbols.0.get(day).ok_or(DateTimeFormatError::MissingData)
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> Result<&Cow<str>> {
        let widths = match month {
            fields::Month::Format => &self.symbols.months.format,
            fields::Month::StandAlone => {
                if let Some(ref widths) = self.symbols.months.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Abbreviated => widths.abbreviated.as_ref(),
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        _ => unreachable!(),
                    };
                    if let Some(symbols) = symbols {
                        return symbols.0.get(num).ok_or(DateTimeFormatError::MissingData);
                    } else {
                        return self.get_symbol_for_month(fields::Month::Format, length, num);
                    }
                } else {
                    return self.get_symbol_for_month(fields::Month::Format, length, num);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Abbreviated => &widths.abbreviated,
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => unreachable!(),
        };
        symbols.0.get(num).ok_or(DateTimeFormatError::MissingData)
    }

    fn get_symbol_for_day_period(
        &self,
        day_period: fields::Period,
        length: fields::FieldLength,
        hour: usize,
    ) -> &Cow<str> {
        let widths = match day_period {
            fields::Period::AmPm => &self.symbols.day_periods.format,
            _ => unimplemented!(),
        };
        let symbols = match length {
            fields::FieldLength::One
            | fields::FieldLength::TwoDigit
            | fields::FieldLength::Abbreviated => &widths.abbreviated,
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => unreachable!(),
        };
        if hour < 12 {
            &symbols.am
        } else {
            &symbols.pm
        }
    }
}
