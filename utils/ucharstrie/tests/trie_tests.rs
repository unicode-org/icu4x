// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
mod test_util;

use ucharstrie::ucharstrie::{TrieResult, UCharsTrie, UCharsTrieIterator};
use zerovec::ZeroVec;

#[test]
fn ucharstrie_test_months() {
    let trie_data = test_util::load_ucharstrie_data("tests/testdata/months.toml");
    let trie = UCharsTrie {
        data: ZeroVec::from_slice(trie_data.as_slice()),
    };

    let mut itor = UCharsTrieIterator::new(trie.data.as_slice(), 0);
    for (chr, expected) in [
        ('j', TrieResult::NoValue),
        ('u', TrieResult::NoValue),
        ('n', TrieResult::Intermediate(6)),
        ('e', TrieResult::FinalValue(6)),
    ] {
        let res = itor.next(chr as i32);
        assert_eq!(res, expected);
    }

    let mut itor = UCharsTrieIterator::new(trie.data.as_slice(), 0);
    for (chr, expected) in [
        ('j', TrieResult::NoValue),
        ('u', TrieResult::NoValue),
        ('l', TrieResult::NoValue),
        ('y', TrieResult::FinalValue(7)),
    ] {
        let res = itor.next(chr as i32);
        assert_eq!(res, expected);
    }
}
