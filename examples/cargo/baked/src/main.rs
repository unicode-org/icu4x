// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using baked data.
//!
//! For more information, see the [index](..).

use icu::locale::locale;
use icu::plurals::PluralCategory;
use icu::plurals::PluralRules;

include!(concat!(env!("OUT_DIR"), "/baked_data/mod.rs"));

pub struct BakedProvider;
impl_data_provider!(BakedProvider);

fn main() {
    let rules = PluralRules::try_new_cardinal_unstable(&BakedProvider, locale!("ru").into())
        .expect("locale 'ru' should be present in the baked data");
    let result = rules.category_for(3);
    assert_eq!(result, PluralCategory::Few);
    println!("{:?}", result);
}
