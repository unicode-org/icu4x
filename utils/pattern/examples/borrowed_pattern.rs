// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::{Pattern, PatternToken};
use std::fmt::Display;

#[derive(Debug)]
enum Element {
    Token(usize),
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(n) => write!(f, "{n}"),
        }
    }
}

struct Data<'s> {
    placeholder_pattern: Pattern<'s, usize>,
    replacement_patterns: Vec<Element>,
}

fn main() {
    let data = Data {
        placeholder_pattern: vec![
            PatternToken::Placeholder(0),
            PatternToken::Literal {
                content: " days".into(),
                quoted: false,
            },
        ]
        .into(),
        replacement_patterns: vec![Element::Token(5)],
    };

    let interpolated_pattern = data
        .placeholder_pattern
        .interpolate(&data.replacement_patterns)
        .expect("Failed to interpolate.");
    let result = interpolated_pattern.to_string();

    assert_eq!(result, "5 days");
}
