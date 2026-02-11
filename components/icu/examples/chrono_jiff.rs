// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::{
    datetime::{fieldsets, DateTimeFormatter},
    locale::locale,
};

fn main() {
    // jiff requires `std` as of 0.2
    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789).unwrap();

    // chrono and chrono_tz are `#[no_std]`
    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789);

    let formatter = DateTimeFormatter::try_new(
        locale!("ja-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .expect("data is present");

    println!("{}", formatter.format(&jiff.in_tz("Asia/Tokyo").unwrap()));

    println!(
        "{}",
        formatter.format(&chrono.with_timezone(&"Asia/Tokyo".parse().unwrap()))
    );

    let formatter = DateTimeFormatter::try_new(
        locale!("en-US").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .expect("data is present");

    println!(
        "{}",
        formatter.format(&jiff.in_tz("Pacific/Honolulu").unwrap())
    );
    println!(
        "{}",
        formatter.format(&chrono.with_timezone(&"Pacific/Honolulu".parse().unwrap()))
    );
}
