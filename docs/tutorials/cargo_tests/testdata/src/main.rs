// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using test data.
//! 
//! For more information, see the tutorial [cargo.md](../../cargo.md).

use icu::decimal::FixedDecimalFormatter;
use icu::locid::locale;

fn main() {
    let formatter = FixedDecimalFormatter::try_new_unstable(
        &icu_testdata::unstable(),
        &locale!("bn").into(),
        Default::default(),
    )
    .expect("Locale 'bn' should be present in testdata");
    let result = formatter.format_to_string(&1000007.into());
    assert_eq!(result, "১০,০০,০০৭");
    println!("{}", result);
}
