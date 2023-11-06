// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using ICU4X compiled data to build a
//! datetime formatting CLI application.
//!
//! For more information, see the tutorial [intro_interactive.md](../../intro_interactive.md).

use icu::calendar::{Date, Iso};
use icu::datetime::options::length;
use icu::datetime::DateFormatter;
use icu::locid::Locale;

fn main() {
    // Get the locale from user input:
    print!("Enter your locale: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let locale_str = {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf
    };

    let locale = match locale_str.trim().parse::<Locale>() {
        Ok(locale) => {
            println!("You entered: {locale}");
            locale
        }
        Err(e) => {
            panic!("Error parsing locale! {e}");
        }
    };

    println!();

    // Get the date to format:
    let iso_date = get_current_date();

    // Create and use an ICU4X date formatter:
    let date_formatter =
        DateFormatter::try_new_with_length(&(&locale).into(), length::Date::Medium)
            .expect("should have data for specified locale");
    println!(
        "Date: {}",
        date_formatter
            .format(&iso_date.to_any())
            .expect("date should format successfully")
    );
}

fn get_current_date() -> Date<Iso> {
    let current_offset_date_time = time::OffsetDateTime::now_local().unwrap();
    Date::try_new_iso_date(
        current_offset_date_time.year(),
        current_offset_date_time.month() as u8,
        current_offset_date_time.day(),
    )
    .unwrap()
}
