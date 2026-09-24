// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::fieldsets::builder;
use crate::options;

fn fieldbag_to_length(bag: &DateTimeFieldBag) -> Option<options::Length> {
    // Get all of the fields that are possibly non-numeric.
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
    // If any of the fields are Long, use a Long length.
    // If any are Short, use a Medium length.
    // If any numeric-or-alphabetic fields are numeric, use a Short length.
    // Else, use the default length.
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
    // Get all of the date fields.
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
    // Select the best DateFields for the fields present in the bag, filling in missing
    // internal fields if necessary.
    match date_field_bag {
        // No fields: return None
        DateFieldBag {
            era: None,
            year: None,
            month: None,
            day: None,
            weekday: None,
        } => None,
        // Y: Era and/or Year
        DateFieldBag {
            era: _,
            year: _,
            month: None,
            day: None,
            weekday: None,
        } => Some(builder::DateFields::Y),
        // M: Month only
        DateFieldBag {
            era: None,
            year: None,
            month: _,
            day: None,
            weekday: None,
        } => Some(builder::DateFields::M),
        // YM: Month and (Era or Year)
        DateFieldBag {
            era: _,
            year: _,
            month: Some(_),
            day: None,
            weekday: None,
        } => Some(builder::DateFields::YM),
        // D: Day only
        DateFieldBag {
            era: None,
            year: None,
            month: None,
            day: Some(_),
            weekday: None,
        } => Some(builder::DateFields::D),
        // MD: Month and Day
        DateFieldBag {
            era: None,
            year: None,
            month: Some(_),
            day: Some(_),
            weekday: None,
        } => Some(builder::DateFields::MD),
        // YMD: Day and (Era or Year) and maybe Month
        DateFieldBag {
            era: _,
            year: _,
            month: _,
            day: Some(_),
            weekday: None,
        } => Some(builder::DateFields::YMD),
        // E: Weekday only
        DateFieldBag {
            era: None,
            year: None,
            month: None,
            day: None,
            weekday: Some(_),
        } => Some(builder::DateFields::E),
        // DE: Day and Weekday
        DateFieldBag {
            era: None,
            year: None,
            month: None,
            day: Some(_),
            weekday: Some(_),
        } => Some(builder::DateFields::DE),
        // MDE: Month and Weekday and maybe Day
        DateFieldBag {
            era: None,
            year: None,
            month: Some(_),
            day: _,
            weekday: Some(_),
        } => Some(builder::DateFields::MDE),
        // YMDE: Weekday and maybe Day and maybe Month and (Era or Year)
        DateFieldBag {
            era: _,
            year: _,
            month: _,
            day: _,
            weekday: Some(_),
        } => Some(builder::DateFields::YMDE),
    }
}

#[test]
fn test_fieldbag_to_date_fields_all_cases() {
    use itertools::Itertools;
    let all_inclusive = "GyMdE";
    for chars in all_inclusive.chars().powerset() {
        let mut bag = DateTimeFieldBag::default();
        if chars.contains(&'G') {
            bag.era = Some(Era::Short);
        }
        if chars.contains(&'y') {
            bag.year = Some(Year::Numeric);
        }
        if chars.contains(&'M') {
            bag.month = Some(Month::Short);
        }
        if chars.contains(&'d') {
            bag.day = Some(Day::Numeric);
        }
        if chars.contains(&'E') {
            bag.weekday = Some(Weekday::Short);
        }
        let actual = fieldbag_to_date_fields(&bag);
        let skeleton = chars.iter().collect::<String>();
        let expected = match &*skeleton {
            // TODO(agent): rewrite this test to use an array of test cases,
            // where each case is a skeleton string and an expected DateFields,
            // instead of Itertools::powerset() and this match statement, which
            // is more brittle/error-prone
            "" => None,
            "E" => Some(builder::DateFields::E),
            "d" => Some(builder::DateFields::D),
            "dE" => Some(builder::DateFields::DE),
            "M" => Some(builder::DateFields::M),
            "ME" => Some(builder::DateFields::MDE), // implied
            "Md" => Some(builder::DateFields::MD),
            "MdE" => Some(builder::DateFields::MDE),
            "y" => Some(builder::DateFields::Y),
            "yE" => Some(builder::DateFields::YMDE), // implied
            "yd" => Some(builder::DateFields::YMD),  // implied
            "ydE" => Some(builder::DateFields::YMDE), // implied
            "yM" => Some(builder::DateFields::YM),
            "yME" => Some(builder::DateFields::YMDE), // implied
            "yMd" => Some(builder::DateFields::YMD),
            "yMdE" => Some(builder::DateFields::YMDE),
            // Both G and y imply a Year field
            "G" => Some(builder::DateFields::Y),
            "GE" => Some(builder::DateFields::YMDE), // implied
            "Gd" => Some(builder::DateFields::YMD),  // implied
            "GdE" => Some(builder::DateFields::YMDE), // implied
            "GM" => Some(builder::DateFields::YM),
            "GME" => Some(builder::DateFields::YMDE), // implied
            "GMd" => Some(builder::DateFields::YMD),
            "GMdE" => Some(builder::DateFields::YMDE),
            "Gy" => Some(builder::DateFields::Y),
            "GyE" => Some(builder::DateFields::YMDE), // implied
            "Gyd" => Some(builder::DateFields::YMD),  // implied
            "GydE" => Some(builder::DateFields::YMDE),
            "GyM" => Some(builder::DateFields::YM),
            "GyME" => Some(builder::DateFields::YMDE), // implied
            "GyMd" => Some(builder::DateFields::YMD),
            "GyMdE" => Some(builder::DateFields::YMDE),
            _ => unreachable!("{skeleton:?}"),
        };
        assert_eq!(actual, expected, "{skeleton:?} {bag:?}")
    }
}

