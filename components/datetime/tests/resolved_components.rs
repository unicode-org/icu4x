// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Gregorian;
use icu_datetime::{
    options::{components, length, preferences},
    DateTimeFormat, DateTimeFormatOptions,
};
use icu_locid::locale;

fn assert_resolved_components(options: &DateTimeFormatOptions, bag: &components::Bag) {
    let provider = icu_testdata::get_provider();
    let dtf = DateTimeFormat::<Gregorian>::try_new(locale!("en"), &provider, options)
        .expect("Failed to create a DateTimeFormat.");

    assert_eq!(dtf.resolve_components(), *bag);
}

#[test]
fn test_length_date() {
    assert_resolved_components(
        &DateTimeFormatOptions::Length(length::Bag {
            date: Some(length::Date::Medium),
            time: None,
            preferences: None,
        }),
        &components::Bag {
            year: Some(components::Year::Numeric),
            month: Some(components::Month::Short),
            day: Some(components::Numeric::Numeric),
            ..Default::default()
        },
    );
}

#[test]
fn test_length_time() {
    assert_resolved_components(
        &DateTimeFormatOptions::Length(length::Bag {
            date: None,
            time: Some(length::Time::Medium),
            preferences: None,
        }),
        &components::Bag {
            hour: Some(components::Numeric::Numeric),
            minute: Some(components::Numeric::TwoDigit),
            second: Some(components::Numeric::TwoDigit),
            preferences: Some(preferences::Bag {
                hour_cycle: Some(preferences::HourCycle::H12),
            }),
            ..Default::default()
        },
    );
}

#[test]
fn test_length_time_preferences() {
    assert_resolved_components(
        &DateTimeFormatOptions::Length(length::Bag {
            date: None,
            time: Some(length::Time::Medium),
            preferences: Some(preferences::Bag {
                hour_cycle: Some(preferences::HourCycle::H24),
            }),
        }),
        &components::Bag {
            hour: Some(components::Numeric::TwoDigit),
            minute: Some(components::Numeric::TwoDigit),
            second: Some(components::Numeric::TwoDigit),
            preferences: Some(preferences::Bag {
                hour_cycle: Some(preferences::HourCycle::H24),
            }),
            ..Default::default()
        },
    );
}

#[test]
fn test_components_bag() {
    assert_resolved_components(
        &DateTimeFormatOptions::Components(components::Bag {
            era: Some(components::Text::Short),
            year: Some(components::Year::Numeric),
            month: Some(components::Month::Numeric),
            day: Some(components::Numeric::TwoDigit),
            weekday: Some(components::Text::Long),
            hour: Some(components::Numeric::Numeric),
            minute: Some(components::Numeric::TwoDigit),
            second: Some(components::Numeric::TwoDigit),
            preferences: None,
            ..Default::default()
        }),
        &components::Bag {
            era: Some(components::Text::Short),
            year: Some(components::Year::Numeric),
            month: Some(components::Month::Short),
            day: Some(components::Numeric::TwoDigit),
            weekday: Some(components::Text::Long),
            hour: Some(components::Numeric::Numeric),
            minute: Some(components::Numeric::TwoDigit),
            second: Some(components::Numeric::TwoDigit),
            preferences: Some(preferences::Bag {
                hour_cycle: Some(preferences::HourCycle::H23),
            }),
            ..Default::default()
        },
    );
}
