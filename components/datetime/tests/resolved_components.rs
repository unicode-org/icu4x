// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Gregorian;
use icu_datetime::{
    options::{components, length, preferences},
    DateTimeFormatterOptions, TypedDateTimeFormatter,
};
use icu_locid::locale;
use icu_locid::Locale;

fn assert_resolved_components(
    options: DateTimeFormatterOptions,
    bag: &components::Bag,
    locale: Locale,
) {
    let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_experimental(&locale.into(), options)
        .expect("Failed to create a TypedDateTimeFormatter.");

    assert_eq!(dtf.resolve_components(), *bag);
}

#[test]
fn test_length_date() {
    let length_bag = length::Bag::from_date_style(length::Date::Medium);

    let mut components_bag = components::Bag::default();
    components_bag.year = Some(components::Year::Numeric);
    components_bag.month = Some(components::Month::Short);
    components_bag.day = Some(components::Day::NumericDayOfMonth);
    assert_resolved_components(
        DateTimeFormatterOptions::Length(length_bag),
        &components_bag,
        locale!("en"),
    );
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
    assert_resolved_components(
        DateTimeFormatterOptions::Length(length_bag),
        &components_bag,
        "en-u-hc-h12".parse::<Locale>().unwrap(),
    );
}

#[test]
fn test_length_time_preferences() {
    let length_bag = length::Bag::from_time_style(length::Time::Medium);

    let mut components_bag = components::Bag::default();
    components_bag.hour = Some(components::Numeric::TwoDigit);
    components_bag.minute = Some(components::Numeric::TwoDigit);
    components_bag.second = Some(components::Numeric::TwoDigit);
    components_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H24,
    ));

    assert_resolved_components(
        DateTimeFormatterOptions::Length(length_bag),
        &components_bag,
        "en-u-hc-h24".parse::<Locale>().unwrap(),
    );
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
    input_bag.fractional_second = Some(4);
    input_bag.preferences = None;
    let mut output_bag = input_bag; // make a copy
    output_bag.month = Some(components::Month::Short);
    output_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H23,
    ));

    assert_resolved_components(
        DateTimeFormatterOptions::Components(input_bag),
        &output_bag,
        "en-u-hc-h23".parse::<Locale>().unwrap(),
    );
}
