// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Code for the [`DoublePlaceholder`] pattern backend.

use core::{cmp::Ordering, str::FromStr};
use writeable::Writeable;

use crate::common::*;
use crate::Error;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// A two-value enum for the [`DoublePlaceholder`] pattern backend.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
/// use icu_pattern::PatternItem;
/// use icu_pattern::DoublePlaceholderKey;
/// use icu_pattern::DoublePlaceholderPattern;
///
/// // Parse the string syntax and check the resulting data store:
/// let pattern =
///     DoublePlaceholderPattern::try_from_str("Hello, {0} and {1}!").unwrap();
///
/// assert_eq!(
///     pattern.iter().cmp(
///         [
///             PatternItem::Literal("Hello, "),
///             PatternItem::Placeholder(DoublePlaceholderKey::Place0),
///             PatternItem::Literal(" and "),
///             PatternItem::Placeholder(DoublePlaceholderKey::Place1),
///             PatternItem::Literal("!")
///         ]
///         .into_iter()
///     ),
///     Ordering::Equal
/// );
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[allow(clippy::exhaustive_enums)] // Defined to have two entries
pub enum DoublePlaceholderKey {
    /// The placeholder `{0}`.
    Place0,
    /// The placeholder `{1}`.
    Place1,
}

impl FromStr for DoublePlaceholderKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Place0),
            "1" => Ok(Self::Place1),
            _ => Err(Error::InvalidPlaceholder),
        }
    }
}

impl<W> PlaceholderValueProvider<DoublePlaceholderKey> for (W, W)
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    #[inline]
    fn value_for(&self, key: DoublePlaceholderKey) -> Self::W<'_> {
        match key {
            DoublePlaceholderKey::Place0 => &self.0,
            DoublePlaceholderKey::Place1 => &self.1,
        }
    }
}

impl<W> PlaceholderValueProvider<DoublePlaceholderKey> for [W; 2]
where
    W: Writeable,
{
    type W<'a> = &'a W where W: 'a;
    #[inline]
    fn value_for(&self, key: DoublePlaceholderKey) -> Self::W<'_> {
        let [item0, item1] = self;
        (item0, item1).value_for(key)
    }
}

/// Internal representation of a placeholder
#[derive(Debug, Copy, Clone)]
struct DoublePlaceholderInfo {
    /// The placeholder key: 0 or 1
    pub key: DoublePlaceholderKey,
    /// An offset field. This can take one of two forms:
    /// - Encoded form: 1 + offset from start of literals
    /// - Decoded form: offset from start of store
    pub offset: usize,
}

impl DoublePlaceholderInfo {
    pub fn from_char(ch: char) -> Self {
        Self {
            key: if ((ch as usize) & 0x1) == 0 {
                DoublePlaceholderKey::Place0
            } else {
                DoublePlaceholderKey::Place1
            },
            offset: (ch as usize) >> 1,
        }
    }
    #[cfg(feature = "alloc")]
    pub fn try_to_char(self) -> Result<char, Error> {
        let usize_val = match self.key {
            DoublePlaceholderKey::Place0 => 0,
            DoublePlaceholderKey::Place1 => 1,
        } | (self.offset << 1);
        u32::try_from(usize_val)
            .ok()
            .and_then(|x| char::try_from(x).ok())
            .ok_or(Error::InvalidPattern)
    }
    /// Creates a PlaceholderInfo for an empty Place0
    pub fn zero() -> Self {
        Self {
            key: DoublePlaceholderKey::Place0,
            offset: 0,
        }
    }
    /// Changes Place0 to Place1 and vice-versa
    pub fn flip(self) -> Self {
        Self {
            key: match self.key {
                DoublePlaceholderKey::Place0 => DoublePlaceholderKey::Place1,
                DoublePlaceholderKey::Place1 => DoublePlaceholderKey::Place0,
            },
            offset: self.offset,
        }
    }
    /// Sets the offset to 0 (ignored placeholder), retaining the key
    #[cfg(feature = "alloc")]
    pub fn clear(self) -> Self {
        Self {
            key: self.key,
            offset: 0,
        }
    }
}

