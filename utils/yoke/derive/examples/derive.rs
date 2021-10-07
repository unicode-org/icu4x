// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused)]

use std::borrow::Cow;
use yoke_derive::{Yokeable, ZeroCopyFrom};
use zerovec::{VarZeroVec, ZeroMap, ZeroVec};

#[derive(Yokeable)]
pub struct StringExample {
    x: String,
}

#[derive(Yokeable)]
pub struct GenericsExample<T> {
    x: String,
    y: T,
}

#[derive(Yokeable, ZeroCopyFrom)]
pub struct CowExample<'a> {
    x: u8,
    y: &'a str,
    z: Cow<'a, str>,
    w: Cow<'a, [u8]>,
}

#[derive(Yokeable, ZeroCopyFrom)]
pub struct ZeroVecExample<'a> {
    var: VarZeroVec<'a, str>,
    vec: ZeroVec<'a, u16>,
}

// Since ZeroMap has generic parameters, the Rust compiler cannot
// prove the covariance of the lifetimes. To use derive(Yokeable)
// with a type such as ZeroMap, you just add the attribute
// yoke(prove_covariance_manually)
#[derive(Yokeable)]
#[yoke(prove_covariance_manually)]
pub struct ZeroMapExample<'a> {
    map: ZeroMap<'a, str, u16>,
}

fn main() {}
