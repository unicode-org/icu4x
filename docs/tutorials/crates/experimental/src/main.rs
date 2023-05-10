// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using test data with experimental features.
//! 
//! For more information, see the tutorial [cargo.md](../../cargo.md).

use icu::displaynames::RegionDisplayNames;
use icu::locid::{locale, subtags_region as region};

fn main() {
    let names = RegionDisplayNames::try_new_unstable(
        &icu_testdata::unstable(),
        &locale!("fr").into(),
        Default::default(),
    )
    .expect("Display names should be present in testdata");
    let name = names.of(region!("DE")).unwrap();
    assert_eq!(name, "Allemagne");
    println!("{}", name);
}
