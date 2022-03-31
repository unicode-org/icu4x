// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused)]

use std::borrow::Cow;
use zerofrom::ZeroFrom;
use zerovec::{maps::ZeroMapKV, ule::AsULE, VarZeroVec, ZeroMap, ZeroVec};

#[derive(ZeroFrom, Copy, Clone)]
pub struct IntExample {
    x: u32,
}

#[derive(ZeroFrom, Copy, Clone)]
pub struct GenericsExample<T> {
    x: u32,
    y: T,
}

#[derive(ZeroFrom)]
pub struct CowExample<'a> {
    x: u8,
    y: &'a str,
    z: Cow<'a, str>,
    w: Cow<'a, [u8]>,
}

#[derive(ZeroFrom)]
pub struct ZeroVecExample<'a> {
    var: VarZeroVec<'a, str>,
    vec: ZeroVec<'a, u16>,
}

#[derive(ZeroFrom)]
pub struct ZeroVecExampleWithGenerics<'a, T: AsULE> {
    gen: ZeroVec<'a, T>,
    vec: ZeroVec<'a, u16>,
    bare: T,
}

#[derive(ZeroFrom)]
pub struct HasTuples<'data> {
    pub bar: (&'data str, &'data str),
}

pub fn assert_zf_tuples<'b, 'data>(x: &'b HasTuples<'data>) -> HasTuples<'b> {
    HasTuples::zero_from(x)
}
pub fn assert_zf_generics<'a, 'b>(
    x: &'b ZeroVecExampleWithGenerics<'a, u8>,
) -> ZeroVecExampleWithGenerics<'b, u8> {
    ZeroVecExampleWithGenerics::<'b, u8>::zero_from(x)
}

#[derive(ZeroFrom)]
pub struct ZeroMapGenericExample<'a, T: for<'b> ZeroMapKV<'b> + ?Sized> {
    map: ZeroMap<'a, str, T>,
}

pub fn assert_zf_map<'a, 'b>(
    x: &'b ZeroMapGenericExample<'a, str>,
) -> ZeroMapGenericExample<'b, str> {
    ZeroMapGenericExample::zero_from(x)
}

#[derive(Clone, ZeroFrom)]
pub struct CloningZF1 {
    #[zerofrom(clone)] // Vec is not ZeroFrom, so it needs to be cloned
    vec: Vec<u8>,
}

#[derive(Clone, ZeroFrom)]
pub struct CloningZF2<'data> {
    #[zerofrom(clone)] // Cow is ZeroFrom, but we force a clone
    cow: Cow<'data, str>,
}

#[derive(ZeroFrom)]
pub enum CloningZF3<'data> {
    Cow(#[zerofrom(clone)] Cow<'data, str>),
}

#[derive(Copy, ZeroFrom)]
pub struct CopyingZF1 {
    #[zerofrom(copy)] // While u8 has a ZeroFrom impl, other copy types might not
    vec: u8,
}

fn main() {}
