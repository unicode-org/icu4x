// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: consider returning Option<(u8, &str)> instead of (1, part) for the case when the power is not found.
// TODO: complete all the cases for the powers.
// TODO: consider using a trie for the powers.
/// Converts a power string to a power.
pub fn get_power(part: &str) -> Option<i8> {
    match part {
        "pow1" => Some(1),
        "square" | "pow2" => Some(2),
        "cubic" | "pow3" => Some(3),
        "pow4" => Some(4),
        "pow5" => Some(5),
        "pow6" => Some(6),
        "pow7" => Some(7),
        "pow8" => Some(8),
        "pow9" => Some(9),
        "pow10" => Some(10),
        "pow11" => Some(11),
        "pow12" => Some(12),
        "pow13" => Some(13),
        "pow14" => Some(14),
        "pow15" => Some(15),
        _ => None,
    }
}
