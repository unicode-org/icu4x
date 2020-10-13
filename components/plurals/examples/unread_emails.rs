// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// An example application which uses icu_plurals to construct a correct
// sentence for English based on the numerical value in Cardinal category.
use icu_locale_macros::langid;
use icu_plurals::{PluralCategory, PluralRuleType, PluralRules};

const VALUES: &[usize] = &[0, 2, 25, 1, 3, 2, 4, 10, 7, 0];

fn print(_input: &str, _value: Option<usize>) {
    #[cfg(debug_assertions)]
    if let Some(value) = _value {
        println!("{}", _input.replace("{}", &value.to_string()));
    } else {
        println!("{}", _input);
    }
}

fn main() {
    let lid = langid!("en");
    let provider = icu_testdata::get_provider();

    {
        print("\n====== Unread Emails (en) example ============", None);
        let pr = PluralRules::try_new(lid, &provider, PluralRuleType::Cardinal)
            .expect("Failed to create a PluralRules instance.");

        for value in VALUES {
            match pr.select(*value) {
                PluralCategory::One => print("You have one unread email.", None),
                _ => print("You have {} unread emails.", Some(*value)),
            }
        }
    }
}
