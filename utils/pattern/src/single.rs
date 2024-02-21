// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{cmp::Ordering, str::FromStr};
use writeable::Writeable;

use crate::common::*;
use crate::Error;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use icu_pattern::PatternItem;
/// use icu_pattern::SinglePlaceholder;
/// use icu_pattern::SinglePlaceholderKey;
/// use icu_pattern::SinglePlaceholderPattern;
///
/// // Parse the string syntax and check the resulting data store:
/// let pattern =
///     SinglePlaceholderPattern::try_from_str("Hello, {0}!").unwrap();
///
/// assert_eq!(
///     pattern.iter().cmp(
///         [
///             PatternItem::Literal("Hello, "),
///             PatternItem::Placeholder(SinglePlaceholderKey::Singleton),
///             PatternItem::Literal("!")
///         ]
///         .into_iter()
///     ),
///     Ordering::Equal
/// );
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_enums)] // Singleton
pub enum SinglePlaceholderKey {
    Singleton,
}

impl FromStr for SinglePlaceholderKey {
    type Err = core::convert::Infallible;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self::Singleton)
    }
}

impl<W> PlaceholderValueProvider<SinglePlaceholderKey> for (W,)
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    fn value_for(&self, _key: SinglePlaceholderKey) -> Self::W<'_> {
        &self.0
    }
}

impl<W> PlaceholderValueProvider<SinglePlaceholderKey> for [W; 1]
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    fn value_for(&self, _key: SinglePlaceholderKey) -> Self::W<'_> {
        let [value] = self;
        value
    }
}

/// For patterns containing zero or one placeholder.
///
/// # Encoding Details
///
/// The first code point of the string is 1 plus the byte offset of the placeholder counting from
/// after that initial code point. If zero, there is no placeholder.
///
/// # Examples
///
/// Parsing a pattern into the encoding:
///
/// ```
/// use icu_pattern::Pattern;
/// use icu_pattern::SinglePlaceholder;
///
/// // Parse the string syntax and check the resulting data store:
/// let store = Pattern::<SinglePlaceholder, _>::try_from_str("Hello, {0}!")
///     .unwrap()
///     .take_store();
///
/// assert_eq!("\u{8}Hello, !", store);
/// ```
///
/// Example patterns supported by this backend:
///
/// ```
/// use icu_pattern::Pattern;
/// use icu_pattern::SinglePlaceholder;
///
/// // Single numeric placeholder:
/// assert_eq!(
///     Pattern::<SinglePlaceholder, _>::try_from_str("{0} days ago")
///         .unwrap()
///         .interpolate_to_string([5]),
///     "5 days ago",
/// );
///
/// // Single named placeholder:
/// assert_eq!(
///     Pattern::<SinglePlaceholder, _>::try_from_str("{name}")
///         .unwrap()
///         .interpolate_to_string(["Alice"]),
///     "Alice",
/// );
///
/// // No placeholder:
/// assert_eq!(
///     Pattern::<SinglePlaceholder, _>::try_from_str("yesterday")
///         .unwrap()
///         .interpolate_to_string([0]),
///     "yesterday",
/// );
///
/// // Escaped placeholder and a real placeholder:
/// assert_eq!(
///     Pattern::<SinglePlaceholder, _>::try_from_str(
///         "'{escaped}' {interpolated}"
///     )
///     .unwrap()
///     .interpolate_to_string(["hi"]),
///     "{escaped} hi",
/// );
/// ```
#[derive(Debug)]
pub struct SinglePlaceholder {
    _not_constructible: core::convert::Infallible,
}

impl crate::Sealed for SinglePlaceholder {}

impl PatternBackend for SinglePlaceholder {
    type PlaceholderKey = SinglePlaceholderKey;
    type Store = str;
    type Iter<'a> = SinglePlaceholderPatternIterator<'a>;

    fn validate_store(store: &Self::Store) -> Result<(), Error> {
        let placeholder_offset_char = store.chars().next().ok_or(Error::InvalidPattern)?;
        let initial_offset = char::len_utf8(placeholder_offset_char);
        let placeholder_offset = placeholder_offset_char as usize;
        if placeholder_offset > store.len() - initial_offset + 1 {
            return Err(Error::InvalidPattern);
        }
        Ok(())
    }

    fn iter_items(store: &Self::Store) -> Self::Iter<'_> {
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
            placeholder_offset: placeholder_offset_char as usize + initial_offset - 1,
            current_offset: initial_offset,
        }
    }

    #[cfg(feature = "alloc")]
    fn try_from_items<
        'a,
        I: Iterator<Item = Result<PatternItemCow<'a, Self::PlaceholderKey>, Error>>,
    >(
        items: I,
    ) -> Result<String, Error> {
        let mut result = String::new();
        let mut seen_placeholder = false;
        for item in items {
            match item? {
                PatternItemCow::Literal(s) => result.push_str(&s),
                PatternItemCow::Placeholder(_) if !seen_placeholder => {
                    seen_placeholder = true;
                    let placeholder_offset =
                        u32::try_from(result.len() + 1).map_err(|_| Error::InvalidPattern)?;
                    let placeholder_offset_char =
                        char::try_from(placeholder_offset).map_err(|_| Error::InvalidPattern)?;
                    result.insert(0, placeholder_offset_char);
                }
                PatternItemCow::Placeholder(_) => {
                    return Err(Error::InvalidPattern);
                }
            }
        }
        if !seen_placeholder {
            result.insert(0, '\0');
        }
        Ok(result)
    }
}

// should be hidden
#[derive(Debug)]
pub struct SinglePlaceholderPatternIterator<'a> {
    store: &'a str,
    placeholder_offset: usize,
    current_offset: usize,
}

impl<'a> Iterator for SinglePlaceholderPatternIterator<'a> {
    type Item = PatternItem<'a, SinglePlaceholderKey>;
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
                Some(PatternItem::Placeholder(SinglePlaceholderKey::Singleton))
            }
            Ordering::Greater => {
                // Suffix or end of string
                let result = self
                    .store
                    .get(self.current_offset..)
                    .and_then(|s| if s.is_empty() { None } else { Some(s) })
                    .map(PatternItem::Literal);
                self.current_offset = self.store.len();
                result
            }
        }
    }
}

// TODO(#1668):  Add more tests