fn fieldbag_to_time_precision(bag: &DateTimeFieldBag) -> Option<options::TimePrecision> {
    // Note: This takes the smallest field and uses it as the time precision, which may
    // "fill in" fields of greater magnitude, even if they aren't in the bag.
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
    // Get all of the fields that have a numeric style.
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
    // If any of the fields are two-digit, set the fieldset to column alignment.
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
    // Determine the year style based on the era and year fields.
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

pub(crate) fn fieldset_to_fieldbag(fieldset: &builder::FieldSetBuilder) -> DateTimeFieldBag {
    DateTimeFieldBag {
        era: match fieldset.year_style {
            Some(options::YearStyle::WithEra) => Some(Era::Short),
            _ => None,
        },
        year: match fieldset.date_fields {
            Some(date_fields) if date_fields.has_year() => match fieldset.year_style {
                Some(options::YearStyle::Auto) => Some(Year::TwoDigit),
                _ => Some(Year::Numeric),
            },
            _ => None,
        },
        month: match fieldset.date_fields {
            Some(date_fields) if date_fields.has_month() => match fieldset.length {
                Some(options::Length::Long) => Some(Month::Long),
                Some(options::Length::Medium) | None => Some(Month::Short),
                Some(options::Length::Short) => match fieldset.alignment {
                    Some(options::Alignment::Column) => Some(Month::TwoDigit),
                    Some(options::Alignment::Auto) | None => Some(Month::Numeric),
                },
            },
            _ => None,
        },
        day: match fieldset.date_fields {
            Some(date_fields) if date_fields.has_day() => match fieldset.alignment {
                Some(options::Alignment::Column) => Some(Day::TwoDigit),
                Some(options::Alignment::Auto) | None => Some(Day::Numeric),
            },
            _ => None,
        },
        weekday: match fieldset.date_fields {
            Some(date_fields) if date_fields.has_weekday() => match fieldset.length {
                Some(options::Length::Long) => Some(Weekday::Long),
                _ => Some(Weekday::Short),
            },
            _ => None,
        },
        // TODO: this crate doesn't support flexible day periods yet in fieldsets
        day_period: None,
        // TODO: figure out how to plumb the hour cycle to here
        hour_kind: None,
        hour: match fieldset.time_precision {
            Some(time_precision) if time_precision.has_hour() => match fieldset.alignment {
                Some(options::Alignment::Column) => Some(Hour::TwoDigit),
                Some(options::Alignment::Auto) | None => Some(Hour::Numeric),
            },
            _ => None,
        },
        minute: match fieldset.time_precision {
            Some(time_precision) if time_precision.has_minute() => match fieldset.alignment {
                Some(options::Alignment::Column) => Some(Minute::TwoDigit),
                Some(options::Alignment::Auto) | None => Some(Minute::Numeric),
            },
            _ => None,
        },
        second: match fieldset.time_precision {
            Some(time_precision) if time_precision.has_second() => match fieldset.alignment {
                Some(options::Alignment::Column) => Some(Second::TwoDigit),
                Some(options::Alignment::Auto) | None => Some(Second::Numeric),
            },
            _ => None,
        },
        fractional_second_digits: match fieldset.time_precision {
            // Requires https://github.com/rust-lang/rust/issues/51114:
            // Some(time_precision) if let Some(ssd) = time_precision.has_subsecond() => match ssd {
            Some(time_precision) => match time_precision.has_subsecond() {
                Some(options::SubsecondDigits::S1) => Some(FractionalSecondDigits::F1),
                Some(options::SubsecondDigits::S2) => Some(FractionalSecondDigits::F2),
                Some(_) => Some(FractionalSecondDigits::F3),
                None => None,
            },
            _ => None,
        },
        #[allow(clippy::todo)] // TODO(agent): finish implementing
        time_zone_name: match fieldset.zone_style {
            Some(_) => todo!(),
            None => None,
        },
    }
}
