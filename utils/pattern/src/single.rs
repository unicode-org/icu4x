// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use core::cmp::Ordering;

use super::{PatternBackend, PatternError, PatternItem, PatternItemCow};

pub struct SinglePlaceholderPatternBackend {
    _not_constructible: core::convert::Infallible,
}

impl PatternBackend for SinglePlaceholderPatternBackend {
    type PlaceholderKey = ();
    type Store = str;
    type Iter<'a> = SinglePlaceholderPatternIterator<'a>;

    fn validate_store(store: &Self::Store) -> Result<(), PatternError> {
        let placeholder_offset_char = store.chars().next().ok_or(PatternError::InvalidPattern)?;
        let initial_offset = char::len_utf8(placeholder_offset_char);
        let pattern_len = store.len() - initial_offset;
        if placeholder_offset_char as usize > pattern_len {
            return Err(PatternError::InvalidPattern);
        }
        Ok(())
    }

    fn iter_items<'a>(store: &'a Self::Store) -> Self::Iter<'a> {
        let placeholder_offset_char = match store.chars().next() {
            Some(i) => i,
            None => {
                debug_assert!(false);
                '\0'
            }
        };
        let initial_offset = char::len_utf8(placeholder_offset_char);
        SinglePlaceholderPatternIterator {
            store,
            placeholder_offset: (placeholder_offset_char as usize) + initial_offset,
            current_offset: initial_offset,
        }
    }

    fn try_from_items<'a, I: Iterator<Item = PatternItemCow<'a, Self::PlaceholderKey>>>(
        items: I,
    ) -> Result<Cow<'a, Self::Store>, PatternError> {
        let mut result = String::new();
        let mut seen_placeholder = false;
        for item in items {
            match item {
                PatternItemCow::Literal(s) => result.push_str(&s),
                PatternItemCow::Placeholder(()) if !seen_placeholder => {
                    seen_placeholder = true;
                    let placeholder_offset =
                        u32::try_from(result.len()).map_err(|_| PatternError::InvalidPattern)?;
                    let placeholder_offset_char = char::try_from(placeholder_offset)
                        .map_err(|_| PatternError::InvalidPattern)?;
                    result.insert(0, placeholder_offset_char);
                }
                PatternItemCow::Placeholder(()) => {
                    return Err(PatternError::InvalidPattern);
                }
            }
        }
        if !seen_placeholder {
            Err(PatternError::InvalidPattern)
        } else {
            Ok(Cow::Owned(result))
        }
    }
}

// should be hidden
pub struct SinglePlaceholderPatternIterator<'a> {
    store: &'a str,
    placeholder_offset: usize,
    current_offset: usize,
}

impl<'a> Iterator for SinglePlaceholderPatternIterator<'a> {
    type Item = PatternItem<'a, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_offset.cmp(&self.placeholder_offset) {
            Ordering::Less => {
                // Prefix
                let result = self
                    .store
                    .get(self.current_offset..self.placeholder_offset)
                    .map(PatternItem::Literal);
                self.current_offset = self.placeholder_offset;
                result
            }
            Ordering::Equal => {
                // Placeholder
                self.placeholder_offset = 0;
                Some(PatternItem::Placeholder(()))
            }
            Ordering::Greater => {
                // Suffix or end of string
                let result = self
                    .store
                    .get(self.current_offset..)
                    .map(PatternItem::Literal);
                self.current_offset = self.store.len();
                result
            }
        }
    }
}
