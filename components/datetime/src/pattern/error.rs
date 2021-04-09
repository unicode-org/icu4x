// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldLengthInvalid(fields::FieldSymbol),
    UnknownSubstitution(char),
    UnclosedLiteral,
    UnclosedPlaceholder,
}

/// These strings follow the recommendations for the serde::de::Unexpected::Other type.
/// https://docs.serde.rs/serde/de/enum.Unexpected.html#variant.Other
///
/// Serde will generate an error such as:
/// "invalid value: unclosed literal in pattern, expected a valid UTS 35 pattern string at line 1 column 12"
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FieldLengthInvalid(symbol) => {
                write!(f, "{:?} invalid field length in pattern", symbol)
            }
            Error::UnknownSubstitution(ch) => write!(f, "unknown substitution {} in pattern", ch),
            Error::UnclosedLiteral => write!(f, "unclosed literal in pattern"),
            Error::UnclosedPlaceholder => write!(f, "unclosed placeholder in pattern"),
        }
    }
}

impl From<fields::Error> for Error {
    fn from(input: fields::Error) -> Self {
        match input {
            fields::Error::TooLong(symbol) => Self::FieldLengthInvalid(symbol),
        }
    }
}
