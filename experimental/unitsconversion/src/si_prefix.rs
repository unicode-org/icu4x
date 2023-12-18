// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use zerotrie::ZeroTrieSimpleAscii;

use crate::provider::{Base, SiPrefix};

// TODO: consider returning Option<(i8, &str)> instead of (0, part) for the case when the prefix is not found.
// TODO: consider using a trie for the prefixes.
// TODO: complete all the cases for the prefixes.
/// Extracts the SI prefix of base 10.
/// NOTE:
///    if the prefix is found, the function will return (power, part without the prefix).
///    if the prefix is not found, the function will return (0, part).
fn get_si_prefix_base_ten(part: &str) -> (i8, &str) {
    let prefixes = vec![
        ("quetta", 30, 0),
        ("ronna", 27, 1),
        ("yotta", 24, 2),
        ("zetta", 21, 3),
        ("exa", 18, 4),
        ("peta", 15, 5),
        ("tera", 12, 6),
        ("giga", 9, 7),
        ("mega", 6, 8),
        ("kilo", 3, 9),
        ("hecto", 2, 10),
        ("deca", 1, 11),
        ("deci", -1, 12),
        ("centi", -2, 13),
        ("milli", -3, 14),
        ("micro", -6, 15),
        ("nano", -9, 16),
        ("pico", -12, 17),
        ("femto", -15, 18),
        ("atto", -18, 19),
        ("zepto", -21, 20),
        ("yocto", -24, 21),
        ("ronto", -27, 22),
        ("quecto", -30, 23),
    ];

    let prefixes_map = prefixes
        .iter()
        .map(|(prefix, _, index)| (prefix.as_bytes().to_vec(), *index))
        .collect::<BTreeMap<Vec<u8>, usize>>();

    let trie = ZeroTrieSimpleAscii::try_from(&prefixes_map).unwrap();
    let mut cursor = trie.cursor();

    let mut longest_match = (0, part);
    for (i, b) in part.bytes().enumerate() {
        cursor.step(b);
        if cursor.is_empty() {
            break;
        }
        if let Some(value) = cursor.take_value() {
            longest_match = (prefixes[value].1, &part[i + 1..]);
        }
    }
    longest_match
}

// TODO: consider returning Option<(i8, &str)> instead of (0, part) for the case when the prefix is not found.
// TODO: consider using a trie for the prefixes.
// TODO: complete all the cases for the prefixes.
/// Extracts the SI prefix of base 2.
/// NOTE:
///     if the prefix is found, the function will return (power, part without the prefix).
///     if the prefix is not found, the function will return (0, part).
fn get_si_prefix_base_two(part: &str) -> (i8, &str) {
    let prefixes = vec![
        ("yobi", 80),
        ("zebi", 70),
        ("exbi", 60),
        ("pebi", 50),
        ("tebi", 40),
        ("gibi", 30),
        ("mebi", 20),
        ("kibi", 10),
    ];
    let prefixes_map = prefixes
        .iter()
        .map(|(prefix, index)| (prefix.as_bytes().to_vec(), *index))
        .collect::<BTreeMap<Vec<u8>, usize>>();
    let trie = ZeroTrieSimpleAscii::try_from(&prefixes_map).unwrap();
    let mut cursor = trie.cursor();

    let mut longest_match = (0, part);
    for (i, b) in part.bytes().enumerate() {
        cursor.step(b);
        if cursor.is_empty() {
            break;
        }
        if let Some(value) = cursor.take_value() {
            longest_match = (value as i8, &part[i + 1..]);
        }
    }
    longest_match
}

// TODO: complete all the cases for the prefixes.
// TODO: consider using a trie for the prefixes.
/// Extracts the SI prefix.
/// NOTE:
///    if the prefix is found, the function will return (SiPrefix, part without the prefix string).
///    if the prefix is not found, the function will return (SiPrefix { power: 0, base: Base::Decimal }, part).
pub fn get_si_prefix(part: &str) -> (SiPrefix, &str) {
    let (si_prefix_base_10, part) = get_si_prefix_base_ten(part);
    if si_prefix_base_10 != 0 {
        return (
            SiPrefix {
                power: si_prefix_base_10,
                base: Base::Decimal,
            },
            part,
        );
    }

    let (si_prefix_base_2, part) = get_si_prefix_base_two(part);
    if si_prefix_base_2 != 0 {
        return (
            SiPrefix {
                power: si_prefix_base_2,
                base: Base::Binary,
            },
            part,
        );
    }

    (
        SiPrefix {
            power: 0,
            base: Base::Decimal,
        },
        part,
    )
}
