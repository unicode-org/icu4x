// Decimal types

use std::prelude::v1::*;
use std::any::Any;

use serde::{Deserialize, Serialize};

use super::Bovine;

#[derive(PartialEq, Copy, Clone)]
pub enum Key {
    SymbolsV1 = 1,
}

// TODO: de-duplicate the name "SymbolsV1" between Key and the struct
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SymbolsV1 {
    pub zero_digit: char,
    pub decimal_separator: String,
    pub grouping_separator: String,
}

impl Bovine for SymbolsV1 {
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
