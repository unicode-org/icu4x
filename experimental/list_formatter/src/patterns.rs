// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::list_formatter::{Type, Width};

// This should be a compact representation of the CLDR data. Each locale entry is a 3 x 3 x 4 array 
// (type x width x 4 patterns) of &str. These refs use 592 bytes of memory on a 64-bit platform,
// plus the memory actually required by the strings. As there aren't many unique patterns, this is
// probably negligible (see the test below).
type RawPatterns<'a> = [[[&'a str;4];3];3];
const RAW_PATTERNS: &[(&'static str, RawPatterns<'static>)] = &[
    ("en", [
        [
            ["{0}, {1}", "{0} and {1}", "{0}, {1}", "{0}, and {1}"], 
            ["{0}, {1}", "{0} and {1}", "{0}, {1}", "{0}, {1}"], 
            ["{0}, {1}", "{0} & {1}", "{0}, {1}", "{0}, & {1}"]
        ],
        [
            ["{0}, {1}", "{0} or {1}", "{0}, {1}", "{0}, or {1}"], 
            ["{0}, {1}", "{0} or {1}", "{0}, {1}", ", or "], 
            ["{0}, {1}", "{0} or {1}", "{0}, {1}", "{1}, or {1}"]
        ],
        [
            ["{0}, {1}", "{0}, {1}", "{0}, {1}", "{0}, {1}"], 
            ["{0}, {1}", "{0}, {1}", "{0}, {1}", "{0}, {1}"], 
            ["{0}, {1}", "{0}, {1}", "{0}, {1}", "{0}, {1}"]
        ]
    ]),
    ("es", [
        [
            ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"], 
            ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"], 
            ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"]
        ],
        [
            ["{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*([^\\d]|$){then}{0} u {1}{else}{0} o {1}", "{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*([^\\d]|$){then}{0} u {1}{else}{0} o {1}"], 
            ["{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*([^\\d]|$){then}{0} u {1}{else}{0} o {1}", "{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*([^\\d]|$){then}{0} u {1}{else}{0} o {1}"], 
            ["{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*([^\\d]|$){then}{0} u {1}{else}{0} o {1}", "{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*([^\\d]|$){then}{0} u {1}{else}{0} o {1}"]
        ],
        [
            ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"], 
            ["{0} {1}", "{0} {1}", "{0} {1}", "{0} {1}"], 
            ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{0}, {1}"]
        ],
    ]),
    // (locale, [
    //     [
    //         standard, 
    //         standard-narrow,
    //         standard-short
    //     ],
    //     [
    //         or,
    //         or-narrow,
    //         or-short
    //     ],
    //     [
    //         unit, 
    //         unit-narrow, 
    //         unit-short
    //     ]
    // ]),
];

pub(crate) fn get_patterns(locale: &str, type_: Type, width: Width) -> Option<[&'static str; 4]> {
    match RAW_PATTERNS.binary_search_by_key(&locale, |(l, _)| l) {
        Ok(index) => Some(RAW_PATTERNS[index].1[match type_ {
            Type::And => 0,
            Type::Or => 1,
            Type::Unit => 2,
        }][match width {
            Width::Wide => 0,
            Width::Narrow => 1,
            Width::Short => 2,
        }]),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn duplicate_strings_only_exist_once() {
        assert_eq!(
            get_patterns("en", Type::Unit, Width::Narrow).expect("exists")[2].as_ptr(),
            get_patterns("en", Type::Unit, Width::Wide).expect("exists")[3].as_ptr()
        );
    }

    #[test]
    fn substrings_are_optimized() {
        // they're not
        // assert_eq!("en-UK".as_ptr(), "en".as_ptr());
    }

    #[test]
    fn count_bytes() {
        let mut string_memory: HashSet<*const u8> = HashSet::new();
        let mut saved_string_memory = 0;
        let mut record_string_ref = |string: &str| unsafe {
            let init = string.as_ptr();
            for i in 0..string.len() {
                if !string_memory.insert(init.offset(i as isize)) {
                    saved_string_memory += 1;
                }
            }
        };

        for (locale, patterns) in RAW_PATTERNS {
            // Key
            record_string_ref(locale);
            // Patterns
            for i in 0..patterns.len() {
                for j in 0..patterns[i].len() {
                    for k in 0..patterns[j].len() {
                        record_string_ref(patterns[i][j][k]);
                    }
                }
            }
        }

        println!("Using {} bytes for strings, {}% of total string length", string_memory.len(), string_memory.len() * 100 / (saved_string_memory + string_memory.len()));
        println!("Using {} bytes for references", core::mem::size_of_val(RAW_PATTERNS));
    }
}