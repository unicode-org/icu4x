// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Code for the [`MultiNamedPlaceholder`] pattern backend.

use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use core::borrow::Borrow;
use core::fmt;
use core::str::FromStr;
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
/// use icu_pattern::MultiNamedPlaceholderKey;
/// use icu_pattern::MultiNamedPlaceholderPattern;
/// use icu_pattern::PatternItem;
///
/// // Parse the string syntax and check the resulting data store:
/// let pattern = MultiNamedPlaceholderPattern::try_from_str(
///     "Hello, {person0} and {person1}!",
/// )
/// .unwrap();
///
/// assert_eq!(
///     pattern.iter().cmp(
///         [
///             PatternItem::Literal("Hello, "),
///             PatternItem::Placeholder(MultiNamedPlaceholderKey(
///                 "person0".into()
///             )),
///             PatternItem::Literal(" and "),
///             PatternItem::Placeholder(MultiNamedPlaceholderKey(
///                 "person1".into()
///             )),
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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MissingNamedPlaceholderError<'a> {
    name: Cow<'a, str>,
}

impl<'a> Writeable for MissingNamedPlaceholderError<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_char('{')?;
        sink.write_str(&self.name)?;
        sink.write_char('}')?;
        Ok(())
    }
}

impl<'k, K, W> PlaceholderValueProvider<MultiNamedPlaceholderKey<'k>> for BTreeMap<K, W>
where
    K: Ord + Borrow<str>,
    W: Writeable,
{
    type Error = MissingNamedPlaceholderError<'k>;
    type W<'a> = Result<&'a W, Self::Error> where W: 'a, Self: 'a;
    const LITERAL_PART: writeable::Part = crate::PATTERN_LITERAL_PART;
    #[inline]
    fn value_for<'a>(
        &'a self,
        key: MultiNamedPlaceholderKey<'k>,
    ) -> (Self::W<'a>, writeable::Part) {
        let writeable = match self.get(&*key.0) {
            Some(value) => Ok(value),
            None => Err(MissingNamedPlaceholderError { name: key.0 }),
        };
        (writeable, crate::PATTERN_PLACEHOLDER_PART)
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
/// The literals and placeholders are stored in context. A placeholder is encoded as a name length
/// in octal code points followed by the placeholder name.
///
/// For example, consider the pattern: "Hello, {person0} and {person1}!"
///
/// The encoding for this would be:
///
/// ```txt
/// Hello, \x00\x07person0 and \x00\x07person1!
/// ```
///
/// where `\x00\x07` is a big-endian octal number representing the length of the placeholder name.
///
/// Consequences of this encoding:
///
/// 1. The maximum placeholder name length is 64 bytes
/// 2. Code points in the range `\x00` through `\x07` are reserved for the placeholder name
///
/// # Examples
///
/// The pattern store syntax is identical to the human-readable syntax:
///
/// ```
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
///
/// // Parse the string syntax and check the resulting data store:
/// let store = Pattern::<MultiNamedPlaceholder, _>::try_from_str(
///     "Hello, {person0} and {person1}!",
/// )
/// .unwrap()
/// .take_store();
///
/// assert_eq!("Hello, {person0} and {person1}!", store);
/// ```
///
/// Example patterns supported by this backend:
///
/// ```
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
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
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
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
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
/// use std::collections::BTreeMap;
///
/// let placeholder_value_map: BTreeMap<&str, &str> = [
///     ("num", "5"),
///     ("letter", "X"),
/// ].into_iter().collect();
///
/// if cfg!(not(debug_assertions)) {
///     assert_eq!(
///         Pattern::<MultiNamedPlaceholder, _>::try_from_str(
///             "Your name is {your_name}"
///         )
///         .unwrap()
///         .interpolate_to_string(&placeholder_value_map),
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
    type PlaceholderKey<'a> = MultiNamedPlaceholderKey<'a>;
    type Error<'a> = MissingNamedPlaceholderError<'a>;
    type Store = str;
    type Iter<'a> = MultiNamedPlaceholderPatternIterator<'a>;

    fn validate_store(store: &Self::Store) -> Result<(), Error> {
        let mut iter = MultiNamedPlaceholderPatternIterator::new(store);
        while let Some(_) = iter.try_next().map_err(|_| Error::InvalidPattern)? {}
        Ok(())
    }

    fn iter_items(store: &Self::Store) -> Self::Iter<'_> {
        MultiNamedPlaceholderPatternIterator::new(store)
    }

    #[cfg(feature = "alloc")]
    fn try_from_items<
        'a,
        I: Iterator<Item = Result<PatternItemCow<'a, Self::PlaceholderKey<'a>>, Error>>,
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
}

// Note: we don't implement ExactSizeIterator since we don't store that metadata in MultiNamed.

impl<'a> Iterator for MultiNamedPlaceholderPatternIterator<'a> {
    type Item = PatternItem<'a, MultiNamedPlaceholderKey<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(next) => next,
            Err(MultiNamedPlaceholderError::InvalidStore) => {
                debug_assert!(
                    false,
                    "invalid store with {} bytes remaining",
                    self.store.len()
                );
                None
            }
            Err(MultiNamedPlaceholderError::Unreachable) => {
                debug_assert!(false, "unreachable");
                None
            }
        }
    }
}

enum MultiNamedPlaceholderError {
    InvalidStore,
    Unreachable,
}

impl<'a> MultiNamedPlaceholderPatternIterator<'a> {
    fn new(store: &'a str) -> Self {
        Self { store }
    }

    fn try_next(
        &mut self,
    ) -> Result<Option<PatternItem<'a, MultiNamedPlaceholderKey<'a>>>, MultiNamedPlaceholderError>
    {
        match self.store.find(|x| (x as usize) <= 0x07) {
            Some(0) => {
                // Placeholder
                let Some(&[lead, trail]) = self.store.as_bytes().get(0..2) else {
                    return Err(MultiNamedPlaceholderError::InvalidStore);
                };
                debug_assert!(lead <= 7);
                if trail > 7 {
                    return Err(MultiNamedPlaceholderError::InvalidStore);
                }
                let placeholder_len = (lead << 3) + trail;
                let boundary = (placeholder_len as usize) + 2;
                // TODO: use .split_at_checked() when available:
                // https://github.com/rust-lang/rust/issues/119128
                let Some(placeholder_name) = self.store.get(2..boundary) else {
                    return Err(MultiNamedPlaceholderError::InvalidStore);
                };
                let Some(remainder) = self.store.get(boundary..) else {
                    debug_assert!(false, "should be a perfect slice");
                    return Err(MultiNamedPlaceholderError::Unreachable);
                };
                self.store = remainder;
                Ok(Some(PatternItem::Placeholder(MultiNamedPlaceholderKey(
                    Cow::Borrowed(placeholder_name),
                ))))
            }
            Some(i) => {
                // Literal
                // TODO: use .split_at_checked() when available:
                // https://github.com/rust-lang/rust/issues/119128
                let Some(literal) = self.store.get(..i) else {
                    debug_assert!(false, "should be a perfect slice");
                    return Err(MultiNamedPlaceholderError::Unreachable);
                };
                let Some(remainder) = self.store.get(i..) else {
                    debug_assert!(false, "should be a perfect slice");
                    return Err(MultiNamedPlaceholderError::Unreachable);
                };
                self.store = remainder;
                Ok(Some(PatternItem::Literal(literal)))
            }
            None => {
                // End of string
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MultiNamedPlaceholderPattern;

    #[test]
    fn test_invalid() {
        let cases = [
            "\x00", // too short
            "\x02", // no placeholder name
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
