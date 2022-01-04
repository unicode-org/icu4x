// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CaseMapping;
use icu_uniset::{UnicodeSet, UnicodeSetBuilder};

struct UnicodeSetWithStrings<'a> {
    chars: UnicodeSet<'a>,
    strings: Vec<String>
}

enum ClosureAttribute {
    CaseInsensitive,
    AddCaseMappings
}

// This is ~UnicodeSet::closeOver
pub fn case_closure<'a>(set: &UnicodeSetWithStrings<'a>, mapping: &CaseMapping) -> UnicodeSetWithStrings<'a> {
    let mut  = UnicodeSetBuilder::new();
    builder.add_set(set.chars);



    builder.build()
}
