// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::Pattern;
use std::{borrow::Cow, convert::TryInto, fmt::Display};

#[derive(Debug)]
enum Element<'s> {
    Token(usize),
    Literal(Cow<'s, str>),
}

impl Display for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(n) => write!(f, "{}", n),
            Self::Literal(s) => f.write_str(s),
        }
    }
}

impl<'s> From<Cow<'s, str>> for Element<'s> {
    fn from(input: Cow<'s, str>) -> Self {
        Self::Literal(input)
    }
}

fn main() {
    let replacements = vec![Element::Token(5)];

    let pattern: Pattern<usize> = "{0} 'days'".try_into().expect("Failed to parse a pattern");

    let interpolated_pattern = pattern
        .interpolate(&replacements)
        .expect("Failed to interpolate a pattern");

    let result = interpolated_pattern.to_string();

    assert_eq!(result, "5 days");
}
