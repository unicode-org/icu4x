// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::{PatternError, PatternItem, TimeGranularity},
    Parser,
};
use alloc::vec::Vec;
use core::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pattern {
    pub items: Vec<PatternItem>,
    pub(crate) time_granularity: TimeGranularity,
}

impl Pattern {
    pub fn items(&self) -> &[PatternItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut [PatternItem] {
        &mut self.items
    }
}

impl From<Vec<PatternItem>> for Pattern {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            time_granularity: items.iter().map(Into::into).max().unwrap_or_default(),
            items,
        }
    }
}

impl From<&str> for Pattern {
    fn from(items: &str) -> Self {
        Self {
            time_granularity: TimeGranularity::default(),
            items: items.chars().map(|ch| ch.into()).collect(),
        }
    }
}

impl FromStr for Pattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse().map(Self::from)
    }
}
