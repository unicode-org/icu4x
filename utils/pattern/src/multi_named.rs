// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Code for the [`MultiNamedPlaceholder`] pattern backend.

use core::borrow::Borrow;
use core::fmt;
use core::str::FromStr;
use alloc::collections::BTreeMap;
use alloc::borrow::Cow;
use either::Either;
use writeable::Writeable;

use crate::common::*;
use crate::Error;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// A string wrapper for the [`MultiNamedPlaceholder`] pattern backend.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use icu_pattern::PatternItem;
/// use icu_pattern::MultiNamedPlaceholderKey;
/// use icu_pattern::MultiNamedPlaceholderPattern;
///
/// // Parse the string syntax and check the resulting data store:
/// let pattern =
///     MultiNamedPlaceholderPattern::try_from_str("Hello, {person0} and {person1}!").unwrap();
///
/// assert_eq!(
///     pattern.iter().cmp(
///         [
///             PatternItem::Literal("Hello, "),
///             PatternItem::Placeholder(MultiNamedPlaceholderKey("person0".into())),
///             PatternItem::Literal(" and "),
///             PatternItem::Placeholder(MultiNamedPlaceholderKey("person1".into())),
///             PatternItem::Literal("!")
///         ]
///         .into_iter()
///     ),
///     Ordering::Equal
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
#[allow(clippy::exhaustive_enums)] // transparent
pub struct MultiNamedPlaceholderKey<'a>(pub Cow<'a, str>);

impl<'a> FromStr for MultiNamedPlaceholderKey<'a> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Can't borrow the str here unfortunately
        Ok(MultiNamedPlaceholderKey(Cow::Owned(String::from(s))))
    }
}

impl fmt::Display for MultiNamedPlaceholderKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{0}}}", self.0)
    }
}

impl<'k, K, W> PlaceholderValueProvider<MultiNamedPlaceholderKey<'k>> for BTreeMap<K, W>
where
    K: Ord + Borrow<str>,
    W: Writeable,
{
    type W<'a> = Either<&'a W, char> where K: 'a, W: 'a, Self: 'a;
    #[inline]
    fn value_for<'a>(&'a self, key: MultiNamedPlaceholderKey) -> Self::W<'a> {
        match self.get(&*key.0) {
            Some(value) => Either::Left(value),
            None => {
                debug_assert!(false, "Named placeholder '{key}' not found in map");
                Either::Right('\u{FFFD}')
            }
        }
    }
}

/// Backend for patterns containing zero or more named placeholders.
///
/// This empty type is not constructible.
///
/// # Placeholder Keys
///
/// The placeholder is [`MultiNamedPlaceholderKey`].
///
/// In [`Pattern::interpolate()`], pass a map-like structure. Missing keys will be replaced
/// with the Unicode replacement character U+FFFD.
///
/// # Encoding Details
///
/// The string store is identical to the human-readable pattern string.
///
/// # Examples
///
/// The pattern store syntax is identical to the human-readable syntax:
///
/// ```
/// use icu_pattern::Pattern;
/// use icu_pattern::MultiNamedPlaceholder;
///
/// // Parse the string syntax and check the resulting data store:
/// let store = Pattern::<MultiNamedPlaceholder, _>::try_from_str("Hello, {person0} and {person1}!")
///     .unwrap()
///     .take_store();
///
/// assert_eq!("Hello, {person0} and {person1}!", store);
/// ```
///
/// Example patterns supported by this backend:
///
/// ```
/// use icu_pattern::Pattern;
/// use icu_pattern::MultiNamedPlaceholder;
/// use std::collections::BTreeMap;
/// 
/// let placeholder_value_map: BTreeMap<&str, &str> = [
///     ("num", "5"),
///     ("letter", "X"),
/// ].into_iter().collect();
///
/// // Single placeholder:
/// assert_eq!(
///     Pattern::<MultiNamedPlaceholder, _>::try_from_str("{num} days ago")
///         .unwrap()
///         .interpolate_to_string(&placeholder_value_map),
///     "5 days ago",
/// );
///
/// // No placeholder (note, the placeholder value is never accessed):
/// assert_eq!(
///     Pattern::<MultiNamedPlaceholder, _>::try_from_str("yesterday")
///         .unwrap()
///         .interpolate_to_string(&placeholder_value_map),
///     "yesterday",
/// );
///
/// // No literals, only placeholders:
/// assert_eq!(
///     Pattern::<MultiNamedPlaceholder, _>::try_from_str("{letter}{num}")
///         .unwrap()
///         .interpolate_to_string(&placeholder_value_map),
///     "X5",
/// );
/// ```
/// 
/// Missing placeholder values cause a debug assertion or are replaced with
/// the Unicode replacement character U+FFFD.
/// 
/// With `debug_assertions` (debug mode):
/// 
/// ```should_panic
/// use icu_pattern::Pattern;
/// use icu_pattern::MultiNamedPlaceholder;
/// use std::collections::BTreeMap;
/// 
/// let placeholder_value_map: BTreeMap<&str, &str> = [
///     ("num", "5"),
///     ("letter", "X"),
/// ].into_iter().collect();
/// 
///         Pattern::<MultiNamedPlaceholder, _>::try_from_str("Your name is {your_name}")
///            .unwrap()
///             .interpolate_to_string(&placeholder_value_map);
/// ```
/// 
/// Without `debug_assertions` (release mode):
/// 
/// ```
/// use icu_pattern::Pattern;
/// use icu_pattern::MultiNamedPlaceholder;
/// use std::collections::BTreeMap;
/// 
/// let placeholder_value_map: BTreeMap<&str, &str> = [
///     ("num", "5"),
///     ("letter", "X"),
/// ].into_iter().collect();
/// 
/// if cfg!(not(debug_assertions)) {
///     assert_eq!(
///         Pattern::<MultiNamedPlaceholder, _>::try_from_str("Your name is {your_name}")
///            .unwrap()
///             .interpolate_to_string(&placeholder_value_map),
///         "Your name is �",
///     );
/// }
/// ```
///
/// [`Pattern::interpolate()`]: crate::Pattern::interpolate
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)] // Empty Enum
pub enum MultiNamedPlaceholder {}

