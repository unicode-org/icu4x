// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using test data and a lazy_static.
//!
//! For more information, see the tutorial [cargo.md](../../cargo.md).

use icu::list::ListFormatter;
use icu::list::ListLength;
use icu::locid::locale;
use lazy_static::lazy_static;

lazy_static! {
    static ref SPANISH_LIST_FORMATTER: ListFormatter =
        ListFormatter::try_new_and_with_length_unstable(
            &icu_testdata::unstable(),
            &locale!("es").into(),
            ListLength::Wide
        )
        .expect("Locale 'es' should be present in testdata");
}

fn main() {
    let result = SPANISH_LIST_FORMATTER.format_to_string(["uno", "dos", "tres"].iter());
    assert_eq!(result, "uno, dos y tres");
    println!("{}", result);
}
