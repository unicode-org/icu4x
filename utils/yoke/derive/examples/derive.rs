// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused)]

use std::borrow::Cow;
use yoke_derive::{Yokeable, ZeroCopyFrom};
use zerovec::{VarZeroVec, ZeroMap, ZeroVec};

#[derive(Yokeable)]
pub struct Foo {
    x: String,
}

#[derive(Yokeable, ZeroCopyFrom)]
pub struct Bar<'a> {
    x: u8,
    y: &'a str,
    z: Cow<'a, str>,
    w: Cow<'a, [u8]>,
}

#[derive(Yokeable, ZeroCopyFrom)]
pub struct Baz<'a> {
    // https://github.com/unicode-org/icu4x/issues/844
    // map: ZeroMap<'a, String, String>,
    var: VarZeroVec<'a, String>,
    vec: ZeroVec<'a, u16>,
}

fn main() {}
