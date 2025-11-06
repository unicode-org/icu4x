// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::{
    calendar::{Date, Iso},
    datetime::{fieldsets, DateTimeFormatter},
    locale::locale,
    time::{
        zone::{models::AtTime, IanaParser, UtcOffset},
        DateTime, Time, TimeZoneInfo, ZonedDateTime,
    },
};

fn main() {
    // jiff requires `std` as of 0.2
    let from_jiff = jiff_to_icu(
        &jiff::Timestamp::from_nanosecond(1726011440123456789)
            .unwrap()
            .to_zoned(jiff::tz::TimeZone::get("Asia/Tokyo").unwrap()),
    );

    // chrono and chrono_tz are `#[no_std]`
    let from_chrono = chrono_to_icu(
        &chrono::DateTime::from_timestamp_nanos(1726011440123456789)
            .with_timezone(&"Asia/Tokyo".parse().unwrap()),
    );

    let from_ixdtf = ZonedDateTime::try_strict_from_str(
        "2024-09-11T08:37:20.123456789+09:00[Asia/Tokyo]",
        Iso,
        IanaParser::new(),
    )
    .expect("valid string");

    assert_eq!(from_jiff, from_chrono);
    assert_eq!(from_chrono, from_ixdtf);

    // A English, Japanese calendar, medium-length, year-month-day-time-specific-zone formatter
    let formatter = DateTimeFormatter::try_new(
        locale!("en-GB-u-ca-japanese").into(),
        fieldsets::YMDT::medium().with_zone(fieldsets::zone::SpecificLong),
    )
    .expect("data is present");

    println!("{}", formatter.format(&from_chrono)); // 11 Sept 6 Reiwa, 08:37:20 Japan Standard Time
}

fn jiff_to_icu(jiff: &jiff::Zoned) -> ZonedDateTime<Iso, TimeZoneInfo<AtTime>> {
    let date = Date::try_new_iso(jiff.year() as i32, jiff.month() as u8, jiff.day() as u8)
        .expect("jiff returns valid fields");

    let time = Time::try_new(
        jiff.hour() as u8,
        jiff.minute() as u8,
        jiff.second() as u8,
        jiff.millisecond() as u32 * 1_000_000
            + jiff.microsecond() as u32 * 1_000
            + jiff.nanosecond() as u32,
    )
    .expect("jiff returns valid fields");

    let date_time = DateTime { date, time };

    let zone =
        // Parse IANA ID into ICU time zone
        IanaParser::new().parse(jiff.time_zone().iana_name().unwrap())
        // In ICU's model, a time zone has a fixed offset, as that's required for formatting
        .with_offset(UtcOffset::try_from_seconds(jiff.offset().seconds()).ok())
        // Display names might change over time for a given zone (e.g. it might change from Eastern Time to
        // Central Time), so the ICU timezone needs a reference date and time.
        .at_date_time_iso(date_time);

    ZonedDateTime { date, time, zone }
}

fn chrono_to_icu(
    chrono: &chrono::DateTime<chrono_tz::Tz>,
) -> ZonedDateTime<Iso, TimeZoneInfo<AtTime>> {
    use chrono::Datelike;
    let date = Date::try_new_iso(chrono.year(), chrono.month() as u8, chrono.day() as u8)
        .expect("chrono returns valid fields");

    use chrono::Timelike;
    let time = Time::try_new(
        chrono.hour() as u8,
        chrono.minute() as u8,
        chrono.second() as u8,
        chrono.nanosecond(),
    )
    .expect("chrono returns valid fields");

    let date_time = DateTime { date, time };

    use chrono_tz::OffsetComponents;
    let zone =
        // Parse IANA ID into ICU time zone
        IanaParser::new().parse(chrono.timezone().name())
        // In ICU's model, a time zone has a fixed offset, as that's required for formatting
        .with_offset(UtcOffset::try_from_seconds((chrono.offset().base_utc_offset() + chrono.offset().dst_offset()).num_seconds() as i32).ok())
        // Display names might change over time for a given zone (e.g. it might change from Eastern Time to
        // Central Time), so the ICU timezone needs a reference date and time.
        .at_date_time_iso(date_time);

    ZonedDateTime { date, time, zone }
}
