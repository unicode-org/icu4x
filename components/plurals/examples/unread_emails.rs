// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which uses icu_plurals to construct a correct
// sentence for English based on the numerical value in Cardinal category.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use icu_locid::locale;
use icu_plurals::{PluralCategory, PluralRules};

const VALUES: &[usize] = &[0, 2, 25, 1, 3, 2, 4, 10, 7, 0];

fn print(_input: &str, _value: Option<usize>) {
    #[cfg(debug_assertions)]
    if let Some(value) = _value {
        println!("{}", _input.replace("{}", &value.to_string()));
    } else {
        println!("{_input}");
    }
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    {
        print("\n====== Unread Emails (en) example ============", None);
        let pr = PluralRules::try_new_cardinal(&locale!("en").into())
            .expect("Failed to create a PluralRules instance.");

        for value in VALUES {
            match pr.category_for(*value) {
                PluralCategory::One => print("You have one unread email.", None),
                _ => print("You have {} unread emails.", Some(*value)),
            }
        }
    }

    0
}
