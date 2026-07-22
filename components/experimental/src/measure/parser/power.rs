// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerotrie::ZeroTrieSimpleAscii;

/// A trie that contains the powers.
pub const POWERS_TRIE: ZeroTrieSimpleAscii<[u8; 64]> = ZeroTrieSimpleAscii::from_sorted_str_tuples(&[
    ("cubic", 3),
    ("pow1", 1),
    ("pow10", 10),
    ("pow11", 11),
    ("pow12", 12),
    ("pow13", 13),
    ("pow14", 14),
    ("pow15", 15),
    ("pow2", 2),
    ("pow3", 3),
    ("pow4", 4),
    ("pow5", 5),
    ("pow6", 6),
    ("pow7", 7),
    ("pow8", 8),
    ("pow9", 9),
    ("square", 2),
]);
