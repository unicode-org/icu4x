// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which displays the number of lines added and
// removed from a series of pull requests.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use fixed_decimal::FixedDecimal;
use icu_decimal::{options, FixedDecimalFormat};
use icu_locid::Locale;
use icu_locid_macros::langid;
use writeable::Writeable;

icu_benchmark_macros::static_setup!();

const LINES_REMOVED_ADDED: [(i64, i64); 5] = [
    (-50, 72),
    (0, 3750),
    (-1201, 0),
    (-9876, 5432),
    (-5000000, 3000000),
];

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let locale: Locale = langid!("bn").into();

    let provider = icu_testdata::get_provider();

    let mut options: options::FixedDecimalFormatOptions = Default::default();
    options.sign_display = options::SignDisplay::ExceptZero;

    let fdf = FixedDecimalFormat::try_new(locale, &provider, options)
        .expect("Failed to create FixedDecimalFormat instance.");

    for line in LINES_REMOVED_ADDED.iter() {
        let decimals: (FixedDecimal, FixedDecimal) = (line.0.into(), line.1.into());
        let removed = fdf.format(&decimals.0);
        let added = fdf.format(&decimals.1);
        assert_ne!("", removed.writeable_to_string());
        assert_ne!("", added.writeable_to_string());
        #[cfg(debug_assertions)]
        println!(
            "Added/Removed: {}/{}",
            removed.writeable_to_string(),
            added.writeable_to_string()
        );
    }

    0
}
