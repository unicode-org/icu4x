// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: consider returning Option<(i8, &str)> instead of (0, part) for the case when the prefix is not found.
// TODO: consider using a trie for the prefixes.
// TODO: complete all the cases for the prefixes.
/// Extracts the SI prefix of base 10.
/// NOTE:
///    if the prefix is found, the function will return (power, part without the prefix).
///    if the prefix is not found, the function will return (0, part).
pub fn get_si_prefix_base_ten(part: &str) -> (i8, &str) {
    if let Some(part) = part.strip_prefix("quetta") {
        (30, part)
    } else if let Some(part) = part.strip_prefix("ronna") {
        (27, part)
    } else if let Some(part) = part.strip_prefix("yotta") {
        (24, part)
    } else if let Some(part) = part.strip_prefix("zetta") {
        (21, part)
    } else if let Some(part) = part.strip_prefix("exa") {
        (18, part)
    } else if let Some(part) = part.strip_prefix("peta") {
        (15, part)
    } else if let Some(part) = part.strip_prefix("tera") {
        (12, part)
    } else if let Some(part) = part.strip_prefix("giga") {
        (9, part)
    } else if let Some(part) = part.strip_prefix("mega") {
        (6, part)
    } else if let Some(part) = part.strip_prefix("kilo") {
        (3, part)
    } else if let Some(part) = part.strip_prefix("hecto") {
        (2, part)
    } else if let Some(part) = part.strip_prefix("deca") {
        (1, part)
    } else if let Some(part) = part.strip_prefix("deci") {
        (-1, part)
    } else if let Some(part) = part.strip_prefix("centi") {
        (-2, part)
    } else if let Some(part) = part.strip_prefix("milli") {
        (-3, part)
    } else if let Some(part) = part.strip_prefix("micro") {
        (-6, part)
    } else if let Some(part) = part.strip_prefix("nano") {
        (-9, part)
    } else if let Some(part) = part.strip_prefix("pico") {
        (-12, part)
    } else if let Some(part) = part.strip_prefix("femto") {
        (-15, part)
    } else if let Some(part) = part.strip_prefix("atto") {
        (-18, part)
    } else if let Some(part) = part.strip_prefix("zepto") {
        (-21, part)
    } else if let Some(part) = part.strip_prefix("yocto") {
        (-24, part)
    } else if let Some(part) = part.strip_prefix("ronto") {
        (-27, part)
    } else if let Some(part) = part.strip_prefix("quecto") {
        (-30, part)
    } else {
        (0, part)
    }
}

// TODO: consider returning Option<(i8, &str)> instead of (0, part) for the case when the prefix is not found.
// TODO: consider using a trie for the prefixes.
// TODO: complete all the cases for the prefixes.
/// Extracts the SI prefix of base 2.
/// NOTE:
///     if the prefix is found, the function will return (power, part without the prefix).
///     if the prefix is not found, the function will return (0, part).
pub fn get_si_prefix_base_two(part: &str) -> (i8, &str) {
    if let Some(part) = part.strip_prefix("kibi") {
        (10, part)
    } else if let Some(part) = part.strip_prefix("mebi") {
        (20, part)
    } else if let Some(part) = part.strip_prefix("gibi") {
        (30, part)
    } else if let Some(part) = part.strip_prefix("tebi") {
        (40, part)
    } else if let Some(part) = part.strip_prefix("pebi") {
        (50, part)
    } else if let Some(part) = part.strip_prefix("exbi") {
        (60, part)
    } else if let Some(part) = part.strip_prefix("zebi") {
        (70, part)
    } else if let Some(part) = part.strip_prefix("yobi") {
        (80, part)
    } else {
        (0, part)
    }
}
