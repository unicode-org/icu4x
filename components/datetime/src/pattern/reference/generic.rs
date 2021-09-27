// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::PatternError,
    super::{GenericPatternItem, PatternItem},
    Pattern,
};
use alloc::vec::Vec;

pub struct GenericPattern {
    pub items: Vec<GenericPatternItem>,
}

impl GenericPattern {
    pub fn combined(self, replacements: Vec<Pattern>) -> Result<Pattern, PatternError> {
        let size = replacements.iter().fold(0, |acc, r| acc + r.items.len());
        let mut result = Vec::with_capacity(self.items.len() + size);

        for item in self.items.into_iter() {
            match item {
                GenericPatternItem::Placeholder(idx) => {
                    let replacement = replacements.get(idx as usize).ok_or_else(|| {
                        let idx = char::from_digit(idx as u32, 10)
                            .expect("Failed to convert placeholder idx to char");
                        PatternError::UnknownSubstitution(idx)
                    })?;
                    result.extend(replacement.items.iter());
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