/// Backend for patterns containing zero, one, or two placeholders.
///
/// This empty type is not constructible.
///
/// # Placeholder Keys
///
/// The placeholder is either [`DoublePlaceholderKey::Place0`] or [`DoublePlaceholderKey::Place1`].
///
/// In [`Pattern::interpolate()`], pass a two-element array or tuple.
///
/// # Encoding Details
///
/// The first two code points contain the indices of the first and second placeholders with
/// the following encoding:
///
/// 1. First bit: 0 for [`DoublePlaceholderKey::Place0`], 1 for [`DoublePlaceholderKey::Place1`].
/// 2. Second and higher bits: 1 plus the byte offset of the placeholder counting from after
///    the placeholder index code points. If zero, skip this placeholder.
///
/// # Examples
///
/// Parsing a pattern into the encoding:
///
/// ```
/// use icu_pattern::Pattern;
/// use icu_pattern::DoublePlaceholder;
///
/// // Parse the string syntax and check the resulting data store:
/// let store = Pattern::<DoublePlaceholder, _>::try_from_str("Hello, {0} and {1}!")
///     .unwrap()
///     .take_store();
///
/// assert_eq!("\x10\x1BHello,  and !", store);
/// ```
///
/// Explanation of the lead code points:
///
/// 1. `\x10` is placeholder 0 at index 7: ((7 + 1) << 1) | 0
/// 2. `\x1B` is placeholder 1 at index 12: ((12 + 1) << 1) | 1
///
/// Example patterns supported by this backend:
///
/// ```
/// use icu_pattern::Pattern;
/// use icu_pattern::DoublePlaceholder;
///
/// // Single numeric placeholder (note, "5" is used):
/// assert_eq!(
///     Pattern::<DoublePlaceholder, _>::try_from_str("{0} days ago")
///         .unwrap()
///         .interpolate_to_string([5, 7]),
///     "5 days ago",
/// );
///
/// // No placeholder (note, the placeholder value is never accessed):
/// assert_eq!(
///     Pattern::<DoublePlaceholder, _>::try_from_str("yesterday")
///         .unwrap()
///         .interpolate_to_string(["foo", "bar"]),
///     "yesterday",
/// );
///
/// // Escaped placeholder and a placeholder value 1 (note, "bar" is used):
/// assert_eq!(
///     Pattern::<DoublePlaceholder, _>::try_from_str("'{0}' {1}")
///         .unwrap()
///         .interpolate_to_string(("foo", "bar")),
///     "{0} bar",
/// );
///
/// // Pattern with the placeholders in the opposite order:
/// assert_eq!(
///     Pattern::<DoublePlaceholder, _>::try_from_str("A {1} B {0} C")
///         .unwrap()
///         .interpolate_to_string(("D", "E")),
///     "A E B D C",
/// );
///
/// // No literals, only placeholders:
/// assert_eq!(
///     Pattern::<DoublePlaceholder, _>::try_from_str("{1}{0}")
///         .unwrap()
///         .interpolate_to_string((1, 2)),
///     "21",
/// );
/// ```
///
/// [`Pattern::interpolate()`]: crate::Pattern::interpolate
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)] // Empty Enum
pub enum DoublePlaceholder {}

impl crate::private::Sealed for DoublePlaceholder {}

impl PatternBackend for DoublePlaceholder {
    type PlaceholderKey = DoublePlaceholderKey;
    type Store = str;
    type Iter<'a> = DoublePlaceholderPatternIterator<'a>;

