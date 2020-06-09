// Decimal types

use std::prelude::v1::*;

use std::borrow::Cow;

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
    // String ownership discussion:
    // https://github.com/unicode-org/icu4x/pull/61#discussion_r429051895
    pub decimal_separator: Cow<'static, str>,
    pub grouping_separator: Cow<'static, str>,
}
