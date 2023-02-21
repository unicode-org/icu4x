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
//! For more information, see the tutorial [cargo.md](../../cargo.md).

use icu::calendar::DateTime;
use icu::datetime::DateTimeFormatter;
use icu::locid::locale;
use icu_provider_blob::BlobDataProvider;

fn main() {
    let blob = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/buffer_data.postcard",))
        .expect("Pre-computed postcard buffer should exist");

    let provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
        .expect("Deserialization should succeed");

    let formatter = DateTimeFormatter::try_new_with_buffer_provider(
        &provider,
        &locale!("my").into(),
        Default::default(),
    )
    .expect("Locale 'my' should be present in testdata");

    let datetime = DateTime::try_new_iso_datetime(2022, 12, 23, 12, 54, 29)
        .expect("Valid constant ISO datetime");

    let result = formatter
        .format_to_string(&datetime.to_any())
        .expect("Calendar matches formatter");

    assert_eq!(result, "၂၀၂၂၊ ဒီ ၂၃ ၁၂:၅၄:၂၉");
    println!("{}", result);
}
