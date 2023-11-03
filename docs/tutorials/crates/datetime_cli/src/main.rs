// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using ICU4X compiled data to build a
//! datetime formatting CLI application.
//!
//! For more information, see the tutorial [cargo.md](../../cargo.md).

use icu::calendar::DateTime;
use icu::datetime::options::length;
use icu::datetime::DateFormatter;
use icu::datetime::DateTimeFormatter;
use icu::datetime::TimeFormatter;
use icu::locid::Locale;

fn main() {
    // Get the locale from user input:
    let mut locale_str = String::new();
    print!("Enter your locale: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::io::stdin().read_line(&mut locale_str).unwrap();

    let parsed_locale: Locale = locale_str.trim().parse().expect("locale should be valid");
    println!();
    println!("OUTPUT:");

    // Get the datetime for formatting:
    let icu4x_datetime = get_current_datetime();

    // Create and use an ICU4X date formatter:
    let date_formatter =
        DateFormatter::try_new_with_length(&(&parsed_locale).into(), length::Date::Medium)
            .expect("should have data for specified locale");
    println!(
        "Date: {}",
        date_formatter
            .format(&icu4x_datetime.to_any())
            .expect("date should format successfully")
    );

    // Create and use an ICU4X time formatter:
    let time_formatter =
        TimeFormatter::try_new_with_length(&(&parsed_locale).into(), length::Time::Medium)
            .expect("should have data for specified locale");
    println!("Time: {}", time_formatter.format(&icu4x_datetime.to_any()));

    // Create and use an ICU4X datetime formatter:
    let datetime_formatter =
        DateTimeFormatter::try_from_date_and_time(date_formatter, time_formatter)
            .expect("date and time formatters should merge successfully");
    println!(
        "DateTime: {}",
        datetime_formatter
            .format(&icu4x_datetime.to_any())
            .expect("datetime should format successfully")
    );
}

fn get_current_datetime() -> DateTime<icu::calendar::Iso> {
    let current_offset_date_time = time::OffsetDateTime::now_local().unwrap();
    DateTime::try_new_iso_datetime(
        current_offset_date_time.year(),
        current_offset_date_time.month() as u8,
        current_offset_date_time.day(),
        current_offset_date_time.hour(),
        current_offset_date_time.minute(),
        current_offset_date_time.second(),
    )
    .unwrap()
}
