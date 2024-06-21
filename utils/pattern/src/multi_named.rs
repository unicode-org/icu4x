// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Code for the [`MultiNamedPlaceholder`] pattern backend.

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, collections::BTreeMap, str::FromStr, string::String};
use core::fmt;
use core::str::Utf8Error;
#[cfg(feature = "litemap")]
use litemap::LiteMap;
use writeable::Writeable;

use crate::common::*;
use crate::Error;

/// A string wrapper for the [`MultiNamedPlaceholder`] pattern backend.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use core::str::FromStr;
/// use icu_pattern::MultiNamedPlaceholderKey;
/// use icu_pattern::MultiNamedPlaceholderPattern;
/// use icu_pattern::PatternItem;
///
/// // Parse the string syntax and check the resulting data store:
/// let pattern = MultiNamedPlaceholderPattern::from_str(
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
#[repr(transparent)]
#[allow(clippy::exhaustive_structs)] // transparent newtype
pub struct MultiNamedPlaceholderKey<'a>(pub &'a str);

/// Cowable version of [`MultiNamedPlaceholderKey`], used during construction.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
#[allow(clippy::exhaustive_structs)] // transparent newtype
#[cfg(feature = "alloc")]
pub struct MultiNamedPlaceholderKeyCow<'a>(pub Cow<'a, str>);

#[cfg(feature = "alloc")]
impl<'a> FromStr for MultiNamedPlaceholderKeyCow<'a> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Can't borrow the str here unfortunately
        Ok(MultiNamedPlaceholderKeyCow(Cow::Owned(String::from(s))))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MissingNamedPlaceholderError<'a> {
    pub name: &'a str,
}

impl<'a> Writeable for MissingNamedPlaceholderError<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_char('{')?;
        sink.write_str(self.name)?;
        sink.write_char('}')?;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<'k, K, W> PlaceholderValueProvider<MultiNamedPlaceholderKey<'k>> for BTreeMap<K, W>
where
    K: Ord + core::borrow::Borrow<str>,
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
        let writeable = match self.get(key.0) {
            Some(value) => Ok(value),
            None => Err(MissingNamedPlaceholderError { name: key.0 }),
        };
        (writeable, crate::PATTERN_PLACEHOLDER_PART)
    }
}

