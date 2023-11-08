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
use icu::locid::{locale, Locale};
use icu_provider_blob::BlobDataProvider;

const CCP_BLOB_PATH: &str = "ccp_smallest.blob";

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
            panic!("Error parsing locale! {e}: '{locale_str}'");
        }
    };

    println!();

    // Get the date to format:
    let iso_date = get_current_date();

    // Create and use an ICU4X date formatter:
    let date_formatter = if locale == locale!("ccp") {
        println!("Using buffer provider");

        let blob = std::fs::read(CCP_BLOB_PATH)
            .expect("blob should read successfully")
            .into();

        let provider =
            BlobDataProvider::try_new_from_blob(blob).expect("deserialization should succeed");

        DateFormatter::try_new_with_length_with_buffer_provider(
            &provider,
            &(&locale).into(),
            length::Date::Medium,
        )
        .expect("should have data for selected locale")
    } else {
        DateFormatter::try_new_with_length(&(&locale).into(), length::Date::Medium)
            .expect("should have data for selected locale")
    };

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
