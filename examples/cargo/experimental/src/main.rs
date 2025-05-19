// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project using test data with experimental features.
//! 
//! For more information, see the [index](..).

use icu::experimental::displaynames::RegionDisplayNames;
use icu::locale::{locale, subtags::region};

fn main() {
    let names = RegionDisplayNames::try_new(
        locale!("fr").into(),
        Default::default(),
    )
    .expect("locale 'fr' should be present in compiled data");
    let name = names.of(region!("DE")).unwrap();
    assert_eq!(name, "Allemagne");
    println!("{name}");
}
