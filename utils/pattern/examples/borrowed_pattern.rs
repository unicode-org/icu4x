// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::{Interpolator, PatternToken};
use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

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

struct Data<'s> {
    pattern: Vec<PatternToken<'s, usize>>,
}

fn main() {
    let data = Data {
        pattern: vec![
            PatternToken::Placeholder(0),
            PatternToken::Literal {
                content: " days".into(),
                quoted: false,
            },
        ],
    };

    let replacements = vec![Some(Element::Token(5))];

    let mut interpolator = Interpolator::new(&data.pattern, replacements);

    let mut result = String::new();

    while let Some(element) = interpolator.try_next().expect("Failed to interpolate") {
        write!(result, "{}", element).expect("Failed to write to a string");
    }
    assert_eq!(result, "5 days");
}
