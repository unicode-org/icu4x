// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::PatternError,
    super::{GenericPatternItem, PatternItem},
    Parser, Pattern,
};
use alloc::vec::Vec;
use core::str::FromStr;

#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
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
                    #[allow(clippy::unwrap_used)] // idx is a valid base-10 digit
                    let replacement = replacements.get(idx as usize).ok_or_else(|| {
                        PatternError::UnknownSubstitution(char::from_digit(idx as u32, 10).unwrap())
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

impl FromStr for GenericPattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse_generic().map(Self::from)
    }
}

#[cfg(test)]
#[cfg(feature = "datagen")]
mod test {
    use super::*;

    #[test]
    fn test_reference_generic_pattern_combine() {
        let pattern: GenericPattern = "{0} 'at' {1}"
            .parse()
            .expect("Failed to parse a generic pattern.");

        let date = "Y/m/d".parse().expect("Failed to parse a date pattern.");
        let time = "HH:mm".parse().expect("Failed to parse a time pattern.");

        let pattern = pattern
            .combined(vec![date, time])
            .expect("Failed to combine date and time.");
        assert_eq!(pattern.to_string(), "Y/m/d 'at' HH:mm");
    }
}
