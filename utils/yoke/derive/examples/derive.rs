// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused)]

use std::borrow::Cow;
use yoke::{Yoke, Yokeable, ZeroCopyFrom};
use zerovec::{map::ZeroMapKV, ule::AsULE, VarZeroVec, ZeroMap, ZeroVec};

#[derive(Yokeable)]
pub struct StringExample {
    x: String,
}

#[derive(Yokeable, ZeroCopyFrom, Copy, Clone)]
pub struct IntExample {
    x: u32,
}

#[derive(Yokeable, ZeroCopyFrom, Copy, Clone)]
pub struct GenericsExample<T> {
    x: u32,
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

#[derive(Yokeable, ZeroCopyFrom)]
pub struct ZeroVecExampleWithGenerics<'a, T: AsULE> {
    gen: ZeroVec<'a, T>,
    vec: ZeroVec<'a, u16>,
    bare: T,
}

pub fn assert_zcf_generics<'a, 'b>(
    x: &'b ZeroVecExampleWithGenerics<'a, u8>,
) -> ZeroVecExampleWithGenerics<'b, u8> {
    ZeroVecExampleWithGenerics::<'static, u8>::zero_copy_from(x)
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

#[derive(Yokeable, ZeroCopyFrom)]
#[yoke(prove_covariance_manually)]
pub struct ZeroMapGenericExample<'a, T: for<'b> ZeroMapKV<'b> + ?Sized> {
    map: ZeroMap<'a, str, T>,
}

pub fn assert_zcf_map<'a, 'b>(
    x: &'b ZeroMapGenericExample<'a, str>,
) -> ZeroMapGenericExample<'b, str> {
    ZeroMapGenericExample::<'static, str>::zero_copy_from(x)
}

pub struct AssertYokeable {
    string: Yoke<StringExample, Box<[u8]>>,
    int: Yoke<IntExample, Box<[u8]>>,
    gen1: Yoke<GenericsExample<u32>, Box<[u8]>>,
    gen2: Yoke<GenericsExample<String>, Box<[u8]>>,
    cow: Yoke<CowExample<'static>, Box<[u8]>>,
    zv: Yoke<ZeroVecExample<'static>, Box<[u8]>>,
    zv_gen1: Yoke<ZeroVecExampleWithGenerics<'static, u8>, Box<[u8]>>,
    zv_gen2: Yoke<ZeroVecExampleWithGenerics<'static, char>, Box<[u8]>>,
    map: Yoke<ZeroMapExample<'static>, Box<[u8]>>,
    map_gen1: Yoke<ZeroMapGenericExample<'static, u32>, Box<[u8]>>,
    map_gen2: Yoke<ZeroMapGenericExample<'static, str>, Box<[u8]>>,
}

fn main() {}
