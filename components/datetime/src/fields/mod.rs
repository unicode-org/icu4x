mod length;
mod symbols;

pub use length::FieldLength;
pub use symbols::*;

use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub enum Error {
    TooLong(FieldSymbol),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Field {
    pub symbol: FieldSymbol,
    pub length: FieldLength,
}

impl Field {}

impl From<(FieldSymbol, FieldLength)> for Field {
    fn from(input: (FieldSymbol, FieldLength)) -> Self {
        Field {
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
        Ok(Field { symbol, length })
    }
}
