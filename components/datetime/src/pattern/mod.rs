// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod error;
mod parser;

use crate::fields::{self, Field, FieldLength, FieldSymbol};
pub use error::Error;
use parser::Parser;
use std::convert::TryFrom;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub enum PatternItem {
    Field(fields::Field),
    Literal(String),
}

impl From<(FieldSymbol, FieldLength)> for PatternItem {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self::Field(Field {
            symbol: input.0,
            length: input.1,
        })
    }
}

impl TryFrom<(FieldSymbol, u8)> for PatternItem {
    type Error = Error;
    fn try_from(input: (FieldSymbol, u8)) -> Result<Self, Self::Error> {
        let length = FieldLength::try_from(input.1).map_err(|_| Error::FieldTooLong(input.0))?;
        Ok(Self::Field(Field {
            symbol: input.0,
            length,
        }))
    }
}

impl<'p> From<&str> for PatternItem {
    fn from(input: &str) -> Self {
        Self::Literal(input.into())
    }
}

impl<'p> From<String> for PatternItem {
    fn from(input: String) -> Self {
        Self::Literal(input)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pattern(pub Vec<PatternItem>);

impl Pattern {
    pub fn from_bytes(input: &str) -> Result<Self, Error> {
        Parser::new(input).parse().map(Self)
    }

    // TODO(#277): This should be turned into a utility for all ICU4X.
    pub fn from_bytes_combination(input: &str, date: Self, time: Self) -> Result<Self, Error> {
        Parser::new(input)
            .parse_placeholders(vec![time, date])
            .map(Self)
    }
}

impl FromIterator<PatternItem> for Pattern {
    fn from_iter<I: IntoIterator<Item = PatternItem>>(iter: I) -> Self {
        let items: Vec<PatternItem> = iter.into_iter().collect();
        Self(items)
    }
}