#[cfg(feature = "litemap")]
impl<'k, K, W, S> PlaceholderValueProvider<MultiNamedPlaceholderKey<'k>> for LiteMap<K, W, S>
where
    K: Ord + core::borrow::Borrow<str>,
    W: Writeable,
    S: litemap::store::Store<K, W>,
{
    type Error = MissingNamedPlaceholderError<'k>;
    type W<'a> = Result<&'a W, Self::Error> where W: 'a, Self: 'a;
    const LITERAL_PART: writeable::Part = crate::PATTERN_LITERAL_PART;
    #[inline]
    fn value_for<'a>(
        &'a self,
        key: MultiNamedPlaceholderKey<'k>,
    ) -> (Self::W<'a>, writeable::Part) {
        let writeable = match self.get(key.0) {
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
/// For example, consider the pattern: "Hello, {user} and {someone_else}!"
///
/// The encoding for this would be:
///
/// ```txt
/// Hello, \x00\x04user and \x01\x04someone_else!
/// ```
///
/// where `\x00\x04` and `\x01\x04` are a big-endian octal number representing the lengths of
/// their respective placeholder names.
///
/// Consequences of this encoding:
///
/// 1. The maximum placeholder name length is 64 bytes
/// 2. Code points in the range `\x00` through `\x07` are reserved for the placeholder name
///
/// # Examples
///
/// Parsing and comparing the pattern store:
///
/// ```
/// use core::str::FromStr;
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
///
/// // Parse the string syntax and check the resulting data store:
/// let store = Pattern::<MultiNamedPlaceholder, _>::from_str(
///     "Hello, {user} and {someone_else}!",
/// )
/// .unwrap()
/// .take_store();
///
/// assert_eq!("Hello, \x00\x04user and \x01\x04someone_else!", store);
/// ```
///
/// Example patterns supported by this backend:
///
/// ```
/// use core::str::FromStr;
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
/// use std::collections::BTreeMap;
///
/// let placeholder_value_map: BTreeMap<&str, &str> = [
///     ("num", "5"),
///     ("letter", "X"),
///     ("", "empty"),
///     ("unused", "unused"),
/// ]
/// .into_iter()
/// .collect();
///
/// // Single placeholder:
/// assert_eq!(
///     Pattern::<MultiNamedPlaceholder, _>::from_str("{num} days ago")
///         .unwrap()
///         .try_interpolate_to_string(&placeholder_value_map)
///         .unwrap(),
///     "5 days ago",
/// );
///
/// // No placeholder (note, the placeholder value is never accessed):
/// assert_eq!(
///     Pattern::<MultiNamedPlaceholder, _>::from_str("yesterday")
///         .unwrap()
///         .try_interpolate_to_string(&placeholder_value_map)
///         .unwrap(),
///     "yesterday",
/// );
///
/// // No literals, only placeholders:
/// assert_eq!(
///     Pattern::<MultiNamedPlaceholder, _>::from_str("{letter}{num}{}")
///         .unwrap()
///         .try_interpolate_to_string(&placeholder_value_map)
///         .unwrap(),
///     "X5empty",
/// );
/// ```
///
/// Use [`LiteMap`] for alloc-free formatting:
///
/// ```
/// use core::str::FromStr;
/// use icu_pattern::MultiNamedPlaceholderPattern;
/// use litemap::LiteMap;
/// use writeable::TryWriteable;
///
/// static placeholder_value_map: LiteMap<&str, usize, &[(&str, usize)]> =
///     LiteMap::from_sorted_store_unchecked(&[("seven", 11)]);
///
/// // Note: String allocates, but this could be a non-allocating sink
/// let mut sink = String::new();
///
/// MultiNamedPlaceholderPattern::from_str("{seven}")
///     .unwrap()
///     .try_interpolate(&placeholder_value_map)
///     .try_write_to(&mut sink)
///     .unwrap()
///     .unwrap();
///
/// assert_eq!(sink, "11");
/// ```
///
/// Missing placeholder values cause an error result to be returned. However,
/// based on the design of [`TryWriteable`], the error can be discarded to get
/// a best-effort interpolation with potential replacement characters.
///
/// ```should_panic
/// use core::str::FromStr;
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
/// use std::collections::BTreeMap;
///
/// let placeholder_value_map: BTreeMap<&str, &str> =
///     [("num", "5"), ("letter", "X")].into_iter().collect();
///
/// Pattern::<MultiNamedPlaceholder, _>::from_str("Your name is {your_name}")
///     .unwrap()
///     .try_interpolate_to_string(&placeholder_value_map)
///     .unwrap();
/// ```
///
/// Recover the best-effort lossy string by directly using [`Pattern::try_interpolate()`]:
///
/// ```
/// use core::str::FromStr;
/// use icu_pattern::MissingNamedPlaceholderError;
/// use icu_pattern::MultiNamedPlaceholder;
/// use icu_pattern::Pattern;
/// use std::borrow::Cow;
/// use std::collections::BTreeMap;
/// use writeable::TryWriteable;
///
/// let placeholder_value_map: BTreeMap<&str, &str> =
///     [("num", "5"), ("letter", "X")].into_iter().collect();
///
/// let pattern = Pattern::<MultiNamedPlaceholder, _>::from_str(
///     "Your name is {your_name}",
/// )
/// .unwrap();
///
/// let mut buffer = String::new();
/// let result = pattern
///     .try_interpolate(&placeholder_value_map)
///     .try_write_to(&mut buffer)
///     .expect("infallible write to String");
///
/// assert!(matches!(result, Err(MissingNamedPlaceholderError { .. })));
/// assert_eq!(result.unwrap_err().name, "your_name");
/// assert_eq!(buffer, "Your name is {your_name}");
/// ```
///
/// [`Pattern::interpolate()`]: crate::Pattern::interpolate
/// [`Pattern::try_interpolate()`]: crate::Pattern::try_interpolate
/// [`TryWriteable`]: writeable::TryWriteable
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)] // Empty Enum
pub enum MultiNamedPlaceholder {}

