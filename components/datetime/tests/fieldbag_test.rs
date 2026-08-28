// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::Date;
use icu_datetime::DateTimeFormatter;
use icu_datetime::DateTimeFormatterPreferences;
use icu_datetime::fieldbag::*;
use icu_datetime::fieldsets::builder::FieldSetBuilder;
use icu_locale_core::locale;
use icu_time::DateTime;
use icu_time::Time;
use writeable::assert_writeable_eq;

#[test]
fn test_basic() {
    let skeleton = "GyMMMdHm";
    let bag_with_preferences =
        DateTimeFieldBagWithPreferences::try_from_skeleton(skeleton).unwrap();
    let builder = FieldSetBuilder::from_field_bag(&bag_with_preferences.bag);
    let field_set = builder.build_composite_datetime().unwrap();
    let locale = locale!("en-u-ca-japanese");
    let preferences = DateTimeFormatterPreferences::from(locale)
        .merge_field_preferences(bag_with_preferences.preferences);

    let formatter = DateTimeFormatter::try_new(preferences, field_set).unwrap();

    assert_writeable_eq!(
        formatter.format(&DateTime {
            date: Date::try_new_iso(2026, 8, 28).unwrap(),
            time: Time::try_new(14, 36, 30, 0).unwrap(),
        }),
        "Aug 28, 8 Reiwa, 14:36"
    );
}

#[test]
fn test_from_str_and_builder_methods() {
    let bag_with_prefs: DateTimeFieldBagWithPreferences = "yMMMd".parse().unwrap();
    let builder = bag_with_prefs.bag.to_field_set_builder();
    let _ = builder.build_date().unwrap();

    let prefs: DateTimeFormatterPreferences = bag_with_prefs.preferences.into();
    let _ = prefs;
}

#[test]
fn test_errors() {
    assert_eq!(
        DateTimeFieldBagWithPreferences::try_from_skeleton("yMMMdX").unwrap_err(),
        DateTimeFieldBagParseError::InvalidSymbol('X')
    );
    assert_eq!(
        DateTimeFieldBagWithPreferences::try_from_skeleton("GGGGGG").unwrap_err(),
        DateTimeFieldBagParseError::InvalidLength('G', 6)
    );
    assert_eq!(
        DateTimeFieldBagWithPreferences::try_from_skeleton("ddd").unwrap_err(),
        DateTimeFieldBagParseError::InvalidLength('d', 3)
    );
    assert_eq!(
        DateTimeFieldBagWithPreferences::try_from_skeleton("yMyM").unwrap_err(),
        DateTimeFieldBagParseError::DuplicateSymbol('y')
    );
}

#[test]
fn test_hour_cycles() {
    use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("h").unwrap();
    assert_eq!(bag.preferences.hour_cycle, Some(HourCycle::H12));
    assert_eq!(bag.bag.hour, Some(field::Hour::Numeric));

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("HH").unwrap();
    assert_eq!(bag.preferences.hour_cycle, Some(HourCycle::H23));
    assert_eq!(bag.bag.hour, Some(field::Hour::TwoDigit));

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("K").unwrap();
    assert_eq!(bag.preferences.hour_cycle, Some(HourCycle::H11));

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("k").unwrap();
    assert_eq!(bag.preferences.hour_cycle, Some(HourCycle::H23));

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("j").unwrap();
    assert_eq!(bag.preferences.hour_cycle, None);
    assert_eq!(bag.bag.hour, Some(field::Hour::Numeric));

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("C").unwrap();
    assert_eq!(bag.bag.day_period, Some(field::DayPeriod::Short));
    assert_eq!(bag.bag.hour, Some(field::Hour::Numeric));
}

#[test]
fn test_zones_and_subseconds() {
    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("S").unwrap();
    assert_eq!(bag.bag.subsecond, Some(field::Subsecond::S1));

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("SSS").unwrap();
    assert_eq!(bag.bag.subsecond, Some(field::Subsecond::S3));

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("z").unwrap();
    assert_eq!(
        bag.bag.time_zone_name,
        Some(field::TimeZoneName::ShortSpecific)
    );

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("OOOO").unwrap();
    assert_eq!(
        bag.bag.time_zone_name,
        Some(field::TimeZoneName::LongOffset)
    );

    let bag = DateTimeFieldBagWithPreferences::try_from_skeleton("vvvv").unwrap();
    assert_eq!(
        bag.bag.time_zone_name,
        Some(field::TimeZoneName::LongGeneric)
    );
}
