// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::{
    maps::ZeroMapKV,
    ule::{AsULE, ZeroVecError, ULE},
};



/// `ConstantValueULE` is a type optimized for efficient storing and
/// deserialization of `CurrencyPatterns` using the `ZeroVec` model.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct ConstantValueULE([u8; 3]);


// TODO: Implement the ULE trait for ConstantValueULE.