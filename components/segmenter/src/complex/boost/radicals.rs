// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::UnihanIrgData; 
use icu::collections::codepointtrie::CodePointTrie;

pub struct UnihanSegmenter<'data> {
    trie: &'data CodePointTrie<'data, u8>,
}

impl<'data> UnihanSegmenter<'data> {
    pub fn new(data: &'data UnihanIrgData<'data>) -> Self {
        Self {
            trie: &data.trie,
        }
    }

    pub fn get_irg_value(&self, c: char) -> u8 {
        self.trie.get(c as u32)
    }
}