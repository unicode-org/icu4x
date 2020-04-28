// Decimal types

use std::prelude::v1::*;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone)]
pub enum Key {
    SymbolsV1 = 1,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Payload {
    // TODO: de-duplicate the name "SymbolsV1" between Key and Payload
    SymbolsV1 {
        zero_digit: char,
        decimal_separator: String,
        grouping_separator: String,
    }
}
