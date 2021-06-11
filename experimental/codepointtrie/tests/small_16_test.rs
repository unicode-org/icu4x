// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_codepointtrie::codepointtrie::{CodePointTrie, CodePointTrieHeader, Small};
use icu_codepointtrie::error::Error;

mod test_util;

use test_util::check_trie;

const INDEX: [u16; 64] = [
    0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0x80, 0xc0, 0xc0, 0xc0, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const DATA_16: [u16; 258] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 0xad,
];

const CHECK_RANGES: [u32; 10] = [0, 1, 0x740, 1, 0x780, 2, 0x880, 3, 0x110000, 1];

// Exported trie data from free-blocks.8.toml. This file represents a
// fast-type trie with 8-bit width data.
fn get_testing_small_type_16_bit_trie<'trie>() -> CodePointTrie<'trie, u16, Small> {
    let index_length: u32 = 64;
    let data_length: u32 = 258;
    let high_start: u32 = 0xa00;
    let shifted12_high_start: u16 = 0x1;
    let index3_null_offset: u16 = 0x7fff;
    let data_null_offset: u32 = 0x0;
    let null_value: u32 = 0x1;

    let header = CodePointTrieHeader {
        index_length,
        data_length,
        high_start,
        shifted12_high_start,
        index3_null_offset,
        data_null_offset,
        null_value,
    };

    let trie_new_result: Result<CodePointTrie<'trie, u16, Small>, Error> =
        CodePointTrie::try_new(header, &INDEX, &DATA_16);
    let trie = trie_new_result.unwrap();

    trie
}

#[test]
pub fn get_test() {
    let trie = get_testing_small_type_16_bit_trie();

    assert_eq!(trie.get(0), 1);
    assert_eq!(trie.get(1), 1);
    assert_eq!(trie.get(2), 1);
    assert_eq!(trie.get(28), 1);
    assert_eq!(trie.get(29), 1);
}

#[test]
pub fn check_ranges_test() {
    let trie = get_testing_small_type_16_bit_trie();

    check_trie(&trie, &CHECK_RANGES);
}