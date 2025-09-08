// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use databake::*;

#[derive(Bake)]
#[databake(path = test)]
pub struct IntExample {
    pub x: u8,
}

#[test]
fn test_int_example() {
    test_bake!(IntExample, const, crate::IntExample { x: 17u8 }, test,);
}

#[derive(Bake)]
#[databake(path = test)]
pub struct GenericsExample<T> {
    pub x: u32,
    pub y: T,
}

#[test]
fn test_generics_example() {
    test_bake!(
        GenericsExample<isize>,
        const,
        crate::GenericsExample {
            x: 17u32,
            y: 100isize
        },
        test
    );
}

#[derive(Bake)]
#[databake(path = test)]
pub struct CowExample<'a> {
    pub x: u8,
    pub y: &'a str,
    pub z: alloc::borrow::Cow<'a, str>,
    pub w: alloc::borrow::Cow<'a, [u8]>,
}

#[test]
fn test_cow_example() {
    test_bake!(
        CowExample<'static>,
        const,
        crate::CowExample {
            x: 17u8,
            y: "foo",
            z: alloc::borrow::Cow::Borrowed("bar"),
            w: alloc::borrow::Cow::Borrowed(b"\x01\x02\x03"),
        },
        test
    );
}
