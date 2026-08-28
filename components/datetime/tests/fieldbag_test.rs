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
    )
}
