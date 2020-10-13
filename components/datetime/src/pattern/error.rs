// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::fields;

#[derive(Debug)]
pub enum Error {
    Unknown,
    FieldTooLong(fields::FieldSymbol),
    UnknownSubstitution(u8),
}

impl From<fields::Error> for Error {
    fn from(input: fields::Error) -> Self {
        match input {
            fields::Error::TooLong(symbol) => Self::FieldTooLong(symbol),
        }
    }
}
