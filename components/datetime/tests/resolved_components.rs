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
    let length_bag = length::Bag::from_date_style(length::Date::Medium);

    let mut components_bag = components::Bag::default();
    components_bag.year = Some(components::Year::Numeric);
    components_bag.month = Some(components::Month::Short);
    components_bag.day = Some(components::Day::NumericDayOfMonth);
    assert_resolved_components(&DateTimeFormatOptions::Length(length_bag), &components_bag);
}

#[test]
fn test_length_time() {
    let length_bag = length::Bag::from_time_style(length::Time::Medium);
    let mut components_bag = components::Bag::default();
    components_bag.hour = Some(components::Numeric::Numeric);
    components_bag.minute = Some(components::Numeric::TwoDigit);
    components_bag.second = Some(components::Numeric::TwoDigit);
    components_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H12,
    ));
    assert_resolved_components(&DateTimeFormatOptions::Length(length_bag), &components_bag);
}

#[test]
fn test_length_time_preferences() {
    let mut length_bag = length::Bag::from_time_style(length::Time::Medium);
    length_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H24,
    ));

    let mut components_bag = components::Bag::default();
    components_bag.hour = Some(components::Numeric::TwoDigit);
    components_bag.minute = Some(components::Numeric::TwoDigit);
    components_bag.second = Some(components::Numeric::TwoDigit);
    components_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H24,
    ));

    assert_resolved_components(&DateTimeFormatOptions::Length(length_bag), &components_bag);
}

#[test]
fn test_components_bag() {
    let mut input_bag = components::Bag::default();
    input_bag.era = Some(components::Text::Short);
    input_bag.year = Some(components::Year::Numeric);
    input_bag.month = Some(components::Month::Numeric);
    input_bag.day = Some(components::Day::TwoDigitDayOfMonth);
    input_bag.weekday = Some(components::Text::Long);
    input_bag.hour = Some(components::Numeric::Numeric);
    input_bag.minute = Some(components::Numeric::TwoDigit);
    input_bag.second = Some(components::Numeric::TwoDigit);
    input_bag.preferences = None;
    let mut output_bag = input_bag; // make a copy
    output_bag.month = Some(components::Month::Short);
    output_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H23,
    ));

    assert_resolved_components(&DateTimeFormatOptions::Components(input_bag), &output_bag);
}
