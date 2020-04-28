// Decimal types

use crate::Str;

#[derive(PartialEq, Copy, Clone)]
pub enum Key {
    SymbolsV1 = 1,
}

#[derive(PartialEq, Clone)]
pub enum Payload {
    // TODO: de-duplicate the name "SymbolsV1" between Key and Payload
    SymbolsV1 {
        zero_digit: char,
        decimal_separator: Str,
        grouping_separator: Str,
    }
}