    fn validate_store(store: &Self::Store) -> Result<(), Error> {
        let mut chars = store.chars();
        let ph_first_char = chars.next().ok_or(Error::InvalidPattern)?;
        let ph_second_char = chars.next().ok_or(Error::InvalidPattern)?;
        let initial_offset = ph_first_char.len_utf8() + ph_second_char.len_utf8();
        let ph_first = DoublePlaceholderInfo::from_char(ph_first_char);
        let ph_second = DoublePlaceholderInfo::from_char(ph_second_char);
        if ph_first.key == ph_second.key {
            debug_assert!(false, "ph_first.key == ph_second.key");
            return Err(Error::InvalidPattern);
        }
        if ph_first.offset > ph_second.offset && ph_second.offset > 0 {
            debug_assert!(false, "ph_first.offset > ph_second.offset");
            return Err(Error::InvalidPattern);
        }
        if ph_second.offset > store.len() - initial_offset + 1 {
            debug_assert!(false, "ph_second.offset out of range");
            return Err(Error::InvalidPattern);
        }
        if (ph_second_char as usize) >= 0xD800 {
            debug_assert!(false, "ph_second too big for char");
            return Err(Error::InvalidPattern);
        }
        Ok(())
    }

    fn iter_items(store: &Self::Store) -> Self::Iter<'_> {
        let mut chars = store.chars();
        let (mut ph_first, ph_first_len) = match chars.next() {
            Some(ch) => (DoublePlaceholderInfo::from_char(ch), ch.len_utf8()),
            None => {
                debug_assert!(false);
                (DoublePlaceholderInfo::zero(), 0)
            }
        };
        let (mut ph_second, ph_second_len) = match chars.next() {
            Some(ch) => (DoublePlaceholderInfo::from_char(ch), ch.len_utf8()),
            None => {
                debug_assert!(false);
                (ph_first.flip(), ph_first_len)
            }
        };
        let initial_offset = ph_first_len + ph_second_len;
        ph_first.offset += initial_offset - 1;
        ph_second.offset += initial_offset - 1;
        DoublePlaceholderPatternIterator {
            store,
            ph_first,
            ph_second,
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
        let mut first_ph = None;
        let mut second_ph = None;
        for item in items {
            match item? {
                PatternItemCow::Literal(s) => result.push_str(&s),
                PatternItemCow::Placeholder(ph_key) => {
                    if second_ph.is_some() {
                        return Err(Error::InvalidPattern);
                    }
                    let placeholder_offset = result.len() + 1;
                    if placeholder_offset >= 0xD800 {
                        return Err(Error::InvalidPattern);
                    }
                    let ph_info = DoublePlaceholderInfo {
                        key: ph_key,
                        offset: placeholder_offset,
                    };
                    if first_ph.is_none() {
                        first_ph.replace(ph_info);
                    } else {
                        second_ph.replace(ph_info);
                    }
                }
            }
        }
        let (first_ph, second_ph) = match (first_ph, second_ph) {
            (Some(a), Some(b)) => (a, b),
            (Some(a), None) => (a, a.flip().clear()),
            (None, None) => (
                DoublePlaceholderInfo::zero(),
                DoublePlaceholderInfo::zero().flip(),
            ),
            (None, Some(_)) => unreachable!("first_ph always populated before second_ph"),
        };
        if first_ph.key == second_ph.key {
            return Err(Error::InvalidPattern);
        }

        result.insert(0, second_ph.try_to_char()?);
        result.insert(0, first_ph.try_to_char()?);

        Ok(result)
    }
}

#[doc(hidden)] // TODO(#4467): Should be internal
#[derive(Debug)]
pub struct DoublePlaceholderPatternIterator<'a> {
    store: &'a str,
    ph_first: DoublePlaceholderInfo,
    ph_second: DoublePlaceholderInfo,
    current_offset: usize,
}

