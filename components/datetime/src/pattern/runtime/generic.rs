// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::{reference, PatternError},
    super::{GenericPatternItem, PatternItem},
    Pattern,
};
use alloc::vec::Vec;
use zerovec::{ule::AsULE, ZeroVec};

pub struct GenericPattern<'data> {
    pub items: ZeroVec<'data, GenericPatternItem>,
}

impl<'data> GenericPattern<'data> {
    /// The function allows for creation of new DTF pattern from a generic pattern
    /// and replacement patterns.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::pattern::{runtime, reference};
    ///
    /// let date: runtime::Pattern =
    ///     reference::Pattern::from_bytes("Y-m-d")
    ///         .expect("Failed to parse pattern")
    ///         .into();
    /// let time: runtime::Pattern =
    ///     reference::Pattern::from_bytes("HH:mm")
    ///         .expect("Failed to parse pattern")
    ///         .into();
    ///
    /// let glue: runtime::GenericPattern =
    ///     reference::GenericPattern::from_bytes("{0} 'at' {1}")
    ///         .expect("Failed to parse generic pattern")
    ///         .into();
    /// assert_eq!(
    ///     glue.combined(vec![date, time])
    ///         .expect("Failed to combine patterns")
    ///         .to_string(),
    ///     "Y-m-d 'at' HH:mm"
    /// );
    /// ```
    pub fn combined(self, replacements: Vec<Pattern>) -> Result<Pattern, PatternError> {
        let size = replacements.iter().fold(0, |acc, r| acc + r.items.len());
        let mut result = Vec::with_capacity(self.items.len() + size);

        for item in self.items.iter() {
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

        Ok(Pattern::from(result))
    }
}

impl From<reference::GenericPattern> for GenericPattern<'_> {
    fn from(input: reference::GenericPattern) -> Self {
        Self {
            items: ZeroVec::Owned(input.items.into_iter().map(|i| i.as_unaligned()).collect()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::{reference, runtime};

    #[test]
    fn test_runtime_generic_pattern_combine() {
        let pattern = reference::GenericPattern::from_bytes("{0} 'at' {1}")
            .expect("Failed to parse a generic pattern.");
        let pattern: runtime::GenericPattern = pattern.into();

        let date =
            reference::Pattern::from_bytes("Y/m/d").expect("Failed to parse a date pattern.");
        let date: runtime::Pattern = date.into();

        let time =
            reference::Pattern::from_bytes("HH:mm").expect("Failed to parse a time pattern.");
        let time: runtime::Pattern = time.into();

        let pattern = pattern
            .combined(vec![date, time])
            .expect("Failed to combine date and time.");
        let pattern = reference::Pattern::from(pattern.items.to_vec());

        assert_eq!(pattern.to_string(), "Y/m/d 'at' HH:mm");
    }
}
