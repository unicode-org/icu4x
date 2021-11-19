// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use icu_codepointtrie::TrieValue;
use icu_provider::DataPayload;
use crate::provider::{UnicodePropertyV1Marker, UnicodePropertyMapV1Marker};

//
// Structs and traits for returning data as sets
//

pub struct CodePointSetData {
    pub payload: DataPayload<UnicodePropertyV1Marker>,
}

pub trait CodePointSetLike {
    fn contains(&self, ch: char) -> bool;

    fn contains_u32(&self, code_point: u32) -> bool;

    // return type cannot be `impl ExactSizeIterator<Item = RangeInclusive<u32>>;`. 
    // compiler says: `impl Trait` not allowed outside of function and method return types
    // For making FFI easier, we should provide a flat structure of the inversion list 
    // instead of an iterator of range structs.
    fn iter_ranges(&self) -> Vec<u32>;
}

impl CodePointSetLike for CodePointSetData {
    fn contains(&self, ch: char) -> bool {
        self.payload.get().inv_list.contains(ch)
    }

    fn contains_u32(&self, code_point: u32) -> bool {
        self.payload.get().inv_list.contains_u32(code_point)
    }

    fn iter_ranges(&self) -> Vec<u32> {
        let set = &self.payload.get().inv_list;
        let mut result: Vec<u32> = Vec::new();
        let ranges = set.iter_ranges();
        for range in ranges {
            result.push(*range.start());
            result.push(*range.end() + 1);
        }
        result
    }
}
