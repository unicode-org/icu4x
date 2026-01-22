// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointtrie::TrieType;

/// Returns the type and width arguments for `umutablecptrie_buildImmutable`
pub(crate) fn args_for_build_immutable<U>(trie_type: TrieType) -> (u32, u32) {
    let trie_type = match trie_type {
        TrieType::Fast => 0,
        TrieType::Small => 1,
    };
    let width = match size_of::<U>() {
        1 => 2,     // UCPTRIE_VALUE_BITS_8
        2 => 0,     // UCPTRIE_VALUE_BITS_16
        3 | 4 => 1, // UCPTRIE_VALUE_BITS_32
        other => panic!("Don't know how to make trie with width {other}"),
    };
    (trie_type, width)
}
