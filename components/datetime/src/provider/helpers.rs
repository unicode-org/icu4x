// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::date;
use crate::error::DateTimeFormatError;
use crate::fields;
use crate::options::{components, style, DateTimeFormatOptions};
use crate::pattern::Pattern;
use crate::provider;
use crate::skeleton;
use std::borrow::Cow;

type Result<T> = std::result::Result<T, DateTimeFormatError>;

pub trait DateTimeDates {
    fn get_pattern_for_options(&self, options: &DateTimeFormatOptions) -> Result<Option<Pattern>>;
    fn get_pattern_for_components_bag<'a>(
        &'a self,
        style: &components::Bag,
    ) -> Result<Option<&'a Pattern>>;
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
    ) -> &Cow<str>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> &Cow<str>;
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str>;
}

impl DateTimeDates for provider::gregory::DatesV1 {
    fn get_pattern_for_options(&self, options: &DateTimeFormatOptions) -> Result<Option<Pattern>> {
        match options {
            DateTimeFormatOptions::Style(bag) => self.get_pattern_for_style_bag(bag),
            // TODO - This pattern could be passed by reference, but lifetimes need to be added
            // further up the chain to do that.
            DateTimeFormatOptions::Components(bag) => self
                .get_pattern_for_components_bag(bag)
                .map(|maybe_pattern| maybe_pattern.cloned()),
        }
    }

    fn get_pattern_for_components_bag<'a>(
        &'a self,
        components: &components::Bag,
    ) -> Result<Option<&'a Pattern>> {
        // Not all skeletons are currently supported.
        let available_format_patterns = skeleton::get_available_format_patterns(&self);
        let requested_fields = components.to_vec_fields();
        Ok(
            match skeleton::get_best_available_format_pattern(
                available_format_patterns,
                &requested_fields,
            ) {
                skeleton::BestSkeleton::AllFieldsMatch(skeleton)
                | skeleton::BestSkeleton::MissingFields(skeleton) => Some(skeleton.pattern),
                skeleton::BestSkeleton::NoMatch => None,
            },
        )
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
            style::Date::Full => &date_time.style_patterns.full,
            style::Date::Long => &date_time.style_patterns.long,
            style::Date::Medium => &date_time.style_patterns.medium,
            style::Date::Short => &date_time.style_patterns.short,
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
        day: date::IsoWeekday,
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
                        return &symbols.0[(day as usize) % 7];
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
        &symbols.0[(day as usize) % 7]
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> &Cow<str> {
        // TODO(#493): Support symbols for non-Gregorian calendars.
        debug_assert!(num < 12);
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
                        return &symbols.0[num];
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
        &symbols.0[num]
    }

    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str> {
        use fields::{DayPeriod::NoonMidnight, FieldLength};
        let widths = &self.symbols.day_periods.format;
        let symbols = match length {
            FieldLength::Wide => &widths.wide,
            FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        match (day_period, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => symbols.midnight.as_ref().unwrap_or(&symbols.am),
            (NoonMidnight, 12, true) => symbols.noon.as_ref().unwrap_or(&symbols.pm),
            (_, hour, _) if hour < 12 => &symbols.am,
            _ => &symbols.pm,
        }
    }
}
