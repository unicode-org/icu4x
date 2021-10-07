// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::list_formatter::{Pattern, Type, Width};
use regex::Regex;

fn p(pattern: &'_ str) -> Pattern<'_> {
    let index_0 = pattern.find("{0}").expect("missing {0}");
    let index_1 = pattern.find("{1}").expect("missing {1}");
    assert!(
        index_0 + 3 <= index_1,
        "{} must appear before {}",
        "{0}",
        "{1}"
    );
    Pattern::Simple {
        parts: (
            &pattern[0..index_0],
            &pattern[index_0 + 3..index_1],
            &pattern[index_1 + 3..],
        ),
    }
}

fn c<'a>(cond: fn(&str) -> bool, then_pattern: &'a str, else_pattern: &'a str) -> Pattern<'a> {
    match (p(then_pattern), p(else_pattern)) {
        (Pattern::Simple { parts: then }, Pattern::Simple { parts: else_ }) => {
            Pattern::Conditional { cond, then, else_ }
        }
        _ => panic!("Nested conditional"),
    }
}

// Of course this is very inefficient as it pulls in the regex crate and repeatedly constructs
// regexes at runtime.
// TODO: Find a way to efficiently encode a predicate that implements a regex.
fn es_starts_with_i_sound(str: &str) -> bool {
    Regex::new("^(?i)i.*|hi|hi[^ae].*$").unwrap().is_match(str)
}

fn es_starts_with_o_sound(str: &str) -> bool {
    Regex::new("^(?i)(o|ho|8).*|11$").unwrap().is_match(str)
}

// This should be a compact representation of the CLDR data. Each locale entry is a 3 x 3 x 4 array
// (type x width x 4 patterns) of &str. These refs use 592 bytes of memory on a 64-bit platform,
// plus the memory actually required by the strings. As there aren't many unique patterns, this is
// probably negligible (see the test below).
type LocalePatterns<'a> = [[[Pattern<'a>; 4]; 3]; 3];
lazy_static! {
    static ref RAW_PATTERNS: Box<[(&'static str, LocalePatterns<'static>)]> = { let r = [
        ("en", [
            [
                [p("{0}, {1}"), p("{0} and {1}"), p("{0}, {1}"), p("{0}, and {1}")],
                [p("{0}, {1}"), p("{0} and {1}"), p("{0}, {1}"), p("{0}, {1}")],
                [p("{0}, {1}"), p("{0} & {1}"), p("{0}, {1}"), p("{0}, & {1}")]
            ],
            [
                [p("{0}, {1}"), p("{0} or {1}"), p("{0}, {1}"), p("{0}, or {1}")],
                [p("{0}, {1}"), p("{0} or {1}"), p("{0}, {1}"), p("{0}, or {1}")],
                [p("{0}, {1}"), p("{0} or {1}"), p("{0}, {1}"), p("{0}, or {1}")]
            ],
            [
                [p("{0}, {1}"), p("{0}, {1}"), p("{0}, {1}"), p("{0}, {1}")],
                [p("{0}, {1}"), p("{0}, {1}"), p("{0}, {1}"), p("{0}, {1}")],
                [p("{0}, {1}"), p("{0}, {1}"), p("{0}, {1}"), p("{0}, {1}")],
            ]
        ]),
        ("es", [
            [
                [p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}"), p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}")],
                [p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}"), p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}")],
                [p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}"), p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}")],
            ],
            [
                [p("{0}, {1}"), c(es_starts_with_o_sound, "{0} u {1}", "{0} o {1}"), p("{0}, {1}"), c(es_starts_with_o_sound, "{0} u {1}", "{0} o {1}")],
                [p("{0}, {1}"), c(es_starts_with_o_sound, "{0} u {1}", "{0} o {1}"), p("{0}, {1}"), c(es_starts_with_o_sound, "{0} u {1}", "{0} o {1}")],
                [p("{0}, {1}"), c(es_starts_with_o_sound, "{0} u {1}", "{0} o {1}"), p("{0}, {1}"), c(es_starts_with_o_sound, "{0} u {1}", "{0} o {1}")],
            ],
            [
                [p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}"), p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}")],
                [p("{0} {1}"), p("{0} {1}"), p("{0} {1}"), p("{0} {1}")],
                [p("{0}, {1}"), c(es_starts_with_i_sound, "{0} e {1}", "{0} y {1}"), p("{0}, {1}"), p("{0}, {1}")]
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
    Box::new(r)
        };
}

pub(crate) fn get_patterns(
    locale: &str,
    type_: Type,
    width: Width,
) -> Option<&'static [Pattern<'static>; 4]> {
    match (*RAW_PATTERNS).binary_search_by_key(&locale, |(l, _)| l) {
        Ok(index) => Some(
            &(*RAW_PATTERNS)[index].1[match type_ {
                Type::And => 0,
                Type::Or => 1,
                Type::Unit => 2,
            }][match width {
                Width::Wide => 0,
                Width::Narrow => 1,
                Width::Short => 2,
            }],
        ),
        Err(_) => None,
    }
}
