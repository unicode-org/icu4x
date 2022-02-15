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
    a: u8,
    b: char,
    c: Cow<'a, str>,
}

fn main() {}
