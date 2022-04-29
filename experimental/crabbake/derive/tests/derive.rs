// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crabbake::{Bakeable, CrateEnv};
use quote::quote;
use std::borrow::Cow;

#[derive(Bakeable)]
#[crabbake(path = test)]
pub struct IntExample {
    x: u8,
}

#[test]
fn test_int_example() {
    assert_eq!(
        IntExample { x: 17 }.bake(&CrateEnv::default()).to_string(),
        quote! { ::test::IntExample { x: 17u8, }}.to_string()
    );
}

#[derive(Bakeable)]
#[crabbake(path = test)]
pub struct GenericsExample<T> {
    x: u32,
    y: T,
}

#[test]
fn test_generics_example() {
    assert_eq!(
        GenericsExample { x: 17, y: 100isize }
            .bake(&CrateEnv::default())
            .to_string(),
        quote! { ::test::GenericsExample { x: 17u32, y: 100isize, }}.to_string()
    );
}

#[derive(Bakeable)]
#[crabbake(path = test)]
pub struct CowExample<'a> {
    x: u8,
    y: &'a str,
    z: Cow<'a, str>,
    w: Cow<'a, [u8]>,
}

#[test]
fn test_cow_example() {
    assert_eq!(
        CowExample {
            x: 17,
            y: "foo",
            z: Cow::Borrowed("bar"),
            w: Cow::Borrowed(&[1, 2, 3]),
        }
        .bake(&CrateEnv::default())
        .to_string(),
        quote! {
            ::test::CowExample {
                x: 17u8,
                y: "foo",
                z: ::alloc::borrow::Cow::Borrowed("bar"),
                w: ::alloc::borrow::Cow::Borrowed(&[1u8,2u8,3u8]),
            }
        }
        .to_string()
    );
}
