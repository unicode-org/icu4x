// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Debug;
use zerovec::*;
use zerovec_derive::*;

#[make_ule]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Struct {
    a: u8,
    b: u32,
    c: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[make_ule]
struct TupleStruct(u8, char);

#[make_ule]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
enum Enum {
    A = 0,
    B = 1,
    D = 2,
    E = 3,
    FooBar = 4,
    F = 5,
}

fn test_zerovec<T: ule::AsULE + Debug + PartialEq>(slice: &[T]) {
    let zerovec: ZeroVec<T> = slice.iter().copied().collect();

    assert_eq!(zerovec, slice);

    let bytes = zerovec.as_bytes();
    let _name = std::any::type_name::<T>();
    let reparsed: ZeroVec<T> = ZeroVec::parse_byte_slice(bytes)
        .unwrap_or_else(|_| panic!("{}", "Parsing {name} should succeed"));

    assert_eq!(reparsed, slice);
}

fn main() {
    test_zerovec(TEST_SLICE_STRUCT);
    test_zerovec(TEST_SLICE_TUPLESTRUCT);
    test_zerovec(TEST_SLICE_ENUM);
}

const TEST_SLICE_STRUCT: &[Struct] = &[
    Struct {
        a: 101,
        b: 924,
        c: '⸘',
    },
    Struct {
        a: 217,
        b: 4228,
        c: 'ə',
    },
    Struct {
        a: 117,
        b: 9090,
        c: 'ø',
    },
];

const TEST_SLICE_TUPLESTRUCT: &[TupleStruct] = &[
    TupleStruct(101, 'ř'),
    TupleStruct(76, '°'),
    TupleStruct(15, 'a'),
];

const TEST_SLICE_ENUM: &[Enum] = &[
    Enum::A,
    Enum::FooBar,
    Enum::F,
    Enum::D,
    Enum::B,
    Enum::FooBar,
    Enum::E,
];
