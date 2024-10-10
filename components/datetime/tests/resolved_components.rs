// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{DateTime, Gregorian};
use icu_datetime::{
    neo::TypedNeoFormatter,
    neo_skeleton::{
        Alignment, FractionalSecondDigits, NeoDateComponents, NeoDateTimeComponents,
        NeoDateTimeSkeleton, NeoSkeletonLength, NeoTimeComponents, YearStyle,
    },
    options::{components, preferences},
};
use icu_locale_core::locale;
use icu_locale_core::Locale;

fn assert_resolved_components(
    skeleton: NeoDateTimeSkeleton,
    bag: &components::Bag,
    locale: Locale,
) {
    let dtf = TypedNeoFormatter::<Gregorian, _>::try_new_with_components(&locale.into(), skeleton)
        .unwrap();
    let datetime = DateTime::local_unix_epoch().to_calendar(Gregorian);
    let resolved_pattern = dtf.format(&datetime).pattern();
    assert_eq!(components::Bag::from(&resolved_pattern), *bag);
}

#[test]
fn test_length_date() {
    let skeleton = NeoDateTimeSkeleton::for_length_and_components(
        NeoSkeletonLength::Medium,
        NeoDateTimeComponents::Date(NeoDateComponents::Auto),
    );

    let mut components_bag = components::Bag::default();
    components_bag.year = Some(components::Year::Numeric);
    components_bag.month = Some(components::Month::Short);
    components_bag.day = Some(components::Day::NumericDayOfMonth);

    assert_resolved_components(skeleton, &components_bag, locale!("en"));
}

#[test]
fn test_length_time() {
    let skeleton = NeoDateTimeSkeleton::for_length_and_components(
        NeoSkeletonLength::Medium,
        NeoDateTimeComponents::Time(NeoTimeComponents::Auto),
    );

    let mut components_bag = components::Bag::default();
    components_bag.hour = Some(components::Numeric::Numeric);
    components_bag.minute = Some(components::Numeric::TwoDigit);
    components_bag.second = Some(components::Numeric::TwoDigit);
    components_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H12,
    ));

    assert_resolved_components(
        skeleton,
        &components_bag,
        "en-u-hc-h12".parse::<Locale>().unwrap(),
    );
}

#[test]
fn test_length_time_preferences() {
    let mut skeleton = NeoDateTimeSkeleton::for_length_and_components(
        NeoSkeletonLength::Medium,
        NeoDateTimeComponents::Time(NeoTimeComponents::Auto),
    );
    skeleton.alignment = Some(Alignment::Column);

    let mut components_bag = components::Bag::default();
    components_bag.hour = Some(components::Numeric::TwoDigit);
    components_bag.minute = Some(components::Numeric::TwoDigit);
    components_bag.second = Some(components::Numeric::TwoDigit);
    components_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H24,
    ));

    assert_resolved_components(
        skeleton,
        &components_bag,
        "en-u-hc-h24".parse::<Locale>().unwrap(),
    );
}

#[test]
fn test_date_and_time() {
    let mut skeleton = NeoDateTimeSkeleton::for_length_and_components(
        NeoSkeletonLength::Medium,
        NeoDateTimeComponents::DateTime(
            NeoDateComponents::YearMonthDayWeekday,
            NeoTimeComponents::Auto,
        ),
    );
    skeleton.year_style = Some(YearStyle::Always);
    skeleton.fractional_second_digits = Some(FractionalSecondDigits::F4);
    skeleton.alignment = Some(Alignment::Column);

    let mut input_bag = components::Bag::default();
    input_bag.era = Some(components::Text::Short);
    input_bag.year = Some(components::Year::Numeric);
    input_bag.month = Some(components::Month::Numeric);
    input_bag.day = Some(components::Day::TwoDigitDayOfMonth);
    input_bag.weekday = Some(components::Text::Short);
    input_bag.hour = Some(components::Numeric::TwoDigit);
    input_bag.minute = Some(components::Numeric::TwoDigit);
    input_bag.second = Some(components::Numeric::TwoDigit);
    input_bag.fractional_second = Some(FractionalSecondDigits::F4);
    input_bag.preferences = None;
    let mut output_bag = input_bag; // make a copy
    output_bag.month = Some(components::Month::Short);
    output_bag.preferences = Some(preferences::Bag::from_hour_cycle(
        preferences::HourCycle::H23,
    ));

    assert_resolved_components(
        skeleton,
        &output_bag,
        "en-u-hc-h23".parse::<Locale>().unwrap(),
    );
}
