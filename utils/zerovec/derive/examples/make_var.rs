// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;
use std::fmt::Debug;
use zerovec::*;
use zerovec_derive::*;

#[make_varule]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct VarStruct<'a> {
    a: u32,
    b: char,
    c: Cow<'a, str>,
}

fn main() {
    use zerovec::ule::AsULE;
    let varzerovec: VarZeroVec<VarStructULE> = TEST_VARSTRUCTS.into();

    assert_eq!(varzerovec.len(), TEST_VARSTRUCTS.len());

    for (stack, zero) in TEST_VARSTRUCTS.iter().zip(varzerovec.iter()) {
        assert_eq!(zero.a, stack.a.as_unaligned());
        assert_eq!(zero.b, stack.b.as_unaligned());
        assert_eq!(zero.c, stack.c);
    }

    let bytes = varzerovec.as_bytes();
    let reparsed: VarZeroVec<VarStructULE> = VarZeroVec::parse_byte_slice(bytes)
        .unwrap_or_else(|_| panic!("{}", "Parsing VarZeroVec<VarStructULE> should succeed"));

    assert_eq!(reparsed.len(), TEST_VARSTRUCTS.len());

    for (stack, zero) in TEST_VARSTRUCTS.iter().zip(reparsed.iter()) {
        assert_eq!(zero.a, stack.a.as_unaligned());
        assert_eq!(zero.b, stack.b.as_unaligned());
        assert_eq!(zero.c, stack.c);
    }
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
