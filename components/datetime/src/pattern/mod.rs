// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod error;
mod parser;

use crate::fields::{self, Field, FieldLength, FieldSymbol};
pub use error::Error;
use parser::Parser;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub enum PatternItem {
    Field(fields::Field),
    Literal(String),
}

impl From<Field> for PatternItem {
    fn from(input: Field) -> Self {
        Self::Field(input)
    }
}

impl From<(FieldSymbol, FieldLength)> for PatternItem {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Field {
            symbol: input.0,
            length: input.1,
        }
        .into()
    }
}

impl<'p> From<&str> for PatternItem {
    fn from(input: &str) -> Self {
        Self::Literal(input.into())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pattern(pub Vec<PatternItem>);

impl Pattern {
    pub fn from_bytes(input: &str) -> Result<Self, Error> {
        Parser::new(input).parse().map(Pattern)
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(
        input: &str,
        date: Pattern,
        time: Pattern,
    ) -> Result<Self, Error> {
        Parser::new(input)
            .parse_placeholders(vec![time, date])
            .map(Pattern)
    }
}

impl FromIterator<PatternItem> for Pattern {
    fn from_iter<I: IntoIterator<Item = PatternItem>>(iter: I) -> Self {
        let items: Vec<PatternItem> = iter.into_iter().collect();
        Self(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_parse() {
        assert_eq!(
            Pattern::from_bytes("dd/MM/y").expect("Parsing pattern failed."),
            vec![
                (fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into(),
                "/".into(),
                (fields::Month::Format.into(), FieldLength::TwoDigit).into(),
                "/".into(),
                (fields::Year::Calendar.into(), FieldLength::One).into(),
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            Pattern::from_bytes("HH:mm:ss").expect("Parsing pattern failed."),
            vec![
                (fields::Hour::H23.into(), FieldLength::TwoDigit).into(),
                ":".into(),
                (FieldSymbol::Minute, FieldLength::TwoDigit).into(),
                ":".into(),
                (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            Pattern::from_bytes("y年M月d日").expect("Parsing pattern failed."),
            vec![
                (fields::Year::Calendar.into(), FieldLength::One).into(),
                "年".into(),
                (fields::Month::Format.into(), FieldLength::One).into(),
                "月".into(),
                (fields::Day::DayOfMonth.into(), FieldLength::One).into(),
                "日".into(),
            ]
            .into_iter()
            .collect()
        );
    }
}
