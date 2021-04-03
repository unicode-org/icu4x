// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::*;
use std::{
    borrow::Cow,
    convert::TryInto,
    fmt::{Display, Write},
};

#[derive(Debug)]
struct Token;

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn iai_parse() {
    let samples = vec![
        ("{0} - {1}", vec![vec!["Hello"], vec!["World"]]),
        ("{1} - {0}", vec![vec!["Hello"], vec!["World"]]),
        (
            "{0}, {1} 'and' {2}",
            vec![vec!["Start"], vec!["Middle"], vec!["End"]],
        ),
        ("{0} 'at' {1}", vec![vec!["Hello"], vec!["World"]]),
    ];

    for sample in &samples {
        let mut p = Parser::new(
            &sample.0,
            ParserOptions {
                allow_raw_letters: false,
            },
        );
        while let Some(_) = p.try_next::<usize>().unwrap() {}
    }
}

#[derive(Debug)]
pub enum Element<'s> {
    Literal(Cow<'s, str>),
}

impl Display for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(s) => f.write_str(s),
        }
    }
}

impl<'s> From<Cow<'s, str>> for Element<'s> {
    fn from(input: Cow<'s, str>) -> Self {
        Self::Literal(input)
    }
}

impl<'s> From<&'s str> for Element<'s> {
    fn from(input: &'s str) -> Self {
        Self::Literal(input.into())
    }
}

fn iai_interpolate() {
    let samples = vec![
        ("{0} - {1}", vec!["Hello", "World"]),
        ("{1} - {0}", vec!["Hello", "World"]),
        ("{0}, {1} 'and' {2}", vec!["Start", "Middle", "End"]),
        ("{0} 'at' {1}", vec!["Hello", "World"]),
    ];

    for sample in &samples {
        let pattern: Vec<_> = Parser::new(
            &sample.0,
            ParserOptions {
                allow_raw_letters: false,
            },
        )
        .try_into()
        .unwrap();

        let replacements: Vec<Element> = sample.1.iter().map(|r| Element::from(*r)).collect();

        let mut i = Interpolator::new(&pattern, &replacements);
        let mut result = String::new();
        while let Some(ik) = i.try_next().unwrap() {
            match ik {
                InterpolatedKind::Element(element) => {
                    write!(result, "{}", element).expect("Failed to write to a string");
                }
                InterpolatedKind::Literal(token) => {
                    write!(result, "{}", token).expect("Failed to write to a string");
                }
            }
        }
    }
}

fn iai_parsed_interpolate() {
    let samples = &[
        (
            vec![
                PatternToken::Placeholder(0),
                PatternToken::Literal {
                    content: " - ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(1),
            ],
            vec!["Hello", "World"],
        ),
        (
            vec![
                PatternToken::Placeholder(1),
                PatternToken::Literal {
                    content: " - ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(0),
            ],
            vec!["Hello", "World"],
        ),
        (
            vec![
                PatternToken::Placeholder(0),
                PatternToken::Literal {
                    content: ", ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(1),
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Literal {
                    content: "and".into(),
                    quoted: true,
                },
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(2),
            ],
            vec!["Start", "Middle", "End"],
        ),
        (
            vec![
                PatternToken::Placeholder(0),
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Literal {
                    content: "at".into(),
                    quoted: true,
                },
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(1),
            ],
            vec!["Hello", "World"],
        ),
    ];

    for sample in samples {
        let pattern = &sample.0;

        let replacements: Vec<Element> = sample.1.iter().map(|r| Element::from(*r)).collect();

        let mut i = Interpolator::new(pattern, &replacements);
        let mut result = String::new();
        while let Some(ik) = i.try_next().unwrap() {
            match ik {
                InterpolatedKind::Element(element) => {
                    write!(result, "{}", element).expect("Failed to write to a string");
                }
                InterpolatedKind::Literal(token) => {
                    write!(result, "{}", token).expect("Failed to write to a string");
                }
            }
        }
    }
}

fn iai_parsed_interpolate_composed() {
    let samples = &[
        (
            vec![
                PatternToken::Placeholder(0),
                PatternToken::Literal {
                    content: " - ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(1),
            ],
            vec!["Hello", "World"],
        ),
        (
            vec![
                PatternToken::Placeholder(1),
                PatternToken::Literal {
                    content: " - ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(0),
            ],
            vec!["Hello", "World"],
        ),
        (
            vec![
                PatternToken::Placeholder(0),
                PatternToken::Literal {
                    content: ", ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(1),
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Literal {
                    content: "and".into(),
                    quoted: true,
                },
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(2),
            ],
            vec!["Start", "Middle", "End"],
        ),
        (
            vec![
                PatternToken::Placeholder(0),
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Literal {
                    content: "at".into(),
                    quoted: true,
                },
                PatternToken::Literal {
                    content: " ".into(),
                    quoted: false,
                },
                PatternToken::Placeholder(1),
            ],
            vec!["Hello", "World"],
        ),
    ];

    for sample in samples {
        let pattern = &sample.0;

        let replacements: Vec<Vec<Element>> =
            sample.1.iter().map(|r| vec![Element::from(*r)]).collect();

        let mut i = Interpolator::<Vec<Vec<_>>, Element>::new(pattern, &replacements);
        let mut result = String::new();
        while let Some(ik) = i.try_next().unwrap() {
            write!(result, "{}", ik).expect("Failed to write to a string");
        }
    }
}

fn iai_named_interpolate() {
    let named_samples = vec![(
        "{start}, {middle} 'and' {end}",
        vec![("start", "Start"), ("middle", "Middle"), ("end", "End")],
    )];

    for sample in &named_samples {
        let pattern: Vec<_> = Parser::new(
            &sample.0,
            ParserOptions {
                allow_raw_letters: false,
            },
        )
        .try_into()
        .unwrap();

        let replacements: std::collections::HashMap<String, Element> = sample
            .1
            .iter()
            .map(|(k, v)| (k.to_string(), Element::from(*v)))
            .collect();

        let mut i = Interpolator::new(&pattern, &replacements);
        while let Some(_) = i.try_next().unwrap() {}
    }
}

iai::main!(
    iai_parse,
    iai_interpolate,
    iai_parsed_interpolate,
    iai_parsed_interpolate_composed,
    iai_named_interpolate
);
