// Decimal types

use std::prelude::v1::*;
use std::any::Any;

use serde::{Deserialize, Serialize};

use super::Bovine;

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

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SymbolsV1 {
    pub zero_digit: char,
    pub decimal_separator: String,
    pub grouping_separator: String,
}

impl Bovine for SymbolsV1 {
    // TODO: How to make this line return Box<SymbolsV1>? Is that necessary?
    fn clone_into_box(&self) -> Box<dyn Bovine> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SymbolsV1 {
    pub fn as_bovine(&self) -> &dyn Bovine {
        self
    }
}
