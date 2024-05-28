// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(non_camel_case_types, non_snake_case)]

extern crate alloc;

use alloc::borrow::Cow;
use core::str::FromStr;
use icu_pattern::{Pattern, SinglePlaceholder};

#[cfg_attr(feature = "yoke", derive(yoke::Yokeable))]
#[cfg_attr(feature = "zerofrom", derive(zerofrom::ZeroFrom))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "databake", derive(databake::Bake), databake(path = crate))]
#[derive(Debug, PartialEq)]
struct DeriveTest_SinglePlaceholderPattern_Cow<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    data: Pattern<SinglePlaceholder, Cow<'data, str>>,
}

#[test]
#[cfg(all(feature = "databake", feature = "alloc"))]
fn bake_SinglePlaceholderPattern_Cow() {
    use databake::*;
    extern crate std;
    test_bake!(
        DeriveTest_SinglePlaceholderPattern_Cow<'static>,
        crate::DeriveTest_SinglePlaceholderPattern_Cow {
            data: icu_pattern::Pattern::<icu_pattern::SinglePlaceholder, _>::from_store_unchecked(
                alloc::borrow::Cow::Borrowed(""),
            )
        },
    );
}

#[test]
#[cfg(feature = "serde")]
fn json_SinglePlaceholderPattern_Cow() {
    let pattern_owned = Pattern::<SinglePlaceholder, String>::from_str("Hello, {0}!").unwrap();
    let pattern_cow: Pattern<SinglePlaceholder, Cow<str>> =
        Pattern::from_store_unchecked(Cow::Owned(pattern_owned.take_store()));
    let data = DeriveTest_SinglePlaceholderPattern_Cow { data: pattern_cow };
    let data_json = serde_json::to_string(&data).unwrap();
    assert_eq!(
        data_json,
        r#"{"data":[{"Literal":"Hello, "},{"Placeholder":"Singleton"},{"Literal":"!"}]}"#
    );
    let data_deserialized: DeriveTest_SinglePlaceholderPattern_Cow =
        serde_json::from_str(&data_json).unwrap();
    assert_eq!(data, data_deserialized);
}

#[test]
#[cfg(feature = "serde")]
fn postcard_SinglePlaceholderPattern_Cow() {
    let pattern_owned = Pattern::<SinglePlaceholder, String>::from_str("Hello, {0}!").unwrap();
    let pattern_cow: Pattern<SinglePlaceholder, Cow<str>> =
        Pattern::from_store_unchecked(Cow::Owned(pattern_owned.take_store()));
    let data = DeriveTest_SinglePlaceholderPattern_Cow { data: pattern_cow };
    let data_postcard = postcard::to_stdvec(&data).unwrap();
    assert_eq!(data_postcard, b"\x09\x08Hello, !");
    let data_deserialized: DeriveTest_SinglePlaceholderPattern_Cow =
        postcard::from_bytes(&data_postcard).unwrap();
    assert_eq!(data, data_deserialized);
}
