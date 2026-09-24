// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::fieldsets::builder;
use crate::options;

fn fieldbag_to_length(bag: &DateTimeFieldBag) -> Option<options::Length> {
    // Get all of the fields that are possibly non-numeric
    let DateTimeFieldBag {
        // ignore era, since it is handled in `YearStyle`
        era: _,
        year: _,
        month,
        day: _,
        weekday,
        day_period,
        hour_kind: _,
        hour: _,
        minute: _,
        second: _,
        fractional_second_digits: _,
        // ignore time zone name, since it has its own length
        time_zone_name: _,
    } = *bag;
    if matches!(month, Some(Month::Long))
        || matches!(weekday, Some(Weekday::Long))
        || matches!(day_period, Some(DayPeriod::FlexibleLong))
    {
        Some(options::Length::Long)
    } else if matches!(month, Some(Month::Short))
        || matches!(weekday, Some(Weekday::Short))
        || matches!(day_period, Some(DayPeriod::FlexibleShort))
    {
        Some(options::Length::Medium)
    } else if matches!(month, Some(Month::Numeric | Month::TwoDigit)) {
        Some(options::Length::Short)
    } else {
        None
    }
}

#[allow(clippy::todo)] // TODO: Finish implementing this
fn fieldbag_to_date_fields(bag: &DateTimeFieldBag) -> Option<builder::DateFields> {
    // Get all of the date fields
    let DateTimeFieldBag {
        era,
        year,
        month,
        day,
        weekday,
        // day period and lower are not date fields
        day_period: _,
        hour_kind: _,
        hour: _,
        minute: _,
        second: _,
        fractional_second_digits: _,
        time_zone_name: _,
    } = *bag;
    /// Internal struct for readability of the match
    struct DateFieldBag {
        pub era: Option<Era>,
        pub year: Option<Year>,
        pub month: Option<Month>,
        pub day: Option<Day>,
        pub weekday: Option<Weekday>,
    }
    let date_field_bag = DateFieldBag {
        era,
        year,
        month,
        day,
        weekday,
    };
    match date_field_bag {
        DateFieldBag {
            era: Some(_),
            year: Some(_),
            month: None,
            day: None,
            weekday: None,
        } => Some(builder::DateFields::Y),
        DateFieldBag {
            era: Some(_),
            year: Some(_),
            month: Some(_),
            day: None,
            weekday: None,
        } => Some(builder::DateFields::YM),
        _ => todo!(),
    }
}

fn fieldbag_to_time_precision(bag: &DateTimeFieldBag) -> Option<options::TimePrecision> {
    if let Some(digits) = bag.fractional_second_digits {
        let ssd = match digits {
            FractionalSecondDigits::F1 => options::SubsecondDigits::S1,
            FractionalSecondDigits::F2 => options::SubsecondDigits::S2,
            FractionalSecondDigits::F3 => options::SubsecondDigits::S3,
        };
        Some(options::TimePrecision::Subsecond(ssd))
    } else if bag.second.is_some() {
        Some(options::TimePrecision::Second)
    } else if bag.minute.is_some() {
        Some(options::TimePrecision::Minute)
    } else if bag.hour.is_some() {
        Some(options::TimePrecision::Hour)
    } else {
        // TODO: What should happen with standalone day period or hour kind?
        None
    }
}

#[allow(clippy::todo)] // TODO: Finish implementing this
fn fieldbag_to_zone_style(bag: &DateTimeFieldBag) -> Option<builder::ZoneStyle> {
    match bag.time_zone_name {
        Some(_) => todo!(),
        None => None,
    }
}

fn fieldbag_to_alignment(bag: &DateTimeFieldBag) -> Option<options::Alignment> {
    // Get all of the fields that have two-digit numerics
    let DateTimeFieldBag {
        era: _,
        // ignore year, since two-digit year is handled in `YearStyle`
        year: _,
        month,
        day,
        weekday: _,
        day_period: _,
        hour_kind: _,
        hour,
        // ignore minute and second, since there is no meaningful difference between
        // numeric and two-digit
        minute: _,
        second: _,
        fractional_second_digits: _,
        time_zone_name: _,
    } = *bag;
    if matches!(month, Some(Month::TwoDigit))
        || matches!(day, Some(Day::TwoDigit))
        || matches!(hour, Some(Hour::TwoDigit))
    {
        Some(options::Alignment::Column)
    } else {
        None
    }
}

fn fieldbag_to_year_style(bag: &DateTimeFieldBag) -> Option<options::YearStyle> {
    match bag.era {
        Some(_era) => Some(options::YearStyle::WithEra),
        None => match bag.year {
            Some(Year::Numeric) => Some(options::YearStyle::Full),
            Some(Year::TwoDigit) => Some(options::YearStyle::Auto),
            None => None,
        },
    }
}

pub(crate) fn fieldbag_to_fieldset(bag: &DateTimeFieldBag) -> builder::FieldSetBuilder {
    builder::FieldSetBuilder {
        length: fieldbag_to_length(bag),
        date_fields: fieldbag_to_date_fields(bag),
        time_precision: fieldbag_to_time_precision(bag),
        zone_style: fieldbag_to_zone_style(bag),
        alignment: fieldbag_to_alignment(bag),
        year_style: fieldbag_to_year_style(bag),
    }
}
