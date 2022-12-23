// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::locid::locale;
use icu::plurals::PluralCategory;
use icu::plurals::PluralRules;

mod baked {
    include!("../baked_data/mod.rs");
}

fn main() {
    let rules =
        PluralRules::try_new_cardinal_unstable(&baked::BakedDataProvider, &locale!("ru").into())
            .expect("Locale 'ru' is present in the databake");
    let result = rules.category_for(&3.into());
    assert_eq!(result, PluralCategory::Few);
    println!("{:?}", result);
}
