// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(non_camel_case_types, non_snake_case)]

extern crate alloc;

use alloc::borrow::Cow;
use icu_pattern::{Pattern, SinglePlaceholder};

#[cfg_attr(feature = "yoke", derive(yoke::Yokeable))]
#[cfg_attr(feature = "zerofrom", derive(zerofrom::ZeroFrom))]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "databake", derive(databake::Bake), databake(path = crate))]
struct DeriveTest_SinglePlaceholderPattern_ZeroVec<'data> {
    // #[cfg_attr(feature = "serde", serde(borrow))]
    _data: Pattern<SinglePlaceholder, Cow<'data, str>>,
}

#[test]
#[cfg(all(feature = "databake", feature = "alloc"))]
fn bake_SinglePlaceholderPattern_ZeroVec() {
    use databake::*;
    extern crate std;
    test_bake!(
        DeriveTest_SinglePlaceholderPattern_ZeroVec<'static>,
        crate::DeriveTest_SinglePlaceholderPattern_ZeroVec {
            _data: icu_pattern::Pattern::<icu_pattern::SinglePlaceholder, _>::from_store_unchecked(
                alloc::borrow::Cow::Borrowed(""),
            )
        },
    );
}
