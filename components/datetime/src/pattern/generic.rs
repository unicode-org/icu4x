// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{Error, Pattern, PatternItem};
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GenericPatternItem {
    Placeholder(u8),
    Literal(char),
}

impl From<u8> for GenericPatternItem {
    fn from(input: u8) -> Self {
        Self::Placeholder(input)
    }
}

impl<'p> From<char> for GenericPatternItem {
    fn from(input: char) -> Self {
        Self::Literal(input)
    }
}

pub struct GenericPattern {
    pub items: Vec<GenericPatternItem>,
}

impl GenericPattern {
    pub fn combined(self, replacements: Vec<Pattern>) -> Result<Pattern, Error> {
        let size = replacements.iter().fold(0, |acc, r| acc + r.items.len());
        let mut result = Vec::with_capacity(self.items.len() + size);

        for item in self.items.into_iter() {
            match item {
                GenericPatternItem::Placeholder(idx) => {
                    let replacement = replacements.get(idx as usize).ok_or_else(|| {
                        let idx = char::from_digit(idx as u32, 10)
                            .expect("Failed to convert placeholder idx to char");
                        Error::UnknownSubstitution(idx)
                    })?;
                    result.extend_from_slice(replacement.items());
                }
                GenericPatternItem::Literal(ch) => result.push(PatternItem::Literal(ch)),
            }
        }

        Ok(result.into())
    }
}

impl From<Vec<GenericPatternItem>> for GenericPattern {
    fn from(items: Vec<GenericPatternItem>) -> Self {
        Self { items }
    }
}
