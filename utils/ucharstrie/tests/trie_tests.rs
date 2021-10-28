// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
mod test_util;

use ucharstrie::trie::{Trie, TrieResult};
use ucharstrie::ucharstrie::UCharsTrie;

#[test]
fn ucharstrie_test_months() {
    let data = test_util::build_ucharstrie_from_test_data("tests/testdata/months.toml");
    let trie_data = unsafe { &*(data.as_slice() as *const [u16] as *const [u8]) };

    let mut trie = UCharsTrie::new(0);
    for (chr, expected) in [
        ('j', TrieResult::NoValue),
        ('u', TrieResult::NoValue),
        ('n', TrieResult::Intermediate),
        ('e', TrieResult::FinalValue),
    ] {
        let res = trie.next(trie_data, chr as i32);
        assert_eq!(res, expected);
    }
    assert_eq!(trie.get_value(data.as_slice()), Some(6));

    let mut trie = UCharsTrie::new(0);
    for (chr, expected) in [
        ('j', TrieResult::NoValue),
        ('u', TrieResult::NoValue),
        ('l', TrieResult::NoValue),
        ('y', TrieResult::FinalValue),
    ] {
        let res = trie.next(trie_data, chr as i32);
        assert_eq!(res, expected);
    }
    assert_eq!(trie.get_value(data.as_slice()), Some(7));
}
