// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project of an interactive app using compiled and blob data.
//!
//! For more information, see these tutorials:
//!
//! - [intro_interactive.md](../../intro_interactive.md).
//! - [data_management_interactive.md](../../data_management_interactive.md).

use icu::locid::Locale;
use icu::calendar::{DateTime, Iso};
use icu::datetime::options::length;
use icu::datetime::DateTimeFormatter;
use icu::locid::locale;
use icu_provider_blob::BlobDataProvider;

const CCP_BLOB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/ccp_all.blob");

/// Helper function to create an ICU4X DateTime for the current local time
fn get_current_datetime() -> DateTime<Iso> {
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

fn main() {
    // In the main() function:
    print!("Enter your locale: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let locale_str = {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf
    };
    
    // Since the string contains whitespace, we must call `.trim()`:
    let locale = match locale_str.trim().parse::<Locale>() {
        Ok(locale) => {
            println!("You entered: {locale}");
            locale
        }
        Err(e) => {
            panic!("Error parsing locale! {e}");
        }
    };
    
    let iso_datetime = get_current_datetime();

    // Create and use an ICU4X date formatter:
    let datetime_formatter = if locale == locale!("ccp") {
        println!("Using buffer provider");

        let blob = std::fs::read(CCP_BLOB_PATH)
            .expect("blob should read successfully")
            .into();

        let provider =
            BlobDataProvider::try_new_from_blob(blob).expect("deserialization should succeed");

        DateTimeFormatter::try_new_with_buffer_provider(
            &provider,
            &(&locale).into(),
            Default::default(),
        )
        .expect("should have data for selected locale")
    } else {
        // As before
        DateTimeFormatter::try_new(&(&locale).into(), Default::default())
            .expect("should have data for selected locale")
    };
    println!(
        "Date: {}",
        datetime_formatter
            .format(&iso_datetime.to_any())
            .expect("date should format successfully")
    );
}
