// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;

use zerofrom::ZeroFrom;
use zerovec::*;
use zerovec_derive::*;

#[make_varule(VarStructULE)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct VarStruct<'a> {
    a: u32,
    b: char,
    c: Cow<'a, str>,
}

#[make_varule(VarTupleStructULE)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct VarTupleStruct<'a>(u32, char, VarZeroVec<'a, str>);

#[make_varule(NoKVULE)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[zerovec::skip_kv]
struct NoKV<'a>(u32, char, VarZeroVec<'a, str>);

#[make_varule(NoOrdULE)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[zerovec::skip_kv]
#[zerovec::skip_ord]
struct NoOrd<'a>(u32, char, VarZeroVec<'a, str>);

/// The `assert` function should have the body `|(stack, zero)| assert_eq!(stack, &U::zero_from(&zero))`
///
/// We cannot do this internally because we technically need a different `U` with a shorter lifetime here
/// which would require some gnarly lifetime bounds and perhaps a Yoke dependency. This is just a test, so it's
/// not important to get this 100% perfect
fn assert_zerovec<T: ule::VarULE + ?Sized, U: ule::EncodeAsVarULE<T>, F: Fn(&U, &T)>(
    slice: &[U],
    assert: F,
) {
    let varzerovec: VarZeroVec<T> = slice.into();

    assert_eq!(varzerovec.len(), slice.len());

    for (stack, zero) in slice.iter().zip(varzerovec.iter()) {
        assert(stack, zero)
    }

    let bytes = varzerovec.as_bytes();
    let name = std::any::type_name::<T>();
    let reparsed: VarZeroVec<T> = VarZeroVec::parse_byte_slice(bytes)
        .unwrap_or_else(|_| panic!("Parsing VarZeroVec<{}> should succeed", name));

    assert_eq!(reparsed.len(), slice.len());

    for (stack, zero) in slice.iter().zip(reparsed.iter()) {
        assert(stack, zero)
    }
}

fn main() {
    assert_zerovec::<VarStructULE, VarStruct, _>(TEST_VARSTRUCTS, |stack, zero| {
        assert_eq!(stack, &VarStruct::zero_from(zero))
    });

    let vartuples = &[
        VarTupleStruct(101, 'ø', TEST_STRINGS1.into()),
        VarTupleStruct(9499, '⸘', TEST_STRINGS2.into()),
        VarTupleStruct(3478, '月', TEST_STRINGS3.into()),
    ];
    assert_zerovec::<VarTupleStructULE, VarTupleStruct, _>(vartuples, |stack, zero| {
        assert_eq!(stack, &VarTupleStruct::zero_from(zero))
    });
}

const TEST_VARSTRUCTS: &[VarStruct<'static>] = &[
    VarStruct {
        a: 101,
        b: 'ø',
        c: Cow::Borrowed("testīng strīng"),
    },
    VarStruct {
        a: 9499,
        b: '⸘',
        c: Cow::Borrowed("a diﬀərənt ştring"),
    },
    VarStruct {
        a: 3478,
        b: '月',
        c: Cow::Borrowed("好多嘅 string"),
    },
];

const TEST_STRINGS1: &[&str] = &["foo", "bar", "baz"];
const TEST_STRINGS2: &[&str] = &["hellø", "wørłd"];
const TEST_STRINGS3: &[&str] = &["łořem", "ɨpsu₥"];
