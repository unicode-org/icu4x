// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use yoke_derive::{Yokeable, ZeroCopyFrom};
use zerovec::{VarZeroVec, ZeroVec};

#[derive(Yokeable, ZeroCopyFrom)]
pub struct ZeroVecExample<'a> {
    var: VarZeroVec<'a, String>,
    vec: ZeroVec<'a, u16>,
}

fn main() {}

