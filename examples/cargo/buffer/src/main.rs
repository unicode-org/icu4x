// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using blob data.
//!
//! This project requires a postcard data file. To generate it, please run:
//!
//! ```sh
//! $ cargo build --release
//! $ make
//! ```
//!
//! For more information, see the [index](..).

use icu::calendar::{Date, Gregorian};
use icu::datetime::{fieldsets::YMDT, FixedCalendarDateTimeFormatter};
use icu::locale::locale;
use icu::time::{DateTime, Time};
use icu_provider_blob::BlobDataProvider;

fn main() {
    let blob = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/buffer_data.postcard",))
        .expect("pre-computed postcard buffer should exist");

    let provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
        .expect("deserialization should succeed");

    let formatter = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new_with_buffer_provider(
        &provider,
        locale!("my").into(),
        YMDT::medium(),
    )
    .expect("locale 'my' should be present in compiled data");

    let date = Date::try_new_gregorian(2022, 12, 23).expect("constant should be valid datetime");
    let time = Time::try_new(12, 54, 29, 0).unwrap();

    let result = formatter.format(&DateTime { date, time }).to_string();

    assert_eq!(result, "၂၀၂၂ ဒီ ၂၃ ၁၂:၅၄:၂၉");
    println!("{result}");
}