impl crate::private::Sealed for MultiNamedPlaceholder {}

impl PatternBackend for MultiNamedPlaceholder {
    type PlaceholderKey<'a> = MultiNamedPlaceholderKey<'a>;
    #[cfg(feature = "alloc")]
    type PlaceholderKeyCow<'a> = MultiNamedPlaceholderKeyCow<'a>;
    type Error<'a> = MissingNamedPlaceholderError<'a>;
    type StoreFromBytesError = Utf8Error;
    type Store = str;
    type Iter<'a> = MultiNamedPlaceholderPatternIterator<'a>;

    #[inline]
    fn try_store_from_utf8(bytes: &[u8]) -> Result<&Self::Store, Self::StoreFromBytesError> {
        core::str::from_utf8(bytes)
    }

    fn validate_store(store: &Self::Store) -> Result<(), Error> {
        let mut iter = MultiNamedPlaceholderPatternIterator::new(store);
        while iter
            .try_next()
            .map_err(|e| match e {
                MultiNamedPlaceholderError::InvalidStore => Error::InvalidPattern,
                MultiNamedPlaceholderError::Unreachable => {
                    debug_assert!(false, "unreachable");
                    Error::InvalidPattern
                }
            })?
            .is_some()
        {}
        Ok(())
    }

    fn iter_items(store: &Self::Store) -> Self::Iter<'_> {
        MultiNamedPlaceholderPatternIterator::new(store)
    }

    #[cfg(feature = "alloc")]
    fn try_from_items<
        'cow,
        'ph,
        I: Iterator<Item = Result<PatternItemCow<'cow, Self::PlaceholderKeyCow<'ph>>, Error>>,
    >(
        items: I,
    ) -> Result<String, Error> {
        let mut string = String::new();
        for item in items {
            match item? {
                PatternItemCow::Literal(s) if s.contains(|x| (x as usize) <= 0x07) => {
                    // TODO: Should this be a different error type?
                    return Err(Error::InvalidPattern);
                }
                PatternItemCow::Literal(s) => string.push_str(&s),
                PatternItemCow::Placeholder(ph_key) => {
                    let name_length = ph_key.0.len();
                    if name_length >= 64 {
                        return Err(Error::InvalidPlaceholder);
                    }
                    let lead = (name_length >> 3) as u8;
                    let trail = (name_length & 0x7) as u8;
                    string.push(char::from(lead));
                    string.push(char::from(trail));
                    string.push_str(&ph_key.0);
                }
            }
        }
        Ok(string)
    }
}

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
                    placeholder_name,
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
            None if self.store.is_empty() => {
                // End of string
                Ok(None)
            }
            None => {
                // Closing literal
                let literal = self.store;
                self.store = "";
                Ok(Some(PatternItem::Literal(literal)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MultiNamedPlaceholderPattern;
    use core::str::FromStr;

    #[test]
    fn test_invalid() {
        let long_str = "0123456789".repeat(1000000);
        let strings = [
            "{",    // invalid syntax
            "{@}",  // placeholder name too long
            "\x00", // invalid character
            "\x07", // invalid character
        ];
        for string in strings {
            let string = string.replace('@', &long_str);
            assert!(
                MultiNamedPlaceholderPattern::from_str(&string).is_err(),
                "{string:?}"
            );
        }
        let stores = [
            "\x00",      // too short
            "\x02",      // too short
            "\x00\x02",  // no placeholder name
            "\x00\x02a", // placeholder name too short
        ];
        for store in stores {
            let store = store.replace('@', &long_str);
            assert!(
                MultiNamedPlaceholderPattern::try_from_store(&store).is_err(),
                "{store:?}"
            );
        }
    }
}
