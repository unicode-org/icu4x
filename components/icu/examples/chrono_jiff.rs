// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use icu::{
    datetime::{fieldsets, DateTimeFormatter},
    locale::locale,
};

fn main() {
    // jiff requires `std` as of 0.2
    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789).unwrap();

    // chrono and chrono_tz are `#[no_std]`
    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789);

    // Japanese, Japanese calendar, Tokyo Time

    let formatter = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .expect("data is present");

    println!(
        "{}",
        formatter.format(&jiff.to_zoned(
            jiff::tz::TimeZone::get("Asia/Tokyo").unwrap_or(jiff::tz::TimeZone::unknown())
        ))
    );

    println!(
        "{}",
        formatter.format(
            &chrono.with_timezone(
                &chrono_tz::Tz::from_str("Asia/Tokyo").unwrap_or(chrono_tz::Tz::UTC)
            )
        )
    );

    // English, Gregorian calendar, Honolulu Time

    let formatter = DateTimeFormatter::try_new(
        locale!("en-US").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .expect("data is present");

    println!(
        "{}",
        formatter.format(&jiff.to_zoned(
            jiff::tz::TimeZone::get("Pacific/Honolulu").unwrap_or(jiff::tz::TimeZone::unknown())
        ))
    );
    println!(
        "{}",
        formatter.format(&chrono.with_timezone(
            &chrono_tz::Tz::from_str("Pacific/Honolulu").unwrap_or(chrono_tz::Tz::UTC)
        ))
    );

    // System locale, system time

    let formatter = DateTimeFormatter::try_new(
        icu_host_info::datetime_preferences().unwrap(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .expect("data is present");

    println!(
        "{}",
        formatter.format(
            &jiff.to_zoned(
                jiff::tz::TimeZone::try_system().unwrap_or(jiff::tz::TimeZone::unknown())
            )
        )
    );
}
