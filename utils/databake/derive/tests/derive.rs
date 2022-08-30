// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use databake::*;

#[derive(Bake)]
#[databake(path = test)]
pub struct IntExample {
    x: u8,
}

#[test]
#[ignore] // https://github.com/rust-lang/rust/issues/98906
fn test_int_example() {
    test_bake!(
        IntExample,
        const: crate::IntExample { x: 17u8, },
        test,
    );
}

#[derive(Bake)]
#[databake(path = test)]
pub struct GenericsExample<T> {
    x: u32,
    y: T,
}

#[test]
#[ignore] // https://github.com/rust-lang/rust/issues/98906
fn test_generics_example() {
    test_bake!(
        GenericsExample<isize>,
        const: crate::GenericsExample { x: 17, y: 100isize },
        test
    );
}

#[derive(Bake)]
#[databake(path = test)]
pub struct CowExample<'a> {
    x: u8,
    y: &'a str,
    z: alloc::borrow::Cow<'a, str>,
    w: alloc::borrow::Cow<'a, [u8]>,
}

#[test]
#[ignore] // https://github.com/rust-lang/rust/issues/98906
fn test_cow_example() {
    test_bake!(
        CowExample<'static>,
        const: CowExample {
            x: 17u8,
            y: "foo",
            z: alloc::borrow::Cow::Borrowed("bar"),
            w: alloc::borrow::Cow::Borrowed(&[1u8, 2u8, 3u8, ]),
        },
        test
    );
}
