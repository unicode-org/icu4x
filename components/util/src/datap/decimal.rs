// Decimal types

use std::prelude::v1::*;

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Key {
    SymbolsV1 = 1,
}

// TODO: de-duplicate the name "SymbolsV1" between Key and the struct
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SymbolsV1 {
    pub zero_digit: char,
    pub decimal_separator: Cow<'static, str>,
    pub grouping_separator: Cow<'static, str>,
}
