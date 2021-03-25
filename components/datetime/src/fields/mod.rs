// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
mod length;
pub(crate) mod symbols;

pub use length::{FieldLength, LengthError};
pub use symbols::*;

use std::{
    cmp::{Ord, PartialOrd},
    convert::{TryFrom, TryInto},
    fmt,
};

#[derive(Debug)]
pub enum Error {
    TooLong(FieldSymbol),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TooLong(symbol) => write!(f, "field {:?} is too long", symbol),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Field {
    pub symbol: FieldSymbol,
    pub length: FieldLength,
}

impl From<(FieldSymbol, FieldLength)> for Field {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Self {
            symbol: input.0,
            length: input.1,
        }
    }
}

impl TryFrom<(FieldSymbol, usize)> for Field {
    type Error = Error;
    fn try_from(input: (FieldSymbol, usize)) -> Result<Self, Self::Error> {
        let (symbol, length) = (
            input.0,
            input
                .1
                .try_into()
                .map_err(|_| Self::Error::TooLong(input.0))?,
        );
        Ok(Self { symbol, length })
    }
}