// Note: This impl is not exported via public bounds, but it might be in the
// future, and the compiler might be able to find it. The code is also
// reachable from `Iterator::size_hint`.
impl ExactSizeIterator for DoublePlaceholderPatternIterator<'_> {
    fn len(&self) -> usize {
        let mut chars = self.store.chars();
        let (mut ph_first, ph_first_len) = match chars.next() {
            Some(ch) => (DoublePlaceholderInfo::from_char(ch), ch.len_utf8()),
            None => {
                debug_assert!(false);
                (DoublePlaceholderInfo::zero(), 0)
            }
        };
        let (mut ph_second, ph_second_len) = match chars.next() {
            Some(ch) => (DoublePlaceholderInfo::from_char(ch), ch.len_utf8()),
            None => {
                debug_assert!(false);
                (ph_first.flip(), ph_first_len)
            }
        };
        let initial_offset = ph_first_len + ph_second_len;
        ph_first.offset += initial_offset - 1;
        ph_second.offset += initial_offset - 1;
        let store_len = self.store.len();

        #[allow(clippy::comparison_chain)]
        if ph_first.offset < initial_offset {
            // No placeholders
            if initial_offset < store_len {
                // No placeholder, non-empty literal
                1
            } else {
                // No placeholder, empty literal
                0
            }
        } else if ph_first.offset == initial_offset {
            // At least 1 placeholder, empty prefix
            if ph_second.offset < initial_offset {
                // Single placeholder
                if ph_first.offset < store_len {
                    // Single placeholder, empty prefix, non-empty suffix
                    2
                } else {
                    // Single placeholder, empty prefix, empty suffix
                    1
                }
            } else if ph_second.offset == ph_first.offset {
                // Two placeholders, empty infix
                if ph_first.offset < store_len {
                    // Two placeholders, empty prefix, empty infix, non-empty suffix
                    3
                } else {
                    // Two placeholders, empty prefix, empty infix, empty suffix
                    2
                }
            } else if ph_second.offset < store_len {
                // Two placeholders, empty prefix, non-empty infix, non-empty suffix
                4
            } else {
                // Two placeholders, empty prefix, non-empty infix, empty suffix
                3
            }
        } else {
            // At least 1 placeholder, non-empty prefix
            if ph_second.offset < initial_offset {
                // Single placeholder
                if ph_first.offset < store_len {
                    // Single placeholder, non-empty prefix, non-empty suffix
                    3
                } else {
                    // Single placeholder, non-empty prefix, empty suffix
                    2
                }
            } else if ph_second.offset == ph_first.offset {
                // Two placeholders, empty infix
                if ph_first.offset < store_len {
                    // Two placeholders, non-empty prefix, empty infix, non-empty suffix
                    4
                } else {
                    // Two placeholders, non-empty prefix, empty infix, empty suffix
                    3
                }
            } else if ph_second.offset < store_len {
                // Two placeholders, non-empty prefix, non-empty infix, non-empty suffix
                5
            } else {
                // Two placeholders, non-empty prefix, non-empty infix, empty suffix
                4
            }
        }
    }
}

impl<'a> Iterator for DoublePlaceholderPatternIterator<'a> {
    type Item = PatternItem<'a, DoublePlaceholderKey>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_offset.cmp(&self.ph_first.offset) {
            Ordering::Less => {
                // Prefix
                let literal_str = match self.store.get(self.current_offset..self.ph_first.offset) {
                    Some(s) => s,
                    None => {
                        debug_assert!(false, "offsets are in range");
                        ""
                    }
                };
                self.current_offset = self.ph_first.offset;
                Some(PatternItem::Literal(literal_str))
            }
            Ordering::Equal => {
                // Placeholder
                self.ph_first.offset = 0;
                Some(PatternItem::Placeholder(self.ph_first.key))
            }
            Ordering::Greater => match self.current_offset.cmp(&self.ph_second.offset) {
                Ordering::Less => {
                    // Infix
                    let literal_str =
                        match self.store.get(self.current_offset..self.ph_second.offset) {
                            Some(s) => s,
                            None => {
                                debug_assert!(false, "offsets are in range");
                                ""
                            }
                        };
                    self.current_offset = self.ph_second.offset;
                    Some(PatternItem::Literal(literal_str))
                }
                Ordering::Equal => {
                    // Placeholder
                    self.ph_second.offset = 0;
                    Some(PatternItem::Placeholder(self.ph_second.key))
                }
                Ordering::Greater => {
                    // Suffix or end of string
                    let literal_str = match self.store.get(self.current_offset..) {
                        Some(s) => s,
                        None => {
                            debug_assert!(false, "offsets are in range");
                            ""
                        }
                    };
                    if literal_str.is_empty() {
                        // End of string
                        None
                    } else {
                        // Suffix
                        self.current_offset = self.store.len();
                        Some(PatternItem::Literal(literal_str))
                    }
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

// TODO(#1668):  Add more tests
