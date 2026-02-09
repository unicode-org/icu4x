// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::{
    datetime::{fieldsets, DateTimeFormatter, FixedCalendarDateTimeFormatter},
    locale::locale,
    time::ZonedDateTime,
};

fn main() {
    // jiff requires `std` as of 0.2
    let jiff = jiff::Timestamp::from_nanosecond(1726011440123456789)
        .unwrap()
        .to_zoned(jiff::tz::TimeZone::get("Asia/Tokyo").unwrap());

    // chrono and chrono_tz are `#[no_std]`
    let chrono = chrono::DateTime::from_timestamp_nanos(1726011440123456789)
        .with_timezone(&"Asia/Tokyo".parse().unwrap());

    // jiff and chrono types can be formatted with a `FixedCalendarDateTimeFormatter<Gregorian>`

    let fixed_formatter = FixedCalendarDateTimeFormatter::try_new(
        locale!("fr").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .unwrap();

    println!("direct jiff: {}", fixed_formatter.format(&jiff));
    println!("direct chrono: {}", fixed_formatter.format(&chrono));

    // For calendrical conversions and usage with `DateTimeFormatter`, conversions between icu
    // types and jiff/chrono types are also available

    let converting_formatter = DateTimeFormatter::try_new(
        locale!("en-GB-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .expect("data is present");

    println!(
        "converted jiff: {}",
        converting_formatter.format(&ZonedDateTime::from(&jiff))
    );
    println!(
        "converted chrono: {}",
        converting_formatter.format(&ZonedDateTime::from(&chrono))
    );
}
