// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using a `OnceLock`.
//!
//! For more information, see the [index](..).

use icu::list::ListFormatter;
use icu::locale::locale;
use std::sync::OnceLock;

static SPANISH_LIST_FORMATTER: OnceLock<ListFormatter> = OnceLock::new();

fn main() {
    let result = SPANISH_LIST_FORMATTER
        .get_or_init(|| {
            ListFormatter::try_new_and(locale!("es").into(), Default::default())
                .expect("locale 'es' should be present in compiled data")
        })
        .format_to_string(["uno", "dos", "tres"].iter());
    assert_eq!(result, "uno, dos y tres");
    println!("{result}");
}
