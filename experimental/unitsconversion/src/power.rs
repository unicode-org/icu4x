// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerotrie::{ZeroTrieSimpleAscii, ZeroTrieSimpleAsciiCursor};

// TODO: consider returning Option<(u8, &str)> instead of (1, part) for the case when the power is not found.
// TODO: complete all the cases for the powers.
// TODO: consider using a trie for the powers.
/// Extracts the power from the given CLDR ID part.
///     - If the power is not found, the function returns (1, part).
///     - If the power is found, the function will return (power, part without the string of the power).
pub fn get_power(part: &str) -> (u8, &str) {
    use std::collections::BTreeMap;
    let mut powers = BTreeMap::<Vec<u8>, usize>::new();
    powers.insert(b"pow1".to_vec(), 1);
    powers.insert(b"pow2".to_vec(), 2);
    powers.insert(b"square".to_vec(), 2);
    powers.insert(b"pow3".to_vec(), 3);
    powers.insert(b"cubic".to_vec(), 3);
    powers.insert(b"pow4".to_vec(), 4);
    powers.insert(b"pow5".to_vec(), 5);
    powers.insert(b"pow6".to_vec(), 6);
    powers.insert(b"pow7".to_vec(), 7);
    powers.insert(b"pow8".to_vec(), 8);
    powers.insert(b"pow9".to_vec(), 9);
    powers.insert(b"pow10".to_vec(), 10);
    powers.insert(b"pow11".to_vec(), 11);
    powers.insert(b"pow12".to_vec(), 12);
    powers.insert(b"pow13".to_vec(), 13);
    powers.insert(b"pow14".to_vec(), 14);
    powers.insert(b"pow15".to_vec(), 15);

    let trie = ZeroTrieSimpleAscii::try_from(&powers).unwrap();

    let mut cursor = trie.cursor();
    let mut longest_match = (1, part);
    for (i, b) in part.bytes().enumerate() {
        cursor.step(b);
        if cursor.is_empty() {
            break;
        }
        if let Some(value) = cursor.take_value() {
            longest_match = (value as u8, &part[i + 1..]);
        }
    }
    longest_match
}
