// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_pattern::*;
use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Write;

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
        let mut p = Parser::new(&sample.0, false);
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
        ("{0} - {1}", vec![vec!["Hello"], vec!["World"]]),
        ("{1} - {0}", vec![vec!["Hello"], vec!["World"]]),
        (
            "{0}, {1} 'and' {2}",
            vec![vec!["Start"], vec!["Middle"], vec!["End"]],
        ),
        ("{0} 'at' {1}", vec![vec!["Hello"], vec!["World"]]),
    ];

    for sample in &samples {
        let iter = Parser::new(&sample.0, false);

        let replacements: Vec<Vec<Element>> = sample
            .1
            .iter()
            .map(|r| r.iter().map(|&t| t.into()).collect())
            .collect();

        let mut i = Interpolator::<_, _, Element>::new(iter, replacements);
        let mut result = String::new();
        while let Some(elem) = i.try_next().unwrap() {
            write!(result, "{}", elem).unwrap();
        }
    }
}

fn iai_named_interpolate() {
    let named_samples = vec![(
        "{start}, {middle} 'and' {end}",
        vec![
            ("start", vec!["Start"]),
            ("middle", vec!["Middle"]),
            ("end", vec!["End"]),
        ],
    )];

    for sample in &named_samples {
        let iter = Parser::new(&sample.0, false);

        let replacements: std::collections::HashMap<String, Vec<Element>> = sample
            .1
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|&t| t.into()).collect()))
            .collect();

        let mut i = Interpolator::new(iter, replacements);
        while let Some(_) = i.try_next().unwrap() {}
    }
}

iai::main!(iai_parse, iai_interpolate, iai_named_interpolate);