impl crate::private::Sealed for MultiNamedPlaceholder {}

impl PatternBackend for MultiNamedPlaceholder {
    type PlaceholderKey = MultiNamedPlaceholder;
    type Store = str;
    type Iter<'a> = MultiNamedPlaceholderPatternIterator<'a>;

    fn validate_store(store: &Self::Store) -> Result<(), Error> {
        let mut iter = MultiNamedPlaceholderPatternIterator::new(store);
        while let Some(_) = iter.try_next()? {}
        Ok(())
    }

    fn iter_items(store: &Self::Store) -> Self::Iter<'_> {
        MultiNamedPlaceholderPatternIterator::new(store)
    }

    #[cfg(feature = "alloc")]
    fn try_from_items<
        'a,
        I: Iterator<Item = Result<PatternItemCow<'a, Self::PlaceholderKey>, Error>>,
    >(
        items: I,
    ) -> Result<String, Error> {
        todo!()
    }
}

#[doc(hidden)] // TODO(#4467): Should be internal
#[derive(Debug)]
pub struct MultiNamedPlaceholderPatternIterator<'a> {
    store: &'a str,
    current_offset: usize,
}

// Note: we don't implement ExactSizeIterator since we don't store that metadata in MultiNamed.

impl<'a> Iterator for MultiNamedPlaceholderPatternIterator<'a> {
    type Item = PatternItem<'a, MultiNamedPlaceholder>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> MultiNamedPlaceholderPatternIterator<'a> {
    fn new(store: &'a str) -> Self {
        Self {
            store,
            current_offset: 0,
        }
    }

    fn try_next(&mut self) -> Result<Option<PatternItem<'a, MultiNamedPlaceholder>>, Error> {
        enum State {
            Start,
            Literal,
            AfterQuote,
            QuotedLiteral,
            AfterQuotedLiteral,
            Placeholder,
        }
        let start = self.current_offset;
        let mut chars_iter = self.store.get(start..).unwrap().chars();
        let initial_len = chars_iter.as_str().len();
        let mut state = State::Start;
        for c in chars_iter {
            match (state, c) {
                (State::Start, '{') => {
                    state = State::Placeholder;
                }
                (State::Start | State::Literal, '\'') => {
                    state = State::AfterQuote;
                }
                (State::AfterQuote, '\'') => {
                    todo!()
                },
                _ => todo!()
            }
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MultiNamedPlaceholderPattern;

    #[test]
    fn test_invalid() {
        let cases = [
            "",               // too short
            "\x00",           // too short
            "\x00\x00",       // duplicate placeholders
            "\x04\x03",       // first offset is after second offset
            "\x04\x05",       // second offset out of range (also first offset)
            "\x04\u{10001}@", // second offset too large for UTF-8
        ];
        let long_str = "0123456789".repeat(1000000);
        for cas in cases {
            let cas = cas.replace('@', &long_str);
            assert!(
                MultiNamedPlaceholderPattern::try_from_store(&cas).is_err(),
                "{cas:?}"
            );
        }
    }
}
