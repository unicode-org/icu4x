// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{options::length, neo::TypedNeoDateTimeFormatter};
use icu_locid::langid;
use icu_calendar::DateTime;

#[test]
fn neo_datetime_lengths() {
    let datetime = DateTime::try_new_gregorian_datetime(2023, 12, 22, 21, 22, 53).unwrap();
    for date_length in [length::Date::Full, length::Date::Long, length::Date::Medium, length::Date::Short] {
        for time_length in [length::Time::Medium, length::Time::Short] {
            for langid in [langid!("en"), langid!("fr"), langid!("zh"), langid!("hi")] {
                let formatter = TypedNeoDateTimeFormatter::try_new_with_lengths(&langid.into(), date_length, time_length).unwrap();
                let formatted = formatter.format(&datetime);
                println!("{formatted}");
            }
        }
    }
}
