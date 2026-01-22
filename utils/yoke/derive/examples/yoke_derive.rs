// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(unused)]

use std::{borrow::Cow, marker::PhantomData};
use yoke::{Yoke, Yokeable};
use zerovec::{maps::ZeroMapKV, ule::AsULE, VarZeroVec, ZeroMap, ZeroVec};

#[derive(Yokeable)]
struct StringExample {
    x: String,
}

#[derive(Yokeable, Copy, Clone)]
struct IntExample {
    x: u32,
}

#[derive(Yokeable, Copy, Clone)]
struct GenericsExample<T> {
    x: u32,
    y: T,
}

#[derive(Yokeable, Copy, Clone)]
struct GenericsExampleWithDefault<T, U = usize> {
    x: T,
    y: U,
}

#[derive(Yokeable)]
struct CowExample<'a> {
    x: u8,
    y: &'a str,
    z: Cow<'a, str>,
    w: Cow<'a, [u8]>,
}

#[derive(Yokeable)]
struct ZeroVecExample<'a> {
    var: VarZeroVec<'a, str>,
    vec: ZeroVec<'a, u16>,
}

#[derive(Yokeable)]
struct ZeroVecExampleWithGenerics<'a, T: AsULE> {
    gen: ZeroVec<'a, T>,
    vec: ZeroVec<'a, u16>,
    bare: T,
}

// Since ZeroMap has generic parameters, the Rust compiler cannot
// prove the covariance of the lifetimes. To use derive(Yokeable)
// with a type such as ZeroMap, you just add the attribute
// yoke(prove_covariance_manually)
#[derive(Yokeable)]
#[yoke(prove_covariance_manually)]
struct ZeroMapExample<'a> {
    map: ZeroMap<'a, str, u16>,
}

#[derive(Yokeable)]
#[yoke(prove_covariance_manually)]
struct ZeroMapGenericExample<'a, T: for<'b> ZeroMapKV<'b> + ?Sized> {
    map: ZeroMap<'a, str, T>,
}

#[derive(Yokeable)]
struct MaybeSizedWrap<T, Q: ?Sized, U: ?Sized> {
    x: T,
    y: Option<T>,
    ignored: PhantomData<U>,
    q: Q,
}

trait Trait {}
impl Trait for u32 {}

#[derive(Yokeable)]
struct WithTraitBounds<T: Trait> {
    x: T,
}

#[derive(Yokeable)]
struct WithTraitBoundsInWhere<T>
where
    T: Trait,
{
    x: T,
}

// TODO(#4119): Make this example compile
/*
#[derive(Yokeable)]
struct MaybeSizedWrapWithLifetime<'a, T, Q: ?Sized, U: ?Sized> {
    x: T,
    y: Option<T>,
    ignored: &'a U,
    q: Q,
}
*/

struct AssertYokeable {
    string: Yoke<StringExample, Box<[u8]>>,
    int: Yoke<IntExample, Box<[u8]>>,
    gen1: Yoke<GenericsExample<u32>, Box<[u8]>>,
    gen2: Yoke<GenericsExample<String>, Box<[u8]>>,
    gen_default1: Yoke<GenericsExampleWithDefault<String>, Box<[u8]>>,
    gen_default2: Yoke<GenericsExampleWithDefault<String, u8>, Box<[u8]>>,
    cow: Yoke<CowExample<'static>, Box<[u8]>>,
    zv: Yoke<ZeroVecExample<'static>, Box<[u8]>>,
    zv_gen1: Yoke<ZeroVecExampleWithGenerics<'static, u8>, Box<[u8]>>,
    zv_gen2: Yoke<ZeroVecExampleWithGenerics<'static, char>, Box<[u8]>>,
    map: Yoke<ZeroMapExample<'static>, Box<[u8]>>,
    map_gen1: Yoke<ZeroMapGenericExample<'static, u32>, Box<[u8]>>,
    map_gen2: Yoke<ZeroMapGenericExample<'static, str>, Box<[u8]>>,
    maybe_sized_wrap: Yoke<MaybeSizedWrap<usize, usize, str>, Box<[u8]>>,
    trait_bounds: Yoke<WithTraitBounds<u32>, Box<[u8]>>,
    trait_bounds_where: Yoke<WithTraitBoundsInWhere<u32>, Box<[u8]>>,
    // TODO(#4119): Make this example compile
    // maybe_sized_wrap_with_lt: Yoke<MaybeSizedWrapWithLifetime<'static, usize, usize, str>, Box<[u8]>>,
}

fn main() {}
