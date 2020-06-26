// Decimal types

use std::prelude::v1::*;

use smallstr::SmallString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Key {
    SymbolsV1 = 1,
}

impl From<Key> for crate::Key {
    fn from(value: Key) -> Self {
        crate::Key::Decimal(value)
    }
}

// TODO: de-duplicate the name "SymbolsV1" between Key and the struct
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SymbolsV1 {
    pub zero_digit: char,
    pub decimal_separator: SmallString<[u8; 4]>,
    pub grouping_separator: SmallString<[u8; 4]>,
}
